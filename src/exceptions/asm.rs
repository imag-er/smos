use core::arch::asm;

#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}