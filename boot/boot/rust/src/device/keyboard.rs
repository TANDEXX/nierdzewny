#!/bin/nano
// this code does not include to the boot crate because I don't writed this fully for now
use core::mem::transmute;
use super::{Device, Output, State};
use crate::keyb::code::Key;

pub struct Keyboard {

	last: Key,
	state: u8,

}

impl Device for KeyBoard {

	fn write(&self, byte: u8) -> Output {

		if self.state == 0 {

			self.last.char = byte;

		} else {

			self.last.press = unsafe {transmute(byte)};

		}

		Output::Ok
	}

	fn read(&self, s: &mut State<u8>) -> (Output, u8) {
		let mut byte = 0;

		if s.val == 0 {

			byte = self.last.char;
			s.val = 1;

		} else {

			byte = unsafe {transmute(self.last.press)};
			s.val = 2;

		}

		(Output::Ok, byte)
	}

}
