// Reference: http://wiki.nesdev.com/w/index.php/CPU_registers

#[derive(Default, Debug)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
}
