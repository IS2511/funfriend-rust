use glfw::Context as _;

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

const CHATTER_TIMER: f64 = 3.0;
const STAY_STILL_AFTER_HELD: f64 = 1.0;
const WANDER_TIMER: f64 = 4.0;
const FOLLOW_DIST: i32 = 120;

pub struct BuddyContext<'a> {
	buddy: &'a dyn funfriend::Buddy,
	renderer: BuddyRenderer,
	chatter_timer: f64,
	chatter_index: i32,
	chatter_array: Option<Vec<Vec<String>>>,
	held: bool,
	held_at: Vec2,
	started_holding_at: Vec2,
	held_timer: f64,
	waiting_for_stable_pos: bool,
	static_pos: Vec2,
	easing_from: Vec2,
	easing_to: Vec2,
	easing_dur: f64,
	easing_t: f64,
	wander_timer: f64,
	window: Window,
}

impl<'a> BuddyContext<'a> {
	pub fn new(buddy: &'a dyn funfriend::Buddy, mut window: Window) -> Self {
		let renderer = BuddyRenderer::new(buddy);
		let window_size = Self::get_window_size(&renderer);

		window
			.window_handle
			.set_size(window_size.x as i32, window_size.y as i32);
		window.window_handle.make_current();
		let chatter_array = buddy.dialog(DialogType::Chatter).sample();

		Self {
			buddy,
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
		}
	}

	// fn random_pos(&self) -> Vec2 {
	// 	let monitor = self.get_primary_monitor();
	//
	// }
	fn get_window_size(renderer: &BuddyRenderer) -> Vec2 {
		let size = renderer.funfriend_size();
		Vec2::new2((size * 1.3).floor() as i32)
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

	pub fn render(&mut self, dt: f64) {
		self.window.window_handle.make_current();
		let window_size = Self::get_window_size(&self.renderer);
		self.renderer
			.render(dt, window_size.x as i32, window_size.y as i32);
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

					if !self.speaking() {
						if stable_pos_dist > 50.0 {
							self.say(self.buddy.dialog(DialogType::Moved));
						} else {
							self.say(self.buddy.dialog(DialogType::Touched));
						}
					}
				}
			} else {
				self.waiting_for_stable_pos = true;
			}
		}
	}

	pub fn say(&mut self, text_groups: Vec<Vec<String>>) {
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
				&self.buddy.font(),
				text_position,
				ChatterContext::DEFAULT_DURATION,
				last_context.take(),
			);

			last_context = Some(Box::new(chatter_context));
			self.buddy.talk_sound();
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

	pub fn update(&mut self, dt: f64) {
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

#[derive(Debug, Clone)]
pub enum Behavior {
	Wander,
	Follow,
	Stay,
}
