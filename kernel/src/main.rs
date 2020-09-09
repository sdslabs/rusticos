#![no_std]
#![no_main]

use core::panic::PanicInfo

// entry point for kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
