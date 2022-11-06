#!/bin/nano
//! very temporary terminal implementation

use crate::mods::core_lib::term::buffer::{TermBuffer};

pub static mut TERM: TermBuffer<80, 50> = TermBuffer::new(25);

pub fn init() {

	unsafe {

		TERM.write_bytes(b"patolska [31;47mgurom\n\ttackf");

	}

	repaint();

}

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
