#!/bin/nano
//! module to manage vga registers

pub mod low;

/// XXX shouldn't be used to change vga mode
pub fn set_mode(mode: u8) {

	low::set_x3c0(0x10, mode);

}

pub fn set
