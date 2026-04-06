#![no_std]
#![no_main]

use core::fmt::{Result, Write};
use ns16550a::*;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut uart = Uart::new(0x1000_0000);

    uart.init(
        UartConfig {
            word_length: WordLength::EIGHT,
            stop_bits: StopBits::ONE,
            parity_bit: ParityBit::DISABLE,
            parity_select: ParitySelect::EVEN,
            stick_parity: StickParity::DISABLE,
            break_: Break::DISABLE,
            dma_mode: DMAMode::MODE0,
        },
        Divisor::BAUD1200,
    );

    // Test register configuration
    test_registers(&mut uart);

    // Test I/O operations
    test_io_operations(&mut uart);

    // Final success message
    test_write_success(&mut uart).unwrap();

    // Wait for host input to test GET operation (for host verification)
    for _ in 0..1000000 {
        if let Some(byte) = uart.get() {
            // Echo back received data
            let _ = uart.put(byte);
            let _ = uart.put(b'!'); // Acknowledgment
            break;
        }
        unsafe {
            core::arch::asm!("nop");
        }
    }

    exit_qemu();
    panic!()
}

fn test_registers(uart: &mut Uart) {
    // Test LCR with different configurations
    uart.set_lcr(LineControlConfig {
        word_length: WordLength::EIGHT,
        stop_bits: StopBits::ONE,
        parity_bit: ParityBit::DISABLE,
        parity_select: ParitySelect::EVEN,
        stick_parity: StickParity::DISABLE,
        break_: Break::DISABLE,
        dlab: DLAB::CLEAR,
    });

    // Test FCR with both DMA modes
    uart.set_fcr(DMAMode::MODE0);
    uart.set_fcr(DMAMode::MODE1);
    uart.set_fcr(DMAMode::MODE0);
}

fn test_io_operations(uart: &mut Uart) {
    // Test PUT operation - send test pattern
    uart.put(b'T');
    uart.put(b'E');
    uart.put(b'S');
    uart.put(b'T');
    uart.put(b'\r');
    uart.put(b'\n');

    // Test GET operation - try to receive and echo back if data available
    for _ in 0..10 {
        if let Some(byte) = uart.get() {
            // Echo received byte back
            let _ = uart.put(byte);
            // Also send to indicate we received something
            let _ = uart.put(b'!');
        }
    }
}

fn test_write_success(uart: &mut Uart) -> Result {
    write!(uart, "UART tests completed successfully!\n\r")
}

fn exit_qemu() {
    unsafe {
        core::ptr::write_volatile(0x100000 as *mut u32, 0x5555);
    }
}
