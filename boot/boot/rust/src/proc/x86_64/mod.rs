#!/bin/nano
#![allow(unused)]

/// low level
pub mod low;
/// interrupt functions
pub mod int;

use core::mem::transmute;
use core::arch::asm;
use pic8259_simple::ChainedPics as Pics;
use low::{Gdt, Idt, Tss, IdtEntry, IntStackFrame as Isf, IO_DEFAULT as DIO};

const IDT_DEFAULT_NO_ENTRY: IdtEntry = IdtEntry::new(0, 0, low::IO_MINIMAL);

static mut GUARD_STACK: [u8; 64] = [0; 64];

pub static mut GDT: Gdt = Gdt::new();
pub static mut IDT: Idt = Idt::new([IDT_DEFAULT_NO_ENTRY; 256]);
pub static mut DTSS: Tss = unsafe {Tss::new([0; 3], [0; 7], 0)};
pub static mut PIC: Pics = unsafe {transmute([0u8; 12])};

pub fn init() {

	unsafe {

		let cs = GDT.push(low::GD_KERNEL_CODE);
		let tss = GDT.push_tss(&DTSS);

		DTSS.interrupt_stack_table[0] = &GUARD_STACK as * const _ as u64 + GUARD_STACK.len() as u64;

		IDT.table[0] = IdtEntry::new(int::div_zero as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[3] = IdtEntry::new(int::breakpoint as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[4] = IdtEntry::new(int::overflow as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[6] = IdtEntry::new(int::invalid_opcode as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[8] = IdtEntry::new(int::double_fault as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[10] = IdtEntry::new(int::invalid_tss as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[11] = IdtEntry::new(int::seg_not_present as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[13] = IdtEntry::new(int::general_prot_fault as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[14] = IdtEntry::new(int::page_fault as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[32] = IdtEntry::new(int::timer as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);
		IDT.table[33] = IdtEntry::new(int::keyboard as * const extern "x86-interrupt" fn(&mut Isf) as u64, cs, DIO);

		IDT.load();
		GDT.load();

		low::set_cs(cs);
		low::load_tss(tss);

		PIC = Pics::new(32, 40);
		PIC.initialize();
		low::enable_int();

	}

}

pub fn halt() {

	unsafe {asm!("hlt");}

}

#[no_mangle]
pub fn end_exec() -> ! {

	unsafe {

		asm!("hlt", "jmp end_exec", options(noreturn))
	}

}

pub fn outb(port: u16, data: u8) {

	unsafe {

		asm!("out dx, al", in("dx") port, in("al") data);

	}

}

pub fn outw(port: u16, data: u16) {

	unsafe {

		asm!("out dx, ax", in("dx") port, in("ax") data);

	}

}

pub fn outd(port: u16, data: u32) {

	unsafe {

		asm!("out dx, eax", in("dx") port, in("eax") data);

	}

}

pub fn inb(port: u16) -> u8 {
	let output: u8;

	unsafe {

		asm!("in al, dx", in("dx") port, out("al") output);

	}

	output
}

pub fn inw(port: u16) -> u16 {
	let output: u16;

	unsafe {

		asm!("in ax, dx", in("dx") port, out("ax") output);

	}

	output
}

pub fn ind(port: u16) -> u32 {
	let output: u32;

	unsafe {

		asm!("in eax, dx", in("dx") port, out("eax") output);

	}

	output
}
