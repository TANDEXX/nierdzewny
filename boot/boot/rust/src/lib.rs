#!/bin/nano
#![no_std]
#![feature(stmt_expr_attributes, panic_info_message, abi_x86_interrupt, exclusive_range_pattern, alloc_error_handler)]
#![warn(unused)]

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
use sc::text::write_bytes;
pub use proc::carch::{halt, end_exec as end, outb, outw, outd, inb, inw, ind};

/// entry point of main system part
#[no_mangle]
pub extern "C" fn main_entry() -> ! {

	mods::early_init();
	sc::vga::disable_text_blink();
//	proc::exception::init();
	proc::x86_64::init();
	device::init();
	mods::init();
	write_bytes(b"\x0fwelcome to the nierdzewny operating system :)\x10\n");

	end()
}

pub fn shutdown(reboot: bool) {

//	unsafe {

		mods::shutdown();

		if reboot {

			mods::reboot_machine();

		} else {

			mods::stop_machine();

		}

		write_bytes(b"\x0fsystem still not stopped, force shutdown your pc or virtual machine.\x10\n");
		end()

//	}

}
