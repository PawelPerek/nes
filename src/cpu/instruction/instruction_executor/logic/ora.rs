use crate::{cpu::{instruction::{addressing_mode::AddressingMode, instruction_arguments::InstructionArguments}, registers::{Registers, StatusMask}}, memory::Memory};

use super::super::InstructionExecutor;

pub struct Ora;

impl InstructionExecutor for Ora {
    fn execute(&self, mode: &AddressingMode, args: &InstructionArguments, memory: &mut Memory, registers: &mut Registers) {
        use AddressingMode::*;

        registers.a |= match mode {
            Immediate => *args.as_one().unwrap(),
            ZeroPage => memory.zp(*args.as_one().unwrap(), 0),
            ZeroPageX => memory.zp(*args.as_one().unwrap(), registers.x),
            IndirectX => memory.ind(*args.as_one().unwrap(), registers.x),
            IndirectY => memory.ind(*args.as_one().unwrap(), registers.y),
            Absolute => memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, 0),
            AbsoluteX => memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.x.into()),
            AbsoluteY => memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.y.into()),
            _ => panic!("Unknown instruction"),
        };

        registers.sr.set(StatusMask::Zero, registers.a == 0);
        registers.sr.set(StatusMask::Negative, registers.a > 0b0100_0000);
    }
}

#[cfg(test)]
mod ora {
    use crate::cpu::{instruction::instruction_executor::test_environment::{TestEnvironmentBuilder, TestEnvironment}};

    use super::*;

    #[test]
    fn immediate_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Immediate)
            .registers(Registers { a: 0b0100_0000, ..Registers::default() })
            .args(InstructionArguments::One(0b0000_0100))
            .build()
            .unwrap();


        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b0100_0100);
    }
    
    #[test]
    fn zero_page_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::ZeroPage)
            .registers(Registers { a: 0b1111_0000, ..Registers::default() })
            .args(InstructionArguments::One(0x10))
            .build()
            .unwrap();

        memory.write(0x10, 0b0000_1010).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b1111_1010);
    }
    
    #[test]
    fn zero_page_x_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::ZeroPageX)
            .registers(Registers { a: 0b1010_1010, x: 0x05, ..Registers::default() })
            .args(InstructionArguments::One(0x10))
            .build()
            .unwrap();

        memory.write(0x15, 0b0101_0101).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b1111_1111);
    }
    
    #[test]
    fn indirect_x_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::IndirectX)
            .registers(Registers { a: 0b0000_0011, x: 0x05, ..Registers::default() })
            .args(InstructionArguments::One(0x10))
            .build()
            .unwrap();

        memory.write(0x15, 0x20).expect("Memory write error");
        memory.write(0x20, 0b0011_0000).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b0011_0011);
    }

    #[test]
    fn indirect_y_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::IndirectY)
            .registers(Registers { a: 0b1100_1100, y: 0x02, ..Registers::default() })
            .args(InstructionArguments::One(0x07))
            .build()
            .unwrap();

        memory.write(0x09, 0x12).expect("Memory write error");
        memory.write(0x12, 0b1000_1101).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b1100_1101);
    }

    #[test]
    fn absolute_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Absolute)
            .registers(Registers { a: 0b0000_1101, ..Registers::default() })
            .args(InstructionArguments::Two(0x21, 0x03))
            .build()
            .unwrap();

        memory.write(0x0321, 0b0000_0010).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b0000_1111);
    }
    
    #[test]
    fn absolute_x_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::AbsoluteX)
            .registers(Registers { a: 0b0010_0010, x: 0x12, ..Registers::default() })
            .args(InstructionArguments::Two(0x21, 0x03))
            .build()
            .unwrap();

        memory.write(0x0333, 0b1000_1101).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b1010_1111);
    }
    
    #[test]
    fn absolute_y_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::AbsoluteY)
            .registers(Registers { a: 0b1000_1000, y: 0x32, ..Registers::default() })
            .args(InstructionArguments::Two(0x11, 0x02))
            .build()
            .unwrap();

        memory.write(0x0243, 0b0010_0110).expect("Memory write error");

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b1010_1110);
    }

    #[test]
    fn negative_flag_updates() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Immediate)
            .registers(Registers { a: 0b0000_0011, ..Registers::default() })
            .args(InstructionArguments::One(0b1000_0000))
            .build()
            .unwrap();

        registers.sr.set(StatusMask::Carry, true);

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0b1000_0011);
        assert_eq!(registers.sr.get(StatusMask::Negative), true);
    }
    
    #[test]
    fn zero_flag_updates() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Immediate)
            .registers(Registers { a: 0b0000_0000, ..Registers::default() })
            .args(InstructionArguments::One(0b0000_0000))
            .build()
            .unwrap();

        let ora = Ora;

        ora.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 0);
        assert_eq!(registers.sr.get(StatusMask::Zero), true);
    }
}