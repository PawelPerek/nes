mod cpu;

fn main() {
    print!("{:?}", cpu::opcode::OPCODES[0xff]);
}
