use std::{cell::RefCell, rc::Rc};

use super::{
	config,
	texture::{SizedTexture, TextureBasket},
};

pub mod buddies;
pub mod chatter;
pub mod context;
pub mod renderer;

pub use context::Context;
pub use renderer::Renderer;

pub trait Buddy {
	fn name(&self) -> &str;
	fn dialog(&self, kind: DialogKind) -> Vec<Vec<String>>;
	fn body(&self) -> TextureBasket;
	fn background(&self) -> Option<SizedTexture> {
		None
	}
	fn play_talk_sound(&self) {}
	fn font(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogKind {
	Chatter,
	Moved,
	Touched,
}

pub fn make_context(
	config: &config::Config,
	buddy: Rc<RefCell<dyn Buddy>>,
) -> Rc<RefCell<dyn context::FFContext>> {
	Rc::new(RefCell::new(Context::new(config, buddy)))
}

pub fn make_buddy(r#type: config::BuddyType) -> Rc<RefCell<dyn Buddy>> {
	match r#type {
		config::BuddyType::Funfriend => Rc::new(RefCell::new(buddies::Funfriend)),
	}
}
