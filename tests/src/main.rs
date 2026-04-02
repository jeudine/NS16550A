#![no_std]
#![no_main]

use core::fmt::Write;
use ns16550a::*;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut uart = Uart::new(0x1000_0000);

    uart.init(
        WordLength::EIGHT,
        StopBits::ONE,
        ParityBit::DISABLE,
        ParitySelect::EVEN,
        StickParity::DISABLE,
        Break::DISABLE,
        DMAMode::MODE0,
        Divisor::BAUD1200,
    );

    write!(&mut uart, "Hello, world!\n\r");

    exit_qemu();

    loop {}
}

fn exit_qemu() {
    unsafe {
        core::ptr::write_volatile(0x100000 as *mut u32, 0x5555);
    }
}
