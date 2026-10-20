//! RCP - Reality Display Processor

use crate::{RO, RW};
use core::ops::Deref;
use proc_bitfield::bitfield;

/// A wrapper around the Reality Display Processor's memory-mapped registers.
///
/// See [`DisplayProcessor::new()`] for usage details.
pub struct DisplayProcessor {
    r: &'static mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub start: RW<u32>,
    pub end: RW<u32>,
    pub current: RO<u32>,
    pub status: RW<StatusReg>,
    pub clock: RO<u32>,
    pub buf_busy: RO<u32>,
    pub pipe_busy: RO<u32>,
    pub tmem: RO<u32>,
}

impl DisplayProcessor {
    /// Creates a new wrapper around the Reality Display Processor registers, starting at
    /// `0xA4100000`.
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
            r: &mut *(0xA4100000 as *mut RegisterBlock),
        }
    }
}

impl Deref for DisplayProcessor {
    type Target = RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.r
    }
}

regfn_rw!(DisplayProcessor, start, START, u32);
regfn_rw!(DisplayProcessor, end, END, u32);
regfn_ro!(DisplayProcessor, current, CURRENT, u32);
regfn_rw_union!(DisplayProcessor, status, STATUS, StatusReg);
regfn_ro!(DisplayProcessor, clock, CLOCK, u32);
regfn_ro!(DisplayProcessor, buf_busy, BUF_BUSY, u32);
regfn_ro!(DisplayProcessor, pipe_busy, PIPE_BUSY, u32);
regfn_ro!(DisplayProcessor, tmem, TMEM, u32);

#[derive(Copy, Clone)]
#[repr(C)]
pub union StatusReg {
    pub raw: u32,
    pub read: StatusRegRead,
    pub write: StatusRegWrite,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusRegRead(pub u32): Debug {
        pub xbus_dmem_dma: bool [ro] @ 0,
        pub freeze: bool [ro] @ 1,
        pub flush: bool [ro] @ 2,
        pub start_gclk: bool [ro] @ 3,
        pub tmem_busy: bool [ro] @ 4,
        pub pipe_busy: bool [ro] @ 5,
        pub command_busy: bool [ro] @ 6,
        pub cbuf_ready: bool [ro] @ 7,
        pub dma_busy: bool [ro] @ 8,
        pub end_valid: bool [ro] @ 9,
        pub start_valid: bool [ro] @ 10,
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StatusRegWrite(pub u32): Debug {
        pub clear_xbus_dmem_dma: bool [wo] @ 0,
        pub set_xbus_dmem_dma: bool [wo] @ 1,
        pub clear_freeze: bool [wo] @ 2,
        pub set_freeze: bool [wo] @ 3,
        pub clear_flush: bool [wo] @ 4,
        pub set_flush: bool [wo] @ 5,
        pub clear_tmem_counter: bool [wo] @ 6,
        pub clear_pipe_counter: bool [wo] @ 7,
        pub clear_command_counter: bool [wo] @ 8,
        pub clear_clock_counter: bool [wo] @ 9,
    }
}

const _: [(); 0x20] = [(); core::mem::size_of::<RegisterBlock>()];
