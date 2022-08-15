#!/bin/nano
#![allow(unused)]

use core::mem::transmute;
use crate::consts::VGA_TEXT;
use crate::outb;

pub fn write_char(char: u8, color: u8, offset: usize) {

	unsafe {

		(((offset << 1) + VGA_TEXT) as * mut u16).write(transmute((char, color)));

	}

}

pub fn term_size_fn() -> Result<(), &'static [u8]> {

	Err(b"does not support")
}

pub fn move_cursor(offset: usize) {

	outb(0x3c4, 0xfu8);
	outb(0x3c5, offset as u8);
	outb(0x3c4, 0xeu8);
	outb(0x3c5, (offset >> 8) as u8);

}

pub fn disable_cursor() {

	outb(0x3c4, 0xau8);
	outb(0x3c5, 0x20u8);

}

pub fn disable_text_blink() {

	unsafe {

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

}
