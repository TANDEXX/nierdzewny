#!/bin/nano
#![allow(unused_macros)]

/// hlt instruction – stops processor until next interrupt
macro_rules! hlt {

	() => {

		#[allow(unused_unsafe)]
		unsafe {

			asm!("hlt");

		}

	}

}

/// function to permamently stop the processor
macro_rules! stop_cpu {

	() => {

		#[allow(unused_unsafe)]
		unsafe {

			asm!("cli", "hlt", options(noreturn));
		}

	}

}

/// enter infinite halting loop
macro_rules! end {

	() => {

		#[allow(unused_unsafe)]
		unsafe {

			asm!("call end_fn", options(noreturn));
		}

	}

}

macro_rules! inb {

	(

		$port:expr, $variable:ident

	) => {

		#[allow(unused_unsafe)]
		unsafe {

			llvm_asm!("inb %dx, %al" : "={al}"($variable) : "{dx}"($port) :: "volatile");

		}

	}

}

macro_rules! inw {

	(

		$port:expr, $variable:ident

	) => {

		#[allow(unused_unsafe)]
		unsafe {

			llvm_asm!("inw %dx, %ax" : "={ax}"($variable) : "{dx}"($port) :: "volatile");

		}

	}

}

macro_rules! ind {

	(

		$port:expr, $variable:ident

	) => {

		#[allow(unused_unsafe)]
		unsafe {

			llvm_asm!("inl %dx, %eax" : "={eax}"($variable) : "{dx}"($port) :: "volatile");

		}

	}

}

/// the infinite halting loop
#[no_mangle]
extern "C" fn end_fn() -> ! {

	unsafe {

		asm!("hlt", "jmp end_fn");
		core::hint::unreachable_unchecked()
	}
}
