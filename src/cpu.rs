pub mod memory;
pub mod registers;

#[allow(non_camel_case_types)]
enum AddressingMode {
    imp,
    imm,
    zp,
    zpx,
    zpy,
    izx,
    izy,
    abs,
    abx,
    aby,
    ind,
    rel,
}

pub struct CPU {
    reg: registers::Registers,
    mem: memory::Memory,
    prg: Vec<u8>,
}

impl CPU {
    pub fn new(prg: Vec<u8>) -> CPU {
        CPU {
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

    // Logical and arithmetic instructions
    fn ora(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn and(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn eor(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn adc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn sbc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn cmp(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn cpx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn cpy(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn dec(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn dex(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn dey(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn inc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn inx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn iny(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn asl(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn rol(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn lsr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn ror(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    //Move instructions
    fn lda(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn sta(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn ldx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpy => (),
            AddressingMode::abs => (),
            AddressingMode::aby => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn stx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpy => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn ldy(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn sty(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn tax(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn txa(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn tay(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn tya(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn tsx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn txs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn pla(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn pha(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn plp(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    fn php(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    // Jump/flag instructions
    fn bpl(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bmi(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bvc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bvs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bcc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bcs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bne(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn beq(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn brk(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn rti(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn jsr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn rts(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn jmp(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::abs => (),
            AddressingMode::ind => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn bit(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn clc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn sec(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn cld(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn sed(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn cli(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn sei(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn clv(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }
    
    fn nop(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction"),
        }
    }

    // Reference: http://obelisk.me.uk/6502/reference.html, http://www.oxyron.de/html/opcodes02.html, http://www.obelisk.me.uk/6502/addressing.html
    pub fn execute(&mut self) -> ! {
        loop {
            let op = self.fetch_op();

            //match op {
            //    0x00 => self.brk(),
            //    0x01 => self.ora(AddressingMode::izx),
            //    0x02 => self.kil(),
            //    0x03 => self.slo(AddressingMode::izx),
            //    0x04 => self.nop(AddressingMode::zp),
            //}
        }
    }
}
