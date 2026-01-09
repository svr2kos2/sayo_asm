use crate::instr::Mnemonic;
use crate::reg::Register;
use crate::span::Spanned;
use serde::{Deserialize, Serialize};

/// An assembly program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Spanned<Item>>,
}

/// A data value that can be either an immediate or a label reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataValue {
    Immediate(i64),
    Label(String),
}

/// Top-level item in assembly file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Directive(Directive),
    Label(String),
    Instruction(Instruction),
}

/// Assembler directives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Directive {
    // Section directives
    Text,
    Data,
    Bss,
    Section(String), // .section <name>
    
    // Symbol visibility directives
    Globl(String),   // .globl <symbol>
    Local(String),   // .local <symbol>
    
    // Symbol metadata
    Type(String, String),  // .type <symbol>, <type>
    Size(String, String),  // .size <symbol>, <size>
    File(String),          // .file <filename>
    Ident(String),         // .ident <string>
    Loc(String),           // .loc <params>
    
    // Data directives (support both immediate values and label references)
    Byte(Vec<DataValue>),   // .byte <values>
    Word(Vec<DataValue>),   // .word <values>
    Short(Vec<DataValue>),  // .short <values> (same as .word, 2 bytes)
    Long(Vec<DataValue>),   // .long <values>
    Quad(Vec<DataValue>),   // .quad <values>
    Ascii(String),          // .ascii <string> (no null terminator)
    Asciz(String),          // .asciz <string> (with null terminator)
    Zero(i64),              // .zero <count>
    
    // Alignment directives
    Align(u32),       // .align <n>
    P2align(u32),     // .p2align <n>
    
    // Location directives
    Org(i64),         // .org <address>
    Skip(i64),        // .skip <count>
}

/// Instruction with operands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Spanned<Operand>>,
    pub encoding: Option<Vec<u8>>,
}

/// Operand types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Label(String),
}

impl Operand {
    pub fn is_register(&self) -> bool {
        matches!(self, Self::Register(_))
    }

    pub fn is_immediate(&self) -> bool {
        matches!(self, Self::Immediate(_))
    }

    pub fn is_label(&self) -> bool {
        matches!(self, Self::Label(_))
    }
}
