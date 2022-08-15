#!/bin/nano
#![allow(unused_imports)]

use core::mem::transmute;
use x86_64::structures::{idt::{InterruptDescriptorTable, InterruptStackFrame}, tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}};
use x86_64::instructions::{segmentation::set_cs, tables::load_tss, port::Port};
use x86_64::VirtAddr;
use pic8259_simple::ChainedPics;
use crate::sc::text::{write_bytes, write_byte};
use crate::util;

const INTERRUPT_STACK_SIZE: usize = 4096;
const PIC_TIMER: u8 = 32;
const PIC_TIMER_USIZE: usize = PIC_TIMER as usize;
const PIC2: u8 = 40;
const PIC_KEYBOARD: u8 = 33;
const PIC_KEYBOARD_USIZE: usize = PIC_KEYBOARD as usize;

pub static mut ITD: InterruptDescriptorTable = unsafe {transmute([0u8; 4096])};
pub static mut TSS: TaskStateSegment = unsafe {transmute([0u8; 104])};
pub static mut GDT: GlobalDescriptorTable = unsafe {transmute([0u8; 72])};
pub static mut CODE_SELECT: SegmentSelector = unsafe {transmute([0u8; 2])};
pub static mut TSS_SELECT: SegmentSelector = unsafe {transmute([0u8; 2])};
pub static mut PICS: ChainedPics = unsafe {transmute([0u8; 12])};
pub static mut STACK: [u8; INTERRUPT_STACK_SIZE] = [0; INTERRUPT_STACK_SIZE];
pub static mut TRIPLE_FAULT: bool = false;
pub static mut KEYBOARD: Port<u8> = unsafe {transmute::<u16, Port<u8>>(0)};

/// keyboard pressed key
pub static mut KEY: u8 = 0;

pub fn init() {

	unsafe {

		ITD = InterruptDescriptorTable::new();
		ITD.breakpoint.set_handler_fn(cpu_interrupt);
		ITD.double_fault.set_handler_fn(double_fault).set_stack_index(0);
		ITD.load();

		TSS = TaskStateSegment::new();
		TSS.interrupt_stack_table[0] = {

			let stack_start = VirtAddr::from_ptr(&STACK);
			let stack_end = stack_start + INTERRUPT_STACK_SIZE;

			stack_end
		};

		GDT = GlobalDescriptorTable::new();
		CODE_SELECT = GDT.add_entry(Descriptor::kernel_code_segment());
		TSS_SELECT = GDT.add_entry(Descriptor::tss_segment(&TSS));
		GDT.load();

		set_cs(CODE_SELECT);
		load_tss(TSS_SELECT);

		PICS = ChainedPics::new(PIC_TIMER, PIC2);
		PICS.initialize();
		x86_64::instructions::interrupts::enable();
		ITD[PIC_TIMER_USIZE].set_handler_fn(timer);
		ITD[PIC_KEYBOARD_USIZE].set_handler_fn(keyboard);

		KEYBOARD = Port::new(0x60);

	}

}

extern "x86-interrupt" fn cpu_interrupt(_sf: &mut InterruptStackFrame) {

	write_bytes(b"\x0fcpu interrupt\x10\n");

}

extern "x86-interrupt" fn double_fault(_: &mut InterruptStackFrame, _: u64) -> ! {

	unsafe {

		if TRIPLE_FAULT {

			asm!("lgdt [0]");

		}

		panic!("double fault")
	}
}

extern "x86-interrupt" fn timer(_: &mut InterruptStackFrame) {
	crate::mods::built::timer_int();

	unsafe {

		PICS.notify_end_of_interrupt(PIC_TIMER);
	}

}

extern "x86-interrupt" fn keyboard(_: &mut InterruptStackFrame) {
	crate::mods::built::keyboard_int();

	unsafe {

		PICS.notify_end_of_interrupt(PIC_KEYBOARD);
	}

}
