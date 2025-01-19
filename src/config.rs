use std::collections::HashMap;
use std::io::Write as _;
use std::path::PathBuf;

use super::APP_NAME;

pub type ConfigValue = ConfigValueEnum;
pub type ConfigSection = HashMap<String, ConfigValue>;
pub type Config = HashMap<String, ConfigSection>;

const CONFIG_FILENAME: &str = "cfg.ini";

static mut CONFIG: Option<Config> = None;

pub fn get() -> Config {
	unsafe { CONFIG.clone().unwrap() }
}

#[derive(Clone, Debug)]
pub enum ConfigValueEnum {
	String(String),
	Int(i32),
	Bool(bool),
	Float(f64),
}

impl ConfigValueEnum {
	pub fn from_str(value: &str, default: &ConfigValueEnum) -> ConfigValueEnum {
		match default {
			ConfigValueEnum::Int(_) => value
				.parse::<i32>()
				.map_or_else(|_| default.clone(), ConfigValueEnum::Int),
			ConfigValueEnum::Bool(_) => {
				let lower = value.to_lowercase();
				ConfigValueEnum::Bool(lower == "1" || lower == "true")
			}
			ConfigValueEnum::Float(_) => value
				.parse::<f64>()
				.map_or_else(|_| default.clone(), ConfigValueEnum::Float),
			ConfigValueEnum::String(_) => ConfigValueEnum::String(value.to_string()),
		}
	}
}

fn default_config() -> Config {
	let mut config = Config::new();

	config.insert(
		"window".to_string(),
		[("funfriend_size".to_string(), ConfigValueEnum::Int(75))]
			.into_iter()
			.collect(),
	);
	config.insert(
		"sound".to_string(),
		[("volume".to_string(), ConfigValueEnum::Float(0.2))]
			.into_iter()
			.collect(),
	);
	config.insert(
		"buddies".to_string(),
		[(
			"types".to_string(),
			ConfigValueEnum::String("funfriend".to_string()),
		)]
		.into_iter()
		.collect(),
	);

	config
}

fn get_config_dir() -> PathBuf {
	if cfg!(windows) {
		PathBuf::from(std::env::var("APPDATA").unwrap()).join(APP_NAME)
	} else if cfg!(target_os = "macos") {
		panic!("fuck you im not making this work on mac lmaoooooooooooooooooo")
	} else {
		std::env::var("XDG_CONFIG_HOME")
			.map(PathBuf::from)
			.unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap()).join(".config"))
			.join(APP_NAME)
	}
}

fn parse_ini(content: &str) -> Config {
	let mut config = Config::new();
	let mut current_section = None;

	for line in content.lines() {
		let trimmed = line.trim();

		if trimmed.starts_with('[') && trimmed.ends_with(']') {
			current_section = Some((&trimmed[1..trimmed.len() - 1]).to_string());
		} else if let Some((key, value)) = trimmed.split_once('=') {
			if let Some(section) = current_section.clone() {
				let section = config.entry(section).or_insert_with(HashMap::new);
				section.insert(
					key.trim().to_string(),
					ConfigValueEnum::String(value.trim().to_string()),
				);
			}
		}
	}

	config
}

fn build_ini(config: &Config) -> String {
	let mut result = String::new();

	for (section, properties) in config {
		result.push_str(&format!("[{section}]\n"));
		for (key, value) in properties {
			result.push_str(&format!("{key}={value}\n"));
		}
		result.push_str("\n");
	}

	result
}

pub fn init() {
	let config_dir = get_config_dir();
	if !config_dir.exists() {
		std::fs::create_dir_all(&config_dir).expect("failed to create config dir");
	}
	if !config_dir.is_dir() {
		panic!("config dir is not a directory");
	}

	let config_file = config_dir.join(CONFIG_FILENAME);
	let mut config = default_config();

	if config_file.exists() {
		let file_content =
			std::fs::read_to_string(&config_file).expect("failed to read config file");

		let parsed_config = parse_ini(&file_content);

		for (section, properties) in parsed_config {
			if let Some(default_section) = config.get_mut(&section) {
				for (key, value) in properties {
					if let Some(default_value) = default_section.get(&key) {
						default_section.insert(
							key,
							ConfigValueEnum::from_str(&value.to_string(), default_value),
						);
					}
				}
			}
		}

		let new_content = build_ini(&config);
		std::fs::File::create_new(&config_file)
			.expect("failed to create config file")
			.write_all(new_content.as_bytes())
			.expect("failed to write config file");
	}

	unsafe {
		CONFIG = Some(config);
	}
}

impl std::fmt::Display for ConfigValueEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConfigValueEnum::String(v) => write!(f, "{}", v),
			ConfigValueEnum::Int(v) => write!(f, "{}", v),
			ConfigValueEnum::Bool(v) => write!(f, "{}", v),
			ConfigValueEnum::Float(v) => write!(f, "{}", v),
		}
	}
}
