use glfw::{Action, Context, Key, WindowEvent};

mod buddy;
mod config;
mod ease;
mod font_manager;
mod glfn;
mod logger;
mod text_renderer;
mod texture;
mod vec2;
mod window;

use buddy::buddies::funfriend::make_buddy;
use window::Window;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Funfriend {
	version: &'static str,
	contexts: Vec<Box<dyn Context>>,
}

impl Funfriend {
	fn new() -> Self {
		Self {
			version: APP_VERSION,
			contexts: Vec::new(),
		}
	}

	fn init_contexts(&mut self) {
		self.add_context(make_buddy("funfriend"));
	}

	fn add_context(&mut self, context: Box<dyn Context>) {
		self.contexts.push(context);
	}

	fn contexts(&self) -> &Vec<Box<dyn Context>> {
		&self.contexts
	}

	fn run(&mut self) {
		logger::init();
		config::init();

		let mut window = Window::new(512, 512, "??_FUNFRIEND_??");

		let mut last_t = window.glfw.get_time();

		while !window.window_handle.should_close() {
			window.glfw.poll_events();
			let dt = window.glfw.get_time();
			-last_t;
			last_t = window.glfw.get_time();

			self.contexts.retain(|context| {
				if context.should_close() {
					context.clean_up();
					false
				} else {
					let _ = context.update(dt);
					true
				}
			});

			for (_, event) in glfw::flush_messages(&window.events) {
				match event {
					WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
						window.window_handle.set_should_close(true);
					}
					_ => (),
				}
			}
			window.window_handle.swap_buffers();
			window.glfw.wait_events_timeout(1.0 / 120.0);
		}
	}
}

fn main() {}
