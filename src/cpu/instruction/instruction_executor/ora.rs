use crate::{cpu::{instruction::{addressing_mode::AddressingMode, instruction_arguments::InstructionArguments}, registers::Registers}, memory::Memory};

use super::InstructionExecutor;

pub struct Ora;

impl InstructionExecutor for Ora {
    fn execute(&self, mode: &AddressingMode, args: &InstructionArguments, mut memory: &Memory, mut registers: &Registers) {
        use AddressingMode::*;

        match mode {
            Immediate => registers.a |= args.as_one().unwrap(),
            ZeroPage => registers.a |= memory.zp(*args.as_one().unwrap(), 0),
            ZeroPageX => registers.a |= memory.zp(*args.as_one().unwrap(), registers.x),
            IndirectX => registers.a |= memory.ind(*args.as_one().unwrap(), registers.x),
            IndirectY => registers.a |= memory.ind(*args.as_one().unwrap(), registers.y),
            Absolute => registers.a |= memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, 0),
            AbsoluteX => registers.a |= memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.x.into()),
            AbsoluteY => registers.a |= memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.y.into()),
            _ => panic!("Unknown instruction"),
        }

    }
}