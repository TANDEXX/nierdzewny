#!/bin/nano

const LEN: usize = 256;

pub struct Str {

	bytes: [u8; LEN],
	len: usize,

}

impl Str {

	/*pub fn new() -> Self {

		Str{bytes: [0; LEN], 0}
	}*/

	pub fn from_bytes(bytes: &'static [u8]) -> Self {
		let mut buffer = [255; LEN];

		for (i, &byte) in bytes.iter().enumerate() {

			buffer[i] = byte;

		}

		Str{bytes: buffer, len: bytes.len()}
	}

	pub fn push(&mut self, bytes: &[u8]) {

		for (i, &byte) in bytes.iter().enumerate() {

			self.bytes[i + self.len] = byte;

		}

		self.len += bytes.len();

	}

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

}
