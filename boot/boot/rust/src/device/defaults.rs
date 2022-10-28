#!/bin/nano

use super::{Type, Output, State, push, new_dev};
use super::io::{IODevice as Dev, DEFAULT};

pub static mut KEY_INDEX: usize = 0;
pub static mut KEY: Dev = DEFAULT;

pub static mut MOUSE_INDEX: usize = 0;
pub static mut MOUSE: Dev = DEFAULT;

io_wrapers!{write_key, read_key, new_key, KEY}
io_wrapers!{write_mouse, read_mouse, new_mouse, MOUSE}

pub fn init() {

	unsafe {

		KEY_INDEX = match push(new_dev(write_key, read_key, new_key, [b'k', b'e', b'y', b'b', b'o', b'a', b'r', b'd'], Type::Input)) {

			Some(tmp) => tmp,
			None => panic!("failed to init `/dev/input/keyboard` device")

		};

		KEY_INDEX = match push(new_dev(write_mouse, read_mouse, new_key, [b'm', b'o', b'u', b's', b'e', 0, 0, 0], Type::Input)) {

			Some(tmp) => tmp,
			None => panic!("failed to init `/dev/input/mouse` device")

		};

	}

}
