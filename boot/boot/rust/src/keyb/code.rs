#!/bin/nano

use core::mem::transmute;
use crate::device::{State, STATE_NULL, write, new, defaults::KEY_INDEX as KEY};

static mut SPECIAL: bool = false;
static mut DEV_DESC: State = STATE_NULL;

/// key definition structure
pub struct Key {

	/// what character it is
	pub char: u8,
	/// do it is pressed or released
	/// true if pressing, false if releasing
	pub press: bool,

}

/// init
pub fn init() {

	unsafe {

		DEV_DESC = new(KEY);

	}

}

/// transforming raw input to ascii and control codes
pub fn read(input: u8) {
	let mut press = true;
	let mut input = input;
	let k: u8;

	if input > 127 {

		press = false;
		input -= 128;

	}

	if unsafe {SPECIAL} {

		k = match input {

			8 => 30,
			28 => 13,
			29 => 29,
			31 => 31,
			56 => 30,
			71 => 1,
			72 => 19,
			73 => 3,
			75 => 18,
			77 => 17,
			80 => 20,
			81 => 4,
			79 => 2,
			82 => 6,
			83 => 5,
			91 => 0,
			_ => 0,

		};

		unsafe {SPECIAL = false;}

	} else {

		k = match input {

			1 => 27,
			2..=10 => input + 47,
			11 => b'0',
			12 => b'-',
			13 => b'=',
			14 => 127,
			15 => 9,
			16 => b'q',
			17 => b'w',
			18 => b'e',
			19 => b'r',
			20 => b't',
			21 => b'y',
			22 => b'u',
			23 => b'i',
			24 => b'o',
			25 => b'p',
			26 => b'[',
			27 => b']',
			28 => 10,
			29 => 31,
			30 => b'a',
			31 => b's',
			32 => b'd',
			33 => b'f',
			34 => b'g',
			35 => b'h',
			36 => b'j',
			37 => b'k',
			38 => b'l',
			39 => b';',
			40 => b'\'',
			41 => b'`',
			42 => 29,
			43 => b'\\',
			44 => b'z',
			45 => b'x',
			46 => b'c',
			47 => b'v',
			48 => b'b',
			49 => b'n',
			50 => b'm',
			51 => b',',
			52 => b'.',
			53 => b'/',
			54 => 29,
			55 => b'*',
			56 => 30,
			57 => b' ',
			58 => 255,
			59 => 14,
			60 => 15,
			61 => 16,
			62 => 22,
			63 => 23,
			64 => 24,
			65 => 25,
			66 => 26,
			67 => 28,
//			68 => 24,
//			87 => 25,
//			88 => 26,
			96 => unsafe {SPECIAL = true; b'!'},
			_ => 0, // TODO num lock and these numers

		};

	}

	unsafe {

		if !SPECIAL {

			write(KEY, &mut DEV_DESC, k);
			write(KEY, &mut DEV_DESC, transmute(press));

		}

	}

}
