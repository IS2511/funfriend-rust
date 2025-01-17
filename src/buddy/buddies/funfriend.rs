use crate::texture::{load_texture, SizedTexture, TextureBasket};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogType {
	Chatter,
	Moved,
	Touched,
}

pub trait Buddy {
	fn name(&self) -> &str;
	fn dialog(&self, dialog_type: DialogType) -> Vec<Vec<String>>;
	fn textures(&self) -> TextureBasket;
	fn bg_texture(&self) -> Option<SizedTexture>;
	fn talk_sound(&self);
	fn font(&self) -> &str;
}

pub struct FunfriendBuddy;

impl Buddy for FunfriendBuddy {
	fn name(&self) -> &str {
		"FUNFRIEND"
	}

	fn dialog(&self, dialog_type: DialogType) -> Vec<Vec<String>> {
		match dialog_type {
			DialogType::Chatter => vec![
				vec!["HELLO AGAIN".to_string()],
				vec!["HI INTERLOPER".to_string()],
				vec![
					"HELLO!".to_string(),
					"IS THE AUTH LAYER STILL DISSOCIATED?".to_string(),
					"I MISS THEM".to_string(),
				],
				vec![
					"INTERLOPER!".to_string(),
					"WELCOME".to_string(),
					"BUT ALSO PLEASE DO NOT BOTHER ME".to_string(),
					"VERY BUSY".to_string(),
				],
			],
			DialogType::Moved => vec![vec!["OK I'LL BE HERE".to_string()]],
			DialogType::Touched => vec![
				vec!["HI INTERLOPER!".to_string()],
				vec!["HELLO!".to_string()],
				vec!["HI!".to_string()],
			],
		}
	}

	fn textures(&self) -> TextureBasket {
		let textures = (0..40)
			.map(|i| {
				let filepath = format!("assets/buddies/funfriend_{:02}.png", i);
				load_texture(&filepath, None).expect("Failed to load texture.")
			})
			.collect();
		TextureBasket::new(textures, 10.0)
	}

	fn bg_texture(&self) -> Option<SizedTexture> {
		None
	}

	fn talk_sound(&self) {}

	fn font(&self) -> &str {
		"assets/fonts/SpaceMono"
	}
}

pub fn make_buddy(name: &str) -> Box<dyn Buddy> {
	match name {
		"funfriend" => Box::new(FunfriendBuddy),
		_ => Box::new(FunfriendBuddy),
	}
}
