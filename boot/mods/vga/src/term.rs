#!/bin/nano

use crate::mods::core_lib::term::buffer::{TermBuffer, Char};

pub static mut TERM: TermBuffer<80, 50> = TermBuffer::new(25);

pub fn init() {

	unsafe {

		TERM.write_bytes(b"patolska gurom\n\ttak");

	}

	repaint();

}

pub fn repaint() {

	unsafe {
		let mut a = 0;

		for char in TERM.iter() {

			((753664 + a * 2) as * mut u8).write(char.utf8_char as u8);
			a += 1;

		}

	}

}
