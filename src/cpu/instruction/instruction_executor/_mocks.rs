use crate::{cpu::{instruction::{addressing_mode::AddressingMode, instruction_arguments::InstructionArguments}, registers::Registers}, memory::Memory};
use derive_builder::Builder;

#[derive(Clone, Builder, Default)]
#[builder(default)]
pub struct TestEnvironment {
    pub mode: AddressingMode,
    pub args: InstructionArguments,
    pub memory: Memory,
    pub registers: Registers
}

