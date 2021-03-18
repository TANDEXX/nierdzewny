#!/bin/nano
#![allow(dead_code)]
// this code does not included to boot crate because it is not finished for now

pub mod keyboard; // key board input device

pub trait Device {

	fn read(&self) -> (Output, u8);
	fn write(&self, byte: u8, s: State<_>) -> Output;

}

pub struct State<T> {

	id: u128,
	val: T,	

}

pub enum Output {

	Ok,
	WrongArg,
	NoPerms,
	NoUsedIO,
	Err,

}
