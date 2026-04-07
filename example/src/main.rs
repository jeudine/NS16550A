#![no_std]
#![no_main]

use core::fmt::Write;
use ns16550a::*;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Create UART instance at memory-mapped address
    let mut uart = Uart::new(0x1000_0000);

    // Configure UART with common settings
    let config = UartConfig {
        word_length: WordLength::EIGHT,     // 8 data bits
        stop_bits: StopBits::ONE,           // 1 stop bit
        parity_bit: ParityBit::DISABLE,     // No parity
        parity_select: ParitySelect::EVEN,  // Even parity (if enabled)
        stick_parity: StickParity::DISABLE, // No stick parity
        break_: Break::DISABLE,             // No break signal
        dma_mode: DMAMode::MODE0,           // DMA mode 0
    };

    // Initialize with 1200 baud rate
    uart.init(config, Divisor::BAUD1200);

    // Write using fmt::Write trait
    writeln!(&mut uart, "Hello, world!").unwrap();
    writeln!(&mut uart, "Simple echo loop (press `q` to quit):").unwrap();

    // Simple echo loop (exit on 'q')
    loop {
        if let Some(byte) = uart.get() {
            if byte == b'q' {
                exit_qemu();
            }
            uart.put(byte).unwrap();
        }
    }
}

fn exit_qemu() {
    unsafe {
        core::ptr::write_volatile(0x100000 as *mut u32, 0x5555);
    }
}
