#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

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
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    kernel::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    kernel::hlt_loop();
}
