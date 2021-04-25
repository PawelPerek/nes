#![allow(dead_code)]

mod cpu;

fn main() {
    print!("{:?}", cpu::opcode::decode_op(0xff));
}
