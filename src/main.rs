#![no_std]
#![no_main]
use bootloader::{entry_point, BootInfo};
use smos::*;
use x86_64::*;

extern crate alloc;
use core::arch::asm;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // 调用lib.rs中的init函数
    smos::init();
    // use x86_64::VirtAddr;

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // let mut mapper = unsafe { memory::map::init(phys_mem_offset) };

    // let mut frame_allocator =
    //     unsafe { memory::map::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
    //     .expect("heap initalization failed");
    // unsafe {
    //     // asm!("mov al, 0x22");
    //     asm!("int 0x22");

    // }
    use x86_64::{instructions::port::Port};


    let mut port = Port::new(0x64);

    let mut port_= Port::new(0x60);
    unsafe {
        port.write(0x64 as u32);
        while (port.read() & 0x1) == 0 {
            
        }
        println!("ready");
        let val:u8 = port_.read();

        print!("{:?}",val);
        
    };
    
    println!("SUCCESSFULLY PROCESSED");
    smos::hlt_loop();
}
