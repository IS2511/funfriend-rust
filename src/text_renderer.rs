use gl::types::*;
use std::ffi::CString;

use super::{font_manager::*, glfn, texture::load_texture};

pub struct TextRenderer {
	pub text: String,
	pub font: String,
	pub sheet: BMSheet,
	pub width: i32,
	pub height: i32,

	pub shader_program: GLuint,
	pub vertex_array: GLuint,
	pub vertex_buffer: GLuint,
	pub font_texture: GLuint,
}

impl TextRenderer {
	pub fn new(text: String, font: String, sheet: BMSheet, width: i32, height: i32) -> Self {
		let shader_program = glfn::shader("nop.frag", "nop.vert");
		let (vertex_array, vertex_buffer) = Self::init_buffers(&text, &sheet, width, height);
		let font_texture = Self::init_textures(&font);
		Self {
			text,
			font,
			sheet,
			width,
			height,
			shader_program,
			vertex_array,
			vertex_buffer,
			font_texture,
		}
	}

	//noinspection RsCStringPointer
	pub fn render(&self, dt: f64) {
		unsafe {
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			gl::ActiveTexture(gl::TEXTURE0);
			gl::BindTexture(gl::TEXTURE_2D, self.font_texture);

			gl::UseProgram(self.shader_program);
			gl::Uniform1i(
				gl::GetUniformLocation(
					self.shader_program,
					CString::new("texture1").unwrap().as_ptr(),
				),
				0,
			);

			gl::BindVertexArray(self.vertex_array);
			gl::DrawElements(
				gl::TRIANGLES,
				6 * self.text.len() as i32,
				gl::UNSIGNED_INT,
				std::ptr::null(),
			);
		}
	}

	pub fn init_textures(font: &str) -> GLuint {
		let texture_data = load_texture(
			&format!("{}.png", font),
			Some([
				(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER),
				(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER),
				(gl::TEXTURE_MIN_FILTER, gl::LINEAR),
				(gl::TEXTURE_MAG_FILTER, gl::LINEAR),
			]),
		);
		texture_data.unwrap().tex
	}

	fn init_buffers(text: &str, sheet: &BMSheet, width: i32, height: i32) -> (GLuint, GLuint) {
		let text_width = FontMan::text_width(text, sheet);
		let text_height = sheet.common.line_height;
		let (vertices, indices) = FontMan::get_text_mesh(
			text,
			sheet,
			width / 2 - text_width / 2,
			height / 2 - text_height / 2,
			width,
			height,
		);

		let mut vertex_array: GLuint = 0;
		let mut vertex_buffer: GLuint = 0;
		let mut element_buffer: GLuint = 0;

		unsafe {
			gl::GenVertexArrays(1, &mut vertex_array);
			gl::GenBuffers(1, &mut vertex_buffer);
			gl::GenBuffers(1, &mut element_buffer);

			gl::BindVertexArray(vertex_array);
			gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
				vertices.as_ptr() as *const _,
				gl::STATIC_DRAW,
			);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer);
			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
				indices.as_ptr() as *const _,
				gl::STATIC_DRAW,
			);

			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				(5 * std::mem::size_of::<f32>()) as GLsizei,
				std::ptr::null(),
			);
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE,
				(5 * std::mem::size_of::<f32>()) as GLsizei,
				(3 * std::mem::size_of::<f32>()) as *const _,
			);
			gl::EnableVertexAttribArray(1);
		}
		(vertex_array, vertex_buffer)
	}

	pub fn clean_up(&self) {
		unsafe {
			gl::DeleteVertexArrays(1, &self.vertex_array);
			gl::DeleteBuffers(1, &self.vertex_buffer);
		}
	}
}
