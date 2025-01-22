use super::super::{
	font_manager::FontMan, text_renderer::TextRenderer, vec2::Vec2, window::Window,
};
use crate::buddy::context::FFContext;
use glfw::Context as _;

pub struct ChatterContext {
	renderer: TextRenderer,
	parent: Option<Box<ChatterContext>>,
	parent_relative_pos: Vec2,
	window_size: Vec2,
	timer: f64,
	window: Window,
}

impl ChatterContext {
	pub const DEFAULT_DURATION: f64 = 6.0;
	const PADDING: f64 = 10.0;

	pub fn new(
		text: &str,
		font: &str,
		position: Vec2,
		duration: f64,
		parent: Option<Box<ChatterContext>>,
	) -> Self {
		let sheet = FontMan::parse_bm(&std::fs::read_to_string(format!("{}.fnt", font)).unwrap());

		let (text_width, text_height, _) = FontMan::position_text(text, &sheet);

		let window_size = Vec2::new(
			text_width as f64 + Self::PADDING * 2.0,
			text_height as f64 + Self::PADDING * 2.0,
		);

		let mut window = Window::new(
			window_size.x as u32,
			window_size.y as u32,
			"??_FUNFRIEND_?? > CHATTER",
		);

		let window_pos = position - window_size / 2.0;

		let renderer = TextRenderer::new(
			text.to_string(),
			font.to_string(),
			sheet,
			window_size.x as i32,
			window_size.x as i32,
		);

		let mut parent_relative_pos = Vec2::zero();
		if let Some(ref p) = parent {
			let parent_window_pos = Vec2::new_t(p.window.window_handle.get_pos());
			let parent_window_size = Vec2::new(parent_window_pos.x, parent_window_pos.y);
			parent_relative_pos = position - (parent_window_size / 2.0);
		}

		window.window_handle.make_current();
		gl::load_with(|s| window.glfw.get_proc_address_raw(s) as *const _);
		
		unsafe {
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}
		
		Self {
			renderer,
			parent,
			parent_relative_pos,
			window_size,
			timer: duration,
			window,
		}
	}

	pub fn update_pos(&mut self) {
		if let Some(ref p) = self.parent {
			let parent_window_pos = Vec2::new_t(p.window.window_handle.get_pos());
			let parent_window_size = Vec2::new(parent_window_pos.x, parent_window_pos.y);
			let new_pos =
				(parent_window_pos + parent_window_size / 2.0 + self.parent_relative_pos.x
					- self.window_size / 2.0);
			self.window
				.window_handle
				.set_pos(new_pos.x as i32, new_pos.y as i32);
		}
	}

	pub fn render(&mut self, dt: f64) {
		self.window.window_handle.make_current();
		gl::load_with(|s| self.window.glfw.get_proc_address_raw(s) as *const _);
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		self.renderer.render(dt);
	}

	pub fn bump(&mut self) {
		self.parent_relative_pos.y -= self.window_size.y + 10.0;
		self.update_pos();
	}
}

impl FFContext for ChatterContext {
	fn update(&mut self, dt: f64) {
		self.timer -= dt;
		if self.timer <= 0.0 {
			self.window.window_handle.set_should_close(true);
		}
		self.update_pos();
		self.render(dt);
	}
	fn clean_up(&mut self) {
		self.renderer.clean_up();
	}

	fn should_close(&self) -> bool {
		self.window.window_handle.should_close()
	}

	fn get_window(&mut self) -> &mut Window {
		&mut self.window
	}
}
