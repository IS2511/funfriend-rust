use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BMCommon {
	pub line_height: i32,
	pub base: i32,
	pub scale_w: i32,
	pub scale_h: i32,
}

#[derive(Debug, Clone)]
pub struct BMChar {
	pub id: i32,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub x_offset: i32,
	pub y_offset: i32,
	pub x_advance: i32,
	pub letter: char,
}

#[derive(Debug, Clone)]
pub struct BMKerning {
	pub first: i32,
	pub second: i32,
	pub amount: i32,
}

#[derive(Debug, Clone)]
pub struct BMSheet {
	pub common: BMCommon,
	pub chars: Vec<BMChar>,
	pub kernings: Vec<BMKerning>,
}

pub struct FontMan;

impl FontMan {
	pub fn parse_bm(data: &str) -> BMSheet {
		let mut common = None;
		let mut chars = Vec::new();
		let mut kernings = Vec::new();

		for line in data.lines() {
			let words: Vec<&str> = line.split_whitespace().collect();
			if words.is_empty() {
				continue;
			}

			let key = words[0];
			let args = words[1..].join(" ");
			let args_map = FontMan::parse_args(&args);

			match key {
				"common" => {
					common = Some(BMCommon {
						line_height: args_map["lineHeight"].parse().unwrap(),
						base: args_map["base"].parse().unwrap(),
						scale_w: args_map["scaleW"].parse().unwrap(),
						scale_h: args_map["scaleH"].parse().unwrap(),
					});
				}
				"char" => chars.push(BMChar {
					id: args_map["id"].parse().unwrap(),
					x: args_map["x"].parse().unwrap(),
					y: args_map["y"].parse().unwrap(),
					width: args_map["width"].parse().unwrap(),
					height: args_map["height"].parse().unwrap(),
					x_offset: args_map["xoffset"].parse().unwrap(),
					y_offset: args_map["yoffset"].parse().unwrap(),
					x_advance: args_map["xadvance"].parse().unwrap(),
					letter: args_map["letter"].parse().unwrap(),
				}),
				"kerning" => kernings.push(BMKerning {
					first: args_map["first"].parse().unwrap(),
					second: args_map["second"].parse().unwrap(),
					amount: args_map["amount"].parse().unwrap(),
				}),
				_ => {}
			}
		}
		BMSheet {
			common: common.unwrap(),
			chars,
			kernings,
		}
	}

	fn parse_args(data: &String) -> HashMap<String, String> {
		let mut args = HashMap::new();
		let pairs: Vec<&str> = data.split_whitespace().collect();
		for pair in pairs {
			let mut split = pair.splitn(2, '=');
			let key = split.next().unwrap().to_string();
			let value = split.next().unwrap().to_string();
			args.insert(key, value);
		}
		args
	}

	pub fn text_width(text: &str, sheet: &BMSheet) -> i32 {
		text.chars().fold(0, |width, char| {
			let bm_char = sheet.chars.iter().find(|&c| c.letter == char).unwrap();
			width + bm_char.x_advance
		})
	}

	pub fn position_text(text: &str, sheet: &BMSheet) -> (i32, i32, Vec<(i32, i32, BMChar)>) {
		let mut positions = Vec::new();
		let x = 0;

		for char in text.chars() {
			let bm_char = sheet.chars.iter().find(|&c| c.letter == char).unwrap();
			positions.push((
				x + bm_char.x_offset,
				sheet.common.base - bm_char.height - bm_char.y_offset,
				bm_char.clone(),
			))
		}

		(x, sheet.common.line_height, positions)
	}

	pub fn get_letter_crop(char: &BMChar, sheet: &BMSheet) -> (f32, f32, f32, f32) {
		let x = char.x as f32 / sheet.common.scale_w as f32;
		let y = char.y as f32 / sheet.common.scale_h as f32;
		let w = char.width as f32 / sheet.common.scale_w as f32;
		let h = char.height as f32 / sheet.common.scale_h as f32;

		(x, y, w, h)
	}

	pub fn get_text_mesh(
		text: &str,
		sheet: &BMSheet,
		offset_x: i32,
		offset_y: i32,
		width: i32,
		height: i32,
	) -> (Vec<f32>, Vec<u32>) {
		let mut vertices = Vec::new();
		let mut indices = Vec::new();

		let (text_width, text_height, positions) = Self::position_text(&text, sheet);
		let mut i = 0;

		for letter in positions {
			let char = &letter.2;
			let (x, y, w, h) = Self::get_letter_crop(char, sheet);

			let pos_x = ((letter.0 + offset_x) as f32 / width as f32) * 2.0 - 1.0;
			let pos_w = (char.width as f32 / width as f32) * 2.0;
			let pos_y = ((letter.1 + offset_y) as f32 / height as f32) * 2.0 - 1.0;
			let pos_h = (char.height as f32 / height as f32) * 2.0;

			vertices.extend_from_slice(&[
				pos_x + pos_w,
				pos_y + pos_h,
				0.0,
				x + w,
				y,
				pos_x + pos_w,
				pos_y,
				0.0,
				x + w,
				y + h,
				pos_x,
				pos_y,
				0.0,
				x,
				y + h,
				pos_x,
				pos_y + pos_h,
				0.0,
				x,
				y,
			]);

			indices.extend_from_slice(&[
				0 + i * 4,
				1 + i * 4,
				3 + i * 4,
				1 + i * 4,
				2 + i * 4,
				3 + i * 4,
			]);
			i += 1;
		}

		(vertices, indices)
	}
}
