#!/bin/nano

pub mod str; // string on stack

pub fn xor(b1: bool, b2: bool) -> bool {

	if b1 {

		!b2

	} else {

		b2

	}

}
