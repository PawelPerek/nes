pub const RAM: std::ops::Range<i32> = 0x0000..0x2000;
pub const IO: std::ops::Range<i32> = 0x2000..0x4020;
pub const EXPANSION: std::ops::Range<i32> = 0x4020..0x6000;
pub const SAVE: std::ops::Range<i32> = 0x6000..0x8000;
pub const PRG_ROM: std::ops::Range<i32> = 0x8000..0x10000;

pub struct Memory {
    pub data: [u8; 0x10000],
}

impl Memory {
    pub fn zp(&self, address: u8, offset: u8) -> u8 {
        let addr: usize = {
            let mut base: u16 = 0;
            base += address as u16;
            base += offset as u16;
            base &= 0xFF;
            base as usize
        };

        self.data[addr]
    }

    pub fn abs(&self, lsd: u8, msd: u8, offset: u16) -> u8 {
        let addr: usize = {
            let mut base: u16 = 0;
            base += msd as u16;
            base <<= 8;
            base += lsd as u16;
            base += offset;
            base as usize
        };

        self.data[addr]
    }

    pub fn ind(&self, lsd: u8, msd: u8, offset: u8) -> u8 {
        let addr_ptr: usize = {
            let mut base: u16 = 0;
            base += msd as u16;
            base <<= 8;
            base += lsd as u16;
            base += offset as u16;
            base as usize
        };

        let addr = self.data[addr_ptr] as usize;

        self.data[addr]
    }
}
