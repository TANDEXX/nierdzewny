#!/bin/nano
//! very temporary terminal implementation

use crate::mods::core_lib::term::buffer::{TermBuffer, Char};

pub static mut TERM: TermBuffer<80, 50> = TermBuffer::new(25, repaint);

pub fn init() {

	unsafe {

		TERM.write_bytes(b"patolska [31;47mgurom\n\ttackf", true);

	}

}

pub fn repaint(x: usize, y: usize, char: Char) {
	let pos = 753664 + (x + y * 80) * 2;

	unsafe {
		(pos as * mut u8).write(char.utf8_char as u8);
		((pos+1) as * mut u8).write(char.color.fg.0 | (char.color.bg.0 << 4));
	}

}
/*
pub fn repaint() {

	unsafe {
		let mut a = 0;

		for char in TERM.iter() {

			((753664 + a * 2) as * mut u8).write(char.utf8_char as u8);
			((753665 + a * 2) as * mut u8).write(char.color.fg.0 | (char.color.bg.0 << 4));
			a += 1;

		}

	}

}
*/