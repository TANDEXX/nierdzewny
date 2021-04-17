#!/bin/nano
#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)]
#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

/// vga device driver
pub mod vga;
/// public constants
pub mod consts;
/// utility
pub mod util;
/// processor manager
pub mod proc;
/// keyboard utilitys
pub mod keyb;
/// devices in /dev
pub mod device;
/// terminal shell
pub mod sh;

use core::panic::PanicInfo;
//use core::mem::transmute;
use x86_64::instructions::{hlt, port::Port};
use consts::auto::PANIC_COLOR;
use consts::VGA_TEXT as VGA;
use vga::write_bytes;
use util::str::Str;

/// default vga text mode width with type `isize`
const WIDTH: isize = 80;
/// default vga text mode height with type `isize`
const HEIGHT: isize = 25;
const LINE: usize = 78;
/// lines in panic on top of screen (cannot be longer than 78 bytes)
const PANIC_MSG: &[&[u8]] = &[

	b"System panic. If you not triggered it then please take a photo of screen, send",
	b"it to tandex.english@gmail.com and say more about that what you do. please",
	b"check it do it panic if you do the same thing next times.",
	b"",
	b"press P button to stop machine, R for reboot (triple fault)",
//	b"19 dollar fortnitr card lol furry gowno looool", // my friend wanted to change this, furry gówno literary means: `furry shit`

]; // one line cannot be longer than 78 characters (looks very ugly)

pub static mut PANICED: bool = false;

/// entry point of system
#[no_mangle]
extern "C" fn _start() -> ! {

	unsafe {

		// turn off text blink with assembler (I only found way to do this with assembler, if you know how to make this without assembler please tell me in github issuse)
		asm!(

			"mov dx, 0x03DA",
			"in al, dx",
			"mov dx, 0x03C0",
			"mov al, 0x30",
			"out dx, al",
			"inc dx",
			"in al, dx",
			"and al, 0xF7",
			"dec dx",
			"out dx, al",

		)

	}


	proc::exception::init();
	device::init();
	keyb::code::init();
	write_bytes(b"\x0fwelcome to the nierdzewny operating system :)\x10\n"); // write your displayed text here (following the terminal rules in "doc/terminal.txt")

	loop {

//		hlt();
		sh::open();

	}
}

pub fn shutdown(_reboot: bool) {

	unsafe {

		Port::new(0xb004).write(0x2000u16);
		Port::new(0x604).write(0x2000u16);
		Port::new(0x4004).write(0x3400u16);
		Port::new(0xf4).write(0x10u16);

	}

}

/// function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

	let mut pos = 80;
	let vga = VGA as *mut u8;

	unsafe {
		let mut port4 = Port::new(0x3d4);
		let mut port5 = Port::new(0x3d5);
		port4.write(0x0au8);
		port5.write(0x20u8);

		*vga.offset(0) = 201;
		*vga.offset(1) = PANIC_COLOR;

		for i in 1..WIDTH - 1 {

			*vga.offset(i as isize * 2) = 205;
			*vga.offset(i as isize * 2 + 1) = PANIC_COLOR;

		}

		*vga.offset(WIDTH * 2 - 2) = 187;
		*vga.offset(WIDTH * 2 - 1) = PANIC_COLOR;

		for msg in PANIC_MSG {

			line_with(&mut pos, msg, vga);

		}

		for _ in 0..HEIGHT - 6 - (PANIC_MSG.len() as isize) {

			line_with(&mut pos, &[], vga);

		}

		match info.message() {

				Some(msg) => {

					match msg.as_str() {

						Some(msg) => {
							let mut buffer = Str::from_bytes(b"message: ");

							buffer.push(msg.as_bytes());
							line_with(&mut pos, &buffer.as_array::<LINE>(), vga);

						},
						None => {

							line_with(&mut pos, b"message: <cannot get message as &'static str>", vga);

						},

					}

				},
				None => {

					line_with(&mut pos, b"message: <message not found>", vga);

				}

			}

		match info.payload().downcast_ref::<&'static str>() {

			Some(pay) => {
				let mut buffer = Str::from_bytes(b"payload: ");

				buffer.push(pay.as_bytes());
				line_with(&mut pos, &buffer.as_array::<LINE>(), vga);

			}
			None => {

				line_with(&mut pos, b"payload: <payload not found>", vga);

			}

		}

		match info.location() {

			Some(loc) => {
				let mut buffer = Str::from_bytes(b"file: ");

				buffer.push(loc.file().as_bytes());
				line_with(&mut pos, &buffer.as_array::<LINE>(), vga);
				buffer = Str::from_bytes(b"line: ");
				buffer.push(&Str::from_unsigned_num(loc.line() as u128).as_array::<5>());
				line_with(&mut pos, &buffer.as_array::<LINE>(), vga);

			},
			None => {

				line_with(&mut pos, b"file: <cannot get file information>", vga);
				line_with(&mut pos, b"line: <cannot get line information>", vga);

			}

		}

		*vga.offset(pos * 2) = 200;
		*vga.offset(pos * 2 + 1) = PANIC_COLOR;

		for i in pos + 1..pos + WIDTH - 1 {

			*vga.offset(i * 2) = 205;
			*vga.offset(i * 2 + 1) = PANIC_COLOR;

		}

		*vga.offset(HEIGHT * WIDTH * 2 - 2) = 188;
		*vga.offset(HEIGHT * WIDTH * 2 - 1) = PANIC_COLOR;

		PANICED = true;

	}

	loop {

		hlt();

	}
}

/// function used only in panic to make line with text with this frame
fn line_with(pos: &mut isize, line: &[u8], vga: *mut u8) {

	unsafe {

		*vga.offset(*pos * 2) = 186;
		*vga.offset(*pos * 2 + 1) = PANIC_COLOR;
		*pos += 1;
		let mut lock = pos.clone();

		for (i, &byte) in line.iter().enumerate() {

			*vga.offset((lock + i as isize) * 2) = byte;
			*vga.offset((lock + i as isize) * 2 + 1) = PANIC_COLOR;
			*pos += 1;

		}

		lock = pos.clone();

		for i in 0..WIDTH - line.len() as isize - 2 {

			*vga.offset((lock + i) * 2) = 32;
			*vga.offset((lock + i) * 2 + 1) = PANIC_COLOR;
			*pos += 1;

		}

		*vga.offset(*pos * 2) = 186;
		*vga.offset(*pos * 2 + 1) = PANIC_COLOR;
		*pos += 1;

	}

}
