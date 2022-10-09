use crate::{cpu::{instruction::{addressing_mode::AddressingMode, instruction_arguments::InstructionArguments}, registers::{Registers, SRMask}}, memory::Memory};

use super::InstructionExecutor;

pub struct Adc;

impl InstructionExecutor for Adc {
    fn execute(&self, mode: &AddressingMode, args: &InstructionArguments, memory: &mut Memory, registers: &mut Registers) {
        use AddressingMode::*;

        let acc_snapshot = registers.a;
        let carry = if registers.sr.get(SRMask::Carry) { 1 } else { 0 };

        match mode {
            Immediate => registers.a += *args.as_one().unwrap() + carry,
            ZeroPage => registers.a += memory.zp(*args.as_one().unwrap(), 0) + carry,
            ZeroPageX => registers.a += memory.zp(*args.as_one().unwrap(), registers.x) + carry,
            IndirectX => registers.a += memory.ind(*args.as_one().unwrap(), registers.x) + carry,
            IndirectY => registers.a += memory.ind(*args.as_one().unwrap(), registers.y) + carry,
            Absolute => registers.a += memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, 0)  + carry,
            AbsoluteX => registers.a += memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.x.into())  + carry,
            AbsoluteY => registers.a += memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.y.into())  + carry,
            _ => panic!("Unknown instruction"),
        }   

        registers.sr.set(SRMask::Carry, acc_snapshot > registers.a)        
    }
}