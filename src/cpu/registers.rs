// Reference: http://wiki.nesdev.com/w/index.php/CPU_registers
#[derive(Clone, Default, Debug)]
pub struct Registers {
    // accumulator
    pub a: u8,
    // index 1
    pub x: u8,
    // index 2
    pub y: u8,
    // program counter
    pub pc: u16,
    // stack pointer
    pub sp: u8,
    // status
    pub sr: StatusRegister,
}

pub enum StatusMask {
    Negative  = 0b1000_0000,
    Overflow  = 0b0100_0000,
    Ignored   = 0b0010_0000,
    Break     = 0b0001_0000,
    Decimal   = 0b0000_1000,
    Interrupt = 0b0000_0100,
    Zero      = 0b0000_0010,
    Carry     = 0b0000_0001,
}

// TODO: Internal u8 probably can be splitted into 7 separate bools sacrificing 6 bytes of memory for conveinence
#[derive(Clone, Default, Debug)]
pub struct StatusRegister(u8);

impl StatusRegister {
    pub fn get(&self, status: StatusMask) -> bool {
        self.0 & (status as u8) != 0
    }
    
    pub fn set(&mut self, status: StatusMask, value: bool) {
        self.0 = if value { self.0 | (status as u8) } else { self.0 & !(status as u8) };
    }
} 

#[cfg(test)]
mod registers {
    use super::*;

    #[test]
    fn sr_register_get() {
        let sr = StatusRegister (0b1001_1100);

        assert_eq!(sr.get(StatusMask::Negative), true);
        assert_eq!(sr.get(StatusMask::Overflow), false);
        assert_eq!(sr.get(StatusMask::Ignored), false);
        assert_eq!(sr.get(StatusMask::Break), true);
        assert_eq!(sr.get(StatusMask::Decimal), true);
        assert_eq!(sr.get(StatusMask::Interrupt), true);
        assert_eq!(sr.get(StatusMask::Zero), false);
        assert_eq!(sr.get(StatusMask::Carry), false);
    }

    #[test]
    fn sr_register_set() {
        let mut sr = StatusRegister (0b0000_1111);

        sr.set(StatusMask::Negative, true);
        sr.set(StatusMask::Carry, false);
        
        assert_eq!(sr.get(StatusMask::Negative), true);
        assert_eq!(sr.get(StatusMask::Carry), false);
        assert_eq!(sr.0, 0b1000_1110)
    }
}