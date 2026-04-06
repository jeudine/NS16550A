//! NS16550A UART driver for embedded systems.
//!
//! This crate provides a no_std driver for the NS16550A UART peripheral,
//! commonly used in embedded systems across various architectures.
//! It supports basic UART functionality including initialization,
//! configuration, and data transmission/reception.
//!
//! ## Usage
//!
//! ```no_run
//! use ns16550a::{Uart, UartConfig, WordLength, StopBits, ParityBit, ParitySelect, StickParity, Break, DMAMode, Divisor};
//!
//! // Create a UART instance at base address 0x10000000
//! let mut uart = Uart::new(0x10000000);
//!
//! // Configure UART settings
//! let config = UartConfig {
//!     word_length: WordLength::EIGHT,
//!     stop_bits: StopBits::ONE,
//!     parity_bit: ParityBit::DISABLE,
//!     parity_select: ParitySelect::EVEN,
//!     stick_parity: StickParity::DISABLE,
//!     break_: Break::DISABLE,
//!     dma_mode: DMAMode::MODE0,
//! };
//!
//! // Initialize UART with 115200 baud rate
//! uart.init(config, Divisor::BAUD115200);
//!
//! // Write data
//! uart.put(b'A');
//!
//! // Read data
//! if let Some(byte) = uart.get() {
//!     // Process received byte
//! }
//! ```

#![no_std]

use core::fmt::{Result, Write};

#[derive(Copy, Clone, Debug)]
/// Struct representing a NS16550A UART peripheral
pub struct Uart {
    /// Base address of the peripheral
    base_address: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Word length configuration for UART communication
pub enum WordLength {
    /// 5 data bits
    FIVE = 0,
    /// 6 data bits
    SIX = 1,
    /// 7 data bits
    SEVEN = 2,
    /// 8 data bits (most common)
    EIGHT = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Number of stop bits for UART communication
pub enum StopBits {
    /// 1 stop bit (most common)
    ONE = 0,
    /// 2 stop bits
    TWO = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity bit enable/disable
pub enum ParityBit {
    /// No parity bit
    DISABLE = 0,
    /// Parity bit enabled
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity type selection
pub enum ParitySelect {
    /// Even parity
    EVEN = 0,
    /// Odd parity
    ODD = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Stick parity configuration
pub enum StickParity {
    /// Stick parity disabled
    DISABLE = 0,
    /// Stick parity enabled (parity bit is fixed)
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Break signal control
pub enum Break {
    /// Break signal disabled (normal operation)
    DISABLE = 0,
    /// Break signal enabled (continuous low signal)
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Divisor Latch Access Bit
pub enum DLAB {
    /// Normal operation mode (access to RBR/THR/IER)
    CLEAR = 0,
    /// Divisor latch mode (access to DLL/DLM)
    SET = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// DMA mode selection
pub enum DMAMode {
    /// DMA mode 0
    MODE0 = 0,
    /// DMA mode 1
    MODE1 = 1,
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Divisor for setting the baud rate
pub enum Divisor {
    BAUD50 = 0x09_00,
    BAUD300 = 0x01_80,
    BAUD1200 = 0x00_60,
    BAUD2400 = 0x00_30,
    BAUD4800 = 0x00_18,
    BAUD9600 = 0x00_0C,
    BAUD19200 = 0x00_06,
    BAUD38400 = 0x00_03,
    BAUD57600 = 0x00_02,
    BAUD115200 = 0x00_01,
}

/// Configuration for UART initialization
///
/// This struct contains all the parameters needed to initialize a UART peripheral.
/// Use this with the `Uart::init()` method to configure the UART.
///
/// # Example
/// ```no_run
/// use ns16550a::{UartConfig, WordLength, StopBits, ParityBit, ParitySelect, StickParity, Break, DMAMode};
/// 
/// let config = UartConfig {
///     word_length: WordLength::EIGHT,
///     stop_bits: StopBits::ONE,
///     parity_bit: ParityBit::DISABLE,
///     parity_select: ParitySelect::EVEN,
///     stick_parity: StickParity::DISABLE,
///     break_: Break::DISABLE,
///     dma_mode: DMAMode::MODE0,
/// };
/// ```
#[derive(Copy, Clone, Debug)]
pub struct UartConfig {
    /// Data word length (5-8 bits)
    pub word_length: WordLength,
    /// Number of stop bits (1 or 2)
    pub stop_bits: StopBits,
    /// Parity bit enable/disable
    pub parity_bit: ParityBit,
    /// Parity type (even or odd)
    pub parity_select: ParitySelect,
    /// Stick parity mode
    pub stick_parity: StickParity,
    /// Break signal control
    pub break_: Break,
    /// DMA mode selection
    pub dma_mode: DMAMode,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            word_length: WordLength::EIGHT,
            stop_bits: StopBits::ONE,
            parity_bit: ParityBit::DISABLE,
            parity_select: ParitySelect::EVEN,
            stick_parity: StickParity::DISABLE,
            break_: Break::DISABLE,
            dma_mode: DMAMode::MODE0,
        }
    }
}

/// Configuration for line control register
///
/// This struct contains parameters for the Line Control Register (LCR).
/// Used internally by the `Uart::init()` method and can be used with `Uart::set_lcr()`.
///
/// # Example
/// ```no_run
/// use ns16550a::{LineControlConfig, WordLength, StopBits, ParityBit, ParitySelect, StickParity, Break, DLAB};
/// 
/// let config = LineControlConfig {
///     word_length: WordLength::EIGHT,
///     stop_bits: StopBits::ONE,
///     parity_bit: ParityBit::DISABLE,
///     parity_select: ParitySelect::EVEN,
///     stick_parity: StickParity::DISABLE,
///     break_: Break::DISABLE,
///     dlab: DLAB::CLEAR,
/// };
/// ```
#[derive(Copy, Clone, Debug)]
pub struct LineControlConfig {
    /// Data word length (5-8 bits)
    pub word_length: WordLength,
    /// Number of stop bits (1 or 2)
    pub stop_bits: StopBits,
    /// Parity bit enable/disable
    pub parity_bit: ParityBit,
    /// Parity type (even or odd)
    pub parity_select: ParitySelect,
    /// Stick parity mode
    pub stick_parity: StickParity,
    /// Break signal control
    pub break_: Break,
    /// Divisor Latch Access Bit
    pub dlab: DLAB,
}

impl Uart {
    /// Creates a new instance of `Uart` with the given base address.
    ///
    /// # Arguments
    /// * `base_address` - Memory-mapped base address of the UART peripheral
    ///
    /// # Example
    /// ```no_run
    /// let uart = Uart::new(0x10000000);
    /// ```
    pub const fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    /// Returns the base address of the UART peripheral
    ///
    /// # Returns
    /// The memory-mapped base address
    ///
    /// # Example
    /// ```no_run
    /// let uart = Uart::new(0x10000000);
    /// assert_eq!(uart.base_address(), 0x10000000);
    /// ```
    pub const fn base_address(&self) -> usize {
        self.base_address
    }

