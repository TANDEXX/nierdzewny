#!/bin/nano
#![no_std]
#![feature(stmt_expr_attributes, panic_info_message, abi_x86_interrupt, exclusive_range_pattern, alloc_error_handler)]
#![warn(unused)]

/// tell compiler to try include it
extern crate alloc;

pub mod boot_alloc;
//pub mod sc;
pub mod consts;
pub mod util;
pub mod proc;
pub mod mods;
pub mod panic;

//use core::mem::transmute;
//use sc::text::write_bytes;
pub use proc::carch::{halt, end_exec as end, outb, outw, outd, inb, inw, ind};

/// entry point of main system part
#[no_mangle]
pub extern "C" fn main_entry() -> ! {

	mods::early_init();
//	sc::vga::disable_text_blink();
	proc::carch::init();
	mods::init();

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

		end()

//	}

}
