use std::f64::consts::PI;

pub struct Ease;

impl Ease{
	pub fn in_sine(x: f64) -> f64{
		1.0 - (x * (PI * 0.5)).cos()
	}
	
	pub fn out_sine(x: f64) -> f64 {
		(x * (PI * 0.5)).sin()
	}
	
	pub fn in_out_sine(x: f64) -> f64{
		0.5 - 0.5 * (x*PI).cos()
	}
}