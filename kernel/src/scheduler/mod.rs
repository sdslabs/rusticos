// Adding boilerplate scheduler functions

use crate::gdt;
use crate::mem;
use crate::serial_println;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Display;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Clone)]
pub struct Context {
    pub rbp: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[inline(never)]
pub unsafe fn jmp_to_usermode(code: mem::VirtAddr, stack_end: mem::VirtAddr) {
    let (cs_idx, ds_idx) = gdt::set_usermode_segs();
    x86_64::instructions::tlb::flush_all(); // flush the TLB after address-space switch
    asm!("\
    push rax   // stack segment
    push rsi   // rsp
    push 0x200 // rflags (only interrupt bit set)
    push rdx   // code segment
    push rdi   // ret to virtual addr
    iretq",
    in("rdi") code.addr(), in("rsi") stack_end.addr(), in("dx") cs_idx, in("ax") ds_idx);
}

pub struct Scheduler {
    tasks: Mutex<Vec<Task>>,
    cur_task: Mutex<Option<usize>>,
}

impl Scheduler{
    pub fn new() -> Scheduler {
        Scheduler {
            tasks: Mutex::new(Vec::new()),
            cur_task: Mutex::new(None), // so that next task is 0
        }
    }
    
    pub unsafe fn schedule(&self, fn_addr: mem::VirtAddr){

    }

    pub unsafe fn save_current_context(&self, ctxp: *const Context) {

    }

    pub unsafe fn run_next(&self) {
        //Call to jump to usermode lies here
    }
}

lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new();
}