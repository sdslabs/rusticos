use crate::gdt;
use alloc::boxed::Box;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::{structures::paging::PageTable, VirtAddr, PhysAddr};


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

#[inline(always)]
pub unsafe fn get_context() -> *const Context {
    let ctx: *const Context;
    asm!("push r15; push r14; push r13; push r12; push r11; push r10; push r9;\
    push r8; push rdi; push rsi; push rdx; push rcx; push rbx; push rax; push rbp;\
    mov {}, rsp; sub rsp, 0x400;",
    out(reg) ctx);
    ctx
}

#[inline(always)]
pub unsafe fn restore_context(ctx: *const Context) {
    asm!("mov rsp, {};\
    pop rbp; pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop r8; pop r9;\
    pop r10; pop r11; pop r12; pop r13; pop r14; pop r15; iretq;",
    in(reg) ctx);
}

#[inline(never)]
pub unsafe fn switch_to_usermode(code: VirtAddr, stack_end: VirtAddr) {
    let (cs_idx, ds_idx) = gdt::set_usermode_segments();
    x86_64::instructions::tlb::flush_all();
    asm!("\
    push rax   // stack segment
    push rsi   // rsp
    push 0x200 // rflags (only interrupt bit set)
    push rdx   // code segment
    push rdi   // ret to virtual addr
    iretq",
    in("rdi") code.as_u64(), in("rsi") stack_end.as_u64(), in("dx") cs_idx, in("ax") ds_idx);
}

#[derive(Clone, Debug)]
enum TaskState {
    SavedContext(Context),
    StartingInfo(VirtAddr, VirtAddr),
}

struct Task {
    state: TaskState,
    task_pt: Box<PageTable>,
    _stack_vec: Vec<u8>,
}

impl Task {
    pub fn new(
        exec_base: VirtAddr,
        stack_end: VirtAddr,
        task_pt: Box<PageTable>,
        _stack_vec: Vec<u8>,
    ) -> Task {
        Task {
            state: TaskState::StartingInfo(exec_base, stack_end),
            task_pt,
            _stack_vec,
        }
    }
}

pub struct Scheduler {
    tasks: Mutex<Vec<Task>>,
    current_task: Mutex<Option<usize>>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            tasks: Mutex::new(Vec::new()),
            current_task: Mutex::new(None),
        }
    }

    pub unsafe fn schedule(&self, mapper: &mut OffsetPageTable, fn_addr: VirtAddr) {
        use x86_64::structures::paging::PageTableFlags as Flags;
        use x86_64::structures::paging::mapper::Translate;

        let userspace_fn_phys = Translate::translate_addr(mapper, fn_addr).unwrap(); // virtual address to physical
        let page_phys_start = (userspace_fn_phys.addr() >> 12) << 12; // zero out page offset to get which page we should map
        let fn_page_offset = userspace_fn_phys.addr() - page_phys_start; // offset of function from page start
        let userspace_fn_virt_base = 0x400000; // target virtual address of page
        let userspace_fn_virt = userspace_fn_virt_base + fn_page_offset; // target virtual address of function
        let mut task_pt = mem::PageTable::new(); // copy over the kernel's page tables
        task_pt.map_virt_to_phys(
            VirtAddr::new(userspace_fn_virt_base),
            PhysAddr::new(page_phys_start),
            Flags::PRESENT | Flags::USER_ACCESSIBLE,
        ); // map the program's code
        task_pt.map_virt_to_phys(
            VirtAddr::new(userspace_fn_virt_base).offset(0x1000),
            PhysAddr::new(page_phys_start).offset(0x1000),
            Flags::PRESENT | Flags::USER_ACCESSIBLE,
        ); // also map another page to be sure we got the entire function in
        let mut stack_space: Vec<u8> = Vec::with_capacity(0x1000); // allocate some memory to use for the stack
        let stack_space_phys = VirtAddr::new(stack_space.as_mut_ptr() as *const u8 as u64)
            .to_phys()
            .unwrap()
            .0;
        // take physical address of stack
        task_pt.map_virt_to_phys(
            mem::VirtAddr::new(0x800000),
            stack_space_phys,
            Flags::PRESENT | Flags::WRITABLE | Flags::USER_ACCESSIBLE,
        ); // map the stack memory to 0x800000
        let task = Task::new(
            VirtAddr::new(userspace_fn_virt),
            VirtAddr::new(0x801000),
            stack_space,
            task_pt,
        ); // create task struct
        self.tasks.lock().push(task); // push task struct to list of tasks
    }

    pub unsafe fn save_current_context(&self, ctxp: *const Context) {
        self.current_task.lock().map(|cur_task_idx| {
            let ctx = (*ctxp).clone();
            self.tasks.lock()[cur_task_idx].state = TaskState::SavedContext(ctx);
        });
    }

    pub unsafe fn run_next(&self) {
        let tasks_len = self.tasks.lock().len();
        if tasks_len > 0 {
            let task_state = {
                let mut cur_task_opt = self.current_task.lock();
                let cur_task = cur_task_opt.get_or_insert(0);
                let next_task = (*cur_task + 1) % tasks_len;
                *cur_task = next_task;
                let task = &self.tasks.lock()[next_task];
                task.task_pt.enable()
                task.state.clone()
            };
            match task_state {
                TaskState::SavedContext(ctx) => {
                    restore_context(&ctx)
                },
                TaskState::StartingInfo(exec_base, stack_end) => {
                    switch_to_usermode(exec_base, stack_end)
                },
            }
        }
    }
}

lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new()
}