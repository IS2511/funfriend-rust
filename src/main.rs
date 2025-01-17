mod window;
mod logger;
mod vec2;
mod texture;
mod text_renderer;
mod font_manager;
mod glfn;
mod ease;
mod config;
mod chatter_context;
mod buddy_context;
mod buddy;
mod buddy_renderer;

use glfw::{Action, Context, Key, WindowEvent};
use tracing::{info, warn, error, debug, trace};
use window::Window;
use logger::Logger;
use crate::buddy::funfriend::make_buddy;
use crate::config::config::config;

pub struct Funfriend {
	version: &'static str,
	contexts: Vec<dyn Context>
}

impl Funfriend{
	const VERSION: &'static str = "0.0.1";
	
	fn new() -> Self {
		Self {
			version: Self::VERSION,
			contexts: Vec::new()
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
		Logger::init();
		config::config::init();

		let mut window = Window::new(512, 512, "??_FUNFRIEND_??");

		let mut last_t = window.glfw.get_time();
		
		while !window.window_handle.should_close() {
			window.glfw.poll_events();
			let dt = window.glfw.get_time(); - last_t;
			last_t = window.glfw.get_time();
			
			self.contexts.retain(|context|{
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
					_ => ()
				}
			}
			window.window_handle.swap_buffers();
			window.glfw.wait_events_timeout(1.0 / 120.0);
		}
	}
}

fn main() {
	
}
