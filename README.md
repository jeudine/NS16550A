# NS16550A

[![crates.io](https://img.shields.io/crates/v/ns16550a)](https://crates.io/crates/ns16550a)
[![doc](https://docs.rs/ns16550a/badge.svg)](https://docs.rs/ns16550a)
[![github](https://img.shields.io/github/license/jeudine/NS16550A)](https://github.com/jeudine/NS16550A/blob/main/LICENSE)

NS16550A UART driver written in Rust.

## Installation

Add the following to Cargo.toml:

``` toml
ns16550a = "0.5"
```

## Example

Example usage:

``` rust
use ns16550a::*;

fn main() {
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
    write!(&mut uart, "Hello, world!\n\r");
    loop {
        uart.put(uart.get().unwrap_or_default());
    }
}
```
