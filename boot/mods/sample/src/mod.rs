#!/bin/nano

use crate::sc::text::write_bytes;

pub fn init() {

	write_bytes(b"sample: init\n");

}

pub fn shutdown() {

	write_bytes(b"sample: shutdown\n");

}
