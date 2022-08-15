#!/bin/nano

use core::mem::{/*transmute, */size_of};

const GDF_RING_POS: u64 = 45;

const GDF_ACCESSED: u64 = 1 << 40;
const GDF_WRITABLE: u64 = 1 << 41;
const GDF_CONFORMING: u64 = 1 << 42;
const GDF_EXECUTABLE: u64 = 1 << 43;
const GDF_USER_SEGMENT: u64 = 1 << 44;
const GDF_RING_1: u64 = 1 << GDF_RING_POS;
const GDF_RING_2: u64 = 2 << GDF_RING_POS;
const GDF_RING_3: u64 = 3 << GDF_RING_POS;
const GDF_PRESENT: u64 = 1 << 47;
const GDF_AVAILABLE: u64 = 1 << 52;
const GDF_LONG_MODE: u64 = 1 << 53;
const GDF_DEFAULT_SIZE: u64 = 1 << 54;
const GDF_GRANULARITY: u64 = 1 << 55;
const GDF_LIMIT_0_15: u64 = 0xFFFF;
const GDF_LIMIT_16_19: u64 = 0xF << 48;
const GDF_BASE_0_23: u64 = 0xFF_FFFF << 16;
const GDF_BASE_24_31: u64 = 0xFF << 56;

const GDF_COMMON: u64 = (

	GDF_USER_SEGMENT
	| GDF_PRESENT
	| GDF_WRITABLE
	| GDF_ACCESSED
	| GDF_LIMIT_0_15
	| GDF_LIMIT_16_19
	| GDF_GRANULARITY

);

pub const GD_KERNEL_CODE: u64 = GDF_COMMON | GDF_EXECUTABLE | GDF_LONG_MODE;

pub const IO_MINIMAL: u16 = 0b0000111000000000;
pub const IO_PRESENT: u16 = 0b1000000000000000;
pub const IO_DEFAULT: u16 = IO_MINIMAL | IO_PRESENT;

#[repr(C, packed)]
pub struct Gdt {

	pub table: [u64; 8],
	pub length: usize,
	pub pointer: Pointer,

}

#[repr(C, align(16))]
pub struct Idt {

	pub table: [IdtEntry; 256],
	pub pointer: Pointer,

}

#[derive(Clone)]
#[repr(C, packed)]
pub struct IdtEntry {

	offset_low: u16,
	seg_select: u16,
	options: u16,
	offset_med: u16,
	offset_high: u32,
	reserved: u32,

}

#[repr(C, packed)]
pub struct Tss {

	reserved0: u32,
	pub privilege_stack_table: [u64; 3],
	reserved1: u64,
	pub interrupt_stack_table: [u64; 7],
	reserved2: u64,
	reserved3: u16,
	pub iomap_base: u16,

}

#[repr(C, packed)]
pub struct Pointer {

	pub limit: u16,
	pub base: u64,

}

#[repr(C, packed)]
pub struct IntStackFrame {

	/// code point to return
	pub instruction_pointer: u64,
	pub code_seg: u64,
	/// flags register before interrupt
	pub cpu_flags: u64,
	/// stack pointer at interrupt
	pub stack_pointer: u64,
	pub stack_seg: u64,

}

impl Gdt {

	pub const fn new() -> Gdt {

		Gdt{table: [0; 8], length: 1, pointer: Pointer{limit: 0, base: 0}}
	}

	pub fn load(&mut self) {

		self.pointer = Pointer::new(self as * const _ as u64, (self.length * size_of::<u64>() - 1) as u16);
		self.pointer.load_gdt();

	}

	pub fn push(&mut self, desc: u64) -> u16 {
		let index = self.length as u16;

		self.table[self.length] = desc;
		self.length += 1;

		new_cs(index as u16, self.table[index as usize])
	}

	pub fn push_sys_seg(&mut self, desc0: u64, desc1: u64) -> u16 {
		let cs = self.push(desc0);
		let _ = self.push(desc1);

		cs
	}

	pub fn push_tss(&mut self, tss: &Tss) -> u16 {
		let ptr = tss as * const _ as u64;
		let desc1 = ptr >> 32;
		let mut desc0 = GDF_PRESENT;
		desc0 |= ptr << 16 & 0b1111111111111111111111110000000000000000;
		desc0 |= ptr & 0b11111111000000000000000000000000 << 32 /* 56 - 24 */;
		desc0 |= (size_of::<Tss>() - 1) as u64 & 0b1111111111111111;
		desc0 |= 0b1001 << 40;

		self.push_sys_seg(desc0, desc1)
	}

}

impl Idt {

	pub const fn new(table: [IdtEntry; 256]) -> Self {

		Idt{table, pointer: Pointer{limit: 0, base: 0}}
	}

	pub fn load(&mut self) {

		self.pointer = Pointer::new(self as * const _ as u64, (self.table.len() * size_of::<IdtEntry>() - 1) as u16);
		self.pointer.load_idt();

	}

}

impl IdtEntry {

	pub const fn empty() -> Self {

		IdtEntry{offset_low: 0, seg_select: 0, options: 0, offset_med: 0, offset_high: 0, reserved: 0}
	}

	pub const fn new(fn_addr: u64, seg_select: u16, options: u16) -> Self {

		IdtEntry{offset_low: fn_addr as u16, seg_select, options, offset_med: (fn_addr >> 16) as u16, offset_high: (fn_addr >> 32) as u32, reserved: 0}
	}

}

impl Tss {

	pub const fn new(privilege_stack_table: [u64; 3], interrupt_stack_table: [u64; 7], iomap_base: u16) -> Self {

		Tss {

			reserved0: 0,
			privilege_stack_table,
			reserved1: 0,
			interrupt_stack_table,
			reserved2: 0,
			reserved3: 0,
			iomap_base,
		}
	}

}

impl Pointer {

	pub fn new(table_addr: u64, limit: u16) -> Pointer {

		Pointer{limit, base: table_addr}
	}

	pub fn load_gdt(&self) {

		unsafe {

			asm!("lgdt [{}]", in(reg) self, options(nostack));

		}

	}

	pub fn load_idt(&self) {

		unsafe {

			asm!("lidt [{}]", in(reg) self, options(nostack));

		}

	}

}

pub fn new_cs(index: u16, desc: u64) -> u16 {

	index << 3 | (desc >> GDF_RING_POS & 0b11) as u16
}

pub fn set_cs(cs: u16) {

	unsafe {

		asm!(
			"push {seg}",
			"lea {tmp}, [1f + rip]",
			"push {tmp}",
			"retfq",
			"1:",
			seg = in(reg) u64::from(cs), tmp = lateout(reg) _,
		);

	}

}

pub fn load_tss(seg: u16) {

	unsafe {

		asm!("ltr {0:x}", in(reg) seg, options(nostack, nomem));

	}

}

pub fn enable_int() {

	unsafe {

		asm!("sti");

	}

}
