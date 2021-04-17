#!/bin/nano
#![allow(const_item_mutation)]

/// string on stack (used for don't use stack)
pub mod str;
/// string argument parser
pub mod args;

/// xor logic gate
/// # Example
/// ```
/// if xor(boolean1, boolean2) {
///
/// 	write_bytes(b"are different\n");
///
/// } else {
///
/// 	write_bytes(b"are the same\n");
///
/// }
/// ```
pub fn xor(b1: bool, b2: bool) -> bool {

	if b1 {

		!b2

	} else {

		b2

	}

}

/// stack ooverflowing function used in dable fault function if it should triple fault
#[allow(unconditional_recursion)]
pub fn stack_overflow() {

	stack_overflow();

}

/// function for triple fault
pub fn triple_fault() {

	unsafe {

		crate::proc::exception::TRIPLE_FAULT = true;
		let _ = (0 as *mut u8).read();

	}

}
