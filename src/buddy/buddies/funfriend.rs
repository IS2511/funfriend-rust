use super::{
	super::{
		super::texture::{load_texture, TextureBasket},
		BuddyDefinition, DialogKind,
	},
	QuickDialogInstantiation as _,
};

mod dialog {
	use super::DialogKind;

	const CHATTER: &[&[&str]] = &[
		&["HELLO AGAIN"],
		&["HI INTERLOPER"],
		&[
			"HELLO!",
			"IS THE AUTH LAYER STILL DISSOCIATED?",
			"I MISS THEM",
		],
		&[
			"INTERLOPER!",
			"WELCOME",
			"BUT ALSO PLEASE DO NOT BOTHER ME",
			"VERY BUSY",
		],
	];
	const MOVED: &[&[&str]] = &[&["OK I'LL BE HERE"]];
	const TOUCHED: &[&[&str]] = &[&["HI INTERLOPER!"], &["HELLO!"], &["HI!"]];

	pub fn get(kind: DialogKind) -> &'static [&'static [&'static str]] {
		match kind {
			DialogKind::Chatter => CHATTER,
			DialogKind::Moved => MOVED,
			DialogKind::Touched => TOUCHED,
		}
	}
}

#[derive(Clone)]
pub struct Funfriend;

impl BuddyDefinition for Funfriend {
	fn name(&self) -> &str {
		"FUNFRIEND"
	}

	fn dialog(&self, kind: DialogKind) -> Vec<Vec<String>> {
		dialog::get(kind).cloned()
	}

	fn body(&self) -> TextureBasket {
		let textures = (0..40)
			.map(|i| {
				let filepath = format!("assets/buddies/funfriend_{:02}.png", i);
				load_texture(&filepath, None).expect("Failed to load texture.")
			})
			.collect();
		TextureBasket::new(textures, 10.0)
	}

	fn font(&self) -> &str {
		"assets/fonts/SpaceMono"
	}
}
