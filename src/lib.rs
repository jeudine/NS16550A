//! NS16550A UART driver.

#![no_std]

use core::fmt::{Result, Write};

#[derive(Copy, Clone, Debug)]
/// Struct representing a NS16550A UART peripheral
pub struct Uart {
	/// Base address of the peripheral
	base_address: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Word length
pub enum WordLength {
	FIVE = 0,
	SIX = 1,
	SEVEN = 2,
	EIGHT = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Number of stop bits
pub enum StopBits {
	ONE = 0,
	TWO = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity bits
pub enum ParityBit {
	DISABLE = 0,
	ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity select
pub enum ParitySelect {
	EVEN = 0,
	ODD = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Stick parity
pub enum StickParity {
	DISABLE = 0,
	ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Break
pub enum Break {
	DISABLE = 0,
	ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Divisor latch access bit
pub enum DLAB {
	CLEAR = 0,
	SET = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// DMA mode select
pub enum DMAMode {
	MODE0 = 0,
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

impl Uart {
	/// Creates a new instance of `Uart` with the given base address.
	pub const fn new(base_address: usize) -> Self {
		Self { base_address }
	}

	/// Returns the base address
	pub const fn base_address(&self) -> usize {
		self.base_address
	}

	/// Initializes the UART peripheral with the given parameters.
	pub fn init(
		&self,
		word_length: WordLength,
		stop_bits: StopBits,
		parity_bit: ParityBit,
		parity_select: ParitySelect,
		stick_parity: StickParity,
		break_: Break,
		dma_mode: DMAMode,
		divisor: Divisor,
	) {
		self.set_lcr(
			word_length,
			stop_bits,
			parity_bit,
			parity_select,
			stick_parity,
			break_,
			DLAB::SET,
		);
		self.set_fcr(dma_mode);
		let ptr = (self.base_address) as *mut u16;
		unsafe {
			ptr.write_volatile(divisor as u16);
		}
		self.set_lcr(
			word_length,
			stop_bits,
			parity_bit,
			parity_select,
			stick_parity,
			break_,
			DLAB::CLEAR,
		);
	}

	/// Sets the line control register with the given parameters.
	pub fn set_lcr(
		&self,
		word_length: WordLength,
		stop_bits: StopBits,
		parity_bit: ParityBit,
		parity_select: ParitySelect,
		stick_parity: StickParity,
		break_: Break,
		dlab: DLAB,
	) {
		let ptr = (self.base_address + 3) as *mut u8;
		unsafe {
			ptr.write_volatile(
				word_length as u8
					| ((stop_bits as u8) << 2)
					| ((parity_bit as u8) << 3)
					| ((parity_select as u8) << 4)
					| ((stick_parity as u8) << 5)
					| ((break_ as u8) << 6)
					| ((dlab as u8) << 7),
			);
		}
	}

	/// Sets the FIFO control register with the given parameter.
	pub fn set_fcr(&self, dma_mode: DMAMode) {
		let ptr = (self.base_address + 2) as *mut u8;
		unsafe {
			ptr.write_volatile(1 | ((dma_mode as u8) << 3));
		}
	}

	/// If the transmitter holding register is empty, writes `c` in the transmitter holding register, and returns `c`. Otherwise returns `None`.
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

	/// If data ready is set, returns the value read in the receiver buffer register. Otherwise
	/// returns `None`.
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
		s.bytes().for_each(|c| while self.put(c) == None {});
		Ok(())
	}
}
