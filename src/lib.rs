#![no_std]
#![feature(abi_x86_interrupt)]

pub mod exceptions;
pub mod io;
pub mod task;
pub mod memory;

extern crate alloc;

// 读取idt
pub fn init() {
	use exceptions::*;
	interrupts::init_idt();
	gdt::init_gdt();
	unsafe {
		interrupts::PICS.lock().initialize();
	};
	x86_64::instructions::interrupts::enable();
}

// halt
pub fn hlt_loop() -> ! {
	loop {
		exceptions::asm::hlt();
	}
}