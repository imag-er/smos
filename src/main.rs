// dont include std libraries which is depend on system
#![no_std]
#![no_main]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);
    loop {}
}



#[no_mangle]
pub extern "C" fn _start() ->! {
    println!("Hello\r123\n123");

    loop{

    }
}
