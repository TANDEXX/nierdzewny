#!/bin/nano
#![allow(unused)]

/// input output device
#[macro_use]
pub mod io;
/// default devices needed by boot
pub mod defaults;

use crate::consts::auto::{DEVICES, MAX_DEV_OUT as OUT_SIZE};

/// device data
#[derive(Clone)]
pub struct Device {

	write: fn(&mut State, u8) -> Output,
	read: fn(&mut State, &mut u8) -> Output,
	new: fn() -> State,
	t: Type,
	name: Name,
	lock: bool,

}

/// descriptor state
pub struct State {

	inp: usize,
	inp_at: usize,
	out_ind: usize,
	out: [u8; OUT_SIZE],

}

/// device IO output
pub enum Output {

	Ok,
	Wait,
	Ignore,
	NoUsedIO,
	WrongData,

}

/// device type
#[derive(Clone)]
pub enum Type {

	Misc, // misc are in device root
	Output,
	Input,
	Disk,
	Memory,
	Terminal,

}

/// device name type alias
pub type Name = [u8; 8];

/// null device that do nothing
const NULL: Device = Device {write: null_write, read: null_read, new: null_new, name: [0; 8], t: Type::Misc, lock: false};
pub const DEFAULT_OUT: [u8; OUT_SIZE] = [0; OUT_SIZE];
pub const STATE_NULL: State = State {inp: 0, inp_at: 0, out: DEFAULT_OUT, out_ind: 0};

/// list of devices
static mut LIST: [Device; DEVICES] = [NULL; DEVICES];
/// current number of devices
static mut LEN: usize = 0;

/// write to device at index
pub fn write(index: usize, s: &mut State, byte: u8) -> Output {

	unsafe {

		((&LIST[index]).write)(s, byte)
	}

}

/// read from device at index
pub fn read(index: usize, s: &mut State, byte: &mut u8) -> Output {

	unsafe {

		((&LIST[index]).read)(s, byte)
	}

}

/// create new descriptor for device at index
pub fn new(index: usize) -> State {

	unsafe {

		((&LIST[index]).new)()
	}

}

/// add new device to index
pub fn push(dev: Device) -> Option<usize> {

	unsafe {

		if LEN == DEVICES {

			None
		} else {
			let old_len = LEN.clone();

			LEN += 1;
			LIST[old_len] = dev;

			Some(old_len)
		}

	}

}

/// create new device object
pub fn new_dev(write: fn(&mut State, u8) -> Output, read: fn(&mut State, &mut u8) -> Output, new: fn() -> State, name: Name, t: Type) -> Device {

	Device {write, read, new, name, t, lock: false}
}

/// init devices
pub fn init() {

	defaults::init();
	io::init();

}

/// null write function
fn null_write(_: &mut State, _: u8) -> Output {Output::NoUsedIO}
/// null read function
fn null_read(_: &mut State, _: &mut u8) -> Output {Output::NoUsedIO}
/// null new function
fn null_new() -> State {STATE_NULL}
