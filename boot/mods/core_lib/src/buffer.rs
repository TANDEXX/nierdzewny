#!/bin/nano
//! variable buffering utility (to push on top and read from bottom)

#[repr(C)]
pub struct Buffer<T: Clone, const LENGTH: usize> {

	buffer: [T; LENGTH],
	start: usize,
	end: usize,
	zero: bool,

}

impl<T: Clone + Copy, const LENGTH: usize> Buffer<T, LENGTH> {

	/// create new buffer
	pub const fn new(default: T) -> Self {

		Buffer {buffer: [default; LENGTH], start: 0, end: 0, zero: true}
	}

	/// pushes variable on top of buffer
	/// outputs `Err(())` if overflowed
	pub fn push(&mut self, v: T) -> Result<(), ()> {

		if self.start != self.end || self.zero {

			self.zero = false;
			self.buffer[self.end] = v;
			self.end += 1;

			if self.end == LENGTH {

				self.end = 0;

			}

			Ok(())
		} else {

			Err(())
		}

	}

	/// pops variable from bottom of buffer
	/// outputs `None` if there are no variables
	pub fn pop(&mut self) -> Option<T> {

		if self.zero {

			None
		} else {
			let result = self.buffer[self.start].clone();

			self.reject_non_zero();

			Some(result)
		}
	}

	/// pops variable from bottom of buffer but doesn't return anything
	pub fn reject(&mut self) {

		if !self.zero {

			self.reject_non_zero();

		}

	}

	fn reject_non_zero(&mut self) {

		self.start += 1;

		if self.start == LENGTH {

			self.start = 0;

		}

		if self.start == self.end {

			self.zero = true;

		}

	}

	/// returns variable from bottom of buffer without removing it
	pub fn head(&mut self) -> &mut T {

		&mut self.buffer[self.start]
	}

	/// says length of buffer
	pub fn len(&self) -> usize {

		if self.start > self.end {

			LENGTH - self.start + self.end
		} else {

			if !self.zero && self.end == self.start {

				LENGTH
			} else {

				self.end - self.start
			}
		}
	}

	/// says do buffer is full
	pub fn full(&self) -> bool {

		self.len() == LENGTH
	}

}
