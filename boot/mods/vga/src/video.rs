#!/bin/nano
//! managing video memory

use core::mem::transmute;

use crate::mods::core_lib::mem::ArrayPointer;

pub static mut TEXT_MODE_80X25: ArrayPointer<Color16Char, {80 * 25}> = unsafe {ArrayPointer::new(0xb8000)};

#[repr(transparent)]
pub struct Colors16 (u8);

#[repr(transparent)]
pub struct Char (u8);

#[repr(C, packed)]
pub struct Color16Char {

	char: Char,
	color: Colors16,

}
