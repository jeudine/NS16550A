#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use ns16550a::*;
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
    let _ = write!(&mut uart, "Hello, world!\n\r");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
