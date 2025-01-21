use serde::{Deserialize, Serialize};
use std::io::{Read, Write as _};
use std::ops::Deref as _;

use super::APP_NAME;
const CONFIG_FILENAME: &str = "cfg.json";

// tried to make this not arc mutex, it does not like when i do that. fix later, or dont i dont care so long as it functions and does its job.
lazy_static::lazy_static! {
	pub static ref CONFIG: std::sync::Arc<std::sync::Mutex<Config>> = Default::default();
}

fn get_config_dir() -> std::path::PathBuf {
	if cfg!(windows) {
		std::path::PathBuf::from(std::env::var("APPDATA").unwrap()).join(APP_NAME)
	} else if cfg!(target_os = "macos") {
		panic!("fuck you im not making this work on mac lmaoooooooooooooooooo")
	} else {
		std::env::var("XDG_CONFIG_HOME")
			.map(std::path::PathBuf::from)
			.unwrap_or_else(|_| {
				std::path::PathBuf::from(std::env::var("HOME").unwrap()).join(".config")
			})
			.join(APP_NAME)
	}
}

/*
 * comment to stop IDE from complaining about Default not being implemented. It is implemented.
 * I don't know why it is doing this.
*/
//noinspection RsDerivableTraitMembers
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
	pub window_settings: WindowSettings,
	pub sound_settings: SoundSettings,
	pub buddy_settings: BuddySettings,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowSettings {
	pub size: super::vec2::Vec2,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundSettings {
	pub master_volume: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuddySettings {
	pub buddy_type: String,
}

pub fn read() {
	let mut config = CONFIG.lock().unwrap();
	*config = if get_config_dir().join(CONFIG_FILENAME).exists() {
		let mut file = std::fs::File::open(get_config_dir().join(CONFIG_FILENAME))
			.expect("Failed to open config file");
		let mut contents = String::new();
		file.read_to_string(&mut contents)
			.expect("Failed to read config file");
		serde_json::from_str(&contents).expect("Failed to parse config file")
	} else {
		Config::default()
	}
}
pub fn write() {
	let json = serde_json::to_string_pretty(&CONFIG.lock().unwrap().deref()).unwrap();
	let path = get_config_dir();
	if !path.exists() {
		std::fs::create_dir(path.clone()).unwrap();
	}
	let mut file = std::fs::File::create(path.join(CONFIG_FILENAME)).unwrap();
	file.write_all(json.as_bytes())
		.expect("Failed to write configuration.");
}
impl Default for WindowSettings {
	fn default() -> Self {
		Self {
			size: super::vec2::Vec2::new(512.0, 512.0),
		}
	}
}
impl Default for SoundSettings {
	fn default() -> Self {
		Self { master_volume: 1.0 }
	}
}
impl Default for BuddySettings {
	fn default() -> Self {
		Self {
			buddy_type: "funfriend".to_string(),
		}
	}
}
