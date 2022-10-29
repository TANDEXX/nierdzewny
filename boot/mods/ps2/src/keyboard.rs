#!/bin/nano

use core::mem::transmute;
use crate::{outb, inb};
use crate::util::{str::Str};
use crate::sc::text::{write_byte, write_bytes};
use crate::mods::core_lib::buffer::Buffer;
use super::QUEUE_LEN;

/// error code first
const ERR0: u8 = 0;
/// error code second
const ERR1: u8 = 0xff;
/// self test passed
const SPASS: u8 = 0xaa;
/// echo response
const ECHO: u8 = 0xee;
/// command acknowledged
const ACK: u8 = 0xfa;
/// self test failed code first
const SFAIL0: u8 = 0xfc;
/// self test failed code second
const SFAIL1: u8 = 0xfd;
/// resend the commmand or data
const RESEND: u8 = 0xfe;
/// used in keymap static to make writing of it easier
const Z: (u8, char) = (0, 0 as char);

static mut INPUT_RETRY: bool = false;
/// do keyboard currently resets
static mut RESET: bool = false;
/// queue of commands to be sent
static mut COMMANDS: Buffer<Command, QUEUE_LEN> = Buffer::new(Command{comm: 0, data: 0, state: 0, resend: 0});
/// current selected keymap
static mut CURRENT_KEY_MAP: usize = 1;
/// reached key map layer
static mut CURRENT_KEY_MAP_LAYER: usize = 0;
/// setted key mapping (default to `US QWERTY`)
/// characters have u8 type too for defining do it is scan code for presing or releasing, (maybe in future it will have more uses)
static KEY_MAP: /* mappings */ &'static [ /* mapping layers */ &'static [(&'static [u8], &'static [(u8, char)])]] = &[

	&[],
	&[

		(&[0xe0, 0xe1], &[
			Z, (1, 27 as char),
			(1, '1'), (1, '2'), (1, '3'), (1, '4'), (1, '5'), (1, '6'), (1, '7'), (1, '8'), (1, '9'), (1, '0'), (1, '-'), (1, '='),
			(1, 8 as char) /* backspace */, (1, '\t'),
			(1, 'q'), (1, 'w'), (1, 'e'), (1, 'r'), (1, 't'), (1, 'y'), (1, 'u'), (1, 'i'), (1, 'o'), (1, 'p'), (1, '['), (1, ']'),
			(1, '\n'), (1, 17 as char),
			(1, 'a'), (1, 's'), (1, 'd'), (1, 'f'), (1, 'g'), (1, 'h'), (1, 'j'), (1, 'k'), (1, 'l'), (1, ';'), (1, '\''), (1, '`'),
			(1, 16 as char), (1, '\\'), (1, 'z'), (1, 'x'), (1, 'c'), (1, 'v'), (1, 'b') /**/, (1, 'n'), (1, 'm'), (1, ','), (1, '.'), (1, '/'),
			(1, 2 as char) /* right shift */, (1, '*'), (1, 18 as char) /* left alt */, (1, ' '), (1, 20 as char) /* caps lock */,
			(1, 112 as char), (1, 113 as char), (1, 114 as char), (1, 115 as char), (1, 116 as char), (1, 117 as char), (1, 118 as char), (1, 119 as char), (1, 120 as char), (1, 121 as char) /* f1-10 */, (1, 144 as char) /* number lock */, (1, 145 as char) /* scroll lock */,
			(1, 103 as char), (1, 104 as char), (1, 105 as char) /* numpad 7-9 */, (1, 109 as char) /* numpad - */, (1, 100 as char), (1, 101 as char), (1, 102 as char) /* numpad 4-6 */, (1, 107 as char) /* numpad + */, (1, 97 as char), (1, 98 as char), (1, 99 as char) /* numpad 1-3 */, (1, 96 as char) /* numpad 0 */, (1, 190 as char) /* numpad decimal */,
			Z, Z, Z, Z, (1, 122 as char), (1, 123 as char) /* f11-12 */,
		]),
		(&[0x1d, 0x2a, 0xb7], &[
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, (1, 177 as char) /* previous track */,
			Z, Z, Z, Z, Z, Z, Z, Z, (1, 176 as char) /* next track */,
			Z, Z, (1, '\r') /* numpad enter */, (1, 1 as char) /* control */, Z, Z, Z, (1, 173 as char) /* mute */, Z /* calculator, not supported */, (1, 179 as char) /* play */, Z, (1, 178 as char) /* stop pressed */,
			Z, Z, Z, Z, Z, Z, Z, Z, Z, (1, 174 as char) /* volume down */, Z, (1, 175 as char) /* volume up */, Z, Z /* www home, not supported */, Z, Z, (1, 111 as char) /* numpad divide */, Z, Z, (1, 225 as char) /* alt gr (right alt) */,
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, (1, 36 as char) /* home */, Z /* cursor up, not supported */, (1, 33 as char) /* page up */, Z, Z /* cursor left, not supported */, Z, Z /* cursor right, not supported */, Z, (1, 35 as char) /* end */, Z /* cursor down, not supported */, (1, 34 as char) /* page down */, (1, 45 as char) /* insert */, (1, 46 as char) /* delete */,
			Z, Z, Z, Z, Z, Z, Z, Z /* left gui, not supported */, Z /* right gui, not supported */, Z /* apps, not supported */, Z /* power, not supported */, (1, 95 as char) /* sleep */, Z, Z, Z, Z /* wake, not supported */, Z, Z, Z /* www search */, Z /* www 
			favorites */, (1, 168 as char) /* www refresh */, Z /* www stop, not supported */, Z /* www forwards, not supported */, Z /* www back, not supported */, Z /* my computer, not supported */, Z /* email, not supported */, Z /* media select, not supported */, 
			Z, Z,
		]),
		(&[0xe0, 0x45], &[]),
		(&[0xe1], &[
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, (1, 44 as char) /* print screen pressed */,
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, (0, 44 as char) /* print screen released */
		]),

	],

];

