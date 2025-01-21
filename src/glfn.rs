use gl::types::*;
use std::ffi::CString;
use std::fs::File;
use std::io::Read as _;
use std::ptr;

pub fn buffer_data_array(target: GLenum, data: &[u8], usage_hint: GLenum) {
	let data_size = data.len();

	unsafe {
		let mut buffer: GLuint = 0;
		gl::GenBuffers(1, &mut buffer);
		gl::BindBuffer(target, buffer);

		gl::BufferData(target, data_size as GLsizeiptr, ptr::null(), usage_hint);

		gl::BufferSubData(
			target,
			0,
			data_size as GLsizeiptr,
			data.as_ptr() as *const _,
		);
	}
}

pub fn shader(fragment: &str, vertex: &str) -> GLuint {
	
	let vertex_shader = compile_shader(vertex, gl::VERTEX_SHADER);
	let fragment_shader = compile_shader(fragment, gl::FRAGMENT_SHADER);

	unsafe {
		let program = gl::CreateProgram();
		gl::AttachShader(program, vertex_shader);
		gl::AttachShader(program, fragment_shader);
		gl::LinkProgram(program);

		let mut success: GLint = 0;
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			let mut info_log: Vec<u8> = Vec::with_capacity(512);
			gl::GetProgramInfoLog(
				program,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut i8,
			);
			tracing::error!("ERROR::PROGRAM::LINKING_FAILED\n{:?}", info_log);
		}
		program
	}
}

fn load_shader_file(vertex_filename: &str) -> String {
	let mut file = File::open(vertex_filename).expect("Failed to open shader file.");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Failed to read shader file.");
	contents
}

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
	let shader = unsafe { gl::CreateShader(shader_type) };
	let c_str = CString::new(source.as_bytes()).unwrap();

	unsafe {
		gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
		gl::CompileShader(shader);
	}

	let mut success: GLint = 0;
	unsafe {
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
	}
	if success == gl::FALSE as GLint {
		let mut info_log: Vec<u8> = Vec::with_capacity(512);
		unsafe {
			gl::GetShaderInfoLog(
				shader,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut i8,
			);
		}
		tracing::error!("ERROR::SHADER::COMPILATION_FAILED\n{:?}", info_log);
	}

	shader
}
