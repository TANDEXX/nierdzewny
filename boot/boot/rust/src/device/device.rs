#!/bin/nano

use super::{Device as DeviceTrait, Output};

pub struct Device {

	last: u8,

}

impl DeviceTrait for Device {

	fn read() -> (Output, u8) {

		(Output::Ok, 0)
	}

	fn write(_: u8) -> (Output) {

		Output::Ok
	}

}
