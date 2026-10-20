//! RCP - Reality Signal Processor

use crate::{RO, RW};
use core::ops::Deref;
use proc_bitfield::bitfield;

/// A wrapper around the Reality Signal Processor's memory-mapped registers.
///
/// See [`Rsp::new()`] for usage details.
pub struct Rsp {
    r: &'static mut RegisterBlock,
    pc: &'static mut PcRegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub mem_addr: RW<u32>,
    pub dram_addr: RW<u32>,
    pub rd_len: RW<u32>,
    pub wr_len: RW<u32>,
    pub status: RW<StatusReg>,
    pub dma_full: RO<u32>,
    pub dma_busy: RO<u32>,
    pub semaphore: RW<SemaphoreReg>,
}

#[repr(C)]
pub struct PcRegisterBlock {
    pub pc: RW<u32>,
    pub ibist: RW<u32>,
}

impl Rsp {
    /// Creates a new wrapper around the Reality Signal Processor registers.
    ///
    /// Developers are recommended to use [`Hardware::take()`][crate::Hardware::take()]
    /// instead.
    ///
    /// # Safety
    /// This provides unrestricted access to memory-mapped registers. Data races could
    /// occur if the same registers are accessed from regular code and interrupt handlers.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self {
            r: &mut *(0xA4040000 as *mut RegisterBlock),
            pc: &mut *(0xA4080000 as *mut PcRegisterBlock),
        }
    }

    #[inline(always)]
    pub fn pc(&self) -> u32 {
        self.pc.pc.read()
    }

    #[inline(always)]
    pub fn set_pc(&self, data: u32) {
        self.pc.pc.write(data);
    }

    #[inline(always)]
    pub fn modify_pc<F: FnOnce(u32) -> u32>(&self, func: F) {
        self.pc.pc.modify(func);
    }

    #[inline(always)]
    pub fn ibist(&self) -> u32 {
        self.pc.ibist.read()
    }

    #[inline(always)]
    pub fn set_ibist(&self, data: u32) {
        self.pc.ibist.write(data);
    }

    #[inline(always)]
    pub fn modify_ibist<F: FnOnce(u32) -> u32>(&self, func: F) {
        self.pc.ibist.modify(func);
    }
}

impl Deref for Rsp {
    type Target = RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.r
    }
}

regfn_rw!(Rsp, mem_addr, MEM_ADDR, u32);
regfn_rw!(Rsp, dram_addr, DRAM_ADDR, u32);
regfn_rw!(Rsp, rd_len, RD_LEN, u32);
regfn_rw!(Rsp, wr_len, WR_LEN, u32);
regfn_rw_union!(Rsp, status, STATUS, StatusReg);
regfn_ro!(Rsp, dma_full, DMA_FULL, u32);
regfn_ro!(Rsp, dma_busy, DMA_BUSY, u32);
regfn_rw_union!(Rsp, semaphore, SEMAPHORE, SemaphoreReg);

#[inline(always)]
pub fn pc() -> u32 {
    unsafe { Rsp::new().pc() }
}

#[inline(always)]
pub unsafe fn set_pc(data: u32) {
    Rsp::new().set_pc(data);
}

#[inline(always)]
pub unsafe fn modify_pc<F: FnOnce(u32) -> u32>(func: F) {
    Rsp::new().modify_pc(func);
}

#[inline(always)]
pub fn ibist() -> u32 {
    unsafe { Rsp::new().ibist() }
}

#[inline(always)]
pub unsafe fn set_ibist(data: u32) {
    Rsp::new().set_ibist(data);
}

#[inline(always)]
pub unsafe fn modify_ibist<F: FnOnce(u32) -> u32>(func: F) {
    Rsp::new().modify_ibist(func);
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusRegRead(pub u32): Debug {
        pub halt: bool [ro] @ 0,
        pub broke: bool [ro] @ 1,
        pub dma_busy: bool [ro] @ 2,
        pub dma_full: bool [ro] @ 3,
        pub io_full: bool [ro] @ 4,
        pub single_step: bool [ro] @ 5,
        pub interrupt_break: bool [ro] @ 6,
        pub signal0: bool [ro] @ 7,
        pub signal1: bool [ro] @ 8,
        pub signal2: bool [ro] @ 9,
        pub signal3: bool [ro] @ 10,
        pub signal4: bool [ro] @ 11,
        pub signal5: bool [ro] @ 12,
        pub signal6: bool [ro] @ 13,
        pub signal7: bool [ro] @ 14,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusRegWrite(pub u32): Debug {
        pub clear_halt: bool [wo] @ 0,
        pub set_halt: bool [wo] @ 1,
        pub clear_broke: bool [wo] @ 2,
        pub clear_interrupt: bool [wo] @ 3,
        pub set_interrupt: bool [wo] @ 4,
        pub clear_single_step: bool [wo] @ 5,
        pub set_single_step: bool [wo] @ 6,
        pub clear_interrupt_break: bool [wo] @ 7,
        pub set_interrupt_break: bool [wo] @ 8,
        pub clear_signal0: bool [wo] @ 9,
        pub set_signal0: bool [wo] @ 10,
        pub clear_signal1: bool [wo] @ 11,
        pub set_signal1: bool [wo] @ 12,
        pub clear_signal2: bool [wo] @ 13,
        pub set_signal2: bool [wo] @ 14,
        pub clear_signal3: bool [wo] @ 15,
        pub set_signal3: bool [wo] @ 16,
        pub clear_signal4: bool [wo] @ 17,
        pub set_signal4: bool [wo] @ 18,
        pub clear_signal5: bool [wo] @ 19,
        pub set_signal5: bool [wo] @ 20,
        pub clear_signal6: bool [wo] @ 21,
        pub set_signal6: bool [wo] @ 22,
        pub clear_signal7: bool [wo] @ 23,
        pub set_signal7: bool [wo] @ 24,
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union StatusReg {
    pub raw: u32,
    pub read: StatusRegRead,
    pub write: StatusRegWrite,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union SemaphoreReg {
    pub raw: u32,
    pub read: SemaphoreRegRead,
    pub write: SemaphoreRegWrite,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct SemaphoreRegRead(pub u32): Debug {
        pub locked: bool [ro] @ 0,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct SemaphoreRegWrite(pub u32): Debug {
        /// The value is ignored; any write clears the semaphore.
        pub value: u32 [wo] @ ..,
    }
}

const _: [(); 0x20] = [(); core::mem::size_of::<RegisterBlock>()];
const _: [(); 0x08] = [(); core::mem::size_of::<PcRegisterBlock>()];
