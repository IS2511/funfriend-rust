use std::{cell::RefCell, rc::Rc};

use super::texture::{SizedTexture, TextureBasket};

pub mod buddies;
pub mod chatter;
pub mod context;
pub mod renderer;

use context::BuddyContext;

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

pub fn make_context(buddy: Rc<RefCell<dyn Buddy>>) -> Rc<RefCell<dyn context::FFContext>> {
	Rc::new(RefCell::new(BuddyContext::new(buddy)))
}

pub fn make_buddy(name: &str) -> Rc<RefCell<dyn Buddy>> {
	match name {
		"funfriend" => Rc::new(RefCell::new(buddies::Funfriend)),
		_ => Rc::new(RefCell::new(buddies::Funfriend)),
	}
}

pub trait QuickDialogInstantiation {
	fn cloned(&self) -> Vec<Vec<String>>;
}

impl QuickDialogInstantiation for &'static [&'static [&'static str]] {
	fn cloned(&self) -> Vec<Vec<String>> {
		self.iter()
			.map(|d| d.iter().map(|s| s.to_string()).collect())
			.collect()
	}
}
