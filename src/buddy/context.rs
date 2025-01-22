use super::super::{
	buddy::{
		buddies::funfriend::{self, DialogType},
		chatter::ChatterContext,
		renderer::BuddyRenderer,
	},
	ease,
	vec2::Vec2,
	window::Window,
};
use glfw::Context as _;
use rand::prelude::SliceRandom;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Mutex;
use glfw::ffi::{GLFWmonitor, GLFWvidmode};

const CHATTER_TIMER: f64 = 3.0;
const STAY_STILL_AFTER_HELD: f64 = 1.0;
const WANDER_TIMER: f64 = 4.0;
const FOLLOW_DIST: i32 = 120;

pub trait FFContext {
	fn should_close(&self) -> bool;
	fn clean_up(&mut self);
	fn update(&mut self, dt: f64);
	fn get_window(&mut self) -> &mut Window;
}

pub struct BuddyContext {
	pub buddy: Rc<RefCell<dyn funfriend::Buddy>>,
	pub renderer: BuddyRenderer,
	pub chatter_timer: f64,
	pub chatter_index: i32,
	pub chatter_array: Option<Vec<Vec<String>>>,
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
}

impl BuddyContext {
	pub fn new(buddy: Rc<RefCell<dyn funfriend::Buddy>>) -> Self {
		let mut b_ref = buddy.borrow_mut();
		let name = format!("!!__{}__!!", b_ref.name());
		drop(b_ref);
		let mut window = Window::new(512, 512, name.as_str());

		let renderer = BuddyRenderer::new(buddy.clone(), &mut window);
		let window_size = Self::get_window_size(&renderer);
		let mut b_ref = buddy.borrow_mut();

		window
			.window_handle
			.set_size(window_size.x as i32, window_size.y as i32);
		window.window_handle.make_current();
		gl::load_with(|s| window.glfw.get_proc_address_raw(s) as *const _);
		let binding = b_ref.dialog(DialogType::Chatter);
		let sample = binding.choose(&mut rand::thread_rng());
		let chatter_array = Some(vec![sample.unwrap().deref().to_owned()]);

		drop(b_ref);

		let mut result = Self {
			buddy: buddy.clone(),
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
		};

		let random_position = Self::random_pos_current_monitor(&result);
		result.window.window_handle.set_pos(random_position.x as i32, random_position.y as i32);
		
		result
	}

	// fn random_pos(&self) -> Vec2 {
	// 	let monitor = self.get_primary_monitor();
	//
	// }
	fn get_window_size(renderer: &BuddyRenderer) -> Vec2 {
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
		let (monitor, x, y, w, h) = Self::get_current_monitor(self.window.window_handle.window_ptr());
		let rand_x = x + (w as f64 * rand::random::<f64>()) as i32;
		let rand_y = y + (h as f64 * rand::random::<f64>()) as i32;
		Vec2::new_i(rand_x, rand_y)
	}
	
