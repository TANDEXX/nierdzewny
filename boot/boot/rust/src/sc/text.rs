#!/bin/nano
#![allow(non_snake_case)]

//use core::mem::transmute;
use crate::consts::auto::{DEFAULT_COLOR, HIGHTLIGHT_COLOR, WIDTH, HEIGHT, BUFFER_LEN};

/// default replacer character
const REPLACER: u8 = b' ';
/// terminal replacer
const DEFAULT_CHAR: VgaChar = VgaChar{char: REPLACER, color: DEFAULT_COLOR};
/// size of vga terminal
const SIZE: usize = WIDTH * HEIGHT;
const MAX_SCREEN_POS: usize = (BUFFER_LEN - SIZE) / WIDTH;

/// enumeration to pass next character
enum Type {

	No,
	Fg,
	Bg,
	BothColor,
	DoNotCheck,

}

/// vga character
#[derive(Clone)]
struct VgaChar {

	char: u8,
	color: u8,

}

/// vga character methods
impl VgaChar {

/*	fn new() -> VgaChar {

		DEFAULT_CHAR
	}
*/
}

/// terminal write function
pub type TermWrite = fn(u8, u8, usize);
/// terminal size function (cange terminal size)
pub type TermSize = fn() -> Result<(), &'static [u8]>;
/// terminal cursor pos change
pub type TermCur = fn(usize);
/// terminal cursor remove
pub type TermCurDisable = fn();

/// terminal buffer
static mut BUFFER: [VgaChar; BUFFER_LEN] = [DEFAULT_CHAR; BUFFER_LEN]; // buffer len is 80 * 25 * 10 by default
/// cursor position
static mut CURPOS: usize = 0;
static mut CURRENTCOLOR: u8 = DEFAULT_COLOR;
static mut LASTCOLOR: u8 = DEFAULT_COLOR;
/// current screen position on terminal
pub static mut SCREEN_POS: usize = 0;
/// current virtual screen position (current scrolled)
pub static mut SCREEN_POS_VIRT: usize = 0;
/// lock vga buffer
static mut LOCK: bool = false;
/// terminal state (change cursor position to right when printing one character)
static mut CHANGE_CUR_POS: bool = true;
/// current terminal state (do change color, how change color, do print control characters as normal, etc)
static mut SETTING: Type = Type::No;
/// terminal write function for writing characters at screen
/// arguments: character, color, offset
static mut TERM_WRITE: TermWrite = term_write_any;
/// terminal set size function for setting terminal size
/// output: operation result
/// new width and height is in `HEIGHT` and `WIDTH` statics
pub static mut TERM_SIZE: TermSize = term_size_any;
/// terminal cursor function for changing cursor position
/// aruments: offset
pub static mut TERM_CUR: TermCur = term_cur_any;
/// terminal cursor disable function for removing cursor 
pub static mut TERM_CUR_DISABLE: TermCurDisable = term_cur_disable_any;

/// default terminal size change function
fn term_size_any() -> Result<(), &'static [u8]> {Err(b"terminal not set")}
/// default terminal write function
fn term_write_any(_: u8, _: u8, _: usize) {}
/// default terminal cursor move function
fn term_cur_any(_: usize) {}
/// default terminal cursor remove function
fn term_cur_disable_any() {}

/// terminal setting function
pub fn set_term(write: TermWrite, size: TermSize, cur: TermCur, cur_disable: TermCurDisable) {

	unsafe {

		while LOCK {}

		LOCK = true;

		TERM_WRITE = write;
		TERM_SIZE = size;
		TERM_CUR = cur;
		TERM_CUR_DISABLE = cur_disable;

		LOCK = false;

	}

}

/// write multiple bytes to vga buffer
/// # Example
/// ```
/// write_bytes(b"hello, ");
/// write_bytes(&[119, 111, 114, 108, 100, 10]); // world\n
/// ```
pub fn write_bytes(bytes: &[u8]) {

	for byte in bytes {

//		write_byte(*byte);

	}

}