    /// Initializes the UART peripheral with the given configuration.
    ///
    /// This method configures the UART with the specified parameters and baud rate divisor.
    /// It sets up the line control register, FIFO control register, and baud rate.
    ///
    /// # Arguments
    /// * `config` - UART configuration struct containing all communication parameters
    /// * `divisor` - Baud rate divisor (e.g., `Divisor::BAUD115200`)
    ///
    /// # Example
    /// ```no_run
    /// use ns16550a::{Uart, UartConfig, WordLength, StopBits, ParityBit, ParitySelect, StickParity, Break, DMAMode, Divisor};
    ///
    /// let uart = Uart::new(0x10000000);
    /// let config = UartConfig::default();
    /// uart.init(config, Divisor::BAUD115200);
    /// ```
    pub fn init(&self, config: UartConfig, divisor: Divisor) {
        self.set_lcr(LineControlConfig {
            word_length: config.word_length,
            stop_bits: config.stop_bits,
            parity_bit: config.parity_bit,
            parity_select: config.parity_select,
            stick_parity: config.stick_parity,
            break_: config.break_,
            dlab: DLAB::SET,
        });
        self.set_fcr(config.dma_mode);
        let ptr = (self.base_address) as *mut u16;
        unsafe {
            ptr.write_volatile(divisor as u16);
        }
        self.set_lcr(LineControlConfig {
            word_length: config.word_length,
            stop_bits: config.stop_bits,
            parity_bit: config.parity_bit,
            parity_select: config.parity_select,
            stick_parity: config.stick_parity,
            break_: config.break_,
            dlab: DLAB::CLEAR,
        });
    }

