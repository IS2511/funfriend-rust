use std::cell::RefCell;
use std::rc::Rc;

use glfw::ffi::{GLFWmonitor, GLFWvidmode};
use glfw::Context as _;
use rand::prelude::SliceRandom;
use rand::Rng as _;

use super::{
	super::{
		buddy::{self, DialogKind},
		config, ease,
		vec2::Vec2,
		window::Window,
	},
	Buddy,
};

const CHATTER_TIMER: f64 = 3.0;
const STAY_STILL_AFTER_HELD: f64 = 1.0;
const WANDER_TIMER: f64 = 4.0;
const FOLLOW_DIST: i32 = 120;

pub trait FFContext {
	fn should_close(&self) -> bool;
	fn clean_up(&mut self);
	fn update(&mut self, dt: f64);
	fn get_window(&mut self) -> &mut Window;
	fn on_click(&mut self, position: Vec2) {}
	fn on_release(&mut self, position: Vec2) {}
}

pub struct Context {
	pub buddy: Rc<RefCell<dyn Buddy>>,
	pub owned_contexts: Vec<Rc<RefCell<dyn FFContext>>>,
	pub renderer: buddy::Renderer,
	pub chatter_timer: f64,
	pub chatter_index: i32,
	pub chatter_array: Option<Vec<String>>,
	pub held: bool,
	pub held_at: Vec2,
	pub started_holding_at: Vec2,
	pub held_timer: f64,
	pub waiting_for_stable_pos: bool,
	pub static_pos: Vec2,
	pub easing_from: Vec2,
	pub easing_to: Vec2,
	pub easing_dur: f64,
	pub easing_t: f64,
	pub wander_timer: f64,
	pub window: Window,
	pub dir_vec: Vec2,
	pub configured_behavior: config::Behavior,
	pub speed: f64,
	pub internal_pos: Vec2,
}

impl Context {
	pub fn new(config: &config::Config, buddy: Rc<RefCell<dyn Buddy>>) -> Self {
		let name = format!("!!__{}__!!", buddy.borrow().name());

		let mut window = Window::new(512, 512, name.as_str());

		let renderer = buddy::Renderer::new(config, buddy.clone(), &mut window);
		let window_size = Self::get_window_size(&renderer);
		tracing::info!("Window size: {:?}", window_size);

		window
			.handle
			.set_size(window_size.x as i32, window_size.y as i32);
		window.handle.make_current();
		// window.window_handle.set_cursor(Some(glfw::Cursor::standard(glfw::StandardCursor::Hand)));
		gl::load_with(|s| window.glfw.get_proc_address_raw(s) as *const _);

		let binding = buddy.borrow().dialog(DialogKind::Chatter);
		let chatter_array = binding.choose(&mut rand::thread_rng()).cloned();

		let mut result = Self {
			buddy: buddy.clone(),
			owned_contexts: Vec::new(),
			renderer,
			chatter_timer: 1.0,
			chatter_index: 0,
			chatter_array,
			held: false,
			held_at: Vec2::zero(),
			held_timer: 0.0,
			started_holding_at: Vec2::zero(),
			waiting_for_stable_pos: false,
			static_pos: Vec2::zero(),
			easing_from: Vec2::zero(),
			easing_to: Vec2::zero(),
			easing_dur: 0.0,
			easing_t: 0.0,
			wander_timer: WANDER_TIMER,
			window,
			dir_vec: match config.buddy.behavior {
				config::Behavior::Normal => Vec2::zero(),
				config::Behavior::Dvd => {
					let mut rng = rand::thread_rng();
					let x = rng.gen_range(-1.0..1.0);
					let y = rng.gen_range(-1.0..1.0);
					Vec2::new(x, y).normalize()
				}
			},
			configured_behavior: config.buddy.behavior.clone(),
			speed: config.buddy.speed.clone(),
			internal_pos: Vec2::zero(),
		};

		let random_position = Self::random_pos_current_monitor(&result);
		tracing::info!("random position: {:?}", random_position);
		result
			.window
			.handle
			.set_pos(random_position.x as i32, random_position.y as i32);
		result.internal_pos = random_position;
		result.static_pos = random_position;
		drop(config);
		result
	}

	// fn random_pos(&self) -> Vec2 {
	// 	let monitor = self.get_primary_monitor();
	//
	// }
	fn get_window_size(renderer: &buddy::Renderer) -> Vec2 {
		let size = renderer.funfriend_size();
		Vec2::new_i(
			(size.0 as f64 * 1.3).floor() as i32,
			(size.1 as f64 * 1.3).floor() as i32,
		)
	}

