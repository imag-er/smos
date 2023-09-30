#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    smos::println!("{}",_info);
    loop {}
}



#[no_mangle]
pub extern "C" fn _start() ->! {
    // 调用lib.rs中的init函数
    smos::init();
    smos::println!("IDT inited");

    // raise stack overflow
    fn raise_sof()
    {
        raise_sof();
    }    
    raise_sof();

    
    loop{

    }
}
