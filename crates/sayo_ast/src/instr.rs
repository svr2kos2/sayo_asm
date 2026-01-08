use serde::{Deserialize, Serialize};
use std::fmt;

/// Operand types for instructions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperandType {
    None,
    Register,
    Label,
    U8,      // 无符号8位
    I8,      // 有符号8位
    U16,     // 无符号16位
    I16,     // 有符号16位
    U32,     // 无符号32位
    I32,     // 有符号32位
    Rgb888,  // RGB888颜色
}

/// Operand definition with semantic information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperandDef {
    pub op_type: OperandType,
    /// Whether this operand is written to (for register operands)
    pub is_write: bool,
}

impl OperandDef {
    pub fn new(op_type: OperandType, is_write: bool) -> Self {
        Self { op_type, is_write }
    }
    
    pub fn read(op_type: OperandType) -> Self {
        Self { op_type, is_write: false }
    }
    
    pub fn write(op_type: OperandType) -> Self {
        Self { op_type, is_write: true }
    }
}

/// Instruction format describing operands
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstructionFormat {
    pub mnemonic: String,
    pub opcode: u8,
    pub length: u8,
    pub operands: Vec<OperandDef>,
    pub description: String,
}

impl Mnemonic {
    /// Get the instruction metadata for this mnemonic
    pub fn metadata(&self) -> InstructionMetadata {
        match self {
            // Sleep instructions
            Self::SLEEP_X256 => InstructionMetadata::new(self, 0x05, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::SLEEP => InstructionMetadata::new(self, 0x06, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::SLEEP_RAND_X256 => InstructionMetadata::new(self, 0x07, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::SLEEP_RAND => InstructionMetadata::new(self, 0x08, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::SLEEP_U16 => InstructionMetadata::new(self, 0x0D, 3, vec![OperandDef::read(OperandType::U16)]),
            Self::SLEEP_RAND_U16 => InstructionMetadata::new(self, 0x0E, 3, vec![OperandDef::read(OperandType::U16)]),
            
            // Jump instructions
            Self::SJMP => InstructionMetadata::new(self, 0x03, 2, vec![OperandDef::read(OperandType::I8)]),
            Self::AJMP => InstructionMetadata::new(self, 0x04, 2, vec![OperandDef::read(OperandType::U8)]),
            
            // Key press/release with immediate values
            Self::PRESS_SK => InstructionMetadata::new(self, 0x10, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::PRESS_GK => InstructionMetadata::new(self, 0x11, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::PRESS_MK => InstructionMetadata::new(self, 0x12, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::PRESS_MU => InstructionMetadata::new(self, 0x13, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::RELEASE_SK => InstructionMetadata::new(self, 0x18, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::RELEASE_GK => InstructionMetadata::new(self, 0x19, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::RELEASE_MK => InstructionMetadata::new(self, 0x1A, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::RELEASE_MU => InstructionMetadata::new(self, 0x1B, 2, vec![OperandDef::read(OperandType::U8)]),
            
            // Mouse movement
            Self::MO_XYZ => InstructionMetadata::new(self, 0x21, 3, vec![OperandDef::read(OperandType::U8), OperandDef::read(OperandType::I8)]),
            Self::TB_XY => InstructionMetadata::new(self, 0x25, 5, vec![OperandDef::read(OperandType::I16), OperandDef::read(OperandType::I16)]),
            
            // Arithmetic with immediate
            Self::AND8 => InstructionMetadata::new(self, 0x57, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            Self::ADD8_A => InstructionMetadata::new(self, 0x59, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::SUB8_A => InstructionMetadata::new(self, 0x5B, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::OR8_A => InstructionMetadata::new(self, 0x5D, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::XOR8 => InstructionMetadata::new(self, 0x63, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            Self::SHL8 => InstructionMetadata::new(self, 0x65, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            Self::SHR8 => InstructionMetadata::new(self, 0x67, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            
            // MOV with immediate
            Self::MOV8 => InstructionMetadata::new(self, 0x6F, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            Self::MOV16 => InstructionMetadata::new(self, 0x70, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U16)]),
            Self::MOV32 => InstructionMetadata::new(self, 0x71, 6, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U32)]),
            
            // ADD/SUB with immediate
            Self::ADD8 => InstructionMetadata::new(self, 0x73, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            Self::ADD16 => InstructionMetadata::new(self, 0x74, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U16)]),
            Self::ADD32 => InstructionMetadata::new(self, 0x7D, 6, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U32)]),
            Self::SUB8 => InstructionMetadata::new(self, 0x76, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            Self::SUB16 => InstructionMetadata::new(self, 0x77, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U16)]),
            Self::SUB32 => InstructionMetadata::new(self, 0x7E, 6, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U32)]),
            
            // Bitwise with immediate
            Self::AND16 => InstructionMetadata::new(self, 0x7A, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U16)]),
            Self::AND32 => InstructionMetadata::new(self, 0x7F, 6, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U32)]),
            Self::OR16 => InstructionMetadata::new(self, 0x7B, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U16)]),
            Self::OR32 => InstructionMetadata::new(self, 0x80, 6, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U32)]),
            Self::XOR16 => InstructionMetadata::new(self, 0x7C, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U16)]),
            Self::XOR32 => InstructionMetadata::new(self, 0x81, 6, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U32)]),
            
            // Sign extension with immediate
            Self::MOV8SX => InstructionMetadata::new(self, 0x8C, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::I8)]),
            Self::MOV16SX => InstructionMetadata::new(self, 0x8D, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::I16)]),
            
            // Register operations that write to first operand
            Self::DEC => InstructionMetadata::new(self, 0x5E, 2, vec![OperandDef::write(OperandType::Register)]),
            Self::INC => InstructionMetadata::new(self, 0x5F, 2, vec![OperandDef::write(OperandType::Register)]),
            Self::CLR => InstructionMetadata::new(self, 0x68, 2, vec![OperandDef::write(OperandType::Register)]),
            Self::NOT => InstructionMetadata::new(self, 0x69, 2, vec![OperandDef::write(OperandType::Register)]),
            Self::PUSH => InstructionMetadata::new(self, 0x6C, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::POP => InstructionMetadata::new(self, 0x6D, 2, vec![OperandDef::write(OperandType::Register)]),
            
            // Two-register operations
            Self::AND => InstructionMetadata::new(self, 0x56, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::ADD => InstructionMetadata::new(self, 0x72, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::SUB => InstructionMetadata::new(self, 0x75, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::OR => InstructionMetadata::new(self, 0x78, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::XOR => InstructionMetadata::new(self, 0x62, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::SHL => InstructionMetadata::new(self, 0x64, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::SHR => InstructionMetadata::new(self, 0x66, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::MOV => InstructionMetadata::new(self, 0x6E, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::XCH => InstructionMetadata::new(self, 0x6A, 3, vec![OperandDef::write(OperandType::Register), OperandDef::write(OperandType::Register)]),
            Self::MOVSX8b => InstructionMetadata::new(self, 0x8A, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::MOVSX16b => InstructionMetadata::new(self, 0x8B, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            
            // Three-register operations (all write to first operand)
            Self::ADD_R => InstructionMetadata::new(self, 0x82, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::SUB_R => InstructionMetadata::new(self, 0x83, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::AND_R => InstructionMetadata::new(self, 0x84, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::OR_R => InstructionMetadata::new(self, 0x85, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::XOR_R => InstructionMetadata::new(self, 0x86, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::MUL_R => InstructionMetadata::new(self, 0x87, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::DIV_R => InstructionMetadata::new(self, 0x88, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::MOD_R => InstructionMetadata::new(self, 0x89, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::IMUL_R => InstructionMetadata::new(self, 0x8F, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            
            // A register operations (write to A)
            Self::ADD_A => InstructionMetadata::new(self, 0x58, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::SUB_A => InstructionMetadata::new(self, 0x5A, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::OR_A => InstructionMetadata::new(self, 0x5C, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // No-operand instructions
            Self::END => InstructionMetadata::new(self, 0x00, 1, vec![]),
            Self::NOP => InstructionMetadata::new(self, 0x01, 1, vec![]),
            Self::UPDATE => InstructionMetadata::new(self, 0x20, 1, vec![]),
            Self::KEY_TO_AXIS => InstructionMetadata::new(self, 0x29, 1, vec![]),
            Self::C2K => InstructionMetadata::new(self, 0x30, 1, vec![]),
            Self::U2K => InstructionMetadata::new(self, 0x31, 1, vec![]),
            Self::C2K_RAND => InstructionMetadata::new(self, 0x32, 1, vec![]),
            Self::U2K_REG => InstructionMetadata::new(self, 0x33, 1, vec![]),
            Self::MUL_A => InstructionMetadata::new(self, 0x60, 1, vec![]),
            Self::DIV_A => InstructionMetadata::new(self, 0x61, 1, vec![]),
            Self::IMUL_A => InstructionMetadata::new(self, 0x8E, 1, vec![]),
            Self::RET => InstructionMetadata::new(self, 0x55, 1, vec![]),
            Self::WHILE_UPDATE => InstructionMetadata::new(self, 0xF4, 1, vec![]),
            Self::MODE_JOG => InstructionMetadata::new(self, 0xF8, 1, vec![]),
            Self::WAIT_IF_RELEASE => InstructionMetadata::new(self, 0xF9, 1, vec![]),
            Self::WAIT_IF_PRESS => InstructionMetadata::new(self, 0xFA, 1, vec![]),
            Self::EXIT_IF_RELEAS => InstructionMetadata::new(self, 0xFB, 1, vec![]),
            Self::EXIT_IF_PRESS => InstructionMetadata::new(self, 0xFC, 1, vec![]),
            Self::EXIT_IF_ANYKEY => InstructionMetadata::new(self, 0xFD, 1, vec![]),
            Self::RES => InstructionMetadata::new(self, 0xFE, 1, vec![]),
            Self::EXIT => InstructionMetadata::new(self, 0xFF, 1, vec![]),
            
            // Label jump instructions
            Self::JMP => InstructionMetadata::new(self, 0x02, 3, vec![OperandDef::read(OperandType::Label)]),
            Self::CALL => InstructionMetadata::new(self, 0x54, 3, vec![OperandDef::read(OperandType::Label)]),
            Self::JC => InstructionMetadata::new(self, 0x4E, 3, vec![OperandDef::read(OperandType::Label)]),
            Self::JNC => InstructionMetadata::new(self, 0x4F, 3, vec![OperandDef::read(OperandType::Label)]),
            
            // Register jump instructions
            Self::JFC => InstructionMetadata::new(self, 0x48, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::JFNC => InstructionMetadata::new(self, 0x49, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // Key press/release with register values
            Self::PRESS_SK_VAL => InstructionMetadata::new(self, 0x14, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::PRESS_GK_VAL => InstructionMetadata::new(self, 0x15, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::PRESS_MK_VAL => InstructionMetadata::new(self, 0x16, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::PRESS_MU_VAL => InstructionMetadata::new(self, 0x17, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::RELEASE_SK_VAL => InstructionMetadata::new(self, 0x1C, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::RELEASE_GK_VAL => InstructionMetadata::new(self, 0x1D, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::RELEASE_MK_VAL => InstructionMetadata::new(self, 0x1E, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::RELEASE_MU_VAL => InstructionMetadata::new(self, 0x1F, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // Sleep with register values
            Self::SLEEP_X256_VAL => InstructionMetadata::new(self, 0x09, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::SLEEP_VAL => InstructionMetadata::new(self, 0x0A, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::SLEEP_RAND_X8_VAL => InstructionMetadata::new(self, 0x0B, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::SLEEP_RAND_VAL => InstructionMetadata::new(self, 0x0C, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // Gamepad keys
            Self::PRESS_GAK => InstructionMetadata::new(self, 0x2C, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::PRESS_GAK_VAL => InstructionMetadata::new(self, 0x2D, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::RELEASE_GAK => InstructionMetadata::new(self, 0x2E, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::RELEASE_GAK_VAL => InstructionMetadata::new(self, 0x2F, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // Dial data
            Self::DIAL_DATA => InstructionMetadata::new(self, 0x27, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::DIAL_DATA_VAL => InstructionMetadata::new(self, 0x28, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // Print register
            Self::PRINT_REG => InstructionMetadata::new(self, 0x34, 2, vec![OperandDef::read(OperandType::Register)]),
            
            // LED control
            Self::LED_CTRL => InstructionMetadata::new(self, 0xE0, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::START => InstructionMetadata::new(self, 0xE2, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::STOP => InstructionMetadata::new(self, 0xE3, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::SYCON => InstructionMetadata::new(self, 0xE8, 2, vec![OperandDef::read(OperandType::U8)]),
            Self::JMP_TO_SCRIPT => InstructionMetadata::new(self, 0xF5, 2, vec![OperandDef::read(OperandType::U8)]),
            
            // Memory management
            Self::MALLOC => InstructionMetadata::new(self, 0xF0, 2, vec![OperandDef::write(OperandType::Register)]),
            Self::FREE => InstructionMetadata::new(self, 0xF1, 2, vec![OperandDef::read(OperandType::Register)]),
            Self::MOV_PC2REG => InstructionMetadata::new(self, 0xF6, 2, vec![OperandDef::write(OperandType::Register)]),
            Self::VALUE_RELOAD => InstructionMetadata::new(self, 0xF7, 2, vec![OperandDef::write(OperandType::Register)]),
            
            // Compare instructions (read both registers)
            Self::CMP => InstructionMetadata::new(self, 0x6B, 3, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            
            // Bitwise with register
            Self::OR8 => InstructionMetadata::new(self, 0x79, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::U8)]),
            
            // Three-operand mouse/joystick instructions
            Self::MO_XYZ_VAL => InstructionMetadata::new(self, 0x22, 3, vec![OperandDef::read(OperandType::U8), OperandDef::read(OperandType::Register)]),
            Self::GA_XYZ_VAL => InstructionMetadata::new(self, 0x24, 3, vec![OperandDef::read(OperandType::U8), OperandDef::read(OperandType::Register)]),
            Self::TB_XY_VAL => InstructionMetadata::new(self, 0x26, 3, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            
            // Three-operand jump instructions with registers
            Self::JFZ => InstructionMetadata::new(self, 0x4A, 3, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::JFNZ => InstructionMetadata::new(self, 0x4B, 3, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::DJFNZ => InstructionMetadata::new(self, 0x4C, 3, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Register)]),
            
            // Jump instructions with register and label
            Self::JZ => InstructionMetadata::new(self, 0x50, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
            Self::JNZ => InstructionMetadata::new(self, 0x51, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
            Self::DJNZ => InstructionMetadata::new(self, 0x52, 4, vec![OperandDef::write(OperandType::Register), OperandDef::read(OperandType::Label)]),
            
            // Four-operand instructions
            Self::JFA => InstructionMetadata::new(self, 0x40, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::JFB => InstructionMetadata::new(self, 0x41, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::JFG => InstructionMetadata::new(self, 0x42, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::JFL => InstructionMetadata::new(self, 0x43, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::CJFNE => InstructionMetadata::new(self, 0x4D, 4, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            Self::GA_XYZ => InstructionMetadata::new(self, 0x23, 4, vec![OperandDef::read(OperandType::U8), OperandDef::read(OperandType::U16)]),
            Self::LED_COL => InstructionMetadata::new(self, 0xE1, 4, vec![OperandDef::read(OperandType::Rgb888)]),
            Self::NEW_THREAD => InstructionMetadata::new(self, 0xF2, 4, vec![OperandDef::read(OperandType::U8), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register)]),
            
            // Five-operand instructions with two registers and label
            Self::JA => InstructionMetadata::new(self, 0x44, 5, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
            Self::JB => InstructionMetadata::new(self, 0x45, 5, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
            Self::JG => InstructionMetadata::new(self, 0x46, 5, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
            Self::JL => InstructionMetadata::new(self, 0x47, 5, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
            Self::CJNE => InstructionMetadata::new(self, 0x53, 5, vec![OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Register), OperandDef::read(OperandType::Label)]),
        }
    }
    
    /// Get the human-readable description for this instruction
    pub fn description(&self) -> (&'static str, &'static str) {
        // Returns (description, note)
        match self {
            Self::END => ("程序结束", ""),
            Self::NOP => ("空操作", ""),
            Self::JMP => ("PC = i;", "长跳转"),
            Self::SJMP => ("PC = PC + i;", "短跳转，偏移量"),
            Self::AJMP => ("PC = (PC & 0xff00) + i;", "在地址的256B范围内跳转"),
            Self::SLEEP_X256 => ("Sleep(i * 256);", "延时范围0-65280ms（256倍率）"),
            Self::SLEEP => ("Sleep(i * 1);", "延时范围0-255ms"),
            Self::SLEEP_RAND_X256 => ("Sleep(rand()%(i * 256)+1);", "随机延时范围1-65281ms（256倍率）"),
            Self::SLEEP_RAND => ("Sleep(rand()%i+1);", "随机延时范围1-256ms"),
            Self::SLEEP_X256_VAL => ("Sleep(i * 256);", "延时的寄存器版本，范围取决于寄存器（256倍率）"),
            Self::SLEEP_VAL => ("Sleep(i);", "延时的寄存器版本，范围取决于寄存器"),
            Self::SLEEP_RAND_X8_VAL => ("Sleep(rand()%(i * 8)+1);", "随机延时的寄存器版本，范围取决于寄存器（8倍率）"),
            Self::SLEEP_RAND_VAL => ("Sleep(rand()%i+1);", "随机延时的寄存器版本，范围取决于寄存器"),
            Self::SLEEP_U16 => ("Sleep(i);", "延时范围1-65536ms"),
            Self::SLEEP_RAND_U16 => ("Sleep(rand()%i+1);", "延时范围1-65536ms"),
            Self::PRESS_SK => ("键盘 修饰键 i 按下", "HID键码"),
            Self::PRESS_GK => ("键盘 普通键 i 按下", "HID键码"),
            Self::PRESS_MK => ("鼠标 鼠标键 i 按下", "HID键码"),
            Self::PRESS_MU => ("按键 多媒体 i 按下", "HID键码"),
            Self::RELEASE_SK => ("键盘 修饰键 i 释放", "HID键码"),
            Self::RELEASE_GK => ("键盘 普通键 i 释放", "HID键码"),
            Self::RELEASE_MK => ("鼠标 鼠标键 i 释放", "HID键码"),
            Self::RELEASE_MU => ("按键 多媒体 i 释放", "HID键码"),
            Self::MOV => ("i=j;", ""),
            Self::MOV8 => ("i=j;", "8位宽度"),
            Self::MOV16 => ("i=j;", "16位宽度"),
            Self::MOV32 => ("i=j;", "32位宽度"),
            Self::ADD => ("i=i+j;", ""),
            Self::ADD8 => ("i=i+j;", "8位宽度"),
            Self::ADD16 => ("i=i+j;", "16位宽度"),
            Self::ADD32 => ("i=i+j;", "32位宽度"),
            Self::SUB => ("i=i-j;", ""),
            Self::SUB8 => ("i=i-j;", "8位宽度"),
            Self::SUB16 => ("i=i-j;", "16位宽度"),
            Self::SUB32 => ("i=i-j;", "32位宽度"),
            Self::AND => ("i=i&j;", "按位与"),
            Self::AND8 => ("i=i&j;", "8位宽度"),
            Self::AND16 => ("i=i&j;", "16位宽度"),
            Self::AND32 => ("i=i&j;", "32位宽度"),
            Self::OR => ("i=i|j;", "按位或"),
            Self::OR8 => ("i=i|j;", "8位宽度"),
            Self::OR16 => ("i=i|j;", "16位宽度"),
            Self::OR32 => ("i=i|j;", "32位宽度"),
            Self::XOR => ("i=i^j;", "按位异或"),
            Self::XOR8 => ("i=i^j;", "8位宽度"),
            Self::XOR16 => ("i=i^j;", "16位宽度"),
            Self::XOR32 => ("i=i^j;", "32位宽度"),
            Self::CALL => ("PUSH PC;PC=i;", "调用子程序，目标地址为label"),
            Self::RET => ("POP PC;", "子程序返回"),
            Self::PUSH => ("", "压栈"),
            Self::POP => ("", "出栈"),
            Self::JZ => ("if (!i) PC = j;", "寄存器为0跳转。目标地址为label"),
            Self::JNZ => ("if (i) PC = j;", "寄存器不为0跳转。目标地址为label"),
            Self::DEC => ("i--;", "减一"),
            Self::INC => ("i++;", "加一"),
            Self::CLR => ("i=0;", "寄存器清零"),
            Self::NOT => ("i=~i;", "寄存器按位取反"),
            Self::CMP => ("CY=i<j;", "比较两个寄存器并设置CY标识"),
            Self::ADD_R => ("i=j+k;", "三操作数加法"),
            Self::SUB_R => ("i=j-k;", "三操作数减法"),
            Self::AND_R => ("i=j&k;", "三操作数按位与"),
            Self::OR_R => ("i=j|k;", "三操作数按位或"),
            Self::XOR_R => ("i=j^k;", "三操作数按位异或"),
            Self::MUL_R => ("i=j*k;", "三操作数乘法"),
            Self::DIV_R => ("i=j/k;", "三操作数除法"),
            Self::MOD_R => ("i=j%k;", "三操作数取模"),
            Self::NEW_THREAD => ("i=TH ID;j=addr or keymode;k=V[4]", "i的取值范围0~3"),
            Self::LED_CTRL => ("SELECTED_LED = i;", "0xff = release"),
            Self::EXIT => ("exit();", "退出脚本"),
            Self::UPDATE => ("更新输出", ""),
            Self::KEY_TO_AXIS => ("键盘转轴", ""),
            Self::C2K => ("点击转按键", ""),
            Self::U2K => ("释放转按键", ""),
            Self::C2K_RAND => ("随机点击转按键", ""),
            Self::U2K_REG => ("寄存器释放转按键", ""),
            Self::MUL_A => ("A = A * B", "乘法，结果存入A"),
            Self::DIV_A => ("A = A / B", "除法，结果存入A"),
            Self::IMUL_A => ("A = A * B", "有符号乘法，结果存入A"),
            Self::WHILE_UPDATE => ("循环更新", ""),
            Self::MODE_JOG => ("慢跑模式", ""),
            Self::WAIT_IF_RELEASE => ("等待释放", ""),
            Self::WAIT_IF_PRESS => ("等待按下", ""),
            Self::EXIT_IF_RELEAS => ("释放则退出", ""),
            Self::EXIT_IF_PRESS => ("按下则退出", ""),
            Self::EXIT_IF_ANYKEY => ("任意键退出", ""),
            Self::RES => ("资源", ""),
            Self::XCH => ("i <=> j;", "交换两个寄存器的值"),
            Self::SHL => ("i = i << j;", "逻辑左移"),
            Self::SHL8 => ("i = i << j;", "逻辑左移8位"),
            Self::SHR => ("i = i >> j;", "逻辑右移"),
            Self::SHR8 => ("i = i >> j;", "逻辑右移8位"),
            Self::JC => ("if (CY) PC = i;", "进位则跳转"),
            Self::JNC => ("if (!CY) PC = i;", "不进位则跳转"),
            Self::JFC => ("if (CY) PC = PC + i;", "进位则短跳转"),
            Self::JFNC => ("if (!CY) PC = PC + i;", "不进位则短跳转"),
            Self::PRESS_SK_VAL => ("键盘 修饰键 i 按下", "寄存器版本"),
            Self::PRESS_GK_VAL => ("键盘 普通键 i 按下", "寄存器版本"),
            Self::PRESS_MK_VAL => ("鼠标 鼠标键 i 按下", "寄存器版本"),
            Self::PRESS_MU_VAL => ("按键 多媒体 i 按下", "寄存器版本"),
            Self::RELEASE_SK_VAL => ("键盘 修饰键 i 释放", "寄存器版本"),
            Self::RELEASE_GK_VAL => ("键盘 普通键 i 释放", "寄存器版本"),
            Self::RELEASE_MK_VAL => ("鼠标 鼠标键 i 释放", "寄存器版本"),
            Self::RELEASE_MU_VAL => ("按键 多媒体 i 释放", "寄存器版本"),
            Self::PRESS_GAK => ("手柄按键 i 按下", "HID键码"),
            Self::PRESS_GAK_VAL => ("手柄按键 i 按下", "寄存器版本"),
            Self::RELEASE_GAK => ("手柄按键 i 释放", "HID键码"),
            Self::RELEASE_GAK_VAL => ("手柄按键 i 释放", "寄存器版本"),
            Self::DIAL_DATA => ("拨盘数据", ""),
            Self::DIAL_DATA_VAL => ("拨盘数据", "寄存器版本"),
            Self::PRINT_REG => ("打印寄存器", "调试用"),
            Self::START => ("启动", ""),
            Self::STOP => ("停止", ""),
            Self::SYCON => ("系统控制", ""),
            Self::JMP_TO_SCRIPT => ("跳转到脚本", ""),
            Self::MALLOC => ("分配内存", ""),
            Self::FREE => ("释放内存", ""),
            Self::MOV_PC2REG => ("PC值存入寄存器", ""),
            Self::VALUE_RELOAD => ("重载值", ""),
            Self::MO_XYZ => ("鼠标移动XYZ", ""),
            Self::MO_XYZ_VAL => ("鼠标移动XYZ", "寄存器版本"),
            Self::GA_XYZ => ("手柄轴XYZ", ""),
            Self::GA_XYZ_VAL => ("手柄轴XYZ", "寄存器版本"),
            Self::TB_XY => ("触控板XY", ""),
            Self::TB_XY_VAL => ("触控板XY", "寄存器版本"),
            Self::JFZ => ("if (!i) PC = PC + j;", "短跳转：寄存器为0"),
            Self::JFNZ => ("if (i) PC = PC + j;", "短跳转：寄存器不为0"),
            Self::DJFNZ => ("i--;if (i) PC = PC + j;", "短跳转：减一后不为0"),
            Self::DJNZ => ("i--;if (i) PC = j;", "长跳转：减一后不为0"),
            Self::JFA => ("if (i > j) PC = PC + k;", "短跳转：i大于j"),
            Self::JFB => ("if (i < j) PC = PC + k;", "短跳转：i小于j"),
            Self::JFG => ("if (i >= j) PC = PC + k;", "短跳转：i大于等于j"),
            Self::JFL => ("if (i <= j) PC = PC + k;", "短跳转：i小于等于j"),
            Self::CJFNE => ("if (i != j) PC = PC + k;", "短跳转：i不等于j"),
            Self::JA => ("if (i > j) PC = k;", "长跳转：i大于j"),
            Self::JB => ("if (i < j) PC = k;", "长跳转：i小于j"),
            Self::JG => ("if (i >= j) PC = k;", "长跳转：i大于等于j"),
            Self::JL => ("if (i <= j) PC = k;", "长跳转：i小于等于j"),
            Self::CJNE => ("if (i != j) PC = k;", "长跳转：i不等于j"),
            Self::LED_COL => ("LED颜色", "RGB888格式"),
            Self::MOVSX8b => ("i = sign_extend(j);", "8位符号扩展到目标位宽"),
            Self::MOVSX16b => ("i = sign_extend(j);", "16位符号扩展到目标位宽"),
            Self::MOV8SX => ("i = sign_extend(imm8);", "8位立即数符号扩展"),
            Self::MOV16SX => ("i = sign_extend(imm16);", "16位立即数符号扩展"),
            Self::IMUL_R => ("i=j*k;", "三操作数有符号乘法"),
            Self::ADD_A => ("A = A + i;", "加到A寄存器"),
            Self::SUB_A => ("A = A - i;", "从A寄存器减"),
            Self::OR_A => ("A = A | i;", "按位或到A寄存器"),
            Self::ADD8_A => ("A = A + i;", "8位立即数加到A"),
            Self::SUB8_A => ("A = A - i;", "8位立即数从A减"),
            Self::OR8_A => ("A = A | i;", "8位立即数按位或到A"),
        }
    }
}

/// Instruction metadata containing semantic information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstructionMetadata {
    pub mnemonic: Mnemonic,
    pub opcode: u8,
    pub length: u8,
    pub operands: Vec<OperandDef>,
    pub description: String,
    pub note: String,
}

impl InstructionMetadata {
    pub fn new(mnemonic: &Mnemonic, opcode: u8, length: u8, operands: Vec<OperandDef>) -> Self {
        Self {
            mnemonic: *mnemonic,
            opcode,
            length,
            operands,
            description: String::new(),
            note: String::new(),
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    pub fn with_note(mut self, note: &str) -> Self {
        self.note = note.to_string();
        self
    }
}

/// Instruction mnemonic enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Mnemonic {
    END, NOP, JMP, SJMP, AJMP,
    SLEEP_X256, SLEEP, SLEEP_RAND_X256, SLEEP_RAND,
    SLEEP_X256_VAL, SLEEP_VAL, SLEEP_RAND_X8_VAL, SLEEP_RAND_VAL,
    SLEEP_U16, SLEEP_RAND_U16,
    
    // Key press/release
    PRESS_SK, PRESS_GK, PRESS_MK, PRESS_MU,
    PRESS_SK_VAL, PRESS_GK_VAL, PRESS_MK_VAL, PRESS_MU_VAL,
    RELEASE_SK, RELEASE_GK, RELEASE_MK, RELEASE_MU,
    RELEASE_SK_VAL, RELEASE_GK_VAL, RELEASE_MK_VAL, RELEASE_MU_VAL,
    
    // Mouse/Joystick
    UPDATE, MO_XYZ, MO_XYZ_VAL, GA_XYZ, GA_XYZ_VAL,
    TB_XY, TB_XY_VAL, DIAL_DATA, DIAL_DATA_VAL,
    KEY_TO_AXIS,
    
    // Gamepad
    PRESS_GAK, PRESS_GAK_VAL, RELEASE_GAK, RELEASE_GAK_VAL,
    
    // Print
    C2K, U2K, C2K_RAND, U2K_REG, PRINT_REG,
    
    // Jump/Branch
    JFA, JFB, JFG, JFL, JA, JB, JG, JL,
    JFC, JFNC, JFZ, JFNZ, DJFNZ, CJFNE,
    JC, JNC, JZ, JNZ, DJNZ, CJNE,
    
    // Call/Return
    CALL, RET,
    
    // Arithmetic and Logic
    AND, AND8, AND16, AND32,
    ADD_A, ADD8_A, SUB_A, SUB8_A,
    OR_A, OR8_A,
    DEC, INC,
    MUL_A, DIV_A, IMUL_A,
    XOR, XOR8, XOR16, XOR32,
    SHL, SHL8, SHR, SHR8,
    CLR, NOT, XCH, CMP,
    
    ADD, ADD8, ADD16, ADD32,
    SUB, SUB8, SUB16, SUB32,
    OR, OR8, OR16, OR32,
    
    ADD_R, SUB_R, AND_R, OR_R, XOR_R,
    MUL_R, DIV_R, MOD_R, IMUL_R,
    
    // Move
    PUSH, POP,
    MOV, MOV8, MOV16, MOV32,
    MOVSX8b, MOVSX16b, MOV8SX, MOV16SX,
    
    // LED Control
    LED_CTRL, LED_COL,
    
    // System
    START, STOP, SYCON,
    
    // Memory Management
    MALLOC, FREE,
    NEW_THREAD,
    
    // Control Flow
    WHILE_UPDATE, JMP_TO_SCRIPT, MOV_PC2REG, VALUE_RELOAD,
    MODE_JOG, WAIT_IF_RELEASE, WAIT_IF_PRESS,
    EXIT_IF_RELEAS, EXIT_IF_PRESS, EXIT_IF_ANYKEY,
    RES, EXIT,
}

impl Mnemonic {
    /// Get all mnemonic variants for completion
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::END, Self::NOP, Self::JMP, Self::SJMP, Self::AJMP,
            Self::SLEEP_X256, Self::SLEEP, Self::SLEEP_RAND_X256, Self::SLEEP_RAND,
            Self::SLEEP_X256_VAL, Self::SLEEP_VAL, Self::SLEEP_RAND_X8_VAL, Self::SLEEP_RAND_VAL,
            Self::SLEEP_U16, Self::SLEEP_RAND_U16,
            Self::PRESS_SK, Self::PRESS_GK, Self::PRESS_MK, Self::PRESS_MU,
            Self::PRESS_SK_VAL, Self::PRESS_GK_VAL, Self::PRESS_MK_VAL, Self::PRESS_MU_VAL,
            Self::RELEASE_SK, Self::RELEASE_GK, Self::RELEASE_MK, Self::RELEASE_MU,
            Self::RELEASE_SK_VAL, Self::RELEASE_GK_VAL, Self::RELEASE_MK_VAL, Self::RELEASE_MU_VAL,
            Self::UPDATE, Self::MO_XYZ, Self::MO_XYZ_VAL, Self::GA_XYZ, Self::GA_XYZ_VAL,
            Self::TB_XY, Self::TB_XY_VAL, Self::DIAL_DATA, Self::DIAL_DATA_VAL,
            Self::KEY_TO_AXIS,
            Self::PRESS_GAK, Self::PRESS_GAK_VAL, Self::RELEASE_GAK, Self::RELEASE_GAK_VAL,
            Self::C2K, Self::U2K, Self::C2K_RAND, Self::U2K_REG, Self::PRINT_REG,
            Self::JFA, Self::JFB, Self::JFG, Self::JFL, Self::JA, Self::JB, Self::JG, Self::JL,
            Self::JFC, Self::JFNC, Self::JFZ, Self::JFNZ, Self::DJFNZ, Self::CJFNE,
            Self::JC, Self::JNC, Self::JZ, Self::JNZ, Self::DJNZ, Self::CJNE,
            Self::CALL, Self::RET,
            Self::AND, Self::AND8, Self::AND16, Self::AND32,
            Self::ADD_A, Self::ADD8_A, Self::SUB_A, Self::SUB8_A,
            Self::OR_A, Self::OR8_A,
            Self::DEC, Self::INC,
            Self::MUL_A, Self::DIV_A, Self::IMUL_A,
            Self::XOR, Self::XOR8, Self::XOR16, Self::XOR32,
            Self::SHL, Self::SHL8, Self::SHR, Self::SHR8,
            Self::CLR, Self::NOT, Self::XCH, Self::CMP,
            Self::ADD, Self::ADD8, Self::ADD16, Self::ADD32,
            Self::SUB, Self::SUB8, Self::SUB16, Self::SUB32,
            Self::OR, Self::OR8, Self::OR16, Self::OR32,
            Self::ADD_R, Self::SUB_R, Self::AND_R, Self::OR_R, Self::XOR_R,
            Self::MUL_R, Self::DIV_R, Self::MOD_R, Self::IMUL_R,
            Self::PUSH, Self::POP,
            Self::MOV, Self::MOV8, Self::MOV16, Self::MOV32,
            Self::MOVSX8b, Self::MOVSX16b, Self::MOV8SX, Self::MOV16SX,
            Self::LED_CTRL, Self::LED_COL,
            Self::START, Self::STOP, Self::SYCON,
            Self::MALLOC, Self::FREE,
            Self::NEW_THREAD,
            Self::WHILE_UPDATE, Self::JMP_TO_SCRIPT, Self::MOV_PC2REG, Self::VALUE_RELOAD,
            Self::MODE_JOG, Self::WAIT_IF_RELEASE, Self::WAIT_IF_PRESS,
            Self::EXIT_IF_RELEAS, Self::EXIT_IF_PRESS, Self::EXIT_IF_ANYKEY,
            Self::RES, Self::EXIT,
        ]
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "END" => Some(Self::END),
            "NOP" => Some(Self::NOP),
            "JMP" => Some(Self::JMP),
            "SJMP" => Some(Self::SJMP),
            "AJMP" => Some(Self::AJMP),
            "SLEEP_X256" => Some(Self::SLEEP_X256),
            "SLEEP" => Some(Self::SLEEP),
            "SLEEP_RAND_X256" => Some(Self::SLEEP_RAND_X256),
            "SLEEP_RAND" => Some(Self::SLEEP_RAND),
            "SLEEP_X256_VAL" => Some(Self::SLEEP_X256_VAL),
            "SLEEP_VAL" => Some(Self::SLEEP_VAL),
            "SLEEP_RAND_X8_VAL" => Some(Self::SLEEP_RAND_X8_VAL),
            "SLEEP_RAND_VAL" => Some(Self::SLEEP_RAND_VAL),
            "SLEEP_U16" => Some(Self::SLEEP_U16),
            "SLEEP_RAND_U16" => Some(Self::SLEEP_RAND_U16),
            "PRESS_SK" => Some(Self::PRESS_SK),
            "PRESS_GK" => Some(Self::PRESS_GK),
            "PRESS_MK" => Some(Self::PRESS_MK),
            "PRESS_MU" => Some(Self::PRESS_MU),
            "PRESS_SK_VAL" => Some(Self::PRESS_SK_VAL),
            "PRESS_GK_VAL" => Some(Self::PRESS_GK_VAL),
            "PRESS_MK_VAL" => Some(Self::PRESS_MK_VAL),
            "PRESS_MU_VAL" => Some(Self::PRESS_MU_VAL),
            "RELEASE_SK" => Some(Self::RELEASE_SK),
            "RELEASE_GK" => Some(Self::RELEASE_GK),
            "RELEASE_MK" => Some(Self::RELEASE_MK),
            "RELEASE_MU" => Some(Self::RELEASE_MU),
            "RELEASE_SK_VAL" => Some(Self::RELEASE_SK_VAL),
            "RELEASE_GK_VAL" => Some(Self::RELEASE_GK_VAL),
            "RELEASE_MK_VAL" => Some(Self::RELEASE_MK_VAL),
            "RELEASE_MU_VAL" => Some(Self::RELEASE_MU_VAL),
            "UPDATE" => Some(Self::UPDATE),
            "MO_XYZ" => Some(Self::MO_XYZ),
            "MO_XYZ_VAL" => Some(Self::MO_XYZ_VAL),
            "GA_XYZ" => Some(Self::GA_XYZ),
            "GA_XYZ_VAL" => Some(Self::GA_XYZ_VAL),
            "TB_XY" => Some(Self::TB_XY),
            "TB_XY_VAL" => Some(Self::TB_XY_VAL),
            "DIAL_DATA" => Some(Self::DIAL_DATA),
            "DIAL_DATA_VAL" => Some(Self::DIAL_DATA_VAL),
            "KEY_TO_AXIS" => Some(Self::KEY_TO_AXIS),
            "PRESS_GAK" => Some(Self::PRESS_GAK),
            "PRESS_GAK_VAL" => Some(Self::PRESS_GAK_VAL),
            "RELEASE_GAK" => Some(Self::RELEASE_GAK),
            "RELEASE_GAK_VAL" => Some(Self::RELEASE_GAK_VAL),
            "C2K" => Some(Self::C2K),
            "U2K" => Some(Self::U2K),
            "C2K_RAND" => Some(Self::C2K_RAND),
            "U2K_REG" => Some(Self::U2K_REG),
            "PRINT_REG" => Some(Self::PRINT_REG),
            "JFA" => Some(Self::JFA),
            "JFB" => Some(Self::JFB),
            "JFG" => Some(Self::JFG),
            "JFL" => Some(Self::JFL),
            "JA" => Some(Self::JA),
            "JB" => Some(Self::JB),
            "JG" => Some(Self::JG),
            "JL" => Some(Self::JL),
            "JFC" => Some(Self::JFC),
            "JFNC" => Some(Self::JFNC),
            "JFZ" => Some(Self::JFZ),
            "JFNZ" => Some(Self::JFNZ),
            "DJFNZ" => Some(Self::DJFNZ),
            "CJFNE" => Some(Self::CJFNE),
            "JC" => Some(Self::JC),
            "JNC" => Some(Self::JNC),
            "JZ" => Some(Self::JZ),
            "JNZ" => Some(Self::JNZ),
            "DJNZ" => Some(Self::DJNZ),
            "CJNE" => Some(Self::CJNE),
            "CALL" => Some(Self::CALL),
            "RET" => Some(Self::RET),
            "AND" => Some(Self::AND),
            "AND8" => Some(Self::AND8),
            "AND16" => Some(Self::AND16),
            "AND32" => Some(Self::AND32),
            "ADD_A" => Some(Self::ADD_A),
            "ADD8_A" => Some(Self::ADD8_A),
            "SUB_A" => Some(Self::SUB_A),
            "SUB8_A" => Some(Self::SUB8_A),
            "OR_A" => Some(Self::OR_A),
            "OR8_A" => Some(Self::OR8_A),
            "DEC" => Some(Self::DEC),
            "INC" => Some(Self::INC),
            "MUL_A" => Some(Self::MUL_A),
            "DIV_A" => Some(Self::DIV_A),
            "IMUL_A" => Some(Self::IMUL_A),
            "XOR" => Some(Self::XOR),
            "XOR8" => Some(Self::XOR8),
            "XOR16" => Some(Self::XOR16),
            "XOR32" => Some(Self::XOR32),
            "SHL" => Some(Self::SHL),
            "SHL8" => Some(Self::SHL8),
            "SHR" => Some(Self::SHR),
            "SHR8" => Some(Self::SHR8),
            "CLR" => Some(Self::CLR),
            "NOT" => Some(Self::NOT),
            "XCH" => Some(Self::XCH),
            "CMP" => Some(Self::CMP),
            "ADD" => Some(Self::ADD),
            "ADD8" => Some(Self::ADD8),
            "ADD16" => Some(Self::ADD16),
            "ADD32" => Some(Self::ADD32),
            "SUB" => Some(Self::SUB),
            "SUB8" => Some(Self::SUB8),
            "SUB16" => Some(Self::SUB16),
            "SUB32" => Some(Self::SUB32),
            "OR" => Some(Self::OR),
            "OR8" => Some(Self::OR8),
            "OR16" => Some(Self::OR16),
            "OR32" => Some(Self::OR32),
            "ADD_R" => Some(Self::ADD_R),
            "SUB_R" => Some(Self::SUB_R),
            "AND_R" => Some(Self::AND_R),
            "OR_R" => Some(Self::OR_R),
            "XOR_R" => Some(Self::XOR_R),
            "MUL_R" => Some(Self::MUL_R),
            "DIV_R" => Some(Self::DIV_R),
            "MOD_R" => Some(Self::MOD_R),
            "IMUL_R" => Some(Self::IMUL_R),
            "PUSH" => Some(Self::PUSH),
            "POP" => Some(Self::POP),
            "MOV" => Some(Self::MOV),
            "MOV8" => Some(Self::MOV8),
            "MOV16" => Some(Self::MOV16),
            "MOV32" => Some(Self::MOV32),
            "MOVSX8B" | "MOVSX8b" => Some(Self::MOVSX8b),
            "MOVSX16B" | "MOVSX16b" => Some(Self::MOVSX16b),
            "MOV8SX" => Some(Self::MOV8SX),
            "MOV16SX" => Some(Self::MOV16SX),
            "LED_CTRL" => Some(Self::LED_CTRL),
            "LED_COL" => Some(Self::LED_COL),
            "START" => Some(Self::START),
            "STOP" => Some(Self::STOP),
            "SYCON" => Some(Self::SYCON),
            "MALLOC" => Some(Self::MALLOC),
            "FREE" => Some(Self::FREE),
            "NEW_THREAD" => Some(Self::NEW_THREAD),
            "WHILE_UPDATE" => Some(Self::WHILE_UPDATE),
            "JMP_TO_SCRIPT" => Some(Self::JMP_TO_SCRIPT),
            "MOV_PC2REG" => Some(Self::MOV_PC2REG),
            "VALUE_RELOAD" => Some(Self::VALUE_RELOAD),
            "MODE_JOG" => Some(Self::MODE_JOG),
            "WAIT_IF_RELEASE" => Some(Self::WAIT_IF_RELEASE),
            "WAIT_IF_PRESS" => Some(Self::WAIT_IF_PRESS),
            "EXIT_IF_RELEAS" => Some(Self::EXIT_IF_RELEAS),
            "EXIT_IF_PRESS" => Some(Self::EXIT_IF_PRESS),
            "EXIT_IF_ANYKEY" => Some(Self::EXIT_IF_ANYKEY),
            "RES" => Some(Self::RES),
            "EXIT" => Some(Self::EXIT),
            _ => None,
        }
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
