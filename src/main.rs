#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

mod cpu;

fn main() {
    //                               LDA   #C0   TAX   INX   BRK
    let mut cpu = cpu::CPU::new(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    cpu.execute();
}
