#!/bin/nano

use core::ops;
use core::mem::transmute;
use core::marker::PhantomData;

pub struct ArrayPointer<T, const LENGTH: usize> (usize, PhantomData<T>);

impl<T, const LENGTH: usize> ArrayPointer<T, LENGTH> {

	pub const unsafe fn new(addr: usize) -> Self {

		Self(addr, PhantomData)
	}

	pub const fn len() -> usize {

		LENGTH
	}

}

impl<T, const LENGTH: usize> ops::Index<usize> for ArrayPointer<T, LENGTH> {
	type Output = T;

	/// this is unsafe, but can't be marked as unsafe
	fn index(&self, idx: usize) -> &Self::Output {

		unsafe {transmute((self.0 + idx) as * const T)}
	}

}

impl<T, const LENGTH: usize> ops::IndexMut<usize> for ArrayPointer<T, LENGTH> {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {

		unsafe {transmute((self.0 + idx) as * mut T)}
	}
}
