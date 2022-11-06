#!/bin/nano

use super::basic::{TermOp, Color, PassByteState, pass_byte};

pub struct TermBuffer<const WIDTH: usize, const TOTAL_HEIGHT: usize> {

	/// shouldn't be changed once set
	pub height: usize,
	pub real_screen_pos: usize,
	pub user_screen_pos: usize,
	pub cursor_x: usize,
	pub cursor_y: usize,
	pub current_color: CharColor,
	pub current_attr: CharAttr,
	pub pass_byte_state: PassByteState,
	pub buffer: [[Char; WIDTH]; TOTAL_HEIGHT],

}

pub struct TermBufferIter<'a, const WIDTH: usize> {

	/// slice of buffer pointers
	pub current_buffer: usize,
	pub x: usize,
	pub y: usize,
	pub buffers: [&'a [[Char; WIDTH]]; 2]

}

#[derive(Clone, Copy)]
pub struct Char {

	pub utf8_char: char,
	pub color: CharColor,
	pub attr: CharAttr,

}

#[derive(Clone, Copy)]
pub struct CharColor {

	/// foreground color (text color)
	pub fg: Color,
	/// background color
	pub bg: Color,
	/// this defines color for cursor, underscore, etc
	pub special: Color,

}

#[derive(Clone, Copy)]
pub struct CharAttr (

	/// corresponding bits:
	/// 0 – italic
	/// 1 – underscore
	/// 2 – blinking
	/// 3 – inverted colors (between bg and fg)
	/// 4 – strikethrough
	u8

);

impl<const WIDTH: usize, const TOTAL_HEIGHT: usize> TermBuffer<WIDTH, TOTAL_HEIGHT> {

	pub const fn new(height: usize) -> Self {

		TermBuffer{
			height,
			real_screen_pos: 0,
			user_screen_pos: 0,
			cursor_x: 0,
			cursor_y: 0,
			current_color: CharColor::default(),
			current_attr: CharAttr::default(),
			pass_byte_state: PassByteState::new(),
			buffer: [[Char::empty(); WIDTH]; TOTAL_HEIGHT],
		}
	}

	fn pass_op(&mut self, op: TermOp) {
		use TermOp::*;

		match op {

			Print(character) => {
				self.buffer[self.cursor_y][self.cursor_x] = Char::create(self.current_color.clone(), self.current_attr.clone(), character);
				self.cursor_x += 1;
			},
			Return => {
				self.color_line_rest();
				self.cursor_x = 0;
				self.cursor_y += 1;
				self.color_line_rest();
			},
			SingleLineReturn => {
				self.color_line_rest();
				self.cursor_x = 0;
			},
			BackSpace => if self.cursor_x != 0 {
				self.cursor_x -= 1;
				self.buffer[self.cursor_y][self.cursor_x] = Char::empty_colored(self.current_color);
			},
			Delete => {
				self.buffer[self.cursor_y][self.cursor_x] = Char::empty_colored(self.current_color);
				self.cursor_x += 1;
			},
			Tab => {
				let old_cursor_x = self.cursor_x;
				self.cursor_x = self.cursor_x / 8 * 8 + 8;
				for a in old_cursor_x..self.cursor_x {
					self.buffer[self.cursor_y][a].color = self.current_color.clone();
				}
			},
			ChFgColor(color) => {
				let mut color = color;
				color.combine_bright_bit(self.current_color.fg.bright_bit());
				self.current_color.fg = color;
				self.current_color.special = color;
			},
			ChBgColor(color) => {
				let mut color = color;
				color.combine_bright_bit(self.current_color.bg.bright_bit());
				self.current_color.bg = color;
			},
			Bright => {
				self.current_color.fg.combine_bright_bit(Color::BRIGHT_BIT);
				self.current_color.special.combine_bright_bit(Color::BRIGHT_BIT);
			},
			Dark => {
				self.current_color.fg.combine_bright_bit(0);
				self.current_color.special.combine_bright_bit(0);
			},
			Italic => self.current_attr.set_italic(CharAttr::ITALIC),
			UnderScore => self.current_attr.set_underscore(CharAttr::UNDERSCORE),
			Blinking => self.current_attr.set_blinking(CharAttr::BLINKING),
			Invert => self.current_attr.set_inverted(CharAttr::INVERTED),
			StrikeThrough => self.current_attr.set_crossed(CharAttr::CROSSED),
			Reset => {
				self.current_attr = CharAttr::default();
				self.current_color = CharColor::default();
			},
			FirstPos => {
				self.cursor_x = 0;
				self.cursor_y = self.real_screen_pos;
			},
			LastPos => {
				self.cursor_x = WIDTH - 1;
				self.cursor_y = self.real_screen_pos + self.height - 1;
			},
			CurUp => self.cursor_y -= 1,
			CurDown => self.cursor_y += 1,
			CurRight => self.cursor_x += 1,
			CurLeft => self.cursor_x -= 1,
			Nothing => {},

		}

		if self.cursor_x >= WIDTH {

			self.cursor_x -= WIDTH;
			self.cursor_y += 1;

		}

		if self.cursor_y >= TOTAL_HEIGHT {

			self.cursor_y -= TOTAL_HEIGHT;

		}

		// yes, negative unsigned
		let cursor_screen = self.real_screen_pos + self.height - self.cursor_y;

		if 0 > cursor_screen as isize {

			self.real_screen_pos -= cursor_screen;
			self.user_screen_pos = self.real_screen_pos;

		}

	}

