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
            _ => panic!("Illegal instruction")
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
            _ => panic!("Illegal instruction")
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
            _ => panic!("Illegal instruction")
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
            _ => panic!("Illegal instruction")
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
            _ => panic!("Illegal instruction")
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
            _ => panic!("Illegal instruction")
        }
    }

    fn cpx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn cpy(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn dec(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn dex(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn dey(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn inc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn inx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn iny(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn asl(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn rol(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn lsr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction")
        }
    }

    fn ror(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Illegal instruction")
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
