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
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
        }
    }

    fn cpx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn cpy(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn dec(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn dex(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn dey(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn inc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn inx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn iny(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn asl(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn rol(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn lsr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn ror(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
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
            _ => panic!("Unknown instruction"),
        }
    }

    fn ldx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpy => (),
            AddressingMode::abs => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn stx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpy => (),
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn ldy(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn sty(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn tax(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn txa(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn tay(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn tya(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn tsx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn txs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn pla(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn pha(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn plp(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn php(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    // Jump/flag instructions
    fn bpl(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bmi(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bvc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bvs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bcc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bcs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bne(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn beq(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::rel => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn brk(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn rti(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn jsr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn rts(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn jmp(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::abs => (),
            AddressingMode::ind => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn bit(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn clc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn sec(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn cld(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn sed(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn cli(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn sei(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn clv(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn nop(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::zpy => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    //Unknown instructions
    fn slo(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn rla(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn sre(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn rra(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn sax(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpy => (),
            AddressingMode::izx => (),
            AddressingMode::abs => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn lax(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            AddressingMode::zp => (),
            AddressingMode::zpy => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn dcp(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn isc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::zp => (),
            AddressingMode::zpx => (),
            AddressingMode::izx => (),
            AddressingMode::izy => (),
            AddressingMode::abs => (),
            AddressingMode::abx => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn anc(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn alr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn arr(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn xaa(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn axs(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imm => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn ahx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::izy => (),
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn shy(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::abx => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn shx(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn tas(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn las(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::aby => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn kil(&self, mode: AddressingMode) {
        match mode {
            AddressingMode::imp => (),
            _ => panic!("Unknown instruction"),
        }
    }

    fn decode(&self, op: u8) {
        match op {
            0x00 => self.brk(AddressingMode::imp),
            0x01 => self.ora(AddressingMode::izx),
            0x02 => self.kil(AddressingMode::imp),
            0x03 => self.slo(AddressingMode::izx),
            0x04 => self.nop(AddressingMode::zp),
            0x05 => self.ora(AddressingMode::zp),
            0x06 => self.asl(AddressingMode::zp),
            0x07 => self.slo(AddressingMode::zp),
            0x08 => self.php(AddressingMode::imp),
            0x09 => self.ora(AddressingMode::imm),
            0x0A => self.asl(AddressingMode::imp),
            0x0B => self.anc(AddressingMode::imm),
            0x0C => self.nop(AddressingMode::abs),
            0x0D => self.ora(AddressingMode::abs),
            0x0E => self.asl(AddressingMode::abs),
            0x0F => self.slo(AddressingMode::abs),

            0x10 => self.bpl(AddressingMode::rel),
            0x11 => self.ora(AddressingMode::izy),
            0x12 => self.kil(AddressingMode::imp),
            0x13 => self.slo(AddressingMode::izy),
            0x14 => self.nop(AddressingMode::zpx),
            0x15 => self.ora(AddressingMode::zpx),
            0x16 => self.asl(AddressingMode::zpx),
            0x17 => self.slo(AddressingMode::zpx),
            0x18 => self.clc(AddressingMode::imp),
            0x19 => self.ora(AddressingMode::aby),
            0x1A => self.nop(AddressingMode::imp),
            0x1B => self.slo(AddressingMode::aby),
            0x1C => self.nop(AddressingMode::abx),
            0x1D => self.ora(AddressingMode::abx),
            0x1E => self.asl(AddressingMode::abx),
            0x1F => self.slo(AddressingMode::abx),

            0x20 => self.jsr(AddressingMode::abs),
            0x21 => self.and(AddressingMode::izx),
            0x22 => self.kil(AddressingMode::imp),
            0x23 => self.rla(AddressingMode::izx),
            0x24 => self.bit(AddressingMode::zp),
            0x25 => self.and(AddressingMode::zp),
            0x26 => self.rol(AddressingMode::zp),
            0x27 => self.rla(AddressingMode::zp),
            0x28 => self.plp(AddressingMode::imp),
            0x29 => self.and(AddressingMode::imm),
            0x2A => self.rol(AddressingMode::imp),
            0x2B => self.anc(AddressingMode::imm),
            0x2C => self.bit(AddressingMode::abs),
            0x2D => self.and(AddressingMode::abs),
            0x2E => self.rol(AddressingMode::abs),
            0x2F => self.rla(AddressingMode::abs),

            0x30 => self.bmi(AddressingMode::rel),
            0x31 => self.and(AddressingMode::izy),
            0x32 => self.kil(AddressingMode::imp),
            0x33 => self.rla(AddressingMode::izy),
            0x34 => self.nop(AddressingMode::zpx),
            0x35 => self.and(AddressingMode::zpx),
            0x36 => self.rol(AddressingMode::zpx),
            0x37 => self.rla(AddressingMode::zpx),
            0x38 => self.sec(AddressingMode::imp),
            0x39 => self.and(AddressingMode::aby),
            0x3A => self.nop(AddressingMode::imp),
            0x3B => self.rla(AddressingMode::aby),
            0x3C => self.nop(AddressingMode::abx),
            0x3D => self.and(AddressingMode::abx),
            0x3E => self.rol(AddressingMode::abx),
            0x3F => self.rla(AddressingMode::abx),

            0x40 => self.rti(AddressingMode::imp),
            0x41 => self.eor(AddressingMode::izx),
            0x42 => self.kil(AddressingMode::imp),
            0x43 => self.sre(AddressingMode::izx),
            0x44 => self.nop(AddressingMode::zp),
            0x45 => self.eor(AddressingMode::zp),
            0x46 => self.lsr(AddressingMode::zp),
            0x47 => self.sre(AddressingMode::zp),
            0x48 => self.pha(AddressingMode::imp),
            0x49 => self.eor(AddressingMode::imm),
            0x4A => self.lsr(AddressingMode::imp),
            0x4B => self.alr(AddressingMode::imm),
            0x4C => self.jmp(AddressingMode::abs),
            0x4D => self.eor(AddressingMode::abs),
            0x4E => self.lsr(AddressingMode::abs),
            0x4F => self.sre(AddressingMode::abs),

            0x50 => self.bvc(AddressingMode::rel),
            0x51 => self.eor(AddressingMode::izy),
            0x52 => self.kil(AddressingMode::imp),
            0x53 => self.sre(AddressingMode::izy),
            0x54 => self.nop(AddressingMode::zpx),
            0x55 => self.eor(AddressingMode::zpx),
            0x56 => self.lsr(AddressingMode::zpx),
            0x57 => self.sre(AddressingMode::zpx),
            0x58 => self.cli(AddressingMode::imp),
            0x59 => self.eor(AddressingMode::aby),
            0x5A => self.nop(AddressingMode::imp),
            0x5B => self.sre(AddressingMode::aby),
            0x5C => self.nop(AddressingMode::abx),
            0x5D => self.eor(AddressingMode::abx),
            0x5E => self.lsr(AddressingMode::abx),
            0x5F => self.sre(AddressingMode::abx),

            0x60 => self.rts(AddressingMode::imp),
            0x61 => self.adc(AddressingMode::izx),
            0x62 => self.kil(AddressingMode::imp),
            0x63 => self.rra(AddressingMode::izx),
            0x64 => self.nop(AddressingMode::zp),
            0x65 => self.adc(AddressingMode::zp),
            0x66 => self.ror(AddressingMode::zp),
            0x67 => self.rra(AddressingMode::zp),
            0x68 => self.pla(AddressingMode::imp),
            0x69 => self.adc(AddressingMode::imm),
            0x6A => self.ror(AddressingMode::imp),
            0x6B => self.arr(AddressingMode::imm),
            0x6C => self.jmp(AddressingMode::ind),
            0x6D => self.adc(AddressingMode::abs),
            0x6E => self.ror(AddressingMode::abs),
            0x6F => self.rra(AddressingMode::abs),

            0x70 => self.bvs(AddressingMode::rel),
            0x71 => self.adc(AddressingMode::izy),
            0x72 => self.kil(AddressingMode::imp),
            0x73 => self.rra(AddressingMode::izy),
            0x74 => self.nop(AddressingMode::zpx),
            0x75 => self.adc(AddressingMode::zpx),
            0x76 => self.ror(AddressingMode::zpx),
            0x77 => self.rra(AddressingMode::zpx),
            0x78 => self.sei(AddressingMode::imp),
            0x79 => self.adc(AddressingMode::aby),
            0x7A => self.nop(AddressingMode::imp),
            0x7B => self.rra(AddressingMode::aby),
            0x7C => self.nop(AddressingMode::abx),
            0x7D => self.adc(AddressingMode::abx),
            0x7E => self.ror(AddressingMode::abx),
            0x7F => self.rra(AddressingMode::abx),

            0x80 => self.nop(AddressingMode::imm),
            0x81 => self.sta(AddressingMode::izx),
            0x82 => self.nop(AddressingMode::imm),
            0x83 => self.sax(AddressingMode::izx),
            0x84 => self.sty(AddressingMode::zp),
            0x85 => self.sta(AddressingMode::zp),
            0x86 => self.stx(AddressingMode::zp),
            0x87 => self.sax(AddressingMode::zp),
            0x88 => self.dey(AddressingMode::imp),
            0x89 => self.nop(AddressingMode::imm),
            0x8A => self.txa(AddressingMode::imp),
            0x8B => self.xaa(AddressingMode::imm),
            0x8C => self.sty(AddressingMode::abs),
            0x8D => self.sta(AddressingMode::abs),
            0x8E => self.stx(AddressingMode::abs),
            0x8F => self.sax(AddressingMode::abs),

            0x90 => self.bcc(AddressingMode::rel),
            0x91 => self.sta(AddressingMode::izy),
            0x92 => self.kil(AddressingMode::imp),
            0x93 => self.ahx(AddressingMode::izy),
            0x94 => self.sty(AddressingMode::zpx),
            0x95 => self.sta(AddressingMode::zpx),
            0x96 => self.stx(AddressingMode::zpy),
            0x97 => self.sax(AddressingMode::zpy),
            0x98 => self.tya(AddressingMode::imp),
            0x99 => self.sta(AddressingMode::aby),
            0x9A => self.txs(AddressingMode::imp),
            0x9B => self.tas(AddressingMode::aby),
            0x9C => self.shy(AddressingMode::abx),
            0x9D => self.sta(AddressingMode::abx),
            0x9E => self.shx(AddressingMode::aby),
            0x9F => self.ahx(AddressingMode::aby),

            0xA0 => self.ldy(AddressingMode::imm),
            0xA1 => self.lda(AddressingMode::izx),
            0xA2 => self.ldx(AddressingMode::imm),
            0xA3 => self.lax(AddressingMode::izx),
            0xA4 => self.ldy(AddressingMode::zp),
            0xA5 => self.lda(AddressingMode::zp),
            0xA6 => self.ldx(AddressingMode::zp),
            0xA7 => self.lax(AddressingMode::zp),
            0xA8 => self.tay(AddressingMode::imp),
            0xA9 => self.lda(AddressingMode::imm),
            0xAA => self.tax(AddressingMode::imp),
            0xAB => self.lax(AddressingMode::imm),
            0xAC => self.ldy(AddressingMode::abs),
            0xAD => self.lda(AddressingMode::abs),
            0xAE => self.ldx(AddressingMode::abs),
            0xAF => self.lax(AddressingMode::abs),

            0xB0 => self.bcs(AddressingMode::rel),
            0xB1 => self.lda(AddressingMode::izy),
            0xB2 => self.kil(AddressingMode::imp),
            0xB3 => self.lax(AddressingMode::izy),
            0xB4 => self.ldy(AddressingMode::zpx),
            0xB5 => self.lda(AddressingMode::zpx),
            0xB6 => self.ldx(AddressingMode::zpy),
            0xB7 => self.lax(AddressingMode::zpy),
            0xB8 => self.clv(AddressingMode::imp),
            0xB9 => self.lda(AddressingMode::aby),
            0xBA => self.tsx(AddressingMode::imp),
            0xBB => self.las(AddressingMode::aby),
            0xBC => self.ldy(AddressingMode::abx),
            0xBD => self.lda(AddressingMode::abx),
            0xBE => self.ldx(AddressingMode::aby),
            0xBF => self.lax(AddressingMode::aby),

            0xC0 => self.cpy(AddressingMode::imm),
            0xC1 => self.cmp(AddressingMode::izx),
            0xC2 => self.nop(AddressingMode::imm),
            0xC3 => self.dcp(AddressingMode::izx),
            0xC4 => self.cpy(AddressingMode::zp),
            0xC5 => self.cmp(AddressingMode::zp),
            0xC6 => self.dec(AddressingMode::zp),
            0xC7 => self.dcp(AddressingMode::zp),
            0xC8 => self.iny(AddressingMode::imp),
            0xC9 => self.cmp(AddressingMode::imm),
            0xCA => self.dex(AddressingMode::imp),
            0xCB => self.axs(AddressingMode::imm),
            0xCC => self.cpy(AddressingMode::abs),
            0xCD => self.cmp(AddressingMode::abs),
            0xCE => self.dec(AddressingMode::abs),
            0xCF => self.dcp(AddressingMode::abs),

            0xD0 => self.bne(AddressingMode::rel),
            0xD1 => self.cmp(AddressingMode::izy),
            0xD2 => self.kil(AddressingMode::imp),
            0xD3 => self.dcp(AddressingMode::izy),
            0xD4 => self.nop(AddressingMode::zpx),
            0xD5 => self.cmp(AddressingMode::zpx),
            0xD6 => self.dec(AddressingMode::zpx),
            0xD7 => self.dcp(AddressingMode::zpx),
            0xD8 => self.cld(AddressingMode::imp),
            0xD9 => self.cmp(AddressingMode::aby),
            0xDA => self.nop(AddressingMode::imp),
            0xDB => self.dcp(AddressingMode::aby),
            0xDC => self.nop(AddressingMode::abx),
            0xDD => self.cmp(AddressingMode::abx),
            0xDE => self.dec(AddressingMode::abx),
            0xDF => self.dcp(AddressingMode::abx),

            0xE0 => self.cpx(AddressingMode::imm),
            0xE1 => self.sbc(AddressingMode::izx),
            0xE2 => self.nop(AddressingMode::imm),
            0xE3 => self.isc(AddressingMode::izx),
            0xE4 => self.cpx(AddressingMode::zp),
            0xE5 => self.sbc(AddressingMode::zp),
            0xE6 => self.inc(AddressingMode::zp),
            0xE7 => self.isc(AddressingMode::zp),
            0xE8 => self.inx(AddressingMode::imp),
            0xE9 => self.sbc(AddressingMode::imm),
            0xEA => self.nop(AddressingMode::imp),
            0xEB => self.sbc(AddressingMode::imm),
            0xEC => self.cpx(AddressingMode::abs),
            0xED => self.sbc(AddressingMode::abs),
            0xEE => self.inc(AddressingMode::abs),
            0xEF => self.isc(AddressingMode::abs),

            0xF0 => self.beq(AddressingMode::rel),
            0xF1 => self.sbc(AddressingMode::izy),
            0xF2 => self.kil(AddressingMode::imp),
            0xF3 => self.isc(AddressingMode::izy),
            0xF4 => self.nop(AddressingMode::zpx),
            0xF5 => self.sbc(AddressingMode::zpx),
            0xF6 => self.inc(AddressingMode::zpx),
            0xF7 => self.isc(AddressingMode::zpx),
            0xF8 => self.sed(AddressingMode::imp),
            0xF9 => self.sbc(AddressingMode::aby),
            0xFA => self.nop(AddressingMode::imp),
            0xFB => self.isc(AddressingMode::aby),
            0xFC => self.nop(AddressingMode::abx),
            0xFD => self.sbc(AddressingMode::abx),
            0xFE => self.inc(AddressingMode::abx),
            0xFF => self.isc(AddressingMode::abx),
        }
    }

    // Reference: http://obelisk.me.uk/6502/reference.html, http://www.oxyron.de/html/opcodes02.html, http://www.obelisk.me.uk/6502/addressing.html
    pub fn execute(&mut self) -> ! {
        loop {
            let op = self.fetch_op();

            self.decode(op);
        }
    }
}

#[cfg(test)]
mod cpu {
    use super::CPU;

    #[test]
    fn no_unknown_instructions() {
        for i in 0x00..0xff {
            let cpu = CPU::new(vec![i]);
            cpu.decode(i);
        }
    }
}
