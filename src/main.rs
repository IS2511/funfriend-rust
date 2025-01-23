use glfw::{Action, Context, Key, WindowEvent};
use std::cell::RefCell;
use std::rc::Rc;

mod buddy;
mod config;
mod ease;
mod font_manager;
mod glfn;
mod graphics;
mod logger;
mod text_renderer;
mod texture;
mod vec2;
mod window;

use buddy::BuddyDefinition;
use vec2::Vec2;
use window::{Window, Windowed};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const FUNFRIEND_FRAG: &[u8] = include_bytes!("glsl/funfriend.frag");
pub const NOP_FRAG: &[u8] = include_bytes!("glsl/nop.frag");
pub const NOP_VERT: &[u8] = include_bytes!("glsl/nop.vert");
pub const BASIC_FRAG: &[u8] = include_bytes!("glsl/basic_fragment.frag");
pub const BASIC_VERT: &[u8] = include_bytes!("glsl/basic_vertex.vert");

pub struct App {
	contexts: Vec<Rc<RefCell<dyn Windowed>>>,
	buddy: Rc<RefCell<dyn BuddyDefinition>>,
	config: config::Config,
}

impl App {
	fn new(config: config::Config) -> Self {
		let buddy = buddy::make_buddy(config.buddy.r#type);

		Self {
			contexts: vec![buddy::make_context(&config, buddy.clone())],
			buddy,
			config,
		}
	}

	// fn run(&mut self) {
	// 	logger::init();
	// 	config_manager::read();
	//
	// 	let window = Rc::new(RefCell::new(Window::new(512, 512, "??_FUNFRIEND_??")));
	// 	self.window = Some(window.clone());
	// 	if let Some(window) = &mut self.window.clone() {
	// 		let buddy = make_buddy(
	// 			config_manager::CONFIG
	// 				.lock()
	// 				.unwrap()
	// 				.buddy_settings
	// 				.buddy_type
	// 				.as_str()
	// 				.clone(),
	// 		);
	// 		self.add_context(make_buddy_context(buddy.clone()));
	// 		self.set_buddy(buddy);
	//
	// 		let mut window = window.borrow_mut();
	//
	// 		let mut last_t = window.glfw.get_time();
	//
	// 		while !window.window_handle.should_close() {
	// 			tracing::info!("new frame");
	// 			window.glfw.poll_events();
	// 			let dt = window.glfw.get_time();
	// 			// -last_t;
	// 			last_t = window.glfw.get_time();
	//
	// 			tracing::info!("about to iterate over contexts");
	// 			for tuple in self.contexts.iter().enumerate() {
	// 				let mut context = tuple.1.borrow_mut();
	// 				if context.should_close() {
	// 					tracing::info!("trying to close?");
	// 					context.clean_up();
	// 				} else {
	// 					tracing::info!("running update");
	// 					let _ = context.update(dt);
	// 				}
	// 			}
	// 			let flushed_events = glfw::flush_messages(&window.events);
	// 			let mut should_close = false;
	// 			for (_, event) in flushed_events {
	// 				match event {
	// 					WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
	// 						should_close = true;
	// 					}
	// 					_ => (),
	// 				}
	// 			}
	// 			if should_close {
	// 				window.window_handle.set_should_close(true);
	// 			}
	// 			window.window_handle.swap_buffers();
	// 			window.glfw.wait_events_timeout(1.0 / 120.0);
	// 		}
	// 	}
	// }

	fn run(&mut self) {
		let config = config::read();

		let mut last_t = 0.0;
		while !self.contexts.is_empty() {
			self.contexts.retain_mut(|context| {
				let mut context = context.borrow_mut();
				// tracing::info!("new frame");
				context.get_window().glfw.poll_events();
				let dt = context.get_window().glfw.get_time() - last_t;
				last_t = context.get_window().glfw.get_time();
				let flushed_events = glfw::flush_messages(&context.get_window().events);
				let mut should_close = false;
				let mut was_clicked = false;
				let mut was_released = false;
				for (_, event) in flushed_events {
					match event {
						WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
							tracing::warn!("should close");
							should_close = true;
						}
						WindowEvent::MouseButton(_, Action::Press, _) => {
							tracing::warn!("was clicked");
							was_clicked = true;
						}
						WindowEvent::MouseButton(_, Action::Release, _) => {
							tracing::warn!("was released");
							was_released = true;
						}
						_ => (),
					}
				}
				if was_clicked {
					let cursor_pos = context.get_window().handle.get_cursor_pos();
					let cursor_pos = Vec2::new(cursor_pos.0, cursor_pos.1);

					context.on_click(cursor_pos);
				}
				if was_released {
					let cursor_pos = context.get_window().handle.get_cursor_pos();
					let cursor_pos = Vec2::new(cursor_pos.0, cursor_pos.1);

					context.on_release(cursor_pos);
				}
				if should_close {
					context.get_window().handle.set_should_close(true);
				}
				if context.should_close() {
					tracing::info!("trying to close?");
					context.clean_up();
					false
				} else {
					// tracing::info!("running update");
					context.update(dt);
					let window = context.get_window();
					window.handle.swap_buffers();
					window.glfw.wait_events_timeout(1.0 / 120.0);
					true
				}
			});
		}

		config::write(&config);
	}
}

fn main() {
	logger::init();

	let config = config::read();

	let mut app = App::new(config);
	app.run();
}
