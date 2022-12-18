#!/bin/nano
//! terminal implementations

pub static mut SAFETY_TERM: &dyn Term = &Empty{};

pub trait Term {

	/// it should start painting thread to paint all changes and start listening to keyboard
	fn set_active(&mut self);
	/// it should end painting thread and stop listening to keyboard
	fn set_unactive(&mut self);
	/// only this needs to be implemented for writing bytes
	/// shouldn't panic because isn't active
	fn write_byte(&mut self, byte: u8);
	fn write_bytes(&mut self, bytes: &[u8]) {

		for byte in bytes {

			self.write_byte(*byte);

		}

	}

	fn write_str(&mut self, str: &str) {
		self.write_bytes(str.as_bytes());
	}

}

pub struct Empty {}

impl Term for Empty {

	fn set_active(&mut self) {}
	fn set_unactive(&mut self) {}
	fn write_byte(&mut self, _byte: u8) {
		panic!("tried to write on unset terminal")
	}

}
