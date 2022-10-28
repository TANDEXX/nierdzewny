#!/bin/nano

use super::{low::{IntStackFrame as Isf}, PIC};
use crate::sc::text::write_bytes;

pub extern "x86-interrupt" fn div_zero(_isf: &mut Isf) {

	panic!("divide by zero exception")
}

pub extern "x86-interrupt" fn breakpoint(_isf: &mut Isf) {

	write_bytes(b"breakpoint\n");

}

pub extern "x86-interrupt" fn overflow(_isf: &mut Isf) {

	panic!("overflow exception")
}

pub extern "x86-interrupt" fn invalid_opcode(_isf: &mut Isf) {

	panic!("invalid opcode exception")
}

pub extern "x86-interrupt" fn double_fault(_isf: &mut Isf) -> ! {

	panic!("double fault")
}

pub extern "x86-interrupt" fn invalid_tss(_isf: &mut Isf) {

	panic!("invalid tss exception");
}

pub extern "x86-interrupt" fn seg_not_present(_isf: &mut Isf) {

	panic!("segment not present exception")
}

pub extern "x86-interrupt" fn general_prot_fault(_isf: &mut Isf) {

	panic!("general protection fault")
}

pub extern "x86-interrupt" fn page_fault(_isf: &mut Isf) {

	panic!("page fault")
}

pub extern "x86-interrupt" fn timer(_isf: &mut Isf) {

	unsafe {
		crate::mods::timer_int();

//		write_bytes(b"T");

		PIC.notify_end_of_interrupt(32);
	}

}

pub extern "x86-interrupt" fn keyboard(_isf: &mut Isf) {

	unsafe {
		crate::mods::keyboard_int();

//		write_bytes(b"K");

		PIC.notify_end_of_interrupt(33);
	}

}
