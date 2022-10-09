pub const RAM: std::ops::Range<i32> = 0x0000..0x2000;
pub const IO: std::ops::Range<i32> = 0x2000..0x4020;
pub const EXPANSION: std::ops::Range<i32> = 0x4020..0x6000;
pub const SAVE: std::ops::Range<i32> = 0x6000..0x8000;
pub const PRG_ROM: std::ops::Range<i32> = 0x8000..0x10000;

pub struct Memory([u8; 0x10000]);

#[derive(Debug)]
pub enum MemoryError {
    BadAddress
}

impl Memory {
    pub fn new() -> Memory {
        Memory([0x00; 0x10000])
    }

    pub fn write(&mut self, address: usize, value: u8) -> Result<(), MemoryError> {
        match address {
            0 ..= 0xFFFF => {
                self.0[address] = value;
                Ok(())
            },
            _ => Err(MemoryError::BadAddress)
        }
    }

    pub fn read(&self, address: usize) -> Result<u8, MemoryError> {
        match address {
            0 ..= 0xFFFF => Ok(self.0[address]),
            _ => Err(MemoryError::BadAddress)
        }
    }

    // TODO: Secure access methods by switching return type to Result<u8, MemoryError>
    pub fn zp(&self, address: u8, offset: u8) -> u8 {
        let addr: usize = {
            let base = ((address as u16) + (offset as u16)) & 0xFF;
            base as usize
        };

        self.0[addr]
    }

    pub fn abs(&self, lsd: u8, msd: u8, offset: u16) -> u8 {
        let addr: usize = {
            let base = ((msd as u16) << 8) + (lsd as u16) + offset;
            base as usize
        };

        self.0[addr]
    }

    pub fn ind(&self, address: u8, offset: u8) -> u8 {
        let addr_ptr: usize = {
            let base = (address as u16) + (offset as u16);
            base as usize
        };

        let addr = self.0[addr_ptr] as usize;

        self.0[addr]
    }
}

#[cfg(test)]
mod memory {
    #[test]
    fn mem_read() {

    }

    #[test]
    fn mem_write() {

    }
}