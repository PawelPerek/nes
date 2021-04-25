// Reference: http://wiki.nesdev.com/w/index.php/CPU_registers

#[derive(Default, Debug)]
pub struct Registers {
    // accumulator
    a: u8,
    // index 1
    x: u8,
    // index 2
    y: u8,
    // program counter
    pc: u16,
    // stack pointer
    s: u8,
    // status
    p: u8,
}
