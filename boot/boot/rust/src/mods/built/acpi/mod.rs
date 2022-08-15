#!/bin/nano

use crate::sc::text::write_bytes;
use x86_64::instructions::port::Port;

pub fn stop_machine() {

	unsafe {

		Port::new(0xb004).write(0x2000u16);
		Port::new(0x604).write(0x2000u16);
		Port::new(0x4004).write(0x3400u16);
		Port::new(0xf4).write(0x10u16);

		write_bytes(b"acpi: device does not support it\n");

	}

}

pub fn reboot_machine() {

	write_bytes(b"acpi: reboot is not implemented\n");

}
