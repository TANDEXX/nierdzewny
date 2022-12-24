#!/bin/nano
//! acpi driver, it can only shut down, can't every reboot, so TODO

use crate::outw;

pub fn stop_machine() {

	unsafe {

		outw(0x2000, 0xb004);
		outw(0x2000, 0x604);
		outw(0x3400, 0x4004);
		outw(0x10, 0xf4);

	}

}

pub fn reboot_machine() {


}
