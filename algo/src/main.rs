#![no_std]
#![no_main]

use flash_algorithm::*;

use crate::pac::nvmctrl::regs::Intflag;
mod pac;

#[cfg(any(feature = "atsaml10x14", feature = "atsaml11x14"))]
const FLASH_SIZE: u32 = 0x4000;

#[cfg(any(feature = "atsaml10x15", feature = "atsaml11x15"))]
const FLASH_SIZE: u32 = 0x8000;

#[cfg(any(feature = "atsaml10x16", feature = "atsaml11x16"))]
const FLASH_SIZE: u32 = 0x10000;

struct Algorithm;

algorithm!(Algorithm, {
    flash_address: 0,
    flash_size: FLASH_SIZE,
    page_size: 256,
    empty_value: 0xFF,
    sectors: [{
        size: 256,
        address: 0,
    }]
});

const INT_BITS: Intflag = Intflag(0x3e);

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        if !pac::PAC.statusb().read().nvmctrl_() {
            pac::PAC.wrctrl().write(|w| {
                w.set_key(pac::pac::vals::Key::CLR);
                w.set_perid(0x22); // Bridge B, index 2
            });
        }

        pac::NVMCTRL.ctrlb().write(|w| {
            w.set_rws(0xF);
            w.set_cachedis(true);
        });
        pac::NVMCTRL.intflag().write_value(INT_BITS);

        Ok(Self)
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        const CMD_ERASE_ROW: u16 = 0xa502;

        while !pac::NVMCTRL.status().read().ready() {}

        pac::NVMCTRL.addr().write(|w| w.0 = addr);
        pac::NVMCTRL.ctrla().write(|w| w.0 = CMD_ERASE_ROW);
        while !pac::NVMCTRL.status().read().ready() {}

        let intflag = pac::NVMCTRL.intflag().read().0;
        if let Some(code) = ErrorCode::new((intflag & INT_BITS.0) as u32) {
            return Err(code);
        }

        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        while !pac::NVMCTRL.status().read().ready() {}

        let dest = addr as *mut u32;
        // Write data to buffer
        unsafe {
            for chunk in data.chunks_exact(4) {
                let word = u32::from_le_bytes(chunk.try_into().unwrap());
                dest.write_volatile(word);
            }
        }

        // Execute "Write page" command
        pac::NVMCTRL.addr().write(|w| w.0 = addr);
        pac::NVMCTRL.ctrla().write(|w| w.0 = 0xa504);

        while !pac::NVMCTRL.status().read().ready() {}

        let intflag = pac::NVMCTRL.intflag().read().0;
        if let Some(code) = ErrorCode::new((intflag & INT_BITS.0) as u32) {
            return Err(code);
        }
        Ok(())
    }
}
