#!/bin/nano
#![no_std]
#![feature(stmt_expr_attributes, panic_info_message, abi_x86_interrupt, asm, global_asm, llvm_asm, const_fn_trait_bound, exclusive_range_pattern, half_open_range_patterns, alloc_error_handler, let_chains)]

/// tell compiler to try include it
extern crate alloc;

/// allocator
pub mod boot_alloc;
/// screen output
pub mod sc;
/// public constants
pub mod consts;
/// utility
#[macro_use]
pub mod util;
/// processor manager
pub mod proc;
/// devices in /dev
pub mod device;
/// boot built modules
pub mod mods;
/// the panic functions
pub mod panic;
/// file systems implementation
pub mod fs;

//use core::mem::transmute;
use util::str::Str;
use x86_64::instructions::{port::Port};
use sc::text::write_bytes;
use mods::built as bmods;

pub fn outb(port: u16, data: u8) {

	unsafe {

		asm!("out dx, al", in("dx") port, in("al") data);

	}

}

pub fn outw(port: u16, data: u16) {

	unsafe {

		asm!("out dx, ax", in("dx") port, in("ax") data);

	}

}

pub fn outd(port: u16, data: u32) {

	unsafe {

		asm!("out dx, eax", in("dx") port, in("eax") data);

	}

}

pub fn inb(port: u16) -> u8 {
	let output: u8;

	unsafe {

		asm!("in al, dx", in("dx") port, out("al") output);

	}

	output
}

/// entry point of main system part
#[no_mangle]
pub extern "C" fn main_entry() -> ! {

	bmods::early_init();
	sc::vga::disable_text_blink();
//	proc::exception::init();
	proc::x86_64::init();
	device::init();
	bmods::init();
	write_bytes(b"\x0fwelcome to the nierdzewny operating system :)\x10\n");

	end!()
}

pub fn shutdown(reboot: bool) {

	unsafe {

		bmods::shutdown();

		if reboot {

			bmods::reboot_machine();

		} else {

			bmods::stop_machine();

		}

		write_bytes(b"\x0fsystem still not stopped, force shutdown your pc or virtual machine.\x10\n");
		stop_cpu!();

	}

}
