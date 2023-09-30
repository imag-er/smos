use lazy_static::lazy_static;
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::{
    Descriptor, GlobalDescriptorTable, SegmentSelector,
};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_top = VirtAddr::from_ptr(unsafe {
                &STACK
            });

            let stack_buttom = stack_top + STACK_SIZE;
            stack_buttom

        };
        tss
    };
    // 包含静态tss段的静态gdt结构
    static ref GDT: (GlobalDescriptorTable,Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt,Selectors{tss_selector,code_selector})
    };


}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
pub fn init_gdt() {
    GDT.0.load();

	unsafe {
		// 修改CS寄存器 指定使用custom的gdt和tss
		x86_64::instructions::segmentation::CS::set_reg(GDT.1.code_selector);
		x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
	}
}
