use rand::Rng as _;
use std::f32::consts::TAU;
use std::ops::{Add, Div, Mul, Neg, Sub};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}

	pub fn new2(xy: f64) -> Self {
		Self { x: xy, y: xy }
	}

	pub fn new_i(x: i32, y: i32) -> Self {
		let x = x as f64;
		let y = y as f64;
		Self { x, y }
	}

	pub fn new_i2(xy: i32) -> Self {
		let xy = xy as f64;
		Self { x: xy, y: xy }
	}

	pub fn new_t(xy: (i32, i32)) -> Self {
		let x = xy.0 as f64;
		let y = xy.1 as f64;
		Self { x, y }
	}

	pub fn xy(&self) -> (f64, f64) {
		(self.x, self.y)
	}
	pub fn yx(&self) -> (f64, f64) {
		(self.y, self.x)
	}

	pub fn xy_i(&self) -> (i64, i64) {
		(self.x as i64, self.y as i64)
	}

	pub fn yx_i(&self) -> (i64, i64) {
		(self.y as i64, self.x as i64)
	}

	// maths (ew)

	pub fn dot(self, other: Self) -> f64 {
		self.x * other.x + self.y * other.y
	}

	pub fn cross(self, other: Self) -> Self {
		Vec2::new(
			self.y * other.y - self.x * other.y,
			self.x * other.x - self.y * other.y,
		)
	}

	pub fn angle(&self) -> f64 {
		self.y.atan2(self.x)
	}

	pub fn len(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn square_len(&self) -> f64 {
		self.x * self.x + self.y * self.y
	}

	pub fn angle_to(&self, other: Self) -> Vec2 {
		self * other / (self.len() * other.len())
	}

	pub fn normalize_mut(&mut self) {
		let m = self.len();

		if m != 0.0 {
			let inverse = 1.0 / m;
			self.x *= inverse;
			self.y *= inverse;
		}
	}

	pub fn normalize(&self) -> Vec2 {
		let mut normalized = *self;
		normalized.normalize_mut();
		normalized
	}

	pub fn scale_mut(&mut self, factor: f64) {
		self.normalize_mut();
		self.x *= factor;
		self.y *= factor;
	}

	pub fn scale(&self, factor: f64) -> Vec2 {
		let mut scaled = *self;
		scaled.scale_mut(factor);
		scaled
	}

	pub fn dist(&self, other: Vec2) -> f64 {
		(*self - other).len()
	}

	pub fn square_dist(&self, other: Vec2) -> f64 {
		(*self - other).square_len()
	}

	pub fn eq(&self, other: Vec2) -> bool {
		self.x == other.x && self.y == other.y
	}

	pub fn ne(&self, other: Vec2) -> bool {
		self.x != other.x || self.y != other.y
	}

	pub fn string(&self) -> String {
		format!("({}, {})", self.x, self.y)
	}

	pub fn inspect(&self) -> String {
		format!("(x: {}, y: {})", self.x, self.y)
	}

	pub fn zero() -> Self {
		Self::new(0.0, 0.0)
	}

	pub fn additive_identity() -> Self {
		Self::zero()
	}

	pub fn multiplicative_identity() -> Self {
		Self::new(1.0, 1.0)
	}

	pub fn from_polar(angle: f64, length: f64) -> Vec2 {
		Self {
			x: angle.cos() * length,
			y: angle.sin() * length,
		}
	}

	pub fn rand(length: std::ops::Range<f64>) -> Vec2 {
		let mut rng = rand::thread_rng();
		let angle = rng.gen_range(0.0..TAU) as f64;
		let len = rng.gen_range(length);
		Vec2::from_polar(angle, len)
	}
}

// operators

impl Add for Vec2 {
	type Output = Vec2;

	fn add(self, other: Self) -> Self::Output {
		Self::new(self.x + other.x, self.y + other.y)
	}
}

impl Add<f64> for Vec2 {
	type Output = Vec2;
	fn add(self, other: f64) -> Self::Output {
		Self::new(self.x + other, self.y + other)
	}
}

impl Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, other: Self) -> Self::Output {
		Self::new(self.x - other.x, self.y - other.y)
	}
}

impl Sub<f64> for Vec2 {
	type Output = Vec2;
	fn sub(self, other: f64) -> Self::Output {
		Self::new(self.x - other, self.y - other)
	}
}

impl Neg for Vec2 {
	type Output = Vec2;
	fn neg(self) -> Self::Output {
		Self::new(-self.x, -self.y)
	}
}

impl Mul for Vec2 {
	type Output = Vec2;

	fn mul(self, other: Self) -> Self::Output {
		Self::new(self.x * other.x, self.y * other.y)
	}
}

impl Mul<Vec2> for &Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: Vec2) -> Self::Output {
		Vec2::new(self.x * rhs.x, self.y * rhs.y)
	}
}

impl Mul<f64> for Vec2 {
	type Output = Vec2;
	fn mul(self, other: f64) -> Self::Output {
		Self::new(self.x * other, self.y * other)
	}
}

impl Div for Vec2 {
	type Output = Vec2;
	fn div(self, other: Self) -> Self::Output {
		Self::new(self.x / other.x, self.y / other.y)
	}
}

impl Div<f64> for Vec2 {
	type Output = Vec2;
	fn div(self, other: f64) -> Self::Output {
		Self::new(self.x / other, self.y / other)
	}
}

impl PartialEq for Vec2 {
	fn eq(&self, other: &Self) -> bool {
		if self.eq(*other) { true } else { false }
	}
}
