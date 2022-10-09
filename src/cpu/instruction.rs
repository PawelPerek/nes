mod instruction_signature;
mod addressing_mode;
mod instruction_type;
mod instruction_arguments;
mod instruction_executor;

use instruction_signature::InstructionSignature;
use instruction_arguments::InstructionArguments;

use crate::{memory::Memory, cpu::instruction::instruction_executor::{adc::Adc, InstructionExecutor}};

use super::registers::Registers;

use self::{instruction_type::InstructionType, instruction_executor::ora::Ora};


pub struct Instruction {
    pub signature: InstructionSignature,
    pub arguments: InstructionArguments
}

impl Instruction {
    fn parse_from_memory(mem: &[u8], addr: usize) -> Self {
        use InstructionArguments::*;

        let signature = InstructionSignature::from_opcode(mem[addr]);
        let size = signature.size;

        Instruction {
            signature,
            arguments: match size {
                1 => Zero,
                2 => One(mem[addr + 1]),
                3 => Two(mem[addr + 1], mem[addr + 2]),
                _ => panic!("Found instruction with more than 2 arguments")
            }
        }
    }

    pub fn execute(&self, memory: &mut Memory, registers: &mut Registers) {
        use InstructionType as I;

        let executor: &dyn InstructionExecutor = match self.signature.instr_type {
            I::Adc => &Adc,
            I::Ahx => todo!(),
            I::Alr => todo!(),
            I::Anc => todo!(),
            I::And => todo!(),
            I::Arr => todo!(),
            I::Asl => todo!(),
            I::Axs => todo!(),
            I::Bcc => todo!(),
            I::Bcs => todo!(),
            I::Beq => todo!(),
            I::Bit => todo!(),
            I::Bmi => todo!(),
            I::Bne => todo!(),
            I::Bpl => todo!(),
            I::Brk => todo!(),
            I::Bvc => todo!(),
            I::Bvs => todo!(),
            I::Clc => todo!(),
            I::Cld => todo!(),
            I::Cli => todo!(),
            I::Clv => todo!(),
            I::Cmp => todo!(),
            I::Cpx => todo!(),
            I::Cpy => todo!(),
            I::Dcp => todo!(),
            I::Dec => todo!(),
            I::Dex => todo!(),
            I::Dey => todo!(),
            I::Eor => todo!(),
            I::Inc => todo!(),
            I::Inx => todo!(),
            I::Iny => todo!(),
            I::Isc => todo!(),
            I::Jmp => todo!(),
            I::Jsr => todo!(),
            I::Kil => todo!(),
            I::Las => todo!(),
            I::Lax => todo!(),
            I::Lda => todo!(),
            I::Ldx => todo!(),
            I::Ldy => todo!(),
            I::Lsr => todo!(),
            I::Nop => todo!(),
            I::Ora => &Ora,
            I::Pha => todo!(),
            I::Php => todo!(),
            I::Pla => todo!(),
            I::Plp => todo!(),
            I::Rla => todo!(),
            I::Rol => todo!(),
            I::Ror => todo!(),
            I::Rra => todo!(),
            I::Rti => todo!(),
            I::Rts => todo!(),
            I::Sax => todo!(),
            I::Sbc => todo!(),
            I::Sec => todo!(),
            I::Sed => todo!(),
            I::Sei => todo!(),
            I::Shx => todo!(),
            I::Shy => todo!(),
            I::Slo => todo!(),
            I::Sre => todo!(),
            I::Sta => todo!(),
            I::Stx => todo!(),
            I::Sty => todo!(),
            I::Tas => todo!(),
            I::Tax => todo!(),
            I::Tay => todo!(),
            I::Tsx => todo!(),
            I::Txa => todo!(),
            I::Txs => todo!(),
            I::Tya => todo!(),
            I::Xaa => todo!(),
        };

        executor.execute(&self.signature.addr_mode, &self.arguments, memory, registers);
    }
}

pub struct InstructionList(pub Vec<Instruction>);

impl From<&Vec<u8>> for InstructionList {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut instructions = Vec::new();
        let mut i = 0;

        while i > bytes.len() {
            let instruction = Instruction::parse_from_memory(bytes, i);
            i += instruction.signature.size as usize;
            
            instructions.push(instruction);
        }

        InstructionList(instructions)
    }
}