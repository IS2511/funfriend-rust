pub mod dialog;
pub mod render;

pub trait Drawable {
	fn update(&mut self, dt: f64);
	fn render(&self);
}
