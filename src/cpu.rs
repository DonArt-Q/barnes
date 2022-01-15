#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec;
use crate::bus::{ConnectedToBus, Bus};

#[allow(non_snake_case)]




pub struct CPU {
    A: u8, //accumulator
    X: u8, //X register
    Y: u8, //Y register
    PC: u16, // program counter
    FLAGS: u8, //status flags
    SP: u8,


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
    // Illegal opcodes
    KIL,
    SLO,
    ANC,
    RLA,
    EOR,
    SRE,
    ALR,
    RRA,
    ARR,
    SAX,
    XAA,
    AHX,
    TAS,
    SHY,
    SHX,
    LAX,
    LAS,
    DCP,
    AXS,
    ISC,
    
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
    ImplicitN,
    Immediate,
    Branching,
    Accumulat
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
            PC: 0,
            address: None,
            fetched: 0
        }
    }

    pub fn generate_instruction_set(&self) -> Vec<Instruction> {
        vec![
            Instruction::new(Mnemonics::BRK, AddressingModes::ImplicitN, 2, 7), Instruction::new(Mnemonics::ORA, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::SLO, AddressingModes::IndirectX, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::ORA, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::ASL, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::SLO, AddressingModes::ZeroPageN, 0, 5),
            Instruction::new(Mnemonics::PHP, AddressingModes::ImplicitN, 1, 3), Instruction::new(Mnemonics::ORA, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::ASL, AddressingModes::Accumulat, 1, 2), Instruction::new(Mnemonics::ANC, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::ORA, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::ASL, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::SLO, AddressingModes::AbsoluteN, 0, 6),
            Instruction::new(Mnemonics::BPL, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::ORA, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::SLO, AddressingModes::IndirectY, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::ORA, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::ASL, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::SLO, AddressingModes::ZeroPageX, 0, 6),
            Instruction::new(Mnemonics::CLC, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::ORA, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::SLO, AddressingModes::AbsoluteY, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::ORA, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::ASL, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::SLO, AddressingModes::AbsoluteX, 0, 7),
            Instruction::new(Mnemonics::JSR, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::AND, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::RLA, AddressingModes::IndirectX, 0, 8),
            Instruction::new(Mnemonics::BIT, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::AND, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::ROL, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::RLA, AddressingModes::ZeroPageN, 0, 5),
            Instruction::new(Mnemonics::PLP, AddressingModes::ImplicitN, 1, 4), Instruction::new(Mnemonics::AND, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::ROL, AddressingModes::Accumulat, 1, 2), Instruction::new(Mnemonics::ANC, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::BIT, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::AND, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::ROL, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::RLA, AddressingModes::AbsoluteN, 0, 6),
            Instruction::new(Mnemonics::BMI, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::AND, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::RLA, AddressingModes::IndirectY, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::AND, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::ROL, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::RLA, AddressingModes::ZeroPageX, 0, 6),
            Instruction::new(Mnemonics::SEC, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::AND, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::RLA, AddressingModes::AbsoluteY, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::AND, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::ROL, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::RLA, AddressingModes::AbsoluteX, 0, 7),
            Instruction::new(Mnemonics::RTI, AddressingModes::ImplicitN, 1, 6), Instruction::new(Mnemonics::EOR, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::SRE, AddressingModes::IndirectX, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::EOR, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::LSR, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::SRE, AddressingModes::ZeroPageN, 0, 5),
            Instruction::new(Mnemonics::PHA, AddressingModes::ImplicitN, 1, 3), Instruction::new(Mnemonics::EOR, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::LSR, AddressingModes::Accumulat, 1, 2), Instruction::new(Mnemonics::ALR, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::JMP, AddressingModes::AbsoluteN, 3, 3), Instruction::new(Mnemonics::EOR, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::LSR, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::SRE, AddressingModes::AbsoluteN, 0, 6),
            Instruction::new(Mnemonics::BVC, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::EOR, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::SRE, AddressingModes::IndirectY, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::EOR, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::LSR, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::SRE, AddressingModes::ZeroPageX, 0, 6),
            Instruction::new(Mnemonics::CLI, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::EOR, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::SRE, AddressingModes::AbsoluteY, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::EOR, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::LSR, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::SRE, AddressingModes::AbsoluteX, 0, 7),
            Instruction::new(Mnemonics::RTS, AddressingModes::ImplicitN, 1, 6), Instruction::new(Mnemonics::ADC, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::RRA, AddressingModes::IndirectX, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::ADC, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::ROR, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::RRA, AddressingModes::ZeroPageN, 0, 5),
            Instruction::new(Mnemonics::PLA, AddressingModes::ImplicitN, 1, 4), Instruction::new(Mnemonics::ADC, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::ROR, AddressingModes::Accumulat, 1, 2), Instruction::new(Mnemonics::ARR, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::JMP, AddressingModes::IndirectN, 3, 5), Instruction::new(Mnemonics::ADC, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::ROR, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::RRA, AddressingModes::AbsoluteN, 0, 6),
            Instruction::new(Mnemonics::BVS, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::ADC, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::RRA, AddressingModes::IndirectY, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::ADC, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::ROR, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::RRA, AddressingModes::ZeroPageX, 0, 6),
            Instruction::new(Mnemonics::SEI, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::ADC, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::RRA, AddressingModes::AbsoluteY, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::ADC, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::ROR, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::RRA, AddressingModes::AbsoluteX, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::Immediate, 2, 2), Instruction::new(Mnemonics::STA, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::NOP, AddressingModes::Immediate, 0, 2), Instruction::new(Mnemonics::SAX, AddressingModes::IndirectX, 0, 6),
            Instruction::new(Mnemonics::STY, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::STA, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::STX, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::SAX, AddressingModes::ZeroPageN, 0, 3),
            Instruction::new(Mnemonics::DEY, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::NOP, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::TXA, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::XAA, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::STY, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::STA, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::STX, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::SAX, AddressingModes::AbsoluteN, 0, 4),
            Instruction::new(Mnemonics::BCC, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::STA, AddressingModes::IndirectY, 2, 6),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::AHX, AddressingModes::IndirectY, 0, 6),
            Instruction::new(Mnemonics::STY, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::STA, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::STX, AddressingModes::ZeroPageY, 2, 4), Instruction::new(Mnemonics::SAX, AddressingModes::ZeroPageY, 0, 4),
            Instruction::new(Mnemonics::TYA, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::STA, AddressingModes::AbsoluteY, 3, 5),
            Instruction::new(Mnemonics::TXS, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::TAS, AddressingModes::AbsoluteY, 0, 5),
            Instruction::new(Mnemonics::SHY, AddressingModes::AbsoluteX, 0, 5), Instruction::new(Mnemonics::STA, AddressingModes::AbsoluteX, 3, 5),
            Instruction::new(Mnemonics::SHX, AddressingModes::AbsoluteY, 0, 5), Instruction::new(Mnemonics::AHX, AddressingModes::AbsoluteY, 0, 5),
            Instruction::new(Mnemonics::LDY, AddressingModes::Immediate, 2, 2), Instruction::new(Mnemonics::LDA, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::LDX, AddressingModes::Immediate, 2, 2), Instruction::new(Mnemonics::LAX, AddressingModes::IndirectX, 0, 6),
            Instruction::new(Mnemonics::LDY, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::LDA, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::LDX, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::LAX, AddressingModes::ZeroPageN, 0, 3),
            Instruction::new(Mnemonics::TAY, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::LDA, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::TAX, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::LAX, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::LDY, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::LDA, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::LDX, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::LAX, AddressingModes::AbsoluteN, 0, 4),
            Instruction::new(Mnemonics::BCS, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::LDA, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::LAX, AddressingModes::IndirectY, 0, 5),
            Instruction::new(Mnemonics::LDY, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::LDA, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::LDX, AddressingModes::ZeroPageY, 2, 4), Instruction::new(Mnemonics::LAX, AddressingModes::ZeroPageY, 0, 4),
            Instruction::new(Mnemonics::CLV, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::LDA, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::TSX, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::LAS, AddressingModes::AbsoluteY, 0, 4),
            Instruction::new(Mnemonics::LDY, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::LDA, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::LDX, AddressingModes::AbsoluteY, 3, 4), Instruction::new(Mnemonics::LAX, AddressingModes::AbsoluteY, 0, 4),
            Instruction::new(Mnemonics::CPY, AddressingModes::Immediate, 2, 2), Instruction::new(Mnemonics::CMP, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::NOP, AddressingModes::Immediate, 0, 2), Instruction::new(Mnemonics::DCP, AddressingModes::IndirectX, 0, 8),
            Instruction::new(Mnemonics::CPY, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::CMP, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::DEC, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::DCP, AddressingModes::ZeroPageN, 0, 5),
            Instruction::new(Mnemonics::INY, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::CMP, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::DEX, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::AXS, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::CPY, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::CMP, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::DEC, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::DCP, AddressingModes::AbsoluteN, 0, 6),
            Instruction::new(Mnemonics::BNE, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::CMP, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::DCP, AddressingModes::IndirectY, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::CMP, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::DEC, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::DCP, AddressingModes::ZeroPageX, 0, 6),
            Instruction::new(Mnemonics::CLD, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::CMP, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::DCP, AddressingModes::AbsoluteY, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::CMP, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::DEC, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::DCP, AddressingModes::AbsoluteX, 0, 7),
            Instruction::new(Mnemonics::CPX, AddressingModes::Immediate, 2, 2), Instruction::new(Mnemonics::SBC, AddressingModes::IndirectX, 2, 6),
            Instruction::new(Mnemonics::NOP, AddressingModes::Immediate, 0, 2), Instruction::new(Mnemonics::ISC, AddressingModes::IndirectX, 0, 8),
            Instruction::new(Mnemonics::CPX, AddressingModes::ZeroPageN, 2, 3), Instruction::new(Mnemonics::SBC, AddressingModes::ZeroPageN, 2, 3),
            Instruction::new(Mnemonics::INC, AddressingModes::ZeroPageN, 2, 5), Instruction::new(Mnemonics::ISC, AddressingModes::ZeroPageN, 0, 5),
            Instruction::new(Mnemonics::INX, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::SBC, AddressingModes::Immediate, 2, 2),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::SBC, AddressingModes::Immediate, 0, 2),
            Instruction::new(Mnemonics::CPX, AddressingModes::AbsoluteN, 3, 4), Instruction::new(Mnemonics::SBC, AddressingModes::AbsoluteN, 3, 4),
            Instruction::new(Mnemonics::INC, AddressingModes::AbsoluteN, 3, 6), Instruction::new(Mnemonics::ISC, AddressingModes::AbsoluteN, 0, 6),
            Instruction::new(Mnemonics::BEQ, AddressingModes::Branching, 2, 2), Instruction::new(Mnemonics::SBC, AddressingModes::IndirectY, 2, 5),
            Instruction::new(Mnemonics::KIL, AddressingModes::ImplicitN, 0, 2), Instruction::new(Mnemonics::ISC, AddressingModes::IndirectY, 0, 8),
            Instruction::new(Mnemonics::NOP, AddressingModes::ZeroPageX, 2, 4), Instruction::new(Mnemonics::SBC, AddressingModes::ZeroPageX, 2, 4),
            Instruction::new(Mnemonics::INC, AddressingModes::ZeroPageX, 2, 6), Instruction::new(Mnemonics::ISC, AddressingModes::ZeroPageX, 0, 6),
            Instruction::new(Mnemonics::SED, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::SBC, AddressingModes::AbsoluteY, 3, 4),
            Instruction::new(Mnemonics::NOP, AddressingModes::ImplicitN, 1, 2), Instruction::new(Mnemonics::ISC, AddressingModes::AbsoluteY, 0, 7),
            Instruction::new(Mnemonics::NOP, AddressingModes::AbsoluteX, 3, 4), Instruction::new(Mnemonics::SBC, AddressingModes::AbsoluteX, 3, 4),
            Instruction::new(Mnemonics::INC, AddressingModes::AbsoluteX, 3, 7), Instruction::new(Mnemonics::ISC, AddressingModes::AbsoluteX, 0, 7),

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
    fn clock() {
        
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
    
impl ConnectedToBus for CPU {
    fn read(address: u16, bus: std::rc::Rc<Bus>) -> u8 {
        todo!()
    }

    fn write(address: u16, data: u8, bus: std::rc::Rc<Bus>) {
        todo!()
    }
}
    
    