    /// Sets the line control register with the given configuration.
    ///
    /// This method directly writes to the Line Control Register (LCR) with the provided parameters.
    ///
    /// # Arguments
    /// * `config` - Line control configuration struct
    ///
    /// # Example
    /// ```no_run
    /// use ns16550a::{Uart, LineControlConfig, WordLength, StopBits, ParityBit, ParitySelect, StickParity, Break, DLAB};
    ///
    /// let uart = Uart::new(0x10000000);
    /// let config = LineControlConfig {
    ///     word_length: WordLength::EIGHT,
    ///     stop_bits: StopBits::ONE,
    ///     parity_bit: ParityBit::DISABLE,
    ///     parity_select: ParitySelect::EVEN,
    ///     stick_parity: StickParity::DISABLE,
    ///     break_: Break::DISABLE,
    ///     dlab: DLAB::CLEAR,
    /// };
    /// uart.set_lcr(config);
    /// ```
    pub fn set_lcr(&self, config: LineControlConfig) {
        let ptr = (self.base_address + 3) as *mut u8;
        unsafe {
            ptr.write_volatile(
                config.word_length as u8
                    | ((config.stop_bits as u8) << 2)
                    | ((config.parity_bit as u8) << 3)
                    | ((config.parity_select as u8) << 4)
                    | ((config.stick_parity as u8) << 5)
                    | ((config.break_ as u8) << 6)
                    | ((config.dlab as u8) << 7),
            );
        }
    }

    /// Sets the FIFO control register with the given DMA mode.
    ///
    /// This method configures the FIFO Control Register (FCR) and enables FIFO mode.
    ///
    /// # Arguments
    /// * `dma_mode` - DMA mode selection (MODE0 or MODE1)
    ///
    /// # Example
    /// ```no_run
    /// use ns16550a::{Uart, DMAMode};
    ///
    /// let uart = Uart::new(0x10000000);
    /// uart.set_fcr(DMAMode::MODE0);
    /// ```
    pub fn set_fcr(&self, dma_mode: DMAMode) {
        let ptr = (self.base_address + 2) as *mut u8;
        unsafe {
            ptr.write_volatile(1 | ((dma_mode as u8) << 3));
        }
    }

    /// Writes a byte to the transmitter holding register.
    ///
    /// This method attempts to write a byte to the UART transmitter. If the transmitter
    /// holding register is empty, it writes the byte and returns `Some(c)`. If the transmitter
    /// is busy, it returns `None`.
    ///
    /// # Arguments
    /// * `c` - Byte to transmit
    ///
    /// # Returns
    /// * `Some(c)` if the byte was successfully written
    /// * `None` if the transmitter is busy
    ///
    /// # Example
    /// ```no_run
    /// use ns16550a::Uart;
    ///
    /// let uart = Uart::new(0x10000000);
    /// if let Some(byte) = uart.put(b'A') {
    ///     println!("Sent: {}", byte);
    /// }
    /// ```
    pub fn put(&self, c: u8) -> Option<u8> {
        let ptr = self.base_address as *mut u8;

        // If THR is not empty
        let ptr_data_ready = (self.base_address + 5) as *mut u8;
        if unsafe { ptr_data_ready.read_volatile() & 0x20 == 0 } {
            return None;
        }

        unsafe {
            ptr.write_volatile(c);
        }
        Some(c)
    }

    /// Reads a byte from the receiver buffer register.
    ///
    /// This method checks if data is available in the receiver buffer. If data is ready,
    /// it reads and returns the byte as `Some(byte)`. If no data is available, it returns `None`.
    ///
    /// # Returns
    /// * `Some(byte)` if data was successfully read
    /// * `None` if no data is available
    ///
    /// # Example
    /// ```no_run
    /// use ns16550a::Uart;
    ///
    /// let uart = Uart::new(0x10000000);
    /// if let Some(byte) = uart.get() {
    ///     println!("Received: {}", byte);
    /// }
    /// ```
    pub fn get(&self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        let ptr_data_ready = (self.base_address + 5) as *mut u8;
        unsafe {
            if ptr_data_ready.read_volatile() & 1 == 0 {
                None
            } else {
                Some(ptr.read_volatile())
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        s.bytes().for_each(|c| while self.put(c).is_none() {});
        Ok(())
    }
}
