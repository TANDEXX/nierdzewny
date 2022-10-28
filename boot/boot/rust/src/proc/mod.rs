#!/bin/nano
//pub mod exception;

/// processor support for x86_64
pub mod x86_64;
pub use x86_64 as carch;
