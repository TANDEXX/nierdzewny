#!/bin/nano
#![allow(const_item_mutation)]

/// string on stack
pub mod str;
/// fast buffer structure
pub mod buffer;
/// string argument parser
pub mod args;
/// most of macros
#[macro_use]
pub mod macros;

/// used by `sh.rs` file for turning number into color
pub fn num_to_color(num: u8) -> u8 {

	if num < 10 {

		num + 48
	} else {

		num + 87
	}

}

/// function for triple fault
pub fn triple_fault() {

	unsafe {

		crate::proc::exception::TRIPLE_FAULT = true;
		asm!("lgdt [0]");

	}

}
