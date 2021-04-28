// Reference: http://wiki.nesdev.com/w/index.php/CPU_registers

#[derive(Default, Debug)]
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
    pub s: u8,
    // status
    pub p: u8,
}