	/// note that it only colors rest of line, it doesn't change any other values
	fn color_line_rest(&mut self) {

		for a in self.cursor_x..WIDTH {

			self.buffer[self.cursor_y][a].color = self.current_color.clone();

		}

	}

	pub fn write_byte(&mut self, byte: u8) {
		let mut pass_byte_state = self.pass_byte_state.clone();

		self.pass_op(pass_byte(&mut pass_byte_state, byte));

		self.pass_byte_state = pass_byte_state;

	}

	pub fn write_bytes(&mut self, bytes: &[u8]) {

		for byte in bytes {

			self.write_byte(*byte);

		}

	}

	pub fn iter(&self) -> TermBufferIter<WIDTH> {
		let mut screen_end = self.user_screen_pos + self.height;

		if screen_end >= TOTAL_HEIGHT {

			screen_end -= TOTAL_HEIGHT;

			TermBufferIter::<WIDTH>::new([&self.buffer[self.user_screen_pos..TOTAL_HEIGHT], &self.buffer[0..screen_end]])
		} else {

			TermBufferIter::<WIDTH>::new([&self.buffer[self.user_screen_pos..screen_end], &[]])
		}

	}

}

impl<const WIDTH: usize> TermBufferIter<'_, WIDTH> {

	fn new<'a, const BUF_WIDTH: usize>(buffers: [&'a [[Char; BUF_WIDTH]]; 2]) -> TermBufferIter<'a, BUF_WIDTH> {

		TermBufferIter::<BUF_WIDTH>{
			current_buffer: 0,
			x: 0,
			y: 0,
			buffers,
		}
	}

}

impl<const WIDTH: usize> Iterator for TermBufferIter<'_, WIDTH> {

	type Item = Char;

	fn next(&mut self) -> Option<Self::Item> {
		let character = self.buffers[self.current_buffer][self.y][self.x].clone();

		self.x += 1;

		if self.x >= WIDTH {

			self.x = 0;
			self.y += 1;

			if self.y >= self.buffers[self.current_buffer].len() {

				self.y = 0;
				self.current_buffer += 1;

				if self.current_buffer >= self.buffers.len() {

					return None
				}

			}

		}

		Some(character)
	}

}

impl Char {

	pub const fn empty() -> Self {

		Self::empty_colored(CharColor::default())
	}

	pub const fn empty_colored(color: CharColor) -> Self {

		Char{
			utf8_char: ' ',
			color,
			attr: CharAttr::default(),
		}
	}

	pub fn create(color: CharColor, attr: CharAttr, utf8_char: char) -> Self {

		Char{
			utf8_char,
			color,
			attr
		}
	}

}

impl CharColor {

	pub const fn default() -> Self {

		Self{
			fg: Color::BRIGHT_GREY,
			bg: Color::BLACK,
			special: Color::BRIGHT_GREY,
		}
	}

}

impl CharAttr {

	pub const ITALIC: u8 =     0b00001;
	pub const UNDERSCORE: u8 = 0b00010;
	pub const BLINKING: u8 =   0b00100;
	pub const INVERTED: u8 =   0b01000;
	pub const CROSSED: u8 =    0b10000;

	/// get default char attr with updated flag
	pub const fn default() -> Self {

		Self(0b00000000)
	}

	/// use `ITALIC` constant or zero
	pub fn set_italic(&mut self, bit: u8) {

		self.0 &= 0b11110;
		self.0 |= bit;

	}

	/// use `UNDERSCORE` constant or zero
	pub fn set_underscore(&mut self, bit: u8) {

		self.0 &= 0b11101;
		self.0 |= bit;

	}

	/// use `BLINKING` constant or zero
	pub fn set_blinking(&mut self, bit: u8) {

		self.0 &= 0b11011;
		self.0 |= bit;

	}

	/// use `INVERTED` constant or zero
	pub fn set_inverted(&mut self, bit: u8) {

		self.0 &= 0b10111;
		self.0 |= bit;

	}

	/// use `CROSSED` constant or zero
	pub fn set_crossed(&mut self, bit: u8) {

		self.0 &= 0b01111;
		self.0 |= bit;

	}

}
