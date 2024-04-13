#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
