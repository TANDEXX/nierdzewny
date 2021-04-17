#!/bin/nano

//use crate::vga::write_byte;
use crate::util::xor;
//use crate::device::{State, write, new};
use super::code::Key;

/// pass key map
pub fn pass(key: Key) -> u8 {

	KEYMAP(key)
}

/// current keymap
static KEYMAP: fn(Key) -> u8 = default_keymap;
/// shift is pressed
static mut SHIFT: bool = false;
/// alt is pressed
static mut ALT: bool = false;
/// capslock on
static mut CAPSLOCK: bool = false;

/// default keymap
fn default_keymap(key: Key) -> u8 {
	let mut code = key.char;
	let big_letter: bool;

	unsafe {

		if code != b'!' && (key.press || (code > 28 && code < 32)) {

			if code == 255 {

				CAPSLOCK = !CAPSLOCK;
//				if CAPSLOCK {write_byte(b'T');} else {write_byte(b'F');}

			}

			if code == 29 {

				SHIFT = key.press;

			}

			big_letter = xor(SHIFT, CAPSLOCK);

			if code == 30 {

				ALT = key.press;

			}

			if big_letter {

				if code > 96 && code < 123 { // abcdefghijklmnoprstuwxyz

					code -= 32;

				} else if code == 39 { // '

					code = b'\"';

				} else if code == 44 { // ,

					code = b'<';

				} else if code == 46 { // .

					code = b'>';

				} else if code == 47 { // /

					code = b'?';

				} else if code == 45 { // -

					code = b'_';

				} else if code == 48 { // 0

					code = b')';

				} else if code == 49 { // 1

					code = b'!';

				} else if code == 50 { // 2

					code = b'@';

				} else if code == 51 { // 3

					code = b'#';

				} else if code == 52 { // 4

					code = b'$';

				} else if code == 53 { // 5

					code = b'%';

				} else if code == 54 { // 6

					code = b'^';

				} else if code == 55 { // 7

					code = b'&';

				} else if code == 56 { // 8

					code = b'*';

				} else if code == 57 { // 9

					code = b'(';

				} else if code == 59 { // ;

					code = b':';

				} else if code == 61 { // =

					code = b'+';

				} else if code > 90 && code < 94 { // [\]

					code += 32;

				} else if code == 96 { // `

					code = b'~';

				}

			}

			if ALT {

				// alts

				if code == 33 { // !

					code = 186;

				} else if code == 35 { // #

					code = 188;

				} else if code == 36 { // $

					code = 200;

				} else if code == 37 { // %

					code = 201;

				} else if code == 38 { // &

					code = 203;

				} else if code == 40 { // (

					code = 205;

				} else if code == 41 { // )

					code = 206;

				} else if code == 42 { // *

					code = 204;

				} else if code == 44 { // ,

					code = 174;

				} else if code == 45 { // -

					code = 196;

				} else if code == 46 { // .

					code = 175;

				} else if code == 47 { // /

					code = 168;

				} else if code == 48 { // 0

					code = 248;

				} else if code == 49 { // 1

					code = 173;

				} else if code == 50 { // 2

					code = 171;

				} else if code == 51 { // 3

					code = 172;

				} else if code == 52 { // 4

					code = 249;

				} else if code == 53 { // 5

					code = 250;

				} else if code == 54 { // 6

					code = 239;

				} else if code == 55 { // 7

					code = 21;

				} else if code == 56 { // 8

					code = 240;

				} else if code == 57 { // 9

					code = 252;

				} else if code == 61 { // =

					code = 241;

				} else if code == 64 { // @

					code = 187;

				} else if code == 65 { // A

					code = 142;

				} else if code == 67 { // C

					code = 128;

				} else if code == 68 { // D

					code = 133;

				} else if code == 69 { // E

					code = 144;

				} else if code == 70 { // F

					code = 157;

				} else if code == 73 { // I

					code = 141;

				} else if code == 78 { // N

					code = 165;

				} else if code == 79 { // O

					code = 153;

				} else if code == 80 { // P

					code = 149;

				} else if code == 81 { // Q

					code = 146;

				} else if code == 82 { // R

					code = 137;

				} else if code == 83 { // S

					code = 143;

				} else if code == 84 { // T

					code = 140;

				} else if code == 85 { // U

					code = 154;

				} else if code == 89 { // Y

					code = 162;

				} else if code == 94 { // ^

					code = 202;

				} else if code == 96 { // `

					code = 253;

				} else if code == 97 { // a

					code = 132;

				} else if code == 99 { // c

					code = 135;

				} else if code == 100 { // d

					code = 131;

				} else if code == 105 { // i

					code = 139;

				} else if code == 101 { // e

					code = 130;

				} else if code == 102 { // f

					code = 159;

				} else if code == 110 { // n

					code = 164;

				} else if code == 111 { // o

					code = 148;

				} else if code == 112 { // p

					code = 147;

				} else if code == 113 { // q

					code = 145;

				} else if code == 115 { // s

					code = 134;

				} else if code == 114 { // r

					code = 136;

				} else if code == 116 { // t

					code = 158;

				} else if code == 117 { // u

					code = 129;

				} else if code == 121 { // y

					code = 152;

				} else if code == 126 { // ~

					code = 185;

				}

			}

			if (code < 29 || code > 31) && code != 255 {

				code
			} else {

				0
			}

		} else {

			0
		}

	}

}
