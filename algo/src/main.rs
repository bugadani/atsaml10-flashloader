#![no_std]
#![no_main]

use flash_algorithm::*;

//#[cfg(any(features = "atsaml10x14", features = "atsaml11x14"))]
const FLASH_SIZE: u32 = 0x4000;

#[cfg(any(features = "atsaml10x15", features = "atsaml11x15"))]
const FLASH_SIZE: u32 = 0x8000;

#[cfg(any(features = "atsaml10x16", features = "atsaml11x16"))]
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

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        // TODO: Add setup code for the flash algorithm.
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        // TODO: Add code here that erases the entire flash.
        Err(ErrorCode::new(0x70d0).unwrap())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        // TODO: Add code here that erases a page to flash.
        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        // TODO: Add code here that writes a page to flash.
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
    }
}
