mod instruction_signature;
mod addressing_mode;
mod instruction_type;
mod instruction_arguments;
mod instruction_executor;

use instruction_signature::InstructionSignature;
use instruction_arguments::InstructionArguments;

use crate::{memory::Memory, cpu::registers::Registers};

use self::{instruction_type::InstructionType, instruction_executor::{
    InstructionExecutor,
    arithmetic::adc, 
    logic::ora
}};

pub struct Instruction {
    pub signature: InstructionSignature,
    pub arguments: InstructionArguments
}

impl Instruction {
    const fn parse_from_memory(mem: &[u8], addr: usize) -> Self {
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
        use InstructionType::*;

        let executor: &dyn InstructionExecutor = match self.signature.instr_type {
            Adc => &adc::Adc,
            Ahx => todo!(),
            Alr => todo!(),
            Anc => todo!(),
            And => todo!(),
            Arr => todo!(),
            Asl => todo!(),
            Axs => todo!(),
            Bcc => todo!(),
            Bcs => todo!(),
            Beq => todo!(),
            Bit => todo!(),
            Bmi => todo!(),
            Bne => todo!(),
            Bpl => todo!(),
            Brk => todo!(),
            Bvc => todo!(),
            Bvs => todo!(),
            Clc => todo!(),
            Cld => todo!(),
            Cli => todo!(),
            Clv => todo!(),
            Cmp => todo!(),
            Cpx => todo!(),
            Cpy => todo!(),
            Dcp => todo!(),
            Dec => todo!(),
            Dex => todo!(),
            Dey => todo!(),
            Eor => todo!(),
            Inc => todo!(),
            Inx => todo!(),
            Iny => todo!(),
            Isc => todo!(),
            Jmp => todo!(),
            Jsr => todo!(),
            Kil => todo!(),
            Las => todo!(),
            Lax => todo!(),
            Lda => todo!(),
            Ldx => todo!(),
            Ldy => todo!(),
            Lsr => todo!(),
            Nop => todo!(),
            Ora => &ora::Ora,
            Pha => todo!(),
            Php => todo!(),
            Pla => todo!(),
            Plp => todo!(),
            Rla => todo!(),
            Rol => todo!(),
            Ror => todo!(),
            Rra => todo!(),
            Rti => todo!(),
            Rts => todo!(),
            Sax => todo!(),
            Sbc => todo!(),
            Sec => todo!(),
            Sed => todo!(),
            Sei => todo!(),
            Shx => todo!(),
            Shy => todo!(),
            Slo => todo!(),
            Sre => todo!(),
            Sta => todo!(),
            Stx => todo!(),
            Sty => todo!(),
            Tas => todo!(),
            Tax => todo!(),
            Tay => todo!(),
            Tsx => todo!(),
            Txa => todo!(),
            Txs => todo!(),
            Tya => todo!(),
            Xaa => todo!(),
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