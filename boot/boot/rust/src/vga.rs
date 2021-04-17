#!/bin/nano
#![allow(non_snake_case)]

//use core::mem::transmute;
use x86_64::instructions::{interrupts::without_interrupts, port::Port};
use crate::consts::auto::{DEFAULT_COLOR, HIGHTLIGHT_COLOR, WIDTH, HEIGHT, BUFFER_LEN};
use crate::consts::VGA_TEXT as VGA;

/// default replacer character
const REPLACER: u8 = b' ';
/// terminal replacer
const DEFAULT_CHAR: VgaChar = VgaChar{char: REPLACER, color: DEFAULT_COLOR};
/// size of vga terminal
const VGA_SIZE: usize = WIDTH * HEIGHT;
const MAX_SCREEN_POS: usize = (BUFFER_LEN - VGA_SIZE) / WIDTH;

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

/// terminal buffer
static mut BUFFER: [VgaChar; BUFFER_LEN] = [DEFAULT_CHAR; BUFFER_LEN]; // buffer len is 80 * 25 * 10 by default
/// cursor position
static mut CURPOS: usize = 0;
static mut CURRENTCOLOR: u8 = DEFAULT_COLOR;
static mut LASTCOLOR: u8 = DEFAULT_COLOR;
/// current screen position on terminal
pub static mut SCREEN_POS: usize = 0;
/// virtual screen position (current scrolled)
pub static mut SCREEN_POS_VIRT: usize = 0;
/// lock vga buffer
static mut LOCK: bool = false;
/// terminal state (change cursor position to right when printing one character)
static mut CHANGE_CUR_POS: bool = true;
/// current terminal state (do change color, how change color, do print control characters as normal, etc)
static mut SETTING: Type = Type::No;

/// write multiple bytes to vga buffer
/// # Example
/// ```
/// write_bytes(b"hello, ");
/// write_bytes(&[119, 111, 114, 108, 100, 10]); // world\n
/// ```
pub fn write_bytes(bytes: &[u8]) {

	for byte in bytes {

		write_byte(*byte);

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

		without_interrupts(move || {

			while LOCK {}

			LOCK = true;

			match &SETTING {

				Type::Fg => {

					LASTCOLOR = CURRENTCOLOR;
					CURRENTCOLOR = hex_to_num(byte) + bits_from(CURRENTCOLOR, 3);
					SETTING = Type::No;

				},
				Type::Bg => {

					LASTCOLOR = CURRENTCOLOR;
					CURRENTCOLOR = (hex_to_num(byte) * 16) + (CURRENTCOLOR - bits_from(CURRENTCOLOR, 3));
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
//			rewrite_vga();
			update_vga_cur();

			LOCK = false;

		});

	}

}

/// four bits on specified index from variable
fn bits_from(num: u8, start: u8) -> u8 {
	let mut buffer = 0;

	for x in start..8 {

		buffer += num & (1 << x);

	}

	buffer
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
		let vga = VGA as *mut u8;
		let pos = (charpos - SCREEN_POS_VIRT * WIDTH) as isize;

		*vga.offset(pos * 2) = BUFFER[charpos].char;
		*vga.offset(pos * 2 + 1) = BUFFER[charpos].color.clone();

	}

}

/// write displayed buffer part to vga (repaint)
pub fn rewrite_vga() {
	let vga = VGA as *mut u8;
	let mut vgapos: isize = 0;

	unsafe {

		let pos = SCREEN_POS_VIRT * WIDTH;

		for char in &BUFFER[pos..if pos + VGA_SIZE >= BUFFER_LEN {BUFFER_LEN} else {pos + VGA_SIZE}] {

			*vga.offset(vgapos) = char.char;
			*vga.offset(vgapos + 1) = char.color.clone();

			vgapos += 2;

		}

		for _ in 0..VGA_SIZE as isize - vgapos / 2 {

			*vga.offset(vgapos) = b'.';
			*vga.offset(vgapos + 1) = 240;

			vgapos += 2;

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

					if (SCREEN_POS * WIDTH + VGA_SIZE - WIDTH < CURPOS) && SCREEN_POS != MAX_SCREEN_POS {

						SCREEN_POS += 1;
						SCREEN_POS_VIRT = SCREEN_POS;
						rewrite_vga();

					}

				}

			}

			if (CURPOS % WIDTH == 0) && (CURPOS > SCREEN_POS_VIRT * WIDTH + VGA_SIZE - WIDTH) && (CURPOS < BUFFER_LEN - WIDTH) {

				SCREEN_POS += 1;
				rewrite_vga();

			}

		}

		if CURPOS >= BUFFER_LEN {

			SCREEN_POS = MAX_SCREEN_POS;
			down();
			rewrite_vga();

		}

	}

}

/// update vga cursor position in vga buffer
fn update_vga_cur() {

	unsafe {

		if SCREEN_POS_VIRT * WIDTH <= CURPOS {
			let mut port4 = Port::new(0x3d4);
			let mut port5 = Port::new(0x3d5);


			port4.write(0x0fu8);
			port5.write(((CURPOS - SCREEN_POS_VIRT * WIDTH) & 255) as u8);
			port4.write(0x0eu8);
			port5.write((((CURPOS - SCREEN_POS_VIRT * WIDTH) >> 8) & 255) as u8);

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
