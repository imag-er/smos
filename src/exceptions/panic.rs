
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    crate::println!("{}",_info);
    loop {}
}