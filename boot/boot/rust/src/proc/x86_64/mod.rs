#!/bin/nano

/// low level
pub mod low;
/// interrupt functions
pub mod int;

use core::mem::transmute;
use pic8259_simple::ChainedPics as Pics;
use low::{Gdt, Idt, Tss, IdtEntry, IntStackFrame as Isf, IO_DEFAULT as DIO};
use crate::sc::text::write_bytes;

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
