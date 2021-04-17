#!/bin/nano

use super::{Type, Output, State, push, new_dev};
use super::io::{IODevice as Dev, DEFAULT};

pub static mut KEY_INDEX: usize = 0;
pub static mut KEY: Dev = DEFAULT;

io_wrapers!{write_key, read_key, new_key, KEY}

pub fn init() {

	unsafe {

		KEY_INDEX = match push(new_dev(write_key, read_key, new_key, [b'k', b'e', b'y', b'b', b'o', b'a', b'r', b'd'], Type::Input)) {

			Some(tmp) => tmp,
			None => panic!("failed to init `/dev/input/keyboard` device")

		};

	}

}
