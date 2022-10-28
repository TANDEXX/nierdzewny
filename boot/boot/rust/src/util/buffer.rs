#!/bin/nano

#[repr(C)]
pub struct Buffer<T: Clone, const LEN: usize> {

	buffer: [T; LEN],
	start: usize,
	end: usize,
	zero: bool,

}

impl<T: Clone + Copy, const LEN: usize> Buffer<T, LEN> {

	pub const fn new(default: T) -> Self {

		Buffer {buffer: [default; LEN], start: 0, end: 0, zero: true}
	}

	pub fn push(&mut self, v: T) -> Result<(), ()> /* Err on overflow */ {

		if self.start != self.end || self.zero {

			self.zero = false;
			self.buffer[self.end] = v;
			self.end += 1;

			if self.end == LEN {

				self.end = 0;

			}

			Ok(())
		} else {

			Err(())
		}

	}

	pub fn pop(&mut self) -> Option<T> {

		if self.zero {

			None
		} else {
			let result = self.buffer[self.start].clone();

			self.start += 1;

			if self.start == LEN {

				self.start = 0;

			}

			if self.start == self.end {

				self.zero = true;

			}

			Some(result)
		}
	}

	pub fn reject(&mut self) {

		if !self.zero {

			self.start += 1;

			if self.start == LEN {

				self.start = 0;

			}

			if self.start == self.end {

				self.zero = true;

			}

		}

	}

	pub fn head(&mut self) -> &mut T {

		&mut self.buffer[self.start]
	}

	pub fn len(&self) -> usize {

		if self.start > self.end {

			LEN - self.start + self.end
		} else {

			if !self.zero && self.end == self.start {

				LEN
			} else {

				self.end - self.start
			}
		}
	}

	pub fn full(&self) -> bool {

		self.len() == LEN
	}

}

/*
impl<T: Clone + Copy, const LEN: usize> core::opt::Index<usize> for Buffer<T, LEN> {

	type Output = T;
	fn index(&self, idx: usize) -> &Output {

		

	}

}
*/
