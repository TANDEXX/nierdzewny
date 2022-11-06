#!/bin/nano

use crate::outw;
//use crate::sc::text::write_bytes;

pub fn stop_machine() {

//	unsafe {

		outw(0x2000, 0xb004);
		outw(0x2000, 0x604);
		outw(0x3400, 0x4004);
		outw(0x10, 0xf4);

//		write_bytes(b"acpi: device does not support it\n");

//	}

}

pub fn reboot_machine() {

//	write_bytes(b"acpi: reboot is not implemented\n");

}
