# NS16550A

![Crates.io](https://img.shields.io/crates/v/ns16550a)
![docs.rs](https://img.shields.io/docsrs/ns16550a)
![GitHub](https://img.shields.io/github/license/jeudine/NS16550A)

NS16550A UART driver written in Rust.

## Installation

Add the following to Cargo.toml:

``` toml
ns16550a = "0.2"
```

## Example

Example usage:

``` rust
use ns16550a::*;

fn main() {
    let mut uart = Uart::new(0x1000_0000);
    uart.init(WordLength::EIGHT,
              StopBits::ONE,
              ParityBit::DISABLE,
              ParitySelect::EVEN,
              StickParity::DISABLE,
              Break::DISABLE,
              DMAMode::MODE0,
              Divisor::BAUD1200,
              );
    write!(&mut uart, "Hello, world!\n\r");
    loop {
        uart.put(uart.get().unwrap_or_default());
    }
}
```
