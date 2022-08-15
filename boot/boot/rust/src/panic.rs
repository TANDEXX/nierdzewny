#!/bin/nano

use core::panic::PanicInfo;
use crate::consts::auto::PANIC_COLOR;
use crate::sc::{vga, vga::write_char, text::write_bytes};
use crate::util::str::Str;

/// default vga text mode width with type `isize`
const WIDTH: usize = 80;
/// default vga text mode height with type `isize`
const HEIGHT: usize = 25;
const LINE: usize = 78;
/// lines in panic on top of screen (cannot be longer than 78 bytes)
const PANIC_MSG: &[&[u8]] = &[

	b"System panic. If you not triggered it then please take a photo of screen, send",
	b"it to tandex.english@gmail.com and say more about that what you do. please",
	b"check it do it panic if you do the same thing next times. [YES, OLD MESSAGE]",
	b"",
	b"press P button to stop machine, R for reboot (triple fault)",

]; // one line cannot be longer than 78 characters (looks very ugly)

pub static mut PANICED: bool = false;

/// function called on panic
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {

//	end_screen(info);

	write_bytes(b"\nsystem panic: (msg: ");
	match info.message() {Some(msg) => {match msg.as_str() {Some(msg) => write_bytes(msg.as_bytes()), None => write_bytes(b"MOH")}}, None => write_bytes(b"NO")}
	write_bytes(b", payl: ");
	match info.payload().downcast_ref::<&'static str>() {Some(pay) => write_bytes(pay.as_bytes()), None => write_bytes(b"NO")}
	write_bytes(b", loc: ");
	match info.location() {Some(loc) => {write_bytes(b"(file: ");write_bytes(loc.file().as_bytes());write_bytes(b", line: ");write_bytes(Str::from_unsigned_num(loc.line() as u128).as_slice());write_bytes(b"))\n");}, None => write_bytes(b"NO)\n")}

	end!();
}

/// paint end screen on vga buffer text mode
pub fn end_screen(info: &PanicInfo) {
	let mut pos = 80;

	unsafe {
		vga::disable_cursor();
		write_char(201, PANIC_COLOR, 0);

		for i in 1..WIDTH - 1 {

			write_char(205, PANIC_COLOR, i);

		}

		write_char(187, PANIC_COLOR, WIDTH);
		const WIDTH_DEC: usize = WIDTH - 1;
		write_char(187, PANIC_COLOR, WIDTH_DEC);

		for msg in PANIC_MSG {

			line_with(&mut pos, msg);

		}

		for _ in 0..HEIGHT - 6 - (PANIC_MSG.len()) {

			line_with(&mut pos, &[]);

		}

		match info.message() {

				Some(msg) => {

					match msg.as_str() {

						Some(msg) => {
							let mut buffer = Str::from_bytes(b"message: ");

							buffer.push(msg.as_bytes());
							line_with(&mut pos, &buffer.as_array::<LINE>());

						},
						None => {

							line_with(&mut pos, b"message: <cannot get message as &'static str>");

						},

					}

				},
				None => {

					line_with(&mut pos, b"message: <message not found>");

				}

			}

		match info.payload().downcast_ref::<&'static str>() {

			Some(pay) => {
				let mut buffer = Str::from_bytes(b"payload: ");

				buffer.push(pay.as_bytes());
				line_with(&mut pos, &buffer.as_array::<LINE>());

			}
			None => {

				line_with(&mut pos, b"payload: <payload not found>");

			}

		}

		match info.location() {

			Some(loc) => {
				let mut buffer = Str::from_bytes(b"file: ");

				buffer.push(loc.file().as_bytes());
				line_with(&mut pos, &buffer.as_array::<LINE>());
				buffer = Str::from_bytes(b"line: ");
				buffer.push(&Str::from_unsigned_num(loc.line() as u128).as_array::<5>());
				line_with(&mut pos, &buffer.as_array::<LINE>());

			},
			None => {

				line_with(&mut pos, b"file: <cannot get file information>");
				line_with(&mut pos, b"line: <cannot get line information>");

			}

		}

		write_char(200, PANIC_COLOR, pos);

		for i in pos + 1..pos + WIDTH - 1 {

			write_char(205, PANIC_COLOR, i);

		}

		const LAST: usize = HEIGHT * WIDTH - 1;
		write_char(188, PANIC_COLOR, LAST);
		PANICED = true;

	}

}

/// function used to make line with text with this frame
fn line_with(pos: &mut usize, line: &[u8]) {

	write_char(186, PANIC_COLOR, *pos);
	*pos += 1;

	for byte in line.iter() {

		write_char(*byte, PANIC_COLOR, *pos);
		*pos += 1;

	}

	for _i in 0..WIDTH - line.len() - 2 {

		write_char(32, PANIC_COLOR, *pos);
		*pos += 1;

	}

	write_char(186, PANIC_COLOR, *pos);
	*pos += 1;

}
