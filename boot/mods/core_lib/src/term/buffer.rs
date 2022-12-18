#!/bin/nano

use super::basic::{TermOp, Color, PassByteState, pass_byte};

pub type PaintFn = fn(usize, usize, Char);

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
	/// arguments are: x, y on screen, char
	pub paint_fn: PaintFn,

}

impl<const WIDTH: usize, const TOTAL_HEIGHT: usize> TermBuffer<WIDTH, TOTAL_HEIGHT> {

	pub const fn new(height: usize, paint_fn: PaintFn) -> Self {

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
			paint_fn,
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
					self.buffer[self.cursor_y][a].attr.set_changed(CharAttr::CHANGED);
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
			self.buffer[self.cursor_y][a].attr.set_changed(CharAttr::CHANGED);

		}

	}

	/// `draw_it` variable should be set to true if you want to output single byte,
	/// should be set to false if you want to output series of bytes
	pub fn write_byte(&mut self, byte: u8, draw_it: bool) {
		let mut pass_byte_state = self.pass_byte_state.clone();

		self.pass_op(pass_byte(&mut pass_byte_state, byte));

		self.pass_byte_state = pass_byte_state;

		if draw_it {
			self.repaint(false);
		}

	}

	/// `draw_it` variable should be set to true
	pub fn write_bytes(&mut self, bytes: &[u8], draw_it: bool) {

		for byte in bytes {

			self.write_byte(*byte, false);

		}

		if draw_it {
			self.repaint(false);
		}

	}

	pub fn repaint(&mut self, all: bool) {
		let mut buffer_y = self.user_screen_pos;
		let mut screen_y = 0;

		while screen_y < self.height {
			let mut x = 0;

			while x < WIDTH {

				if all || self.buffer[buffer_y][x].attr.is_changed() {
					(self.paint_fn)(x, screen_y, self.buffer[buffer_y][x].clone());
				}
				self.buffer[buffer_y][x].attr.set_changed(0);

				x += 1;
			}

			buffer_y += 1;
			screen_y += 1;

			if buffer_y >= TOTAL_HEIGHT {

				buffer_y = 0;

			}

		}

	}

}

#[derive(Clone, Copy)]
pub struct Char {

	pub utf8_char: char,
	pub color: CharColor,
	pub attr: CharAttr,

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

#[derive(Clone, Copy)]
pub struct CharColor {

	/// foreground color (text color)
	pub fg: Color,
	/// background color
	pub bg: Color,
	/// this defines color for cursor, underscore, etc
	pub special: Color,

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

#[derive(Clone, Copy)]
pub struct CharAttr (

	/// corresponding bits:
	/// 0 – italic
	/// 1 – underscore
	/// 2 – blinking
	/// 3 – inverted colors (between bg and fg)
	/// 4 – strikethrough
	/// 5 – changed, flag is set when this character is changed and is unset when painting
	u8

);

impl CharAttr {

	pub const ITALIC: u8 =     0b000001;
	pub const UNDERSCORE: u8 = 0b000010;
	pub const BLINKING: u8 =   0b000100;
	pub const INVERTED: u8 =   0b001000;
	pub const CROSSED: u8 =    0b010000;
	pub const CHANGED: u8 =    0b100000;

	/// get default char attr with changed flag
	pub const fn default() -> Self {

		Self(0b00100000)
	}

	/// use `ITALIC` constant or zero
	pub fn set_italic(&mut self, bit: u8) {

		self.0 &= 0b111110;
		self.0 |= bit;

	}

	/// use `UNDERSCORE` constant or zero
	pub fn set_underscore(&mut self, bit: u8) {

		self.0 &= 0b111101;
		self.0 |= bit;

	}

	/// use `BLINKING` constant or zero
	pub fn set_blinking(&mut self, bit: u8) {

		self.0 &= 0b111011;
		self.0 |= bit;

	}

	/// use `INVERTED` constant or zero
	pub fn set_inverted(&mut self, bit: u8) {

		self.0 &= 0b110111;
		self.0 |= bit;

	}

	/// use `CROSSED` constant or zero
	pub fn set_crossed(&mut self, bit: u8) {

		self.0 &= 0b101111;
		self.0 |= bit;

	}

	/// use `CHANGED` constant or zero
	pub fn set_changed(&mut self, bit: u8) {

		self.0 &= 0b011111;
		self.0 |= bit;

	}

	pub fn is_changed(self) -> bool {

		self.0 & Self::CHANGED != 0
	}

	pub fn change(self) -> Self {
		let mut result = self;
		result.set_changed(Self::CHANGED);

		result
	}

}
