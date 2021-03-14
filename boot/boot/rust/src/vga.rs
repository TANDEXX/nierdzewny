#!/bin/nano
#![allow(non_snake_case)]

use crate::consts::auto;
use auto::{WIDTH, HEIGHT, BUFFER_LEN};
use crate::consts::VGA;
use core::mem::transmute;

const REPLACER: u8 = b' ';
const DEFAULT_COLOR: Color = Color(auto::DEFAULT_COLOR);
const HIGHTLIGHT_COLOR: Color = Color(auto::HIGHTLIGHT_COLOR);
const DEFAULT_CHAR: VgaChar = VgaChar{char: REPLACER, color: DEFAULT_COLOR};
const VGA_SIZE: usize = WIDTH * HEIGHT;

enum Type {

	No,
	Fg,
	Bg,
	BothColor,
	DoNotCheck,

}

#[derive(Clone)]
struct VgaChar {

	char: u8,
	color: Color,

}

#[derive(Clone)]
struct Color (

	u8,

);

impl VgaChar {

	fn new() -> VgaChar {

		DEFAULT_CHAR
	}

}

static mut BUFFER: [VgaChar; BUFFER_LEN] = [DEFAULT_CHAR; BUFFER_LEN]; // buffer len is 80 * 25 * 10 by default
static mut CURPOS: usize = 0;
static mut CURRENTCOLOR: Color = DEFAULT_COLOR;
static mut SCREEN_POS: usize = 0;
static mut LOCK: bool = false;
static mut CHANGE_CUR_POS: bool = true;
static mut SETTING: Type = Type::No;

pub fn write_bytes(bytes: &[u8]) {

	for byte in bytes {

		write_byte(*byte);

	}

}

pub fn write_byte(byte: u8) {

	unsafe {

		while LOCK {}

		LOCK = true;

		match &SETTING {

			Type::Fg => {

				CURRENTCOLOR = Color(hex_to_num(byte) + bits_from(CURRENTCOLOR.0, 3));
				SETTING = Type::No;

			},
			Type::Bg => {

				CURRENTCOLOR = Color((hex_to_num(byte) * 16) + (CURRENTCOLOR.0 - bits_from(CURRENTCOLOR.0, 3)));
				SETTING = Type::No;

			},

			Type::BothColor => {

				CURRENTCOLOR = Color(byte);
				SETTING = Type::No;

			},

			Type::DoNotCheck => {

				write_vga_char(VgaChar{char: byte, color: CURRENTCOLOR.clone()});
				SETTING = Type::DoNotCheck;

			},

			Type::No => {

				if byte == 0 {

					write_vga_char(VgaChar{char: 10, color: CURRENTCOLOR.clone()});

				} else if byte == 1 {

					CURPOS = SCREEN_POS * WIDTH;

				} else if byte == 2 {

					CURPOS = SCREEN_POS * WIDTH + VGA_SIZE - WIDTH;

				} else if byte == 3 {

					SETTING = Type::Fg;

				} else if byte == 4 {

					SETTING = Type::Bg;

				} else if byte == 5 {

					SETTING = Type::BothColor;

				} else if byte == 6 {

					for _ in 0..(CURPOS % WIDTH + 1) % 2 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

				} else if byte == 7 {

					for _ in 0..(WIDTH - (CURPOS % WIDTH + 1)) % 4 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

				} else if byte == 8 {

					for _ in 0..(WIDTH - (CURPOS % WIDTH) + 3) % 6 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

				} else if byte == 9 {

					for _ in 0..(WIDTH - (CURPOS % WIDTH) + 7) % 8 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

				} else if byte == 10 {

					for _ in 0..WIDTH - CURPOS % WIDTH {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

				} else if byte == 12 {

					if CURPOS + WIDTH < BUFFER_LEN {

						CURPOS += WIDTH - CURPOS % WIDTH;

					}

				} else if byte == 13 {

					CURPOS -= CURPOS % WIDTH;

				} else if byte == 14 {

					SETTING = Type::DoNotCheck;

				} else if byte == 15 {

					CURRENTCOLOR = HIGHTLIGHT_COLOR;

				} else if byte == 16 {

					CURRENTCOLOR = DEFAULT_COLOR;

				} else if byte == 17 {

					if CURPOS < BUFFER_LEN {

						CURPOS += 1;

					}

				} else if byte == 18 {

					if CURPOS != 0 {

						CURPOS -= 1;

					}

				} else if byte == 19 {

					if CURPOS >= WIDTH {

						CURPOS -= WIDTH;

					}

				} else if byte == 20 {

					if CURPOS + WIDTH < BUFFER_LEN {

						CURPOS += WIDTH;

					}

				} else if byte == 24 {

					CHANGE_CUR_POS = !CHANGE_CUR_POS;

				} else if byte == 127 {

					if CURPOS != 0 {
						let ccp = CHANGE_CUR_POS.clone();

						CHANGE_CUR_POS = false;
						CURPOS -= 1;
						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});
						CHANGE_CUR_POS = ccp;

					}

				} else {

					write_vga_char(VgaChar{char: byte, color: CURRENTCOLOR.clone()});

				}

			},

		}

		rewrite_vga();

		LOCK = false;

	}

}

fn bits_from(num: u8, start: u8) -> u8 {
	let mut buffer = 0;

	for x in start..8 {

		buffer += num & (1 << x);

	}

	buffer
}

fn hex_to_num(char: u8) -> u8 {

	if char >= 97 && char <= 102 {

		return char - 87;

	} else if char >= 48 && char <= 57 {

		return char - 48;

	} else {

		0

	}

}

fn rewrite_vga() {
	let vga = VGA as *mut u8;
	let mut vgapos: isize = 0;

	unsafe {

		let pos = SCREEN_POS * WIDTH;

		for char in &BUFFER[pos..if pos + VGA_SIZE >= BUFFER_LEN {BUFFER_LEN - pos} else {pos + VGA_SIZE}] {

			*vga.offset(vgapos) = char.char;
			*vga.offset(vgapos + 1) = transmute(char.color.clone());

			vgapos += 2;

		}

		for _ in 0..VGA_SIZE as isize - vgapos {

			*vga.offset(vgapos) = b'.';
			*vga.offset(vgapos + 1) = 240;

			vgapos += 2;

		}

	}

}

fn write_vga_char(b: VgaChar) {

	unsafe {

		BUFFER[CURPOS] = b;

		if CHANGE_CUR_POS {

			CURPOS += 1;

			if (CURPOS % WIDTH == 0) && (CURPOS >= HEIGHT * WIDTH) && (CURPOS < BUFFER_LEN - WIDTH) {

				SCREEN_POS += 1;

			}

		}

		if CURPOS + WIDTH >= BUFFER_LEN {

			down();

		}

	}

}

pub fn down() {

	unsafe {

		for i in WIDTH..BUFFER_LEN - WIDTH {

			BUFFER[i - WIDTH] = BUFFER[i].clone();

		}

		for i in BUFFER_LEN - (WIDTH * 2)..BUFFER_LEN - WIDTH {

			BUFFER[i] = VgaChar::new();

		}

		CURPOS -= WIDTH;

	}

}
