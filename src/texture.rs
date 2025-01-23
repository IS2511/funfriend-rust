use gl::types::*;
// use glium::Display;
// use glium::texture::{Texture2d, RawImage2d, SrgbTexture2d};

#[derive(Debug)]
pub struct SizedTexture {
	pub tex: GLuint,
	pub width: u32,
	pub height: u32,
}

#[derive(Debug)]
pub struct TextureBasket {
	pub textures: Vec<SizedTexture>,
	pub fps: f64,
	pub t: f64,
}

const DEFAULT_TEXTURE_PARAMS: [(GLuint, GLuint); 4] = [
	(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE),
	(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE),
	(gl::TEXTURE_MIN_FILTER, gl::NEAREST),
	(gl::TEXTURE_MAG_FILTER, gl::NEAREST),
];

impl TextureBasket {
	pub fn new(textures: Vec<SizedTexture>, fps: f64) -> TextureBasket {
		Self {
			textures,
			fps,
			t: 0.0,
		}
	}

	pub fn frame(&self) -> usize {
		let frame = (self.t * self.fps).floor() as usize;
		frame % self.textures.len()
	}

	pub fn texture(&self) -> &SizedTexture {
		&self.textures[self.frame()]
	}

	pub fn update(&mut self, delta: f64) {
		self.t += delta;
	}
}

pub fn load_texture(
	filepath: &str,
	params: Option<[(GLuint, GLuint); 4]>,
) -> Result<SizedTexture, String> {
	let img = image::open(filepath).map_err(|e| format!("Failed to open image: {}", e));
	let img = img?.to_rgba8();
	let (width, height) = img.dimensions();
	let params = params.unwrap_or(DEFAULT_TEXTURE_PARAMS);

	let mut texture: GLuint = 0;
	unsafe {
		gl::GenTextures(1, &mut texture);
		gl::BindTexture(gl::TEXTURE_2D, texture);

		for &(param, value) in &params {
			gl::TexParameteri(gl::TEXTURE_2D, param as GLenum, value as GLint);
		}

		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGBA as GLint,
			width as GLint,
			height as GLint,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			img.as_ptr() as *const GLvoid,
		);

		gl::GenerateMipmap(gl::TEXTURE_2D);
	}

	Ok(SizedTexture {
		tex: texture,
		width,
		height,
	})
}
