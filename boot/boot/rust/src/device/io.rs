#!/bin/nano

use super::{DEFAULT_OUT, Output, State, Type, new_dev as new_device, push};
use crate::consts::auto::DEV_BUFFER as BUFFER;

const BUFFER_END: usize = BUFFER - 1;
pub const DEFAULT: IODevice = IODevice {at: BUFFER, buffer: [0; BUFFER]};

pub static mut DEV: IODevice = DEFAULT;

/// macro generating wrapers to io devices.
/// My first macro that works propertly :)
macro_rules! io_wrapers {

	(

		$write:ident, $read:ident, $new:ident, $DEV:ident

	) => {

		fn $write(s: &mut State, byte: u8) -> Output {

			unsafe {

				$DEV.write(s, byte)
			}
		}

		fn $read(s: &mut State, byte: &mut u8) -> Output {

			unsafe {

				$DEV.read(s, byte)
			}
		}

		fn $new() -> State {

			unsafe {

				$DEV.new()
			}
		}

	}

}

/// Basic io device data structure
pub struct IODevice {

	at: usize,
	buffer: [u8; BUFFER],

}

impl IODevice {

	/// write to device
	pub fn write(&mut self, _: &mut State, byte: u8) -> Output {

		for x in 0..BUFFER_END {

			self.buffer[x] = self.buffer[x + 1];

		}

		self.buffer[BUFFER_END] = byte;
		self.at += 1;

		Output::Ok
	}

	/// read from device
	pub fn read(&self, s: &mut State, byte: &mut u8) -> Output {

		if s.inp == self.at {

			Output::Wait
		} else if s.inp < self.at - BUFFER {

			s.inp += 1;
			Output::Ignore
		} else {

			*byte = self.buffer[s.inp - (self.at - BUFFER)];
			s.inp += 1;

			Output::Ok
		}

	}

	pub fn new(&self) -> State {

		State {inp: self.at, inp_at: 0, out: DEFAULT_OUT, out_ind: 0}
	}

}

// old code that I writen before I figure out I can use macros
/*
fn write_dev(s: &mut State, byte: u8) -> Output {

	unsafe {DEV.write(s, byte)}

}

fn read_dev(s: &mut State, byte: &mut u8) -> Output {

	unsafe {DEV.read(s, byte)}

}

fn new_dev() -> State {

	unsafe {DEV.new()}

}
*/

io_wrapers!{write_dev, read_dev, new_dev, DEV}

pub fn init() {

	match push(new_device(write_dev, read_dev, new_dev, [b'i', b'o', 0, 0, 0, 0, 0, 0], Type::Misc)) {

		None => panic!("failed to init `/dev/io` device"),
		Some(_) => {},

	}

}
