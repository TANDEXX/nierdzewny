#!/bin/nano

use super::str::Str;

/// argument separator
pub const SEPARATOR: u8 = b' ';
const STR_END: usize = crate::consts::auto::STR_SIZE - 1;

/// pass arguments
pub fn pass<const SIZE: usize>(str: &Str) -> ([(usize, usize); SIZE], usize) {
	let mut buffer = [(0, 0); SIZE];
	let mut len = 0usize;
	let mut i = first(&str);
	let mut last = 0usize;
	let mut last_separator = 0usize;
	let mut last_isnt_separator = true;

	if str.len == 0 {

		(buffer, 0)
	} else {

		while i < str.len {

			if i == STR_END {

				break;

			}

			if str[i] == SEPARATOR {

				if last_isnt_separator {

					last_separator = i;
					last_isnt_separator = false;

				}

				if if str.len == i - 1 {false} else {str[i + 1] != SEPARATOR} {

					if len == 0 {

						buffer[len].0 = last;

					} else {

						buffer[len].0 = last + 1;

					}

					buffer[len].1 = last_separator;
					last = i;
					len += 1;
					last_isnt_separator = true;

				}

			}

			i += 1;

		}

		if str[i] == SEPARATOR {

			len += 1;

		} else {

			buffer[len] = (last + 1, i);
			len += 2;

		}

		(buffer, len)
	}
}

fn first(str: &Str) -> usize {
	let mut i = 0usize;

	while i < str.len {

		if str[i] != SEPARATOR {

			break;

		}

		i += 1;

	}

	i
}
