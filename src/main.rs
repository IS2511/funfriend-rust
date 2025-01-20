use std::cell::RefCell;
use std::sync::Mutex;
use std::rc::Rc;
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
use crate::buddy::context::FFContext;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const FUNFRIEND_FRAG: &[u8] = include_bytes!("glsl/funfriend.frag");
pub const NOP_FRAG: &[u8] = include_bytes!("glsl/nop.frag");
pub const NOP_VERT: &[u8] = include_bytes!("glsl/nop.vert");

pub struct Funfriend {
	version: &'static str,
	contexts: Vec<Rc<RefCell<dyn FFContext>>>,
	buddy: Option<Rc<RefCell<dyn Buddy>>>,
	window: Option<Rc<RefCell<Window>>>,
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

	fn set_buddy(&mut self, buddy: Rc<RefCell<dyn Buddy>>) {
		self.buddy = Some(buddy);
	}
	
	fn add_context(&mut self, context: Rc<RefCell<dyn FFContext>>) {
		self.contexts.push(context);
	}

	fn contexts(&self) -> &Vec<Rc<RefCell<dyn FFContext>>> {
		&self.contexts
	}

	fn run(&mut self) {
		logger::init();
		config_manager::read();

		let window = Rc::new(RefCell::new(Window::new(512, 512, "??_FUNFRIEND_??")));
		self.window = Some(window.clone());
		if let Some(window) = &mut self.window.clone() {
			let buddy = make_buddy(config_manager::CONFIG.lock().unwrap().buddy_settings.buddy_type.as_str().clone());
			self.add_context(make_buddy_context(buddy.clone(), if self.window.is_some(){window.clone()} else {panic!("window doesn't exist!")}));
			self.set_buddy(buddy);
			
			let mut window = window.borrow_mut();
			
			let mut last_t = window.glfw.get_time();
			
			while !window.window_handle.should_close() {
				tracing::info!("new frame");
				window.glfw.poll_events();
				let dt = window.glfw.get_time();
				-last_t;
				last_t = window.glfw.get_time();

				tracing::info!("about to iterate over contexts");
				for tuple in self.contexts.iter().enumerate() {
					let mut context = tuple.1.borrow_mut();
					if context.should_close() {
						tracing::info!("trying to close?");
						context.clean_up();
					} else {
						tracing::info!("running update");
						let _ = context.update(dt);
					}
				};
				let flushed_events = glfw::flush_messages(&window.events);
				let mut should_close = false;
				for (_, event) in flushed_events {
					match event {
						WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
							should_close = true;
						}
						_ => (),
					}
				}
				if should_close {
					window.window_handle.set_should_close(true);
				}
				window.window_handle.swap_buffers();
				window.glfw.wait_events_timeout(1.0 / 120.0);
			}
		}
	}
}

fn main() {
	let mut app = Funfriend::new();
	app.run();
}
