use crate::exceptions::gdt;
use crate::*;
use crate::{println, serial_println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::{instructions::port::Port};

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, // IRQ0 Timer
    Keyboard,             // IRQ1 Keyboard
    PIC,                  // Progamable Interrupts Controller
    COM2,                 // 串口2
    COM1,                 // 串口1
    LPT2,                 // 并行port 2
    FloppyDiskController, // Floppy Disk Controller
    LPT1,                 // Parallel Port 1
    RealTimeClock,        // Real Time Clock
    RDIRQ,                // ReDirect IRQ 通常给显卡使用
    _IRQ10,
    _IRQ11,
    Mouse = 0x74, // IRQ12 Mouse
    MathematicalCoprocessor,
    HardDiskController, // Hard Disk Controller
    _IRQ15,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        unsafe  {
            // 当触发double_fault的时候自动切换到自定义的stack frame里
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        };

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::PIC.as_usize()] .set_handler_fn(pic_interrupt_handler);
    
        idt
    };
}

// ----------------------EXCEPTION HANDLERS--------------------------
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:?}", stack_frame);
    serial_println!("EXCEPTION: BREAKPOINT");
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE_FAULT\n{:?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // 通知cpu int已经结束

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    // print!("{0:b} {0:o} {0:x}\n",scancode);

    use io::keyboard::{Key, KeyBoardMapper};
    use spin::Mutex;

    lazy_static! {
        pub static ref KBMAPPER: Mutex<KeyBoardMapper> = Mutex::new(KeyBoardMapper::new());
    }

    match KBMAPPER.lock().scancode_to_char(scancode) {
        Key::Character(k) => print!("{}", char::from_u32(k as u32).unwrap()),
        Key::Control(k) => println!("{{{:#?}}}", k),
        _ => {}
    };

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2; // 从CR2寄存器读取页目录

    println!(
        "\
	EXCEPTION: PAGE FAULT
	Access Addr:\t{:?}
	Error Code:\t{:?}
	{:#?}
	",
        Cr2::read(),
        error_code,
        stack_frame
    );

    hlt_loop();
}
extern "x86-interrupt" fn pic_interrupt_handler(pic_frame: InterruptStackFrame) {
    print!("PIC Int {:?}",pic_frame);
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::PIC.as_u8());
    }
}

pub fn init_idt() {
    IDT.load();
    crate::println!("int descri table LOADED");
}