	fn random_pos(&self) -> Vec2 {
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		let mut width = 0;
		let mut height = 0;
		unsafe {
			let mut monitor_count = 0;
			let monitors = glfw::ffi::glfwGetMonitors(&mut monitor_count);
			if monitor_count > 0 {
				let monitor_ptr = *monitors;
				glfw::ffi::glfwGetMonitorPos(monitor_ptr, &mut x, &mut y);
				let video_mode = glfw::ffi::glfwGetVideoMode(monitor_ptr);
				width = video_mode.as_ref().unwrap().width as i32;
				height = video_mode.as_ref().unwrap().height as i32;
			}
		}
		let rand_x = x + (width as f64 * rand::random::<f64>()) as i32;
		let rand_y = y + (height as f64 * rand::random::<f64>()) as i32;
		Vec2::new_i(rand_x, rand_y)
	}

	fn random_pos_current_monitor(&self) -> Vec2 {
		let (monitor, x, y, w, h, _mx, _my, _mw, _mh) =
			Self::get_current_monitor(self.window.handle.window_ptr());
		let rand_x = x + (w as f64 * rand::random::<f64>()) as i32;
		let rand_y = y + (h as f64 * rand::random::<f64>()) as i32;
		Vec2::new_i(rand_x, rand_y)
	}

	fn get_current_monitor(
		window: *mut glfw::ffi::GLFWwindow,
	) -> (*mut GLFWmonitor, i32, i32, i32, i32, i32, i32, i32, i32) {
		let mut monitor_count: std::ffi::c_int = 0;

		let mut wx: std::ffi::c_int = 0;
		let mut wy: std::ffi::c_int = 0;
		let mut ww: std::ffi::c_int = 0;
		let mut wh: std::ffi::c_int = 0;

		let mut mx: std::ffi::c_int = 0;
		let mut my: std::ffi::c_int = 0;
		let mut mw: std::ffi::c_int = 0;
		let mut mh: std::ffi::c_int = 0;
		let mut overlap: std::ffi::c_int;
		let mut best_overlap: std::ffi::c_int;

		let mut mode: *const GLFWvidmode;
		let mut best_monitor: (*mut GLFWmonitor, i32, i32, i32, i32, i32, i32, i32, i32) =
			(std::ptr::null_mut(), 0, 0, 0, 0, 0, 0, 0, 0);
		let mut monitors: *mut *mut GLFWmonitor;
		best_overlap = 0;
		unsafe {
			glfw::ffi::glfwGetWindowPos(window, &mut wx, &mut wy);
			glfw::ffi::glfwGetWindowSize(window, &mut ww, &mut wh);

			monitors = glfw::ffi::glfwGetMonitors(&mut monitor_count);

			for i in 0..monitor_count {
				let monitor = *monitors.add(i as usize);
				mode = glfw::ffi::glfwGetVideoMode(monitor);
				glfw::ffi::glfwGetMonitorPos(monitor, &mut mx, &mut my);
				mw = mode.as_ref().unwrap().width;
				mh = mode.as_ref().unwrap().height;

				overlap = (0.max((wx + ww).min(mx + mw) - wx.max(mx)))
					* (0.max((wy + wh).min(my + mh) - wy.max(my)));

				if best_overlap < overlap {
					best_overlap = overlap;
					best_monitor.0 = monitor;
					best_monitor.1 = wx;
					best_monitor.2 = wy;
					best_monitor.3 = ww;
					best_monitor.4 = wh;
					best_monitor.5 = mx;
					best_monitor.6 = my;
					best_monitor.7 = mw;
					best_monitor.8 = mh;
				}
			}
		}
		best_monitor
	}

	pub fn render(&mut self, dt: f64) {
		self.window.handle.make_current();
		gl::load_with(|s| self.window.glfw.get_proc_address_raw(s) as *const _);
		let window_size = Self::get_window_size(&self.renderer);
		self.renderer
			.render(dt, window_size.x as i32, window_size.y as i32, &self.window);
	}

	pub fn goto(&mut self, pos: Vec2, dur: f64, set_as_static: bool) {
		self.easing_t = 0.0;
		self.easing_dur = dur;
		self.easing_from = Vec2::new(
			self.window.handle.get_pos().0 as f64,
			self.window.handle.get_pos().1 as f64,
		);
		self.easing_to = pos;

		if set_as_static {
			self.static_pos = self.easing_to;
		}

		tracing::info!("going from {:?} to {:?}", self.easing_from, self.easing_to);
	}

	pub fn update_wander(&mut self, dt: f64) {
		// tracing::info!("dt: {}", dt);
		if self.moving() {
			self.easing_t += dt;
			let a = ease::in_out_sine(self.easing_t / self.easing_dur);
			let new_position = self.easing_from * (1.0 - a) + self.easing_to * a;
			self.window
				.handle
				.set_pos(new_position.x as i32, new_position.y as i32);

			self.wander_timer = WANDER_TIMER;
		} else {
			tracing::info!("behavior: {:?}", self.behavior());
			match self.behavior() {
				Behavior::Wander => {
					self.wander_timer -= dt;
					if self.wander_timer <= 0.0 {
						self.goto(self.static_pos + Vec2::rand(0.0..40.0), 4.0, false)
					}
				}
				Behavior::Follow => {
					if !self.moving() {
						let cursor_pos = self.window.handle.get_cursor_pos();
						let mut x_target = self.window.handle.get_pos().0 as f64;
						let mut y_target = self.window.handle.get_pos().1 as f64;

						let x_dist = cursor_pos.0;
						let y_dist = cursor_pos.1;

						if x_dist.abs() > FOLLOW_DIST as f64 {
							x_target = self.window.handle.get_pos().0 as f64 + x_dist
								- FOLLOW_DIST as f64 * x_dist.signum();
						}

						if y_dist.abs() > FOLLOW_DIST as f64 {
							y_target = self.window.handle.get_pos().1 as f64 + y_dist
								- FOLLOW_DIST as f64 * y_dist.signum();
						}

						self.goto(Vec2::new(x_target, y_target), 1.0, true);
					}
				}
				Behavior::Stay => {}
			}
		}
	}

	pub fn update_pos(&mut self, dt: f64) {
		let cursor_pos = self.window.handle.get_cursor_pos();
		let cursor_pos = Vec2::new(cursor_pos.0, cursor_pos.1);
		if self.held {
			self.static_pos = self.static_pos - self.held_at + cursor_pos;
			self.window
				.handle
				.set_pos(self.static_pos.x as i32, self.static_pos.y as i32);
		} else {
			self.held_timer -= dt;
			if self.held_timer <= 0.0 {
				self.update_wander(dt);

				if self.waiting_for_stable_pos {
					self.waiting_for_stable_pos = false;

					let stable_pos_dist = self.static_pos.dist(self.started_holding_at);
					tracing::info!("travelled {:?}", stable_pos_dist);
					let buddy = self.buddy.borrow();
					if !self.speaking() {
						if stable_pos_dist > 50.0 {
							let dialog = buddy.dialog(DialogKind::Moved);
							drop(buddy);
							// TODO(is2511): Make `say()` work
							// self.say(dialog);
						} else {
							let dialog = buddy.dialog(DialogKind::Touched);
							drop(buddy);
							// self.say(dialog);
						}
					}
				}
			} else {
				self.waiting_for_stable_pos = true;
			}
		}
	}

	fn update_dvd(&mut self, dt: f64) {
		// tracing::info!("FRAME START");
		// tracing::info!("init pos: {:?}", self.internal_pos);
		// tracing::info!("dir: {:?}", self.dir_vec);

		let cursor_pos = self.window.handle.get_cursor_pos();
		let cursor_pos = Vec2::new(cursor_pos.0, cursor_pos.1);

		if self.held {
			tracing::info!("cursor pos: {:?}", cursor_pos);
			tracing::info!("held at: {:?}", self.held_at);
			tracing::info!(
				"should set to: {:?}",
				self.internal_pos - self.held_at + self.internal_pos
			);
			self.internal_pos = self.internal_pos - self.held_at + cursor_pos;
			self.window
				.handle
				.set_pos(self.internal_pos.x as i32, self.internal_pos.y as i32);
			return;
		}
		let (_, _, _, w, h, _, _, mw, mh) =
			Self::get_current_monitor(self.window.handle.window_ptr());
		// tracing::info!("w: {}, h: {}", w, h);

		self.internal_pos += self.dir_vec * self.speed * dt;
		// tracing::info!("new pos: {:?}", self.internal_pos);
		// tracing::info!("current window pos: {:?}", self.window.window_handle.get_pos());
		self.window
			.handle
			.set_pos(self.internal_pos.x as i32, self.internal_pos.y as i32);
		// tracing::info!("new window pos: {:?}", self.window.window_handle.get_pos());
		// tracing::info!("FRAME END");

		// tracing::info!("internal pos: {:?}, w: {}, h: {}", self.internal_pos, w, h);

		if self.internal_pos.x <= 0.0 {
			tracing::info!("hit left wall");
			self.dir_vec.x = -self.dir_vec.x;
			self.internal_pos.x = 0.0;
		}
		if self.internal_pos.y <= 0.0 {
			tracing::info!("hit top wall");
			self.dir_vec.y = -self.dir_vec.y;
			self.internal_pos.y = 0.0;
		}
		if self.internal_pos.x + w as f64 >= mw as f64 {
			tracing::info!("hit right wall");
			self.dir_vec.x = -self.dir_vec.x;
			self.internal_pos.x = (mw - w) as f64;
		}
		if self.internal_pos.y + h as f64 >= mh as f64 {
			tracing::info!("hit bottom wall");
			self.dir_vec.y = -self.dir_vec.y;
			self.internal_pos.y = (mh - h) as f64;
		}
	}

