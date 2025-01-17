use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use ini::ini;

pub mod config{
	use ini::configparser::ini::Ini;
	use super::*;
	
	pub type ConfigValue = ConfigValueEnum;
	pub type ConfigSection = HashMap<String, ConfigValue>;
	pub type Config = HashMap<String, ConfigSection>;
	
	#[derive(Clone, Debug)]
	pub enum ConfigValueEnum{
		String(String),
		Int(i32),
		Bool(bool),
		Float(f64)
	}
	
	impl ConfigValueEnum {
		pub fn from_str(value: &str, default: &ConfigValueEnum) -> ConfigValueEnum {
			match default{
				ConfigValueEnum::Int(_) => value.parse::<i32>().map_or_else(|_| default.clone(), ConfigValueEnum::Int),
				ConfigValueEnum::Bool(_) => {
					let lower = value.to_lowercase();
					ConfigValueEnum::Bool(lower=="1"||lower=="true")
				},
				ConfigValueEnum::Float(_) => value.parse::<f64>().map_or_else(|_| default.clone(), ConfigValueEnum::Float),
				ConfigValueEnum::String(_) => ConfigValueEnum::String(value.to_string()),
			}
		}
	}
	
	const APP_NAME: &str = "funfriend";
	const CFG_NAME: &str = "cfg.ini";
	
	fn default_config() -> Config {
		let mut config = Config::new();
		config.insert(
			"window".to_string(),
			[("funfriend_size".to_string(), ConfigValueEnum::Int(75))]
				.iter()
				.cloned()
				.collect()
		);
		config.insert(
			"sound".to_string(),
			[("volume".to_string(), ConfigValueEnum::Float(0.2))]
				.iter()
				.cloned()
				.collect()
		);
		config.insert(
			"buddies".to_string(),
			[("types".to_string(), ConfigValueEnum::String("funfriend".to_string()))]
				.iter()
				.cloned()
				.collect()
		);
		
		config
	}
	
	fn get_config_path() -> PathBuf {
		if cfg!(windows) {
			PathBuf::from(env::var("APPDATA").unwrap()).join(APP_NAME)
		} else if cfg!(target_os = "macos") {
			panic!("fuck you im not making this work on mac lmaoooooooooooooooooo")
		} else {
			env::var("XDG_CONFIG_HOME")
				.map(PathBuf::from)
				.unwrap_or_else(|_| PathBuf::from(env::var("HOME").unwrap()).join(".config"))
				.join(APP_NAME)
		}
	}
	
	static mut CFG_INITIALIZED: bool = false;
	static mut CONFIG: Option<Config> = None;
	
	pub fn config() -> Config {
		unsafe {
			if !CFG_INITIALIZED {
				panic!("Config read before init.");
			}
			CONFIG.clone().unwrap()
		}
	}
	
	fn parse_ini(content: &str) -> Config {
		let mut config = Config::new();
		let mut current_section = None;
		
		for line in content.lines(){
			let trimmed = line.trim();
			
			if trimmed.starts_with('[') && trimmed.ends_with(']') {
				current_section = Some((&trimmed[1..trimmed.len()-1]).to_string());
			} else if let Some((key, value)) = trimmed.split_once('='){
				if let Some(section) = current_section.clone() {
					let section = config.entry(section).or_insert_with(HashMap::new);
					section.insert(key.trim().to_string(), ConfigValueEnum::String(value.trim().to_string()));
				}
			}
		}
		
		config
	}
	
	fn build_ini(config: &Config) -> String {
		let mut result = String::new();
		
		for (section, properties) in config {
			result.push_str(&format!("[{}]\n", section));
			for (key, value) in properties {
				let value_str = match value {
					ConfigValueEnum::String(s) => s.clone(),
					ConfigValueEnum::Int(i) => i.to_string(),
					ConfigValueEnum::Bool(b) => (if *b {"true"} else {"false"}).to_string(),
					ConfigValueEnum::Float(f) => f.to_string(),
				};
				result.push_str(&format!("{}={}\n", key, value_str));
			}
		}
		result
	}
	
	pub fn init() {
		let config_path = get_config_path();
		if !config_path.exists() {
			fs::create_dir_all(&config_path).expect("Failed to create config dir.");
		}
		
		let config_file = config_path.join(CFG_NAME);
		let mut current_config = default_config();
		
		if config_file.exists() {
			let mut file_content = String::new();
			File::open(&config_file)
				.expect("Failed to open config file.")
				.read_to_string(&mut file_content)
				.expect("Failed to read config file.");
			
			let parsed_config = parse_ini(&file_content);
			
			for (section, properties) in parsed_config {
				if let Some(default_section) = current_config.get_mut(&section){
					for (key, value) in properties {
						if let Some(default_value) = default_section.get(&key){
							default_section.insert(key, ConfigValueEnum::from_str(&value.to_string(), default_value));
						}
					}
				}
			}
			
			let new_content = build_ini(&current_config);
			File::create(&config_file)
				.expect("Failed to create config file.")
				.write_all(new_content.as_bytes())
				.expect("Failed to write config file.");
		}
	}
}