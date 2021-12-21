#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec;

#[allow(non_snake_case)]




pub struct CPU {
    A: u8, //accumulator
    X: u8, //X register
    Y: u8, //Y register
    PC: u16, // program counter
    FLAGS: u8, //status flags
    SP: u8,
    RAM: [u8; 0xFFFF],
    address: Option<u16>,
    fetched: u8,
}
pub struct Instruction {
    mnemonic: Mnemonics,
    addressing_mode: AddressingModes,
    length: u8,
    cycles: u8
}

impl Instruction {
    fn new(mnemonic: Mnemonics, addressing_mode: AddressingModes, length: u8, cycles: u8) -> Self {
        Instruction {
            mnemonic,
            addressing_mode,
            length,
            cycles,
        }
    }
    
}


pub enum Flag {
    C,
    Z,
    I,
    //D,
    V,
    N,
}

pub enum Mnemonics {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY, FOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

pub enum AddressingModes {
    ZeroPageN,
    ZeroPageX,
    ZeroPageY,
    AbsoluteN,
    AbsoluteX,
    AbsoluteY,
    IndirectN,
    IndirectX,
    IndirectY,
    ImplicitA,
    Immediate,
    Branching
}



#[allow(non_snake_case)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            A: 0,
            X: 0,
            Y: 0,
            SP: 0xFD,
            FLAGS: 0x34,
            RAM: [0; 0xFFFF],
            PC: 0,
            address: None,
            fetched: 0
        }
    }
    pub fn generate_instruction_set(&self) -> Vec<Instruction> {
        vec![
            Instruction::new(Mnemonics::BRK, AddressingModes::ImplicitA, 1, 7), Instruction::new(Mnemonics::ORA, AddressingModes::IndirectX, 2, 6), //00
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //02
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::ORA, AddressingModes::ZeroPageN, 2, 3), //04
            Instruction::new(Mnemonics::ASL, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //06
            Instruction::new(Mnemonics::PHP, AddressingModes::ImplicitA, 1, 3), Instruction::new(Mnemonics::ORA, AddressingModes::Immediate, 2, 2), //08
            Instruction::new(Mnemonics::ASL, AddressingModes::ImplicitA, 1, 2), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //0A
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::ORA, AddressingModes::AbsoluteN, 3, 4), //0C
            Instruction::new(Mnemonics::ASL, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //0E
            Instruction::new(Mnemonics::BPL, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::ORA, AddressingModes::IndirectX, 2, 5), //10
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //12
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::ORA, AddressingModes::ZeroPageX, 2, 4), //14
            Instruction::new(Mnemonics::ASL, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //16
            Instruction::new(Mnemonics::CLC, AddressingModes::ImplicitA, 1, 2), Instruction::new(Mnemonics::ORA, AddressingModes::AbsoluteY, 3, 4), //18
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //1A
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), Instruction::new(Mnemonics::ORA, AddressingModes::AbsoluteX, 3, 4), //1C
            Instruction::new(Mnemonics::ASL, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitA, 1, 1), //1E
        ]
    }
    fn get_flag(&mut self, flag: Flag) -> u8 {
        
        match flag {
            Flag::C => self.FLAGS & 1,
            Flag::Z => (self.FLAGS & 1 << 1) >> 1,
            Flag::I => (self.FLAGS & 1 << 2) >> 2,
            //Flag::D => (self.FLAGS & 1 << 3) >> 3,
            Flag::V => (self.FLAGS & 1 << 6) >> 6,
            Flag::N => (self.FLAGS & 1 << 7) >> 7,
        }
        
    }
    fn set_flag(&mut self, flag: Flag) {
        match flag {
            Flag::C => self.FLAGS |= 1,
            Flag::Z => self.FLAGS |= 1 << 1,
            Flag::I => self.FLAGS |= 1 << 2,
            //Flag::D => self.FLAGS |= 1 << 3,
            Flag::V => self.FLAGS |= 1 << 6,
            Flag::N => self.FLAGS |= 1 << 7,
        }
    }
    fn unset_flag(&mut self, flag: Flag) {
        match flag {
            Flag::C => self.FLAGS |= 1,
            Flag::Z => self.FLAGS |= 1 << 1,
            Flag::I => self.FLAGS |= 1 << 2,
            //Flag::D => self.FLAGS |= 1 << 3,
            Flag::V => self.FLAGS |= 1 << 6,
            Flag::N => self.FLAGS |= 1 << 7,
        }
    }
    pub fn call(&mut self, instruction: Instruction) {
        let length = instruction.length;
        let mut arguments: Vec<u8> = Vec::new();
        while length > 0 {
            self.PC += 1;
            arguments.push(self.RAM[self.PC as usize])
           
        }
        self.address = match instruction.addressing_mode {
            AddressingModes::AbsoluteN => Some(((arguments[0] as u16) << 8)| arguments[1] as u16),
            AddressingModes::AbsoluteX => Some((((arguments[0] as u16) << 8)| arguments[1] as u16) + self.X as u16),
            AddressingModes::AbsoluteY => Some((((arguments[0] as u16) << 8)| arguments[1] as u16) + self.Y as u16),
            AddressingModes::ZeroPageN => Some(arguments[0] as u16),
            AddressingModes::ZeroPageX => Some(arguments[0] as u16 + self.X as u16),
            AddressingModes::ZeroPageY => Some(arguments[0] as u16 + self.X as u16),
            
            _ => None,
        };
        
        
    }
    fn ADC() {}
    fn AND() {}
    fn ASL() {}
    fn BCC() {}
    fn BCS() {}
    fn BEQ() {}
    fn BIT() {}
    fn BMI() {}
    fn BNE() {}
    fn BPL() {}
    fn BRK() {}
    fn BVC() {}
    fn BVS() {}
    fn CLC(&mut self) {
        self.set_flag(Flag::C)
    }
    fn LDA(&mut self, value: u8) {
        self.A = value;
        if value == 0 {
            self.set_flag(Flag::Z)
        }
        else {

        }
        
    }
    fn NOP(&mut self) {}
    
}  
    

    
    





