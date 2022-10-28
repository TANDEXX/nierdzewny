#!/bin/nano

use core::mem::transmute;
use x86_64::instructions::port::Port;
//use x86_64::structures::port::PortWrite;
use crate::util::{num_to_color, str::Str, args::{/*SEPARATOR, */pass as pass_args}};
use crate::device::{Output, new as new_dev, read, defaults::KEY_INDEX as KEY};
use crate::consts::auto::{STR_SIZE, SCROLL_PER_PRESS as SCROLL};
use crate::keyb::map::pass as pass_map;
use crate::keyb::code::Key;
use crate::sc::text;
use text::{SCREEN_POS_VIRT, SCREEN_POS, rewrite_vga, update_vga_cur};

const ARGS: usize = 128;
const SCROLL_ISIZE: isize = SCROLL as isize;
pub static mut BUFFER: Str = Str::new();

fn next(byte: u8) {

	unsafe {

		match byte {

			3 => {
				let after = SCREEN_POS_VIRT as isize - SCROLL_ISIZE;

				if after > -1 {

					SCREEN_POS_VIRT -= SCROLL;

				} else {

					SCREEN_POS_VIRT = 0;

				}

				rewrite_vga();
				update_vga_cur();

			},
			10 | 13 => {

				print_byte(byte);
				pass();
				prompt();
				BUFFER = Str::new();

			},
			4 => {
				let after = SCREEN_POS_VIRT + SCROLL;

				if after > SCREEN_POS {

					SCREEN_POS_VIRT = SCREEN_POS;

				} else {

					SCREEN_POS_VIRT = after;

				}

				rewrite_vga();
				update_vga_cur();

			},
			127 => {

				if BUFFER.len != 0 {
					let sym = to_symbol(BUFFER.pop());

					match sym {

						Some(key) => {

							for _ in 0..key.len() {

								print_byte(127);

							}

						},
						None => {

							print_byte(127);

						},

					}

				}

			},
			_ => {

				if BUFFER.len != STR_SIZE {
					let sym = to_symbol(byte);

					match sym {

						Some(key) => {

							print_byte(15);
							print_bytes(key);
							print_byte(22);

						},
						None => print_byte(byte),

					}

					BUFFER.push(&[byte]);

				}

			},

		}

	}

}

pub fn open() {
	unsafe {
		let mut at = false;
		let mut key = 0u8;
		let mut s = new_dev(KEY);
		let mut buffer = 0u8;
		let mut press;

		print_bytes(b"\x0fsecurity shell started, type `help` for any information\x10\n");
		prompt();

		loop {

			match read(KEY, &mut s, &mut buffer) {

				Output::Ok => {

					if at {
						press = transmute(buffer);
						let char = pass_map(Key {char: key, press: press});

						if char != 0 && press {

//							print_byte(14);
//							print_byte(char);
							next(char);

						}

						at = false;

					} else {

						at = true;
						key = buffer;

					}

				},
				Output::Wait => {

//					print_bytes(b", wait");
					hlt!();

				},
				Output::Ignore => {

//					print_bytes(b", ignore");
					key = !key;

				},
				_ => {},

			}
		
		}

	}

}

fn prompt() {

	print_bytes(b"nierdzewny* ");

}

