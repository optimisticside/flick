#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

/// Temporary panic handler.
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Main program entry point. Called by `start` routine implemented in assembly.
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
