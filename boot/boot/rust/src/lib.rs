#!/bin/nano
#![no_std]
#![warn(unused)]
#![feature(stmt_expr_attributes, panic_info_message, abi_x86_interrupt, exclusive_range_pattern, alloc_error_handler)]
//! nierdzewny core


/// tell compiler to try include it
extern crate alloc;

pub mod boot_alloc;
pub mod proc;
pub mod mods;

use core::panic::PanicInfo;
pub use proc::carch::{halt, end_exec as end, outb, outw, outd, inb, inw, ind};

/// entry point of main system part
#[no_mangle]
pub extern "C" fn main_entry() -> ! {

	mods::early_init();
	proc::carch::init();
	mods::init();

	end()
}

/// function which handles panic
#[panic_handler]
pub fn panic(_panic_info: &PanicInfo) -> ! {

	mods::panic();

	end()
}

/// function used to stop machine
pub fn shutdown(reboot: bool) {

	mods::shutdown();

	if reboot {

		mods::reboot_machine();

	} else {

		mods::stop_machine();

	}

	end()

}
