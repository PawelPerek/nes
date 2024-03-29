pub mod ora;
pub mod adc;
mod _mocks;

use crate::{memory::Memory, cpu::Registers};

use super::{addressing_mode::AddressingMode, instruction_arguments::InstructionArguments};

/// RATIONALE:
/// In theory InstructionExecutor and it's instances are completely unnecessary 
/// because execution of instructions could be handled within class Instruction itself.
/// But even though this solution potentially could slowdown runtime code a bit 
/// (by creating vtable for each instruction), it provides way cleaner, more maintainable and testable code.
/// If final results will be significantly slow I will reconsider handling function execution by simple enum pattern-match
/// But I don't think it's gonna be neccesarry since Rust itself should provide decent enough performance for emulating 6502 ISA :)


// TODO: Update status registers in executors
pub trait InstructionExecutor {
    fn execute(&self, mode: &AddressingMode, args: &InstructionArguments, memory: &mut Memory, registers: &mut Registers); 
}