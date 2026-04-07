# NS16550A UART Driver

[![crates.io](https://img.shields.io/crates/v/ns16550a)](https://crates.io/crates/ns16550a)
[![doc](https://docs.rs/ns16550a/badge.svg)](https://docs.rs/ns16550a)
[![CI](https://github.com/jeudine/NS16550A/actions/workflows/ci.yml/badge.svg)](https://github.com/jeudine/NS16550A/actions/workflows/ci.yml)
[![github](https://img.shields.io/github/license/jeudine/NS16550A)](https://github.com/jeudine/NS16550A/blob/main/LICENSE)

A no_std Rust driver for the NS16550A UART peripheral, designed for embedded systems.

## Features

- **No_std compatible**: Works in embedded environments without standard library
- **FIFO support**: Utilizes NS16550A's 16-byte transmit/receive FIFOs
- **Comprehensive configuration**: Full control over baud rate, word length, parity, stop bits
- **Easy integration**: Simple API with `core::fmt::Write` support
- **Memory-mapped I/O**: Direct hardware register access for maximum performance

## Basic Usage

```rust
use ns16550a::*;
use core::fmt::Write;

fn main() {
    // Create UART instance at memory-mapped address
    let mut uart = Uart::new(0x1000_0000);
    
    // Configure UART with common settings
    let config = UartConfig {
        word_length: WordLength::EIGHT,   // 8 data bits
        stop_bits: StopBits::ONE,        // 1 stop bit
        parity_bit: ParityBit::DISABLE,   // No parity
        parity_select: ParitySelect::EVEN, // Even parity (if enabled)
        stick_parity: StickParity::DISABLE, // No stick parity
        break_: Break::DISABLE,           // No break signal
        dma_mode: DMAMode::MODE0,        // DMA mode 0
    };
    
    // Initialize with 1200 baud rate
    uart.init(config, Divisor::BAUD1200);
    
    // Write using fmt::Write trait
    writeln!(&mut uart, "Hello, world!").unwrap();
    
    // Simple echo loop
    loop {
        if let Some(byte) = uart.get() {
            while uart.put(byte).is_none() {}
        }
    }
}
```

For a complete working example that runs with QEMU, see the [example directory](/example).

## License

This project is licensed under the [MIT License](LICENSE).
