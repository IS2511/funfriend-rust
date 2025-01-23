use glfw::fail_on_errors;

use super::vec2::Vec2;

pub trait Windowed {
	fn get_window(&mut self) -> &mut Window;
	fn update(&mut self, dt: f64);
	fn should_close(&self) -> bool;
	fn clean_up(&mut self);
	fn on_click(&mut self, position: Vec2) {}
	fn on_release(&mut self, position: Vec2) {}
}

pub struct Window {
	pub(crate) glfw: glfw::Glfw,
	pub(crate) handle: glfw::PWindow,
	pub(crate) events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl Window {
	pub fn new(width: u32, height: u32, title: &str) -> Self {
		let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

		glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
		glfw.window_hint(glfw::WindowHint::Decorated(false));
		glfw.window_hint(glfw::WindowHint::Resizable(false));
		glfw.window_hint(glfw::WindowHint::Focused(false));
		glfw.window_hint(glfw::WindowHint::FocusOnShow(false));
		glfw.window_hint(glfw::WindowHint::Floating(true));
		glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
		glfw.window_hint(glfw::WindowHint::AlphaBits(Some(8)));
		// glfw.window_hint(glfw::WindowHint::Visible(false));

		let (mut window, events) = glfw
			.create_window(width, height, title, glfw::WindowMode::Windowed)
			.expect("failed to create GLFW window");

		window.set_framebuffer_size_polling(true);
		window.set_key_polling(true);
		window.set_cursor_enter_polling(true);
		window.set_cursor_pos_polling(true);
		window.set_mouse_button_polling(true);

		Self {
			glfw,
			handle: window,
			events,
		}
	}
}
