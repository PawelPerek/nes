pub mod instructions;
pub mod memory;
pub mod opcode;
pub mod registers;

use instructions::Instruction;

struct Cpu {
    reg: registers::Registers,
    mem: memory::Memory,
    prg: Vec<u8>,
}

impl Cpu {
    fn new(prg: Vec<u8>) -> Cpu {
        Cpu {
            reg: Default::default(),
            mem: memory::Memory { data: [0; 0x10000] },
            prg,
        }
    }

    fn fetch_op(&mut self) -> u8 {
        let instr = self.prg[self.reg.pc as usize];
        self.reg.pc += 1;
        instr
    }

    fn execute(&mut self) -> ! {
        loop {
            let op = self.fetch_op();
            let instruction = opcode::decode_op(op);

            match instruction {
                Instruction::NOP => {}
                _ => panic!(),
            }
        }
    }
}
