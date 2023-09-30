#![no_std]
#![feature(abi_x86_interrupt)]
pub mod interrupts;
pub mod vga_buffer;
pub mod serial;
pub mod gdt;


// 读取idt
pub fn init() {
	interrupts::init_idt();
	gdt::init_gdt();
	unsafe {
		interrupts::PICS.lock().initialize();
	};
	x86_64::instructions::interrupts::enable();
}