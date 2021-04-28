pub const RAM: std::ops::Range<i32> = 0x0000..0x2000;
pub const IO: std::ops::Range<i32> = 0x2000..0x4020;
pub const EXPANSION: std::ops::Range<i32> = 0x4020..0x6000;
pub const SAVE: std::ops::Range<i32> = 0x6000..0x8000;
pub const PRG_ROM: std::ops::Range<i32> = 0x8000..0x10000;

pub struct Memory {
    pub data: [u8; 0x10000],
}
