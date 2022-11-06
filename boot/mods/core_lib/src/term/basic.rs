#!/bin/nano
//! functions providing terminal syntax

pub enum TermOp {

	Nothing,
	Print(char),
	Return,
	/// like normal return, but doesn't take cursor to next line
	SingleLineReturn,
	Tab,
	BackSpace,
	Delete,
	ChFgColor(Color),
	ChBgColor(Color),
	Bright,
	Dark,
	Italic,
	UnderScore,
	Blinking,
	Invert,
	StrikeThrough,
	Reset,
	/// home button, takes cursor to first character on screen (not buffer start)
	FirstPos,
	/// end button, takes cursor to last character on screen (not buffer end)
	LastPos,
	CurUp,
	CurDown,
	CurRight,
	CurLeft,

}

#[derive(Clone)]
pub enum PassByteState {

	Normal,
	HalfEsc,
	Esc,
	EscNum(u8),
	EscDoubleNum(u8, u8),

}

#[derive(Clone, Copy)]
pub struct Color (pub u8);

impl PassByteState {

	/// creates new, empty
	pub const fn new() -> Self {

		Self::Normal
	}

}

impl Color {

	pub const BLACK: Self =        Self(0b0000);
	pub const BLUE: Self =         Self(0b0001);
	pub const GREEN: Self =        Self(0b0010);
	pub const CYAN: Self =         Self(0b0011);
	pub const RED: Self =          Self(0b0100);
	pub const VIOLET: Self =       Self(0b0101);
	pub const ORANGE: Self =       Self(0b0110);
	pub const BRIGHT_GREY: Self =  Self(0b0111);

	pub const DARK_GREY: Self =    Self(0b1000);
	pub const BRIGHT_BLUE: Self =  Self(0b1001);
	pub const BRIGHT_GREEN: Self = Self(0b1010);
	pub const BRIGHT_CYAN: Self =  Self(0b1011);
	pub const BRIGHT_RED: Self =   Self(0b1100);
	pub const PINK: Self =         Self(0b1101);
	pub const YELLOW: Self =       Self(0b1110);
	pub const WHITE: Self =        Self(0b1111);

	pub const BRIGHT_BIT: u8 = 0b1000;

	pub fn from_byte_code(byte: u8) -> Option<Self> {

		match byte {
			b'0' => Some(Self::BLACK),
			b'1' => Some(Self::RED),
			b'2' => Some(Self::GREEN),
			b'3' => Some(Self::ORANGE),
			b'4' => Some(Self::BLUE),
			b'5' => Some(Self::VIOLET),
			b'6' => Some(Self::CYAN),
			b'7' => Some(Self::BRIGHT_GREY),
			_ => None,
		}
	}

	pub fn bright_bit(&self) -> u8 {

		self.0 & 0b1000
	}

	/// if you want to set bright bit manualy, then enter BRIGHT_BIT constant or zero depending on what you want to set
	pub fn combine_bright_bit(&mut self, bright_bit: u8) {

		self.0 &= 0b0111;
		self.0 |= bright_bit;

	}

}

pub fn pass_byte(state: &mut PassByteState, byte: u8) -> TermOp {

	use PassByteState::*;

	fn esc_num_pass(first_byte: u8, sec_byte: u8) -> TermOp {

		match first_byte {
			b'0' => match sec_byte {
				b'0' => TermOp::Reset,
				b'1' => TermOp::Bright,
				b'2' => TermOp::Dark,
				b'3' => TermOp::Italic,
				b'4' => TermOp::UnderScore,
				b'5' => TermOp::Blinking,
				b'6' => TermOp::Nothing, // mainly implemented of astetic, but what it does?
				b'7' => TermOp::Invert,
				b'8' => TermOp::Nothing, // same as 6, don't know what it does
				b'9' => TermOp::StrikeThrough,
				_ => TermOp::Nothing,
			},
			b'3' => match Color::from_byte_code(sec_byte) {
				Some(color) => TermOp::ChFgColor(color),
				None => TermOp::Nothing,
			},
			b'4' => match Color::from_byte_code(sec_byte) {
				Some(color) => TermOp::ChBgColor(color),
				None => TermOp::Nothing,
			},
			_ => TermOp::Nothing,
		}
	}

	match state {

		Normal => match byte {
			9 => TermOp::Tab,
			10 => TermOp::Return,
			13 => TermOp::SingleLineReturn,
			27 => {
				*state = HalfEsc;
				TermOp::Nothing
			},
			127 => TermOp::BackSpace,
			_ => TermOp::Print(byte as char),
		},
		HalfEsc => {
			if byte == b'[' {
				*state = Esc;
			} else {
				*state = Normal;
			}
			TermOp::Nothing
		},
		Esc => {
			*state = Normal;
			match byte {

				b'0'..b'9' => {
					*state = EscNum(byte);
					TermOp::Nothing
				},
				b'A' => TermOp::CurUp,
				b'B' => TermOp::CurDown,
				b'C' => TermOp::CurRight,
				b'D' => TermOp::CurLeft,
				b'F' => TermOp::LastPos,
				b'H' => TermOp::FirstPos,
				_ => TermOp::Nothing,

		}},
		EscNum(first_byte_borrow) => {
			let first_byte = first_byte_borrow.clone();
			*state = Normal;

			match byte {

				b'0'..b'9' => {
					*state = EscDoubleNum(first_byte, byte);
					TermOp::Nothing
				},
				b';' => {
					*state = Esc;
					esc_num_pass(b'0', first_byte)
				},
				b'm' => esc_num_pass(b'0', first_byte),
				b'~' => match first_byte {
					b'3' => TermOp::Delete,
					_ => TermOp::Nothing,
				},
				_ => TermOp::Nothing,

		}},
		EscDoubleNum(first_byte_borrow, sec_byte_borrow) => {
			let first_byte = first_byte_borrow.clone();
			let sec_byte = sec_byte_borrow.clone();
			*state = Normal;

			match byte {
				b';' => {
					*state = Esc;
					esc_num_pass(first_byte, sec_byte)
				},
				b'm' => esc_num_pass(first_byte, sec_byte),
				_ => TermOp::Nothing,
			}
		},

	}
}
