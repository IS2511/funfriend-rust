use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{vec2::Vec2, APP_NAME};

const CONFIG_FILENAME: &str = "cfg.json";

impl Default for Config {
	fn default() -> Self {
		Self {
			window: Window {
				size: Vec2::new(75.0, 75.0),
			},
			sound: Sound { master_volume: 1.0 },
			buddy: Buddy {
				r#type: BuddyType::Funfriend,
				behavior: Behavior::Normal,
				speed: 50.0,
			},
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	pub window: Window,
	pub sound: Sound,
	pub buddy: Buddy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Window {
	pub size: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
	pub master_volume: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Buddy {
	pub r#type: BuddyType,
	pub behavior: Behavior,
	pub speed: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum BuddyType {
	Funfriend,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Behavior {
	Normal,
	Dvd,
}

pub fn read() -> Config {
	match std::fs::read_to_string(get_config_dir().join(CONFIG_FILENAME)) {
		Ok(contents) => match serde_json::from_str(&contents) {
			Ok(config) => config,
			Err(err) => {
				tracing::warn!("failed to parse config file: {err}");
				Config::default()
			}
		},
		Err(err) => {
			tracing::warn!("failed to read config file: {err}");
			Config::default()
		}
	}
}

pub fn write(config: &Config) {
	let json = serde_json::to_string_pretty(config).expect("failed to serialize config");

	let config_dir = get_config_dir();
	if !config_dir.exists() {
		std::fs::create_dir(config_dir.clone()).expect("failed to create config dir");
	}

	std::fs::write(config_dir.join(CONFIG_FILENAME), json).expect("failed to write config file");
}

fn get_config_dir() -> std::path::PathBuf {
	if cfg!(windows) {
		PathBuf::from(std::env::var("APPDATA").expect("APPDATA env variable undefined"))
			.join(APP_NAME)
	} else if cfg!(target_os = "macos") {
		unimplemented!("fuck you im not making this work on mac lmaoooooooooooooooooo")
	} else {
		std::env::var("XDG_CONFIG_HOME")
			.map(PathBuf::from)
			.unwrap_or_else(|_| {
				PathBuf::from(std::env::var("HOME").expect("HOME env variable undefined"))
					.join(".config")
			})
			.join(APP_NAME)
	}
}
