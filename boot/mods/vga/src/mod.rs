#!/bin/nano

pub mod registers;
pub mod video;
pub mod term;

pub fn init() {

	term::init();

}
