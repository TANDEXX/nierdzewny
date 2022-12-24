#!/bin/nano
//! low level register managment

use crate::{outb, inb};

unsafe fn reset_x3c0() {

	let _ = inb(0x3c0);

}

pub unsafe fn write_x3c0(index: u8, data: u8) {

	reset_x3c0();
	outb(0x3c0, index);
	outb(0x3c0, data);

}

pub unsafe fn read_x3c0(index: u8) -> u8 {

	reset_x3c0();
	outb(0x3c0, index);
	let output = inb(0x3c1);
	let _ = inb(0x3da);

	output
}

pub unsafe fn write_x3c2(data: u8) {

	outb(0x3c2, data);

}

pub unsafe fn read_x3cc() -> u8 {

	inb(0x3cc)
}

pub unsafe fn write_x3c4(index: u8, data: u8) {

	outb(0x3c4, index);
	outb(0x3c5, data);

}

pub unsafe fn read_x3c4(index: u8) -> u8 {

	outb(0x3c4, index);

	inb(0x3c5)
}

pub unsafe fn write_x3ce(index: u8, data: u8) {

	outb(0x3ce, index);
	outb(0x3cf, data);

}

pub unsafe fn read_x3ce(index: u8) -> u8 {

	outb(0x3ce, index);

	inb(0x3cf)
}

pub unsafe fn write_x3d4(index: u8, data: u8) {

	outb(0x3d4, index);
	outb(0x3d5, data);

}

pub unsafe fn read_x3d4(index: u8) -> u8 {

	outb(0x3d4, index);

	inb(0x3d5)
}

pub unsafe fn write_x3c6(data: u8) {

	outb(0x3c6, data);

}

pub unsafe fn read_x3c6() -> u8 {

	inb(0x3c6)
}
