use glfw::{Action, Context, Key, WindowEvent};

mod buddy;
mod config_manager;
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
use crate::buddy::buddies::funfriend::{make_buddy_context, Buddy};
use crate::buddy::context::BuddyContext;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Funfriend {
	version: &'static str,
	contexts: Vec<Box<BuddyContext<'_>>>,
	buddy: Option<dyn Buddy>,
	window: Option<Window>,
}

impl Funfriend {
	fn new() -> Self {
		Self {
			version: APP_VERSION,
			contexts: Vec::new(),
			buddy: None,
			window: None,
		}
	}

	fn set_buddy(&mut self, buddy: Box<dyn Buddy>) {
		self.buddy = Some(buddy);
	}
	
	fn add_context(&mut self, context: BuddyContext<'_>) {
		self.contexts.push(Box::new(context));
	}

	fn contexts(&self) -> &Vec<&dyn Context> {
		&self.contexts
	}

	fn run(&mut self) {
		logger::init();
		config_manager::read();

		let window = Window::new(512, 512, "??_FUNFRIEND_??");
		self.window = Some(window);
		if let Some(mut window) = &self.window {
			let buddy = make_buddy(config_manager::CONFIG.lock().unwrap().buddy_settings.buddy_type.as_str().clone());
			self.add_context(make_buddy_context(buddy,if self.window.is_some(){&mut window} else {panic!("window doesn't exist!")}));
			self.set_buddy(Box::new(buddy));
			
			let mut last_t = window.glfw.get_time();

			while !window.window_handle.should_close() {
				window.glfw.poll_events();
				let dt = window.glfw.get_time();
				-last_t;
				last_t = window.glfw.get_time();

				self.contexts.retain(|mut context| {
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
}

fn main() {}
