use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{PrivilegeLevel, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PRIVILEGE_USER_INDEX: u16 = 0;
const STACK_SIZE: usize = 4096 * 5;
pub static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
pub static mut PRIV_TSS_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss.privilege_stack_table[PRIVILEGE_USER_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr(unsafe { &PRIV_TSS_STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::gdt::{Descriptor, DescriptorFlags, GlobalDescriptorTable};

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_data_flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT | DescriptorFlags::WRITABLE;
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment()); // kernel code segment
        let data_selector = gdt.add_entry(Descriptor::UserSegment(kernel_data_flags.bits())); // kernel data segment
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS)); // task state segment
        let user_data_selector = gdt.add_entry(Descriptor::user_data_segment()); // user data segment
        let user_code_selector = gdt.add_entry(Descriptor::user_code_segment()); // user code segment
        (
            gdt,
            Selectors {
                code_selector,
                data_selector,
                tss_selector,
                user_data_selector,
                user_code_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
    user_data_selector: SegmentSelector,
    user_code_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{load_ds, set_cs};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    let _stack = unsafe { &STACK as *const _ };
    let _user_stack = unsafe { &PRIV_TSS_STACK as *const _ };
    // Printing loading of GDT table
    println!(
        " - Loaded GDT: {:p} TSS: {:p} Stack {:p} User stack: {:p} CS segment: {} TSS segment: {}",
        &GDT.0 as *const _, &*TSS as *const _, _stack, _user_stack, GDT.1[0].0, GDT.1[1].0
    );
    unsafe {
        set_cs(GDT.1.code_selector);
        load_ds(GDT.1.data_selector);
        load_tss(GDT.1.tss_selector);
    }
}

#[inline(always)]
pub unsafe fn set_usermode_segs() -> (u16, u16) {
    use x86_64::instructions::segmentation::load_ds;

    // set ds ans tss, return cs and ds
    let (mut cs, mut ds) = (GDT.1.user_code_selector, GDT.1.user_data_selector);
    cs.0 |= PrivilegeLevel::Ring3 as u16;
    ds.0 |= PrivilegeLevel::Ring3 as u16;
    load_ds(ds);
    (cs.0, ds.0)
}