/// pass control characters and write it to buffer
/// # Example
/// ```
/// write_byte(b'h');
/// write_byte(b'101'); // e in ascii
///
/// for byte in b"llo\n" {
///
/// 	write_byte(byte);
///
/// }
/// ```
pub fn write_byte(byte: u8) {

	unsafe {

		while LOCK {}

		LOCK = true;

		match &SETTING {

			Type::Fg => {

				LASTCOLOR = CURRENTCOLOR;
				CURRENTCOLOR = hex_to_num(byte) + (CURRENTCOLOR & 0b11110000);
				SETTING = Type::No;

			},
			Type::Bg => {

				LASTCOLOR = CURRENTCOLOR;
				CURRENTCOLOR = (hex_to_num(byte) << 4) + (CURRENTCOLOR & 0b00001111);
				SETTING = Type::No;

			},

			Type::BothColor => {

				LASTCOLOR = CURRENTCOLOR;
				CURRENTCOLOR = byte;
				SETTING = Type::No;

			},

			Type::DoNotCheck => {

				write_vga_char(VgaChar{char: byte, color: CURRENTCOLOR.clone()});
				SETTING = Type::No;

			},

			Type::No => {

				if byte == 0 {

					write_vga_char(VgaChar{char: 10, color: CURRENTCOLOR.clone()});

				} else if byte == 1 {

					CURPOS = SCREEN_POS * WIDTH;

				} else if byte == 2 {

					CURPOS = SCREEN_POS * WIDTH + SIZE - WIDTH;

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

					rewrite_vga();

				} else if byte == 7 {

					for _ in 0..(WIDTH - (CURPOS % WIDTH + 1)) % 4 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

					rewrite_vga();

				} else if byte == 8 {

					for _ in 0..(WIDTH - (CURPOS % WIDTH) + 3) % 6 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

					rewrite_vga();

				} else if byte == 9 {

					for _ in 0..(WIDTH - (CURPOS % WIDTH) + 7) % 8 + 1 {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

					rewrite_vga();

				} else if byte == 10 {

					for _ in 0..WIDTH - CURPOS % WIDTH {

						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});

					}

					SCREEN_POS_VIRT = SCREEN_POS.clone();
					rewrite_vga();

				} else if byte == 12 {

					if CURPOS + WIDTH < BUFFER_LEN {

						CURPOS += WIDTH - CURPOS % WIDTH;

					}

				} else if byte == 13 {

					CURPOS -= CURPOS % WIDTH;

				} else if byte == 14 {

					SETTING = Type::DoNotCheck;

				} else if byte == 15 {

					LASTCOLOR = CURRENTCOLOR;
					CURRENTCOLOR = HIGHTLIGHT_COLOR;

				} else if byte == 16 {

					LASTCOLOR = CURRENTCOLOR;
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

				} else if byte == 22 {
					let last = LASTCOLOR;

					LASTCOLOR = CURRENTCOLOR;
					CURRENTCOLOR = last;

				} else if byte == 24 {

					CHANGE_CUR_POS = !CHANGE_CUR_POS;

				} else if byte == 127 {

					if CURPOS != 0 {
						let ccp = CHANGE_CUR_POS.clone();

						CHANGE_CUR_POS = false;
						CURPOS -= 1;
						write_vga_char(VgaChar{char: REPLACER, color: CURRENTCOLOR.clone()});
						rewrite_vga_char(CURPOS);
						CHANGE_CUR_POS = ccp;

					}

				} else {

					write_vga_char(VgaChar{char: byte, color: CURRENTCOLOR.clone()});

				}

			},

		}

		if SCREEN_POS_VIRT != SCREEN_POS {

			SCREEN_POS_VIRT = SCREEN_POS.clone();
			rewrite_vga();

		}
		rewrite_vga_char(if CURPOS == 0 {0} else {CURPOS - 1});
//		rewrite_vga();
		update_vga_cur();

		LOCK = false;

	}

}


/// pass to hexadecimal
fn hex_to_num(char: u8) -> u8 {

	if char >= 97 && char <= 102 {

		return char - 87;

	} else if char >= 48 && char <= 57 {

		return char - 48;

	} else {

		0

	}

}

/// rewrites to vga buffer but only one character
fn rewrite_vga_char(charpos: usize) {

	unsafe {
		let pos = charpos - SCREEN_POS_VIRT * WIDTH;

//		TERM_WRITE(BUFFER[charpos].char, BUFFER[charpos].color, pos);

	}

}

/// write displayed buffer part to vga (repaint)
pub fn rewrite_vga() {
	let mut vgapos: usize = 0;

	unsafe {

		let pos = SCREEN_POS_VIRT * WIDTH;

		for char in &BUFFER[pos..if pos + SIZE >= BUFFER_LEN {BUFFER_LEN} else {pos + SIZE}] {

//			TERM_WRITE(char.char, char.color, vgapos);
			vgapos += 1;

		}

		for _ in 0..SIZE - vgapos / 2 {

//			TERM_WRITE(b'.', 0xf, vgapos);
			vgapos += 1;

		}

	}

}

/// simple write to buffer with two bytes (vga character)
fn write_vga_char(b: VgaChar) {

	unsafe {
//		let raw_screen_pos = SCREEN_POS * WIDTH;

		BUFFER[CURPOS] = b;

		if CHANGE_CUR_POS {

			CURPOS += 1;

			if CURPOS % WIDTH == 0 {

				for x in CURPOS..CURPOS + WIDTH {

					if x == BUFFER_LEN {

						break;

					}

					BUFFER[x].color = CURRENTCOLOR;

				}

				if SCREEN_POS != 0 {

					if (SCREEN_POS * WIDTH + SIZE - WIDTH < CURPOS) && SCREEN_POS != MAX_SCREEN_POS {

						SCREEN_POS += 1;
						SCREEN_POS_VIRT = SCREEN_POS;
//						rewrite_vga();

					}

				}

			}

			if (CURPOS % WIDTH == 0) && (CURPOS > SCREEN_POS_VIRT * WIDTH + SIZE - WIDTH) && (CURPOS < BUFFER_LEN - WIDTH) {

				SCREEN_POS += 1;
//				rewrite_vga();

			}

		}

		if CURPOS >= BUFFER_LEN {

			SCREEN_POS = MAX_SCREEN_POS;
//			down();
//			rewrite_vga();

		}

	}

}

/// update vga cursor position in vga buffer
pub fn update_vga_cur() {

	unsafe {
		let screen_first_char = SCREEN_POS_VIRT * WIDTH;

		if SCREEN_POS_VIRT * WIDTH <= CURPOS {

			TERM_CUR(CURPOS - SCREEN_POS_VIRT * WIDTH);

		} else if screen_first_char + SIZE > CURPOS {

			TERM_CUR_DISABLE();

		}

	}

}

/// scroll down removing upper line
pub fn down() {

	unsafe {

		for i in WIDTH..BUFFER_LEN {

			BUFFER[i - WIDTH] = BUFFER[i].clone();

		}

		for i in BUFFER_LEN - WIDTH..BUFFER_LEN {

			BUFFER[i] = VgaChar {char: DEFAULT_CHAR.char, color: CURRENTCOLOR};

		}

		CURPOS -= WIDTH;

	}

}
