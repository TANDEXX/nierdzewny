#!/bin/nano

use crate::sc::{text, vga};

pub fn early_init() {

	text::set_term(vga::write_char, vga::term_size_fn, vga::move_cursor, vga::disable_cursor);

}
