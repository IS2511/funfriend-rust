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
					letter: args_map["letter"].parse().unwrap_or(' '),
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
		tracing::info!("Got data to parse: {}", data);

		let mut args = HashMap::new();
		let mut chars = data.chars().peekable();
		let mut current_key = String::new();
		let mut current_value = String::new();
		let mut in_quotes = false;
		let mut parsing_key = true; // Flag to differentiate between key and value parsing
		let mut is_value = false; // To track when we're parsing values after '='

		// Iterate through each character in the string
		while let Some(ch) = chars.next() {
			match ch {
				'=' if parsing_key => {
					// Once we encounter '=', switch to value parsing mode
					parsing_key = false;
					is_value = true;  // Now we will be capturing the value
				}
				'"' => {
					// Handle quotes for value
					if in_quotes {
						// Closing quote, store value if key and value are set
						args.insert(current_key.clone(), current_value.clone());
						current_key.clear();
						current_value.clear();
						parsing_key = true; // We expect a new key now
					}
					in_quotes = !in_quotes; // Toggle the quote flag
				}
				' ' | '\t' | '\n' => {
					// Skip whitespace, unless we are inside quotes or currently parsing a value
					if !in_quotes && !parsing_key && !current_key.is_empty() && !current_value.is_empty() {
						// Finished parsing a key-value pair
						args.insert(current_key.clone(), current_value.clone());
						current_key.clear();
						current_value.clear();
						parsing_key = true; // Now we expect a new key
					}
				}
				_ => {
					// Accumulate characters for key or value
					if parsing_key {
						current_key.push(ch); // If parsing key, build the key
					} else if in_quotes {
						current_value.push(ch); // If in quotes, accumulate the value
					} else {
						current_value.push(ch); // Otherwise, accumulate the value normally
					}
				}
			}
		}

		// Handle any remaining key-value pair
		if !current_key.is_empty() && !current_value.is_empty() {
			args.insert(current_key, current_value);
		}
		tracing::info!("Got args: {:?}", args);
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
