#!/bin/nano
#![allow(unused_imports)]

use core::mem::transmute;
use x86_64::structures::{idt::{InterruptDescriptorTable, InterruptStackFrame}, tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}};
use x86_64::instructions::{segmentation::set_cs, tables::load_tss, port::Port};
use x86_64::VirtAddr;
use pic8259_simple::ChainedPics;
use crate::vga::{write_bytes, write_byte};

const INTERRUPT_STACK_SIZE: usize = 4096;
const PIC1: u8 = 32;
const PIC1_USIZE: usize = PIC1 as usize;
const PIC2: u8 = 40;
const PIC_KEYBOARD: u8 = 33;
const PIC_KEYBOARD_USIZE: usize = PIC_KEYBOARD as usize;

static mut ITD: InterruptDescriptorTable = unsafe {transmute([0u8; 4096])};
static mut TSS: TaskStateSegment = unsafe {transmute([0u8; 104])};
static mut GDT: GlobalDescriptorTable = unsafe {transmute([0u8; 72])};
static mut CODE_SELECT: SegmentSelector = unsafe {transmute([0u8; 2])};
static mut TSS_SELECT: SegmentSelector = unsafe {transmute([0u8; 2])};
static mut PICS: ChainedPics = unsafe {transmute([0u8; 12])};
static mut KEYBOARD: Port<u8> = unsafe {transmute([0u8; 2])};
static mut STACK: [u8; INTERRUPT_STACK_SIZE] = [0; INTERRUPT_STACK_SIZE];

pub fn init() {

	unsafe {

		ITD = InterruptDescriptorTable::new();
		ITD.breakpoint.set_handler_fn(interupt);
		ITD.double_fault.set_handler_fn(double_interrupt).set_stack_index(0);
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

		PICS = ChainedPics::new(PIC1, PIC2);
		PICS.initialize();
		x86_64::instructions::interrupts::enable();
		ITD[PIC1_USIZE].set_handler_fn(timer_interrupt);
		ITD[PIC_KEYBOARD_USIZE].set_handler_fn(keyboard);

		KEYBOARD = Port::new(0x60);

	}

}

extern "x86-interrupt" fn interupt(_sf: &mut InterruptStackFrame) {

	write_bytes(b"\x0fcpu interupt\x10\n");

}

extern "x86-interrupt" fn double_interrupt(_: &mut InterruptStackFrame, _: u64) -> ! {

	panic!("double fault");

}

extern "x86-interrupt" fn timer_interrupt(_: &mut InterruptStackFrame) {

//	write_byte(b'#');

	unsafe {

		PICS.notify_end_of_interrupt(PIC1);

	}

}

extern "x86-interrupt" fn keyboard(_: &mut InterruptStackFrame) {

	unsafe {
		let code = KEYBOARD.read();

		crate::keyb::code::read(code);
		PICS.notify_end_of_interrupt(PIC_KEYBOARD);

	}

}