	pub fn say(&mut self, text: String) {
		// for context in Rc::get_mut(self.app_contexts.as_mut()).unwrap().into_iter().enumerate() {
		// 	if context
		// }
	}

	// pub fn say(&mut self, text_groups: Vec<Vec<String>>) {
	// 	let buddy = self.buddy.borrow();
	// 	let flattened_texts: Vec<String> = text_groups.into_iter().flatten().collect();
	// 	let window_size = Self::get_window_size(&self.renderer);
	// 	let window_size = Vec2::new(window_size.x, window_size.y);
	//
	// 	let mut last_context: Option<Box<ChatterContext>> = None;
	//
	// 	let text_position = Vec2::new(
	// 		self.window.window_handle.get_pos().0 as f64 + window_size.x / 2.0,
	// 		self.window.window_handle.get_pos().1 as f64 - 20.0,
	// 	);
	// 	for text in flattened_texts {
	// 		let chatter_context = ChatterContext::new(
	// 			&text,
	// 			&buddy.font(),
	// 			text_position,
	// 			ChatterContext::DEFAULT_DURATION,
	// 			last_context.take(),
	// 		);
	//
	// 		last_context = Some(Box::new(chatter_context));
	// 		buddy.talk_sound();
	// 	}
	// }
	//
	// pub fn say_array(&mut self, text: Vec<String>) {
	// 	self.chatter_array = Some(text);
	// 	self.chatter_timer = 0.0;
	// 	self.chatter_index = 0;
	// }

	pub fn speaking(&self) -> bool {
		if let Some(ref chatter_array) = self.chatter_array {
			self.chatter_index < chatter_array.len() as i32
		} else {
			false
		}
	}

	pub fn behavior(&self) -> Behavior {
		if self.speaking() {
			Behavior::Follow
		} else {
			Behavior::Wander
		}
	}

	pub fn moving(&self) -> bool {
		self.easing_dur != 0.0 && self.easing_t <= self.easing_dur
	}
}

impl FFContext for Context {
	fn should_close(&self) -> bool {
		self.window.handle.should_close()
	}

	fn clean_up(&mut self) {
		self.renderer.clean_up();
	}

	fn update(&mut self, dt: f64) {
		// tracing::info!("current behavior: {:?}", self.behavior());
		tracing::info!("expected behavior: {:?}", self.behavior());
		match self.configured_behavior {
			config::Behavior::Dvd => {
				self.update_dvd(dt);
			}
			config::Behavior::Normal => {
				self.chatter_timer -= dt;
				if self.chatter_timer <= 0.0 {
					tracing::info!("allowed to speak");
					self.chatter_timer += CHATTER_TIMER;

					if let Some(ref chatter_array) = self.chatter_array {
						if let Some(chatter) = chatter_array.get(self.chatter_index as usize) {
							tracing::info!("should speak from update");
							self.say(chatter.clone());
						}
					}
					self.chatter_index += 1;
				}
				self.update_pos(dt);
			}
		}

		self.render(dt);

		self.window.handle.swap_buffers();
	}

	fn get_window(&mut self) -> &mut Window {
		&mut self.window
	}

	fn on_click(&mut self, position: Vec2) {
		self.held = true;
		self.held_at = position;
		if self.held_timer <= 0.0 {
			self.started_holding_at = Vec2::new_t(self.window.handle.get_pos());
		}
		self.held_timer = STAY_STILL_AFTER_HELD;
		self.easing_dur = 0.0;
		self.window
			.handle
			.set_cursor(Some(glfw::Cursor::standard(glfw::StandardCursor::Hand)));
	}

	fn on_release(&mut self, _: Vec2) {
		self.held = false;
		self.window
			.handle
			.set_cursor(Some(glfw::Cursor::standard(glfw::StandardCursor::Arrow)));
	}
}

#[derive(Debug, Clone)]
pub enum Behavior {
	Wander,
	Follow,
	Stay,
}
