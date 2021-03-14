#!/bin/nano
#![no_std]
#![no_main]

pub mod vga;
pub mod consts;

use core::panic::PanicInfo;
//use core::mem::transmute;
use consts::auto::PANIC_COLOR;
use consts::VGA;
use vga::write_bytes;

const WIDTH: isize = 80;
const HEIGHT: isize = 25;
const PANIC_MSG: &[u8] = b"System panic. Take a photo of screen and send it to tandex.english@gmail.com."; // massage cannot be longer than 78 characters

#[no_mangle]
extern "C" fn _start() -> ! {

	write_bytes(b""); // write your displayed text here (following the terminal rulesin "doc/terminal.txt")

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	let mut pos = 80;
	let vga = VGA as *mut u8;

	unsafe {

			*vga.offset(0) = 201;
			*vga.offset(1) = PANIC_COLOR;

		for i in 1..WIDTH - 1 {

			*vga.offset(i as isize * 2) = 205;
			*vga.offset(i as isize * 2 + 1) = PANIC_COLOR;

		}

		*vga.offset(WIDTH * 2 - 2) = 187;
		*vga.offset(WIDTH * 2 - 1) = PANIC_COLOR;

		line_with(&mut pos, PANIC_MSG, vga);

		for _ in 0..HEIGHT - 6 {

			line_with(&mut pos, b"", vga);

		}

/*		match info.message() {

				Some(msg) => {
					let mut buf = [32; 48];

					buf[0] = b'm';
					buf[1] = b'e';
					buf[2] = b's';
					buf[3] = b's';
					buf[4] = b'a';
					buf[5] = b'g';
					buf[6] = b'e';
					buf[7] = b':';

					for (i, &byte) in msg.as_bytes().iter().enumerate() {

						buf[i + 8] = byte;

					}

					line_with(&mut pos, buf, vga);

				},
				None => {

					line_with(&mut pos, b"message: <cannot get message information>", vga);

				}

			}
*/
		match info.payload().downcast_ref::<&'static str>() {

			Some(pay) => {
				let mut buf = [32; 48];

				buf[0] = b'p';
				buf[1] = b'a';
				buf[2] = b'y';
				buf[3] = b'l';
				buf[4] = b'o';
				buf[5] = b'a';
				buf[6] = b'd';
				buf[7] = b':';

				for (i, &byte) in pay.as_bytes().iter().enumerate() {

					buf[i + 8] = byte;

				}

				line_with(&mut pos, pay.as_bytes(), vga);

			}
			None => {

				line_with(&mut pos, b"payload: <cannot get payload information>", vga);

			}

		}

		match info.location() {

			Some(loc) => {

				let mut buf = [32; 45];

				buf[0] = b'f';
				buf[1] = b'i';
				buf[2] = b'l';
				buf[3] = b'e';
				buf[4] = b':';

				for (i, &byte) in loc.file().as_bytes().iter().enumerate() {

					buf[i + 6] = byte;

				}

				line_with(&mut pos, &buf, vga);
				line_with(&mut pos, b"line: <line in panic not supported yet>", vga);

			},
			None => {

				line_with(&mut pos, b"file: <cannot get file information>", vga);
				line_with(&mut pos, b"line: <cannot get line information>", vga);

			}

		}

		*vga.offset(pos * 2) = 200;
		*vga.offset(pos * 2 + 1) = PANIC_COLOR;

		for i in pos + 1..pos + WIDTH - 1 {

			*vga.offset(i * 2) = 205;
			*vga.offset(i * 2 + 1) = PANIC_COLOR;

		}

		*vga.offset(HEIGHT * WIDTH * 2 - 2) = 188;
		*vga.offset(HEIGHT * WIDTH * 2 - 1) = PANIC_COLOR;

	}

	loop {}
}

fn line_with(pos: &mut isize, line: &[u8], vga: *mut u8) {

	unsafe {

		*vga.offset(*pos * 2) = 186;
		*vga.offset(*pos * 2 + 1) = PANIC_COLOR;
		*pos += 1;
		let mut lock = pos.clone();

		for (i, &byte) in line.iter().enumerate() {

			*vga.offset((lock + i as isize) * 2) = byte;
			*vga.offset((lock + i as isize) * 2 + 1) = PANIC_COLOR;
			*pos += 1;

		}

		lock = pos.clone();

		for i in 0..WIDTH - line.len() as isize - 2 {

			*vga.offset((lock + i) * 2) = 32;
			*vga.offset((lock + i) * 2 + 1) = PANIC_COLOR;
			*pos += 1;

		}

		*vga.offset(*pos * 2) = 186;
		*vga.offset(*pos * 2 + 1) = PANIC_COLOR;
		*pos += 1;

	}

}
/*
fn to_str(num: u32) -> [u8; 5] {
	let mut buffer: [u8; 5] = [0; 5];

	buffer[4] = (num % 10 + 48) as u8;
	buffer[3] = (num % 100 - buffer[4] as u32 + 48) as u8;
	buffer[2] = (num % 1000 - buffer[4] as u32 - buffer[3] as u32 + 48) as u8;
	buffer[1] = (num % 10000 - buffer[4] as u32 - buffer[3] as u32 - buffer[2] as u32 + 48) as u8;
	buffer[0] = (num % 100000 - buffer[4] as u32 - buffer[3] as u32 - buffer[2] as u32 - buffer[1] as u32 + 48) as u8;

	buffer
}
*/