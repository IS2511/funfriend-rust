pub mod funfriend;

pub use funfriend::Funfriend;

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
