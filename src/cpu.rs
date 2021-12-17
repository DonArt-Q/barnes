#![allow(dead_code)]
#![allow(unused_variables)]
#[allow(non_snake_case)]


pub struct CPU {
    A: u8, //accumulator
    X: u8, //X register
    Y: u8, //Y register
    PC: u16, // program counter
    FLAGS: u8, //status flags
    SP: u8,
    RAM: [u8; 0xFFFF],
}
pub struct Instruction {
    pub mnemonic: Mnemonics,
    pub addressing_mode: AddressingModes,
    pub length: u8,
    pub cycles: u8
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
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implicit,
    Accumulator,
    Immediate,
    Relative,
}



#[allow(non_snake_case)]
impl CPU {
    pub fn new() -> Self {
        todo!();
    }
    pub fn get_flag(&mut self, flag: Flag) -> u8 {
        match flag {
            Flag::C => self.FLAGS & 1,
            Flag::Z => (self.FLAGS & 1 << 1) >> 1,
            Flag::I => (self.FLAGS & 1 << 2) >> 2,
            //Flag::D => (self.FLAGS & 1 << 3) >> 3,
            Flag::V => (self.FLAGS & 1 << 6) >> 6,
            Flag::N => (self.FLAGS & 1 << 7) >> 7,
        }
        
    }
    pub fn set_flag(&mut self, flag: Flag) {
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
        match instruction.mnemonic {
            _=> self.NOP()
        }
    }
    

    fn CLC(&mut self) {

        self.set_flag(Flag::C)
    }
    fn NOP(&mut self) {
        todo!()
    }
    
}  
    

    
    





