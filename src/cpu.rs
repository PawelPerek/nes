mod registers;
mod instruction;

use registers::Registers;
use instruction::InstructionList;
use crate::memory::Memory;

pub struct CPU {
    reg: Registers,
    mem: Memory,
    prg: InstructionList,
}
use instruction::addressing_mode::AddressingMode;

impl CPU {
    pub fn new(prg: Vec<u8>) -> CPU {
        CPU {
            reg: Registers::default(),
            mem: Memory::new(),
            prg: InstructionList::from(&prg),
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
        assert_eq!(cpu.reg.a, ADDR.wrapping_add(TO_ADD));
        assert_eq!(cpu.reg.sr.get(registers::SRMask::Carry), true);

        cpu.execute();
    }
}
