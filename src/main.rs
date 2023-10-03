#![no_std]
#![no_main]
use bootloader::{entry_point, BootInfo};
use smos::{*, memory::BootInfoFrameAllocator};
use x86_64::structures::paging::FrameAllocator;

extern crate alloc;
use alloc::boxed::Box;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // 调用lib.rs中的init函数
    smos::init();
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper,&mut frame_allocator)
        .expect("heap initalization failed");
    
    let x = Box::new(41);


    smos::hlt_loop();
}
