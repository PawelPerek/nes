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

use AddressingMode::*;
use registers::SRMask;

impl CPU {

    pub fn new(prg: Vec<u8>) -> CPU {
        CPU {
            reg: Default::default(),
            mem: memory::Memory::new(),
            prg,
        }
    }

    fn fetch_op(&mut self) -> u8 {
        let instr = self.prg[self.reg.pc as usize];
        self.reg.pc += 1;
        instr
    }

    fn fetch_value(&mut self, mode: AddressingMode) -> u8 {
        match mode {
            imp => 0,
            imm => self.fetch_op(),
            zp => {
                let op = self.fetch_op();
                self.mem.zp(op, 0)
            }
            zpx => {
                let op = self.fetch_op();
                self.mem.zp(op, self.reg.x)
            }
            zpy => {
                let op = self.fetch_op();
                self.mem.zp(op, self.reg.y)
            }
            ind => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.ind(lsd, msd, 0)
            }
            izx => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.ind(lsd, msd, self.reg.x)
            }
            izy => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.ind(lsd, msd, self.reg.y)
            }
            abs => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.abs(lsd, msd, 0)
            }
            abx => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.abs(lsd, msd, self.reg.x as u16)
            }
            aby => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.abs(lsd, msd, self.reg.y as u16)
            }
            rel => {
                let lsd = self.fetch_op();
                let msd = self.fetch_op();
                self.mem.abs(lsd, msd, self.reg.pc)
            }
        }
    }

    // Logical and arithmetic instructions
    fn ora(&mut self, mode: AddressingMode) {
        match mode {
            imm => self.reg.a |= self.fetch_op(),
            zp => {
                let addr = self.fetch_op();
                self.reg.a |= self.mem.access(addr as usize);
            }
            zpx => {
                let addr = self.fetch_op();
                self.reg.a |= self.mem.access(addr as usize);
            }
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn and(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn eor(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn adc(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sbc(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn cmp(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn cpx(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn cpy(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn dec(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn dex(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn dey(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn inc(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn inx(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn iny(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn asl(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn rol(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn lsr(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn ror(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    //Move instructions
    fn lda(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sta(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn ldx(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpy => {}
            abs => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn stx(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpy => {}
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn ldy(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpx => {}
            abs => {}
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sty(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn tax(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn txa(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn tay(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn tya(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn tsx(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn txs(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn pla(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn pha(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn plp(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn php(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    // Jump/flag instructions
    fn bpl(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bmi(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bvc(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bvs(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bcc(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bcs(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bne(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn beq(&self, mode: AddressingMode) {
        match mode {
            rel => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn brk(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn rti(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn jsr(&self, mode: AddressingMode) {
        match mode {
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn rts(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn jmp(&self, mode: AddressingMode) {
        match mode {
            abs => {}
            ind => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn bit(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn clc(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sec(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn cld(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sed(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn cli(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sei(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn clv(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn nop(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            imm => {}
            zp => {}
            zpx => {}
            zpy => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    //Unknown instructions
    fn slo(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn rla(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sre(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn rra(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn sax(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpy => {}
            izx => {}
            abs => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn lax(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            zp => {}
            zpy => {}
            izx => {}
            izy => {}
            abs => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn dcp(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn isc(&self, mode: AddressingMode) {
        match mode {
            zp => {}
            zpx => {}
            izx => {}
            izy => {}
            abs => {}
            abx => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn anc(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn alr(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn arr(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn xaa(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn axs(&self, mode: AddressingMode) {
        match mode {
            imm => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn ahx(&self, mode: AddressingMode) {
        match mode {
            izy => {}
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn shy(&self, mode: AddressingMode) {
        match mode {
            abx => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn shx(&self, mode: AddressingMode) {
        match mode {
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn tas(&self, mode: AddressingMode) {
        match mode {
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn las(&self, mode: AddressingMode) {
        match mode {
            aby => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn kil(&self, mode: AddressingMode) {
        match mode {
            imp => {}
            _ => panic!("Unknown instruction"),
        }
    }

    fn decode(&mut self, op: u8) {
        match op {
            0x00 => self.brk(imp),
            0x01 => self.ora(izx),
            0x02 => self.kil(imp),
            0x03 => self.slo(izx),
            0x04 => self.nop(zp),
            0x05 => self.ora(zp),
            0x06 => self.asl(zp),
            0x07 => self.slo(zp),
            0x08 => self.php(imp),
            0x09 => self.ora(imm),
            0x0A => self.asl(imp),
            0x0B => self.anc(imm),
            0x0C => self.nop(abs),
            0x0D => self.ora(abs),
            0x0E => self.asl(abs),
            0x0F => self.slo(abs),

            0x10 => self.bpl(rel),
            0x11 => self.ora(izy),
            0x12 => self.kil(imp),
            0x13 => self.slo(izy),
            0x14 => self.nop(zpx),
            0x15 => self.ora(zpx),
            0x16 => self.asl(zpx),
            0x17 => self.slo(zpx),
            0x18 => self.clc(imp),
            0x19 => self.ora(aby),
            0x1A => self.nop(imp),
            0x1B => self.slo(aby),
            0x1C => self.nop(abx),
            0x1D => self.ora(abx),
            0x1E => self.asl(abx),
            0x1F => self.slo(abx),

            0x20 => self.jsr(abs),
            0x21 => self.and(izx),
            0x22 => self.kil(imp),
            0x23 => self.rla(izx),
            0x24 => self.bit(zp),
            0x25 => self.and(zp),
            0x26 => self.rol(zp),
            0x27 => self.rla(zp),
            0x28 => self.plp(imp),
            0x29 => self.and(imm),
            0x2A => self.rol(imp),
            0x2B => self.anc(imm),
            0x2C => self.bit(abs),
            0x2D => self.and(abs),
            0x2E => self.rol(abs),
            0x2F => self.rla(abs),

            0x30 => self.bmi(rel),
            0x31 => self.and(izy),
            0x32 => self.kil(imp),
            0x33 => self.rla(izy),
            0x34 => self.nop(zpx),
            0x35 => self.and(zpx),
            0x36 => self.rol(zpx),
            0x37 => self.rla(zpx),
            0x38 => self.sec(imp),
            0x39 => self.and(aby),
            0x3A => self.nop(imp),
            0x3B => self.rla(aby),
            0x3C => self.nop(abx),
            0x3D => self.and(abx),
            0x3E => self.rol(abx),
            0x3F => self.rla(abx),

            0x40 => self.rti(imp),
            0x41 => self.eor(izx),
            0x42 => self.kil(imp),
            0x43 => self.sre(izx),
            0x44 => self.nop(zp),
            0x45 => self.eor(zp),
            0x46 => self.lsr(zp),
            0x47 => self.sre(zp),
            0x48 => self.pha(imp),
            0x49 => self.eor(imm),
            0x4A => self.lsr(imp),
            0x4B => self.alr(imm),
            0x4C => self.jmp(abs),
            0x4D => self.eor(abs),
            0x4E => self.lsr(abs),
            0x4F => self.sre(abs),

            0x50 => self.bvc(rel),
            0x51 => self.eor(izy),
            0x52 => self.kil(imp),
            0x53 => self.sre(izy),
            0x54 => self.nop(zpx),
            0x55 => self.eor(zpx),
            0x56 => self.lsr(zpx),
            0x57 => self.sre(zpx),
            0x58 => self.cli(imp),
            0x59 => self.eor(aby),
            0x5A => self.nop(imp),
            0x5B => self.sre(aby),
            0x5C => self.nop(abx),
            0x5D => self.eor(abx),
            0x5E => self.lsr(abx),
            0x5F => self.sre(abx),

            0x60 => self.rts(imp),
            0x61 => self.adc(izx),
            0x62 => self.kil(imp),
            0x63 => self.rra(izx),
            0x64 => self.nop(zp),
            0x65 => self.adc(zp),
            0x66 => self.ror(zp),
            0x67 => self.rra(zp),
            0x68 => self.pla(imp),
            0x69 => self.adc(imm),
            0x6A => self.ror(imp),
            0x6B => self.arr(imm),
            0x6C => self.jmp(ind),
            0x6D => self.adc(abs),
            0x6E => self.ror(abs),
            0x6F => self.rra(abs),

            0x70 => self.bvs(rel),
            0x71 => self.adc(izy),
            0x72 => self.kil(imp),
            0x73 => self.rra(izy),
            0x74 => self.nop(zpx),
            0x75 => self.adc(zpx),
            0x76 => self.ror(zpx),
            0x77 => self.rra(zpx),
            0x78 => self.sei(imp),
            0x79 => self.adc(aby),
            0x7A => self.nop(imp),
            0x7B => self.rra(aby),
            0x7C => self.nop(abx),
            0x7D => self.adc(abx),
            0x7E => self.ror(abx),
            0x7F => self.rra(abx),

            0x80 => self.nop(imm),
            0x81 => self.sta(izx),
            0x82 => self.nop(imm),
            0x83 => self.sax(izx),
            0x84 => self.sty(zp),
            0x85 => self.sta(zp),
            0x86 => self.stx(zp),
            0x87 => self.sax(zp),
            0x88 => self.dey(imp),
            0x89 => self.nop(imm),
            0x8A => self.txa(imp),
            0x8B => self.xaa(imm),
            0x8C => self.sty(abs),
            0x8D => self.sta(abs),
            0x8E => self.stx(abs),
            0x8F => self.sax(abs),

            0x90 => self.bcc(rel),
            0x91 => self.sta(izy),
            0x92 => self.kil(imp),
            0x93 => self.ahx(izy),
            0x94 => self.sty(zpx),
            0x95 => self.sta(zpx),
            0x96 => self.stx(zpy),
            0x97 => self.sax(zpy),
            0x98 => self.tya(imp),
            0x99 => self.sta(aby),
            0x9A => self.txs(imp),
            0x9B => self.tas(aby),
            0x9C => self.shy(abx),
            0x9D => self.sta(abx),
            0x9E => self.shx(aby),
            0x9F => self.ahx(aby),

            0xA0 => self.ldy(imm),
            0xA1 => self.lda(izx),
            0xA2 => self.ldx(imm),
            0xA3 => self.lax(izx),
            0xA4 => self.ldy(zp),
            0xA5 => self.lda(zp),
            0xA6 => self.ldx(zp),
            0xA7 => self.lax(zp),
            0xA8 => self.tay(imp),
            0xA9 => self.lda(imm),
            0xAA => self.tax(imp),
            0xAB => self.lax(imm),
            0xAC => self.ldy(abs),
            0xAD => self.lda(abs),
            0xAE => self.ldx(abs),
            0xAF => self.lax(abs),

            0xB0 => self.bcs(rel),
            0xB1 => self.lda(izy),
            0xB2 => self.kil(imp),
            0xB3 => self.lax(izy),
            0xB4 => self.ldy(zpx),
            0xB5 => self.lda(zpx),
            0xB6 => self.ldx(zpy),
            0xB7 => self.lax(zpy),
            0xB8 => self.clv(imp),
            0xB9 => self.lda(aby),
            0xBA => self.tsx(imp),
            0xBB => self.las(aby),
            0xBC => self.ldy(abx),
            0xBD => self.lda(abx),
            0xBE => self.ldx(aby),
            0xBF => self.lax(aby),

            0xC0 => self.cpy(imm),
            0xC1 => self.cmp(izx),
            0xC2 => self.nop(imm),
            0xC3 => self.dcp(izx),
            0xC4 => self.cpy(zp),
            0xC5 => self.cmp(zp),
            0xC6 => self.dec(zp),
            0xC7 => self.dcp(zp),
            0xC8 => self.iny(imp),
            0xC9 => self.cmp(imm),
            0xCA => self.dex(imp),
            0xCB => self.axs(imm),
            0xCC => self.cpy(abs),
            0xCD => self.cmp(abs),
            0xCE => self.dec(abs),
            0xCF => self.dcp(abs),

            0xD0 => self.bne(rel),
            0xD1 => self.cmp(izy),
            0xD2 => self.kil(imp),
            0xD3 => self.dcp(izy),
            0xD4 => self.nop(zpx),
            0xD5 => self.cmp(zpx),
            0xD6 => self.dec(zpx),
            0xD7 => self.dcp(zpx),
            0xD8 => self.cld(imp),
            0xD9 => self.cmp(aby),
            0xDA => self.nop(imp),
            0xDB => self.dcp(aby),
            0xDC => self.nop(abx),
            0xDD => self.cmp(abx),
            0xDE => self.dec(abx),
            0xDF => self.dcp(abx),

            0xE0 => self.cpx(imm),
            0xE1 => self.sbc(izx),
            0xE2 => self.nop(imm),
            0xE3 => self.isc(izx),
            0xE4 => self.cpx(zp),
            0xE5 => self.sbc(zp),
            0xE6 => self.inc(zp),
            0xE7 => self.isc(zp),
            0xE8 => self.inx(imp),
            0xE9 => self.sbc(imm),
            0xEA => self.nop(imp),
            0xEB => self.sbc(imm),
            0xEC => self.cpx(abs),
            0xED => self.sbc(abs),
            0xEE => self.inc(abs),
            0xEF => self.isc(abs),

            0xF0 => self.beq(rel),
            0xF1 => self.sbc(izy),
            0xF2 => self.kil(imp),
            0xF3 => self.isc(izy),
            0xF4 => self.nop(zpx),
            0xF5 => self.sbc(zpx),
            0xF6 => self.inc(zpx),
            0xF7 => self.isc(zpx),
            0xF8 => self.sed(imp),
            0xF9 => self.sbc(aby),
            0xFA => self.nop(imp),
            0xFB => self.isc(aby),
            0xFC => self.nop(abx),
            0xFD => self.sbc(abx),
            0xFE => self.inc(abx),
            0xFF => self.isc(abx),
        }
    }

    // Reference: http://www.oxyron.de/html/opcodes02.html, https://www.masswerk.at/6502/6502_instruction_set.html, http://www.6502.org/tutorials/6502opcodes.html
    pub fn execute(&mut self) -> ! {
        loop {
            let op = self.fetch_op();

            self.decode(op);
        }
    }
}

#[cfg(test)]
mod cpu {
    use super::*;

    #[test]
    fn no_unknown_instructions() {
        for i in 0x00..0xff {
            let mut cpu = CPU::new(vec![i]);
            cpu.decode(i);
        }
    }

    // TODO: this should work after implementation of all instructions
    // #[test]
    fn execute_easy_program() {
        const VALUE: u8 = 0xBE;
        const ADDR: u8 = 0xEF;
        const TO_ADD: u8 = 0x42;


        let mut cpu = CPU::new(vec![
            0xA2, VALUE,      // LDX #$BE
            0x8E, 0x00, ADDR, // STX $00EF
            0xA4, ADDR,       // LDY $EF
            0xC8,             // INY
            0x98,             // TYA
            0x69, TO_ADD,     // ADC #$42
            0x00              // BRK
        ]);

        assert_eq!(cpu.mem.zp(ADDR, 0), VALUE);
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.sr.get(SRMask::Carry), true);

        cpu.execute();
    }
}