	fn get_current_monitor(window: *mut glfw::ffi::GLFWwindow) -> (*mut GLFWmonitor, i32, i32, i32, i32) {
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
		let mut best_monitor: (*mut GLFWmonitor, i32, i32, i32, i32) = (std::ptr::null_mut(), 0, 0, 0, 0);
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
				wh = mode.as_ref().unwrap().height;
				
				overlap =
					(0.max((wx+ww).min(mx+mw)-wx.max(mx))) * 
						(0.max((wy+wh).min(my+mh) - wy.max(my)));
				
				if best_overlap < overlap {
					best_overlap = overlap;
					best_monitor.0 = monitor;
					best_monitor.1 = wx;
					best_monitor.2 = wy;
					best_monitor.3 = ww;
					best_monitor.4 = wh;
				}
			}
		}
		best_monitor
	}

	pub fn render(&mut self, dt: f64) {
		self.window.window_handle.make_current();
		gl::load_with(|s| self.window.glfw.get_proc_address_raw(s) as *const _);
		let window_size = Self::get_window_size(&self.renderer);
		self.renderer
			.render(dt, window_size.x as i32, window_size.y as i32, &self.window);
	}

	pub fn goto(&mut self, pos: Vec2, dur: f64, set_as_static: bool) {
		self.easing_t = 0.0;
		self.easing_dur = dur;
		self.easing_from = Vec2::new(
			self.window.window_handle.get_pos().0 as f64,
			self.window.window_handle.get_pos().1 as f64,
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
				.window_handle
				.set_pos(new_position.x as i32, new_position.y as i32);

			self.wander_timer = WANDER_TIMER;
		} else {
			match self.behavior() {
				Behavior::Wander => {
					self.wander_timer -= dt;
					if self.wander_timer <= 0.0 {
						self.goto(self.static_pos + Vec2::rand(0.0..40.0), 4.0, false)
					}
				}
				Behavior::Follow => {
					if !self.moving() {
						let cursor_pos = self.window.window_handle.get_cursor_pos();
						let mut x_target = self.window.window_handle.get_pos().0 as f64;
						let mut y_target = self.window.window_handle.get_pos().1 as f64;

						let x_dist = cursor_pos.0;
						let y_dist = cursor_pos.1;

						if x_dist.abs() > FOLLOW_DIST as f64 {
							x_target = self.window.window_handle.get_pos().0 as f64 + x_dist
								- FOLLOW_DIST as f64 * x_dist.signum();
						}

						if y_dist.abs() > FOLLOW_DIST as f64 {
							y_target = self.window.window_handle.get_pos().1 as f64 + y_dist
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
		let cursor_pos = self.window.window_handle.get_cursor_pos();
		let cursor_pos = Vec2::new(cursor_pos.0, cursor_pos.1);
		if self.held {
			tracing::info!("should be held");
			self.static_pos = cursor_pos - self.held_at + cursor_pos;
			self.window
				.window_handle
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
							let dialog = buddy.dialog(DialogType::Moved);
							drop(buddy);
							self.say(dialog);
						} else {
							let dialog = buddy.dialog(DialogType::Touched);
							drop(buddy);
							self.say(dialog);
						}
					}
				}
			} else {
				self.waiting_for_stable_pos = true;
			}
		}
	}

	pub fn say(&mut self, text_groups: Vec<Vec<String>>) {
		let buddy = self.buddy.borrow();
		let flattened_texts: Vec<String> = text_groups.into_iter().flatten().collect();
		let window_size = Self::get_window_size(&self.renderer);
		let window_size = Vec2::new(window_size.x, window_size.y);

		let mut last_context: Option<Box<ChatterContext>> = None;

		let text_position = Vec2::new(
			self.window.window_handle.get_pos().0 as f64 + window_size.x / 2.0,
			self.window.window_handle.get_pos().1 as f64 - 20.0,
		);
		for text in flattened_texts {
			let chatter_context = ChatterContext::new(
				&text,
				&buddy.font(),
				text_position,
				ChatterContext::DEFAULT_DURATION,
				last_context.take(),
			);

			last_context = Some(Box::new(chatter_context));
			buddy.talk_sound();
		}
	}

	pub fn say_array(&mut self, text: Vec<Vec<String>>) {
		self.chatter_array = Some(text);
		self.chatter_timer = 0.0;
		self.chatter_index = 0;
	}

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

impl FFContext for BuddyContext {
	fn should_close(&self) -> bool {
		self.window.window_handle.should_close()
	}

	fn clean_up(&mut self) {
		self.renderer.clean_up();
	}

	fn update(&mut self, dt: f64) {
		// tracing::info!("current behavior: {:?}", self.behavior());
		self.chatter_timer -= dt;
		if self.chatter_timer <= 0.0 {
			self.chatter_timer += CHATTER_TIMER;

			if let Some(ref chatter_array) = self.chatter_array {
				if let Some(chatter) = chatter_array.get(self.chatter_index as usize) {
					self.say(vec![chatter.clone()]);
				}
			}
			self.chatter_index += 1;

			if let Some(ref chatter_array) = self.chatter_array {
				if self.chatter_index >= chatter_array.len() as i32 {
					self.chatter_array = None;
					self.chatter_index = 0;
				}
			}
		}

		self.update_pos(dt);
		self.render(dt);

		self.window.window_handle.swap_buffers();
	}

	fn get_window(&mut self) -> &mut Window {
		&mut self.window
	}
}

#[derive(Debug, Clone)]
pub enum Behavior {
	Wander,
	Follow,
	Stay,
}
