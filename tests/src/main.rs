#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
