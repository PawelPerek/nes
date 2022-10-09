#[derive(Clone, Default)]
pub enum AddressingMode {
    #[default] Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    IndirectX,
    IndirectY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
}