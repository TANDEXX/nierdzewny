#!/bin/nano
//! system allocator, it'sn't in module because it would be little hard to make (it should work like just implemented but wouldn't work without any module which makes it work)

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

#[global_allocator]
static ALLOC: Alloc = Alloc;

pub struct Alloc;

unsafe impl GlobalAlloc for Alloc {

	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
		null_mut()
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

	}

}

#[alloc_error_handler]
fn alloc_err(_layout: Layout) -> ! {

	panic!("allocator error");
}
