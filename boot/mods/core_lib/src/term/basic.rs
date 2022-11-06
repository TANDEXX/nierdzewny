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
	ChFgColor(BasicColor),
	ChBgColor(BasicColor),
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

#[repr(u8)]
pub enum BasicColor {

	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Violet,
	Cyan,
	White,

}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FullColor {

	Black,
	Red,
	Green,
	Orange,
	Blue,
	Violet,
	Cyan,
	LightGrey,

	DarkGrey,
	LightRed,
	LightGreen,
	Yellow,
	LightBlue,
	Pink,
	LightCyan,
	White,

}

#[derive(Clone)]
pub enum PassByteState {

	Normal,
	HalfEsc,
	Esc,
	EscNum(u8),
	EscDoubleNum(u8, u8),

}

impl BasicColor {

	fn from_byte(byte: u8) -> Option<Self> {
		use BasicColor::*;

		match byte {
			b'0' => Some(Black),
			b'1' => Some(Red),
			b'2' => Some(Green),
			b'3' => Some(Yellow),
			b'4' => Some(Blue),
			b'5' => Some(Violet),
			b'6' => Some(Cyan),
			b'7' => Some(White),
			_ => None,
		}
	}

	pub fn into_full_color(self, bright_bit: bool) -> FullColor {
		// how to do it safely without transmute?
		let mut result = unsafe {core::mem::transmute::<BasicColor, FullColor>(self)};

		result.set_bright(bright_bit);

		result
	}

}

impl FullColor {

	pub fn set_bright(&mut self, bright: bool) {
		let mut self_u8 = self as * const _ as u8;

		self_u8 |= (bright as u8) << 3;
		// I don't know how to do it safely without transmute
		*self = unsafe {core::mem::transmute(self_u8)};

	}

}

impl PassByteState {

	/// creates new, empty
	pub const fn new() -> Self {

		Self::Normal
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
			b'3' => match BasicColor::from_byte(sec_byte) {
				Some(color) => TermOp::ChFgColor(color),
				None => TermOp::Nothing,
			},
			b'4' => match BasicColor::from_byte(sec_byte) {
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
