use crate::{cpu::{instruction::{addressing_mode::AddressingMode, instruction_arguments::InstructionArguments}, registers::{Registers, StatusMask}}, memory::Memory};

use super::InstructionExecutor;


pub struct Adc;

impl InstructionExecutor for Adc {
    fn execute(&self, mode: &AddressingMode, args: &InstructionArguments, memory: &mut Memory, registers: &mut Registers) {
        use AddressingMode::*;

        let acc_snapshot = registers.a;
        let carry = if registers.sr.get(StatusMask::Carry) { 1 } else { 0 };

        match mode {
            Immediate => registers.a = registers.a.wrapping_add(*args.as_one().unwrap() + carry),
            ZeroPage => registers.a = registers.a.wrapping_add(memory.zp(*args.as_one().unwrap(), 0) + carry),
            ZeroPageX => registers.a = registers.a.wrapping_add(memory.zp(*args.as_one().unwrap(), registers.x) + carry),
            IndirectX => registers.a = registers.a.wrapping_add(memory.ind(*args.as_one().unwrap(), registers.x) + carry),
            IndirectY => registers.a = registers.a.wrapping_add(memory.ind(*args.as_one().unwrap(), registers.y) + carry),
            Absolute => registers.a = registers.a.wrapping_add(memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, 0) + carry),
            AbsoluteX => registers.a = registers.a.wrapping_add(memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.x.into()) + carry),
            AbsoluteY => registers.a = registers.a.wrapping_add(memory.abs(*args.as_two().unwrap().0, *args.as_two().unwrap().1, registers.y.into()) + carry),
            _ => panic!("Unknown instruction"),
        }   

        registers.sr.set(StatusMask::Carry, acc_snapshot > registers.a)        
    }
}

#[cfg(test)]
mod adc {
    use crate::cpu::{instruction::instruction_executor::_mocks::{TestEnvironmentBuilder, TestEnvironment}};

    use super::*;

    #[test]
    fn immediate_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Immediate)
            .registers(Registers { a: 7, ..Registers::default() })
            .args(InstructionArguments::One(12))
            .build()
            .unwrap();


        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 19);
    }
    
    #[test]
    fn zero_page_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::ZeroPage)
            .registers(Registers { a: 40, ..Registers::default() })
            .args(InstructionArguments::One(0x10))
            .build()
            .unwrap();

        memory.write(0x10, 24).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 64);
    }
    
    #[test]
    fn zero_page_x_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::ZeroPageX)
            .registers(Registers { a: 88, x: 0x05, ..Registers::default() })
            .args(InstructionArguments::One(0x10))
            .build()
            .unwrap();

        memory.write(0x15, 8).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 96);
    }
    
    #[test]
    fn indirect_x_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::IndirectX)
            .registers(Registers { a: 7, x: 0x05, ..Registers::default() })
            .args(InstructionArguments::One(0x10))
            .build()
            .unwrap();

        memory.write(0x15, 0x20).expect("Memory write error");
        memory.write(0x20, 12).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 19);
    }

    #[test]
    fn indirect_y_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::IndirectY)
            .registers(Registers { a: 7, y: 0x02, ..Registers::default() })
            .args(InstructionArguments::One(0x07))
            .build()
            .unwrap();

        memory.write(0x09, 0x12).expect("Memory write error");
        memory.write(0x12, 1).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 8);
    }

    #[test]
    fn absolute_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Absolute)
            .registers(Registers { a: 9, ..Registers::default() })
            .args(InstructionArguments::Two(0x21, 0x03))
            .build()
            .unwrap();

        memory.write(0x0321, 34).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 43);
    }
    
    #[test]
    fn absolute_x_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::AbsoluteX)
            .registers(Registers { a: 3, x: 0x12, ..Registers::default() })
            .args(InstructionArguments::Two(0x21, 0x03))
            .build()
            .unwrap();

        memory.write(0x0333, 34).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 37);
    }
    
    #[test]
    fn absolute_y_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::AbsoluteY)
            .registers(Registers { a: 11, y: 0x32, ..Registers::default() })
            .args(InstructionArguments::Two(0x11, 0x02))
            .build()
            .unwrap();

        memory.write(0x0243, 77).expect("Memory write error");

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 88);
    }

    #[test]
    fn carry_flag_works() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Immediate)
            .registers(Registers { a: 40, ..Registers::default() })
            .args(InstructionArguments::One(26))
            .build()
            .unwrap();

        registers.sr.set(StatusMask::Carry, true);

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 67);
    }
    
    #[test]
    fn carry_flag_updates() {
        let TestEnvironment { mode, args, mut memory, mut registers } = TestEnvironmentBuilder::default()
            .mode(AddressingMode::Immediate)
            .registers(Registers { a: 240, ..Registers::default() })
            .args(InstructionArguments::One(30))
            .build()
            .unwrap();

        let adc = Adc;

        adc.execute(&mode, &args, &mut memory, &mut registers);

        assert_eq!(registers.a, 14);
        assert_eq!(registers.sr.get(StatusMask::Carry), true);
    }
}