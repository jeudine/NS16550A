//
#![no_std]
use core::fmt::{Result, Write};

#[derive(Copy, Clone)]
pub struct Uart {
    base_address: usize,
}

#[derive(Copy, Clone)]
pub enum WordLength {
    FIVE = 0,
    SIX = 1,
    SEVEN = 2,
    EIGHT = 3,
}

#[derive(Copy, Clone)]
pub enum StopBits {
    ONE = 0,
    TWO = 1,
}

#[derive(Copy, Clone)]
pub enum ParityBit {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone)]
pub enum ParitySelect {
    EVEN = 0,
    ODD = 1,
}

#[derive(Copy, Clone)]
pub enum StickParity {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone)]
pub enum Break {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone)]
pub enum DLAB {
    CLEAR = 0,
    SET = 1,
}

#[derive(Copy, Clone)]
pub enum DMAMode {
    MODE0 = 0,
    MODE1 = 1,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    pub fn init(
        &self,
        word_length: WordLength,
        stop_bits: StopBits,
        parity_bit: ParityBit,
        parity_select: ParitySelect,
        stick_parity: StickParity,
        break_: Break,
        dma_mode: DMAMode,
        divisor: u16,
        ) {
        self.init_lcr(
            word_length,
            stop_bits,
            parity_bit,
            parity_select,
            stick_parity,
            break_,
            DLAB::SET,
            );
        self.init_fcr(dma_mode);
        let ptr = (self.base_address) as *mut u16;
        unsafe {
            ptr.write_volatile(divisor);
        }
        self.init_lcr(
            word_length,
            stop_bits,
            parity_bit,
            parity_select,
            stick_parity,
            break_,
            DLAB::CLEAR,
            );
    }

    pub fn init_lcr(
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

    pub fn init_fcr(&self, dma_mode: DMAMode) {
        let ptr = (self.base_address + 2) as *mut u8;
        unsafe {
            ptr.write_volatile(1 | ((dma_mode as u8) << 3));
        }
    }

    pub fn put(&self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.write_volatile(c);
        }
    }

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
        s.bytes().for_each(|c| self.put(c));
        Ok(())
    }
}
