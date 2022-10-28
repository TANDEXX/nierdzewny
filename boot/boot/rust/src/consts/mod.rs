#!/bin/nano

pub mod auto;

/// vga text mode memory address
pub const VGA_TEXT: usize = 0xb8000; // b8000
/// used memory offset
pub const MEM_START: usize = 0xffff;
