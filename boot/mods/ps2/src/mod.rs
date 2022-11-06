#!/bin/nano

/// ps/2 keyboard
pub mod keyboard;

pub const QUEUE_LEN: usize = 8;

pub fn init() {

	keyboard::reinit();

}

pub fn keyboard_int() {

	keyboard::interrupt();

}

pub fn mouse_int() {

}

pub fn timer_int() {

	keyboard::timer();

}