pub fn pass() {

	unsafe {
		let (args, mut len) = pass_args::<ARGS>(&BUFFER);
		let command: &[u8] = &BUFFER.bytes[0..args[0].1];

		if command == b"echo" || command == b"println" {

			if len != 2 {

				for x in (&args).iter().skip(1) {
					let tmp = &BUFFER.bytes[x.0..x.1];

					len -= 1;
					print_bytes(tmp);

					if len == 2 {

						break;

					}

					print_byte(b' ');

				}

			}

			print_byte(b'\n');

		} else if command == b"print" {

			if len != 2 {

				for x in (&args).iter().skip(1) {
					let tmp = &BUFFER.bytes[x.0..x.1];

					len -= 1;
					print_bytes(tmp);

					if len == 2 {

						break;

					}

					print_byte(b' ');

				}

			}

		} else if command == b"help" {

			if len == 2 {

				print_bytes(b"help:
	command that show help of different components of this shell.
	usage is `help COMPONENT` or `help COMMAND`
components:
	numbers
command list:
	echo, println, print, help, panic, poweroff, mem, port, term
");

			} else {
				let arg = arg(args, 1);

				print_bytes(b"help: ");
				print_bytes(match arg {

					b"commands" => b"type a command name no `commands`, example: `help echo`",
					b"help" => b"output help message",
					b"echo" | b"println" => b"print given text with new line",
					b"print" => b"print given text without new line",
					b"panic" => b"calls system panic",
					b"poweroff" => b"turns off power (shutdown system)",
					b"mem" => b"manage ram memory, operations:
	be carfour because writing or reading from wrong addresses causes DOUBLE FAULT
	read [ADDRESS]: reads memory address
	write [ADDRESS] [NUMBER_VALUE]: writes to address number_value",
					b"port" => b"IO with serial ports, operations:
	read [PORT] [OPTIONS]: reads data from port
	write [PORT] [NUMBER_VALUE] [OPTIONS]: writes to port
options:
	--8bit	manipulate with 8 bit value
	--16bit	manipulate with 16 bit value
	--32bit	manipulate with 32 bit value",
					b"term" => b"manage terminal, operations:
	clear	clear terminal buffer (TODO)
	creset	reset terminal color
	chightlight	set terminal color to hightlight
	cbg	set terminal background color
	cfg	set terminal foreground (text) color
	refresh	repaint visible buffer to vga",
	b"numbers" => b"info:
	numbers are defined by just writing them, but there are some number formats
number formats:
	to write normal number you write: `123`, `753664`
	to write hexadecimal number you write: `0x7b`, `xb8000`
	to write binary number you write: `0b1111011`, `b1011100000000000000`
	to write octal number you write: `0o173`, `o2700000`",
					_ => b"unknown component",

				});
				print_byte(10);

			}

		} else if command == b"panic" {

			print_bytes(b"\x0fsystem panic\x10\n");
			panic!("user triggered panic");

		} else if command == b"mem" {

			if len < 4 {

				print_bytes(b"mem: expected more arguments, try `help mem`\n");

			} else {
				let op = arg(args, 1);
				let address = arg(args, 2);

				if op == b"read" {
					let pointer = parse(address) as usize as *mut u8;

					print_bytes(&Str::from_unsigned_num(pointer.read() as u128).as_slice());
					print_byte(10);

				} else if op == b"write" {

					if len == 4 {

						print_bytes(b"mem: number_value expected\n");

					} else {
						let val = arg(args, 3);
						let pointer = parse(address) as usize as *mut u8;

						pointer.write(parse(val) as u8);

					}

				} else {

					print_bytes(b"mem: unknown operation `");
					print_bytes(op);
					print_bytes(b"`\n");

				}

			}

		} else if command == b"port" {

			if len < 4 {

				print_bytes(b"port: expected more arguments, try `help port` for more information\n");

			} else {
				let op = arg(args, 1);
				let port = arg(args, 2);

				if op == b"read" {
					let bits = port_args(3, &args, len);

					if bits == 0 {
						let mut port: Port<u8> = Port::new(parse(port) as u16);

						print_bytes(&Str::from_unsigned_num(port.read() as u128).as_slice());
						print_byte(10);					

					} else if bits == 1 {
						let mut port: Port<u16> = Port::new(parse(port) as u16);

						print_bytes(&Str::from_unsigned_num(port.read() as u128).as_slice());
						print_byte(10);					

					} else if bits == 2 {
						let mut port: Port<u32> = Port::new(parse(port) as u16);

						print_bytes(&Str::from_unsigned_num(port.read() as u128).as_slice());
						print_byte(10);					

					}

				} else if op == b"write" {

					if len == 4 {

						print_bytes(b"port: number_value expected\n");

					} else {
						let val = arg(args, 3);
						let bits = port_args(3, &args, len);

						if bits == 0 {
							outb!(parse(port) as u16, parse(val) as u8);

						} else if bits == 1 {
							let mut port = Port::new(parse(port) as u16);

							port.write(parse(val) as u16);

						} else if bits == 2 {
							let mut port = Port::new(parse(port) as u16);

							port.write(parse(val) as u32);

						}

					}

				} else {

					print_bytes(b"port: unknown operation `");
					print_bytes(op);
					print_bytes(b"`\n");

				}

			}

		} else if command == b"term" {

			if len < 3 {

				print_bytes(b"term: expected more arguments, try `help term`\n");

			} else {
				let op = arg(args, 1);

				if op == b"clear" {

					print_bytes(b"clear not supported yet\n");

				} else if op == b"creset" {

					print_byte(16);

				} else if op == b"chightlight" {

					print_byte(15);

				} else if op == b"cboth" {

					if len < 4 {

						print_bytes(b"term: color: expected color value but not found, try `help term`\n");

					} else {
						let arg = arg(args, 2);

						print_byte(5);
						print_byte(parse(arg) as u8);

					}

				} else if op == b"cbg" {

					if len < 4 {

						print_bytes(b"term: cbg: expected color value but not found, try `help term`\n");

					} else {
						let arg = arg(args, 2);

						print_byte(4);
						print_byte(num_to_color(parse(arg) as u8));

					}

				} else if op == b"cfg" {

					if len < 4 {

						print_bytes(b"term: cfg: expected color value but not found, try `help term`\n");

					} else {
						let arg = arg(args, 2);

						print_byte(3);
						print_byte(num_to_color(parse(arg) as u8));

					}

				} else if op == b"refresh" {

					text::rewrite_vga();

				} else {

					print_bytes(b"term: unknown operation `");
					print_bytes(op);
					print_bytes(b"`\n");

				}

			}

		} else if command == b"poweroff" {

			if len == 2 {

				crate::shutdown(false);

			} else {

				print_bytes(b"poweroff: use with no arguments\n");

			}

		} else if BUFFER.len == 0 {} else {

			print_bytes(b"nierdzewny: `");
			print_bytes(command);
			print_bytes(b"` command not found\n");

		}

	}

}

fn port_args(start: usize, args: &[(usize, usize)], len: usize) -> u8 {
	let mut i = start;
	let mut bitnes = 0u8;

	while i < len {
		let arg = &unsafe {&BUFFER}.bytes[args[i].0..args[i].1];

		if arg == b"--8bit" {

			bitnes = 0;

		} else if arg == b"--16bit" {

			bitnes = 1;

		} else if arg == b"--32bit" {

			bitnes = 2;

		}

		i += 1;
	}

	bitnes
}

fn parse(bytes: &[u8]) -> u128 {

	Str::from_bytes(unsafe {transmute::<&[u8], &'static [u8]>(bytes)}).parse_unsigned()
}

fn arg(args: [(usize, usize); ARGS], index: usize) -> &'static [u8] {

	unsafe {transmute::<&[u8], &'static [u8]>(&BUFFER.bytes[args[index].0..args[index].1])}
}

fn print_byte(byte: u8) {

	text::write_byte(byte);

}

fn print_bytes(bytes: &[u8]) {

	for x in bytes {

		print_byte(*x);

	}

}

fn to_symbol(byte: u8) -> Option<&'static [u8]> {

	match byte {

		1 => Some(b"Home"),
		2 => Some(b"End"),
		3 => Some(b"PgUp"),
		4 => Some(b"PgDown"),
		5 => Some(b"Del"),
		6 => Some(b"Insert"),
		9 => Some(b"Tab"),
		14 => Some(b"F1"),
		15 => Some(b"F2"),
		16 => Some(b"F3"),
		17 => Some(b"ArrRight"),
		18 => Some(b"ArrLeft"),
		19 => Some(b"ArrUp"),
		20 => Some(b"ArrDown"),
		22 => Some(b"F4"),
		23 => Some(b"F5"),
		24 => Some(b"F6"),
		25 => Some(b"F7"),
		26 => Some(b"F8"),
		27 => Some(b"Esc"),
		28 => Some(b"F10"),
		_ => None,

	}
}
