#!/bin/nano

use core::ops::Index;
use core::mem::transmute;
use crate::consts::auto::STR_SIZE as LEN;

/// the string structure
/// use it if you can't or if you not want to use heap
#[derive(Clone)]
pub struct Str {

	pub bytes: [u8; LEN],
	pub len: usize,

}

/// methods for string
impl Str {

	/// creates new constant Str
	/// # Examples
	/// ```
	/// let string = Str::new();
	/// ```
	/// ```
	/// const DEFAULT: Str = Str::new();
	/// ```
	pub const fn new() -> Self {

		Str {bytes: [0; LEN], len: 0}
	}

	/// turns string into unsigned 128-bit number
	pub fn parse_unsigned(self) -> u128 {
		let mut buffer = 0u128;
		let mut multiple = 1u128;

		if &self.bytes[0..2] == b"0x" || self[0] == b'x' {
			let start = if self[0] == b'x' {1} else {2};

			for x in (start..self.len).rev() {

				if self[x] > 47 && self[x] < 58 {

					buffer += ((self[x] - 48) as u128 * multiple) as u128;

				} else if self[x] > 96 && self[x] < 103 {

					buffer += ((self[x] - 87) as u128 * multiple) as u128;

				}

				multiple <<= 4;

			}

		} else if &self.bytes[0..2] == b"0b" || self[0] == b'b' {
			let start = if self[0] == b'b' {1} else {2};

			for x in (start..self.len).rev() {

				if self[x] == b'0' || self[x] == b'1' {

					buffer += ((self[x] - 48) as u128 * multiple) as u128;

				}

				multiple <<= 1;

			}

		} else if &self.bytes[0..2] == b"0o" || self[0] == b'o' {
			let start = if self[0] == b'o' {1} else {2};

			for x in (start..self.len).rev() {

				if self[x] > 47 && self[x] < 56 {

					buffer += ((self[x] - 48) as u128 * multiple) as u128;

				}

				multiple <<= 3;

			}

		} else {

			for x in (0..self.len).rev() {

				if self[x] > 47 && self[x] < 58 {

		 			buffer += ((self[x] - 48) as u128 * multiple) as u128;
					multiple *= 10;

				}

			}

		}

		buffer
	}

	/// change num into Str
	/// # Example
	/// ```
	/// let num = 132u128;
	/// let str = Str::from_unsigned_num(num);
	/// ```
	pub fn from_unsigned_num(num: u128) -> Self {
		let mut buffer = [0u8; LEN];
		let mut to_subtract = 0;
		let mut i = 35;

		for x in 1..36 {
			let pow = 10u128.pow(x);
			let result = (num % pow - to_subtract) / (pow / 10);

			to_subtract += result;
			buffer[i] = result as u8 + 48;
			i -= 1;

		}

		i = 1;

		while buffer[i] == b'0' {

			i += 1;

		}

		Str::from_bytes(unsafe {transmute::<&[u8], &'static [u8]>(&buffer[i..36])}.clone())
	}

	/// create from raw bytes
	pub fn from_bytes(bytes: &'static [u8]) -> Self {
		let mut buffer = [255; LEN];

		for (i, &byte) in bytes.iter().enumerate() {

			buffer[i] = byte;

		}

		Str {bytes: buffer, len: bytes.len()}
	}

	/// push new bytes
	/// # Example
	/// ```
	/// let mut str = Str::from_bytes(b"hello");
	///
	/// str.push(b", world");
	/// assert_eq!(str, Str::from_bytes(b"hello, world"));
	/// ```
	pub fn push(&mut self, bytes: &[u8]) {

		for (i, &byte) in bytes.iter().enumerate() {

			self.bytes[i + self.len] = byte;

		}

		self.len += bytes.len();

	}

	pub fn pop(&mut self) -> u8 {

		self.len -= 1;
		self.bytes[self.len].clone()

	}

	/// convert to array with defined size.
	/// if defined size is smaller than actual string then you get only defined size.
	/// if defined size is too big then you get zero bytes at unused
	pub fn as_array<const LEN: usize>(self) -> [u8; LEN] {
		let mut buffer = [0; LEN];

		for (i, &byte) in self.bytes.iter().enumerate() {

			buffer[i] = byte;

			if i >= LEN - 1 {

				break;

			}

		}

		buffer
	}

	pub fn as_slice(self) -> &'static [u8] {

		unsafe {transmute::<&[u8], &'static [u8]>(&self.bytes[0..self.len])}
	}

}

/// implement index for Str
impl Index<usize> for Str {

	type Output = u8;

	fn index(&self, index: usize) -> &u8 {

		&self.bytes[index]
	}

}