//static _KEY_MAP: &'static [&'static [&'static [(u8, char)]]] = &[

//	&[

//		&[
//			Z, 27 as char,
//			'1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '+',
//			8 /* tab */, '\t',
//			'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']',
//			'\n', 17, /* left control */
//			'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
//			16 /* left shift */, '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',
//			2 /* right shift */, '*', 16 /* left alt */, ' ', 20 /* caps lock */,
//			112, 113, 114, 115, 116, 117, 118, 119, 120, 121 /* f1-10 */, 144 /* number lock */, 145 /* scroll lock */,
//			103, 104, 105 /* numpad 7-9 */, 109 /* numpad minus */, 100, 101, 102 /* numpad 4-6 */, 107 /* numpad + */, 97, 98, 99 /* numpad 1-3 */, 96 /* numpad 0 */, 190 /* numpad decimal */,
//			122, 123 /* f11-12 */
			// TODO "next" flag
//		],

//		&[
//			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, 177 /* previus track */,
//			Z, Z, Z, Z, Z, Z, Z, Z, 176 /* next track */,
//			Z, Z, b'\r' /* numpad enter */, 1 /* control */, Z, Z, 173 /* mute */, Z /* calculator, not supported */, 179 /* play */, Z, 178 /* stop pressed */,
//			Z, Z, Z, Z, Z, Z, Z, Z, Z, 174 /* volume down */, Z, 175 /* volume up */, Z, Z /* www home, not supported */, Z, Z, 111 /* numpad divide */, Z, Z, 225 /* alt gr (right alt) */,
//			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, 36 /* home */, Z /* cursor up, not supported */, 33 /* page up */, Z, Z /* cursor left, not supported */, Z, Z /* cursor right, not supported */, Z, 35 /* end */, Z /* cursor down, not supported */, 34 /* page 
//			down*/, 45 /* insert */, 46 /* delete */
//			Z, Z, Z, Z, Z, Z, Z, Z /* left gui, not supported */, Z /* right gui, not supported */, Z /* apps, not supported */, Z /* power, not suspported (TODO support this) */, 95 /* sleep */, Z, Z, Z, Z /* wake, not supported */, Z, Z, Z /* www search */, Z /* 
//		www favorites */, 168 /* www favorites */, Z /* www stop, not supported */, Z /* www forwards, not supported */, Z /* www back, not supported */, Z /* my computer, not supported */, Z /* email, not supported */, Z /* media select, not supported */, Z, Z,
//		],

//	],

//];

#[derive(Clone, Copy)]
struct Command {

	comm: u8,
	data: u8,
	state: u8, // 0 bit: command sent, 1 bit: do send data
	resend: u8,

}

/// called at keyboard interrupt
pub fn interrupt() {

	unsafe {

//write_bytes(b"keyb interrupt\n");
		if can_read() {

			read();

		} else {

			INPUT_RETRY = true;

		}

	}

}

fn read() {

	unsafe {
		let input = inb(0x60);

		match input {

			ERR0 | ERR1 => {
				write_byte(b'E');

				// TODO

			},
			ECHO => {
				write_byte(b'e');

				// TODO

			},
			ACK => {
				let comm = COMMANDS.head();

				if comm.state & 0b1 != 0 || comm.state & 0b10 == 0 { // command send || do not send data

					COMMANDS.reject();

				} else {

					comm.state |= 0b1;

				}

			},
			RESEND => {
				let comm = COMMANDS.head();

				comm.resend += 1;

				if comm.resend == 3 { // command not supported or hardware failure

					COMMANDS.reject();

				}

			},
			_ => {
				let mut get_scan = true;
				let layer = KEY_MAP[CURRENT_KEY_MAP][CURRENT_KEY_MAP_LAYER];

				if RESET {

					get_scan = false;
					RESET = false;

					if input == SPASS {



					} else if input == SFAIL0 || input == SFAIL1 {



					}

				} else {

					for next in layer.0 {

						if input == *next {

							CURRENT_KEY_MAP_LAYER += 1;
							get_scan = false;

						}

					}

				}

				if get_scan && layer.1.len() > if CURRENT_KEY_MAP == 1 {input & 0b01111111} else {input} as usize {
					let mut press = true;
					let addr = if CURRENT_KEY_MAP == 1 && layer.1.len() <= input as usize {

						if input & 0b10000000 != 0 {

							press = false;

						}

						input & 0b01111111
					} else {

						input
					};
					let (attr, key) = layer.1[addr as usize];

					if CURRENT_KEY_MAP != 1 {

						press = transmute(attr & 0b1);

					}

					CURRENT_KEY_MAP_LAYER = 0;

					if key as u32 != 0 {
						let column = input & 0b11100000 >> 5; // TODO FIKSZ YT
						let row = input & 0b00011111;

						write_bytes(b"KBD character: "); // TODO complete
						write_byte(key as u8);
						write_bytes(b", press: ");
						write_bytes(if press {b"true"} else {b"false"});
						write_bytes(b", col: ");
						write_bytes(Str::from_unsigned_num(column as u128).as_slice());
						write_bytes(b", row: ");
						write_bytes(Str::from_unsigned_num(row as u128).as_slice());
						write_byte(10);

					}

				}

			},

		}

	}

}

/// called at timer interrupt
pub fn timer() {

	unsafe {

		if INPUT_RETRY && can_read() {

//write_bytes(b"read retry\n");
			read();
			INPUT_RETRY = false;

		} else {

			if COMMANDS.len() != 0 && can_write() {
				let comm = COMMANDS.head();

//write_bytes(b"write to 0x60\n");

				if comm.state & 0b1 == 0 { // command not send

//write_bytes(b"C");
					outb(0x60, comm.comm);

				} else if comm.state & 0b10 != 0 { // command send && do send data

//write_bytes(b"D");
					outb(0x60, comm.data);

				}

			}

		}

	}

}

pub fn reinit() {

	reset();


}

pub fn reset() {

	command_send(0xff, 0, false);

}

pub fn enable_scanning() {

	command_send(0xf4, 0, false);

}

pub fn disable_scanning() {

	command_send(0xf5, 0, false);

}

pub fn set_leds(data: u8) {

	command_send(0xed, data, true);

}

pub fn set_scan_code_set(keymap: u8) {

	command_send(0xf0, keymap, true);

}

/// reset scan code data in driver
pub fn check_scan_code_set() {

	command_send(0xf0, 0, false);

}

/// convert to ps/2 format and set typematic rate and delay
pub fn set_typematic(repeat: u8, delay_before: u16) {
	let mut data = match repeat {

		_ => 0,

	};

	data |= match delay_before {

		..375 => 0,
		376..625 => 1,
		626..875 => 2,
		_ => 3,

	} << 4;
	set_formatted_typematic(data);

}

/// set typematic rate and delay with ps/2 format
pub fn set_formatted_typematic(data: u8) {

	command_send(0xf3, data, true);

}

/// append new command to queue
fn command_send(comm: u8, data: u8, send_data: bool) {

	unsafe {

		if !COMMANDS.full() {

			match COMMANDS.push(Command {comm, data, state: transmute::<bool, u8>(send_data) << 1, resend: 0}) {

				Ok(()) => {},
				Err(()) => {},

			};

		}

	}

}

fn can_write() -> bool {

	inb(0x64) & 0b10 == 0
}

fn can_read() -> bool {

	inb(0x64) & 1 != 0
}
