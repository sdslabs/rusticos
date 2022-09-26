#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::println;
use kernel::task::{executor::Executor, keyboard, Task};

// panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    kernel::hlt_loop();
}

// panic handler for tests
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kernel::test_panic_handler(_info)
}

// entry point for kernel
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kernel::memory::{MapperFrameAllocaterInfo};
    use kernel::{allocator};

    println!("Hello World{}", "!");
    kernel::init();


    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    

    let mut mapper_frame_allocator = unsafe{ MapperFrameAllocaterInfo::init(boot_info) };
    allocator::init_heap(&mut mapper_frame_allocator.mapper, &mut mapper_frame_allocator.frame_allocator).expect("heap initialization failed");
    // unsafe {
    //     syscalls::init_syscalls();
    // }

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
