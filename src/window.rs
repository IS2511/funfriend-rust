use glfw::fail_on_errors;

pub struct Window {
	pub(crate) glfw: glfw::Glfw,
	pub(crate) window_handle: glfw::PWindow,
	pub(crate) events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl Window {
	pub fn new(width: u32, height: u32, title: &str) -> Self {
		let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
		glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
		glfw.window_hint(glfw::WindowHint::Decorated(true));
		glfw.window_hint(glfw::WindowHint::Resizable(false));
		glfw.window_hint(glfw::WindowHint::Focused(false));
		glfw.window_hint(glfw::WindowHint::FocusOnShow(false));
		glfw.window_hint(glfw::WindowHint::Floating(true));
		glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
		let (mut window, events) = glfw
			.create_window(width, height, title, glfw::WindowMode::Windowed)
			.expect("Failed to create GLFW window");
		window.set_framebuffer_size_polling(true);
		window.set_key_polling(true);
		Self {
			glfw,
			window_handle: window,
			events,
		}
	}
}
