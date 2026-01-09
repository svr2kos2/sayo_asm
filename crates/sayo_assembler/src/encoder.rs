use crate::binary::{self, BinaryError};
use crate::layout::Layout;
use crate::Section;
use sayo_ast::{Instruction, Item, Operand, Program};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Undefined label: {0}")]
    UndefinedLabel(String),

    #[error("Immediate value {0} out of range for {1}-bit operand")]
    ImmediateOutOfRange(i64, u8),

    #[error("Expected {expected} operands, found {found}")]
    OperandCountMismatch { expected: usize, found: usize },

    #[error("Invalid operand type at position {0}")]
    InvalidOperandType(usize),

    #[error("PC-relative offset {0} out of range for 8-bit signed")]
    PcRelOffsetOutOfRange(i64),
    
    #[error("Binary format error: {0}")]
    BinaryFormat(#[from] BinaryError),
}

/// Encoder generates machine code from AST + layout
pub struct Encoder<'a> {
    layout: &'a Layout,
}

impl<'a> Encoder<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { layout }
    }

    /// Encode the entire program into machine code with header
    pub fn encode(&mut self, program: &Program) -> Result<Vec<u8>, EncodeError> {
        // First, find the main label
        let main_addr = self.layout.symbols.resolve("main")
            .ok_or(BinaryError::NoMainLabel)?;
        
        // Calculate text section size
        let text_size = self.layout.section_sizes
            .get(&Section::Text)
            .copied()
            .unwrap_or(0);
        
        if text_size > 65535 {
            return Err(BinaryError::TextSectionTooLarge(text_size).into());
        }
        
        // Generate header
        let mut output = binary::generate_header(main_addr as u16, text_size as u16);
        
        // Encode text section
        let text_bytes = self.encode_section(program, Section::Text)?;
        output.extend_from_slice(&text_bytes);
        
        // Encode data section
        let data_bytes = self.encode_data_section(program)?;
        output.extend_from_slice(&data_bytes);

        Ok(output)
    }
    
    /// Encode only the text section (without header, for raw output)
    pub fn encode_raw(&mut self, program: &Program) -> Result<Vec<u8>, EncodeError> {
        let mut output = Vec::new();
        
        // Encode text section
        let text_bytes = self.encode_section(program, Section::Text)?;
        output.extend_from_slice(&text_bytes);
        
        // Encode data section (raw, without structured format)
        for (idx, item) in program.items.iter().enumerate() {
            if let Item::Directive(dir) = &item.node {
                // Check if we're in data section by looking at item address
                let item_addr = self.layout.item_addresses[idx];
                let text_start = self.layout.text_section_start();
                let data_start = self.layout.data_section_start();
                
                if item_addr >= data_start || (data_start == text_start && self.is_data_directive(dir)) {
                    let bytes = self.encode_data_directive(dir)?;
                    output.extend_from_slice(&bytes);
                }
            }
        }

        Ok(output)
    }
    
    /// Encode items in a specific section
    fn encode_section(&mut self, program: &Program, section: Section) -> Result<Vec<u8>, EncodeError> {
        let mut output = Vec::new();
        let section_start = match section {
            Section::Text => self.layout.text_section_start(),
            Section::Data => self.layout.data_section_start(),
        };
        let section_end = match section {
            Section::Text => self.layout.data_section_start(),
            Section::Data => u32::MAX,
        };

        for (idx, item) in program.items.iter().enumerate() {
            let item_addr = self.layout.item_addresses[idx];
            
            // Check if this item is in the target section
            if item_addr < section_start || item_addr >= section_end {
                continue;
            }

            match &item.node {
                Item::Instruction(instr) => {
                    let bytes = self.encode_instruction(instr, item_addr, idx)?;
                    output.extend_from_slice(&bytes);
                }
                Item::Label(_) => {
                    // Labels don't emit bytes
                }
                Item::Directive(_) => {
                    // In text section, most directives don't emit bytes
                    // Data directives are handled separately
                }
            }
        }

        Ok(output)
    }
    
    /// Encode the data section with linear layout (no length prefixes)
    fn encode_data_section(&mut self, program: &Program) -> Result<Vec<u8>, EncodeError> {
        let mut output = Vec::new();
        let mut in_data_section = false;
        
        for item in &program.items {
            match &item.node {
                Item::Directive(dir) => {
                    use sayo_ast::Directive;
                    match dir {
                        Directive::Data => {
                            in_data_section = true;
                            continue;
                        }
                        Directive::Section(ref s) if s.contains("rodata") => {
                            in_data_section = true;
                            continue;
                        }
                        Directive::Text => {
                            in_data_section = false;
                            continue;
                        }
                        _ => {}
                    }
                    
                    if in_data_section && self.is_data_directive(dir) {
                        let bytes = self.encode_data_directive(dir)?;
                        output.extend_from_slice(&bytes);
                    }
                }
                Item::Label(_) | Item::Instruction(_) => {
                    // Labels and instructions don't emit bytes in data section
                }
            }
        }
        
        Ok(output)
    }
    
    fn is_data_directive(&self, dir: &sayo_ast::Directive) -> bool {
        use sayo_ast::Directive;
        matches!(dir, 
            Directive::Byte(_) | 
            Directive::Word(_) | 
            Directive::Short(_) |
            Directive::Long(_) | 
            Directive::Quad(_) | 
            Directive::Ascii(_) |
            Directive::Asciz(_) |
            Directive::Zero(_) |
            Directive::Skip(_)
        )
    }
    
    fn encode_data_directive(&self, dir: &sayo_ast::Directive) -> Result<Vec<u8>, EncodeError> {
        use sayo_ast::{Directive, DataValue};
        let mut output = Vec::new();
        
        match dir {
            Directive::Byte(values) => {
                for v in values {
                    let val = self.resolve_data_value(v, 1)?;
                    output.push(val as u8);
                }
            }
            Directive::Word(values) | Directive::Short(values) => {
                for v in values {
                    let val = self.resolve_data_value(v, 2)?;
                    output.extend_from_slice(&(val as u16).to_le_bytes());
                }
            }
            Directive::Long(values) => {
                for v in values {
                    let val = self.resolve_data_value(v, 4)?;
                    output.extend_from_slice(&(val as u32).to_le_bytes());
                }
            }
            Directive::Quad(values) => {
                for v in values {
                    let val = self.resolve_data_value(v, 8)?;
                    output.extend_from_slice(&(val as u64).to_le_bytes());
                }
            }
            Directive::Ascii(s) => {
                output.extend_from_slice(s.as_bytes());
            }
            Directive::Asciz(s) => {
                output.extend_from_slice(s.as_bytes());
                output.push(0); // Null terminator
            }
            Directive::Zero(count) => {
                output.resize(*count as usize, 0);
            }
            Directive::Skip(count) => {
                output.resize(*count as usize, 0);
            }
            _ => {}
        }
        
        Ok(output)
    }
    
    fn resolve_data_value(&self, value: &sayo_ast::DataValue, _size: usize) -> Result<i64, EncodeError> {
        use sayo_ast::DataValue;
        match value {
            DataValue::Immediate(n) => Ok(*n),
            DataValue::Label(name) => {
                self.layout.symbols.resolve(name)
                    .map(|addr| addr as i64)
                    .ok_or_else(|| EncodeError::UndefinedLabel(name.clone()))
            }
        }
    }

    /// Encode a single instruction
    fn encode_instruction(
        &self,
        instr: &Instruction,
        instr_addr: u32,
        item_idx: usize,
    ) -> Result<Vec<u8>, EncodeError> {
        let metadata = instr.mnemonic.metadata();
        let mut bytes = Vec::new();

        // First byte is always the opcode
        bytes.push(metadata.opcode);

        // Check operand count
        if instr.operands.len() != metadata.operands.len() {
            return Err(EncodeError::OperandCountMismatch {
                expected: metadata.operands.len(),
                found: instr.operands.len(),
            });
        }

        // Encode each operand based on its expected type
        for (i, (operand_spanned, operand_def)) in
            instr.operands.iter().zip(&metadata.operands).enumerate()
        {
            let operand = &operand_spanned.node;

            use sayo_ast::instr::OperandType;
            match operand_def.op_type {
                OperandType::Register => {
                    if let Operand::Register(reg) = operand {
                        let reg_metadata = reg.metadata();
                        bytes.push(reg_metadata.index);
                    } else {
                        return Err(EncodeError::InvalidOperandType(i));
                    }
                }
                OperandType::U8 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    if value < 0 || value > 255 {
                        return Err(EncodeError::ImmediateOutOfRange(value, 8));
                    }
                    bytes.push(value as u8);
                }
                OperandType::I8 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    
                    // Check if this is a PC-relative jump instruction
                    use sayo_ast::instr::Mnemonic;
                    if matches!(instr.mnemonic, Mnemonic::SJMP) && matches!(operand, Operand::Label(_)) {
                        // PC-relative offset for SJMP
                        let next_pc = instr_addr + metadata.length as u32;
                        let offset = value - next_pc as i64;
                        if offset < -128 || offset > 127 {
                            return Err(EncodeError::PcRelOffsetOutOfRange(offset));
                        }
                        bytes.push(offset as i8 as u8);
                    } else {
                        // Regular I8 immediate
                        if value < -128 || value > 127 {
                            return Err(EncodeError::ImmediateOutOfRange(value, 8));
                        }
                        bytes.push(value as i8 as u8);
                    }
                }
                OperandType::U16 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    if value < 0 || value > 65535 {
                        return Err(EncodeError::ImmediateOutOfRange(value, 16));
                    }
                    bytes.extend_from_slice(&(value as u16).to_le_bytes());
                }
                OperandType::I16 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    if value < -32768 || value > 32767 {
                        return Err(EncodeError::ImmediateOutOfRange(value, 16));
                    }
                    bytes.extend_from_slice(&(value as i16 as u16).to_le_bytes());
                }
                OperandType::U32 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    bytes.extend_from_slice(&(value as u32).to_le_bytes());
                }
                OperandType::I32 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    bytes.extend_from_slice(&(value as i32 as u32).to_le_bytes());
                }
                OperandType::Rgb888 => {
                    let value = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    if value < 0 || value > 0xFFFFFF {
                        return Err(EncodeError::ImmediateOutOfRange(value, 24));
                    }
                    // RGB888: 3 bytes in little-endian
                    bytes.push((value & 0xFF) as u8);
                    bytes.push(((value >> 8) & 0xFF) as u8);
                    bytes.push(((value >> 16) & 0xFF) as u8);
                }
                OperandType::Label => {
                    // For label operands, check if the instruction expects PC-relative or absolute
                    let target_addr = self.resolve_operand_value(operand, instr_addr, item_idx)?;
                    
                    // Determine how to encode based on instruction type
                    use sayo_ast::instr::Mnemonic;
                    match instr.mnemonic {
                        Mnemonic::SJMP => {
                            // SJMP uses I8 PC-relative offset
                            // PC-relative offset = target - (current + instruction_length)
                            let next_pc = instr_addr + metadata.length as u32;
                            let offset = target_addr - next_pc as i64;
                            if offset < -128 || offset > 127 {
                                return Err(EncodeError::PcRelOffsetOutOfRange(offset));
                            }
                            bytes.push(offset as i8 as u8);
                        }
                        Mnemonic::AJMP => {
                            // AJMP uses U8 absolute address (low byte only)
                            if target_addr < 0 || target_addr > 255 {
                                return Err(EncodeError::ImmediateOutOfRange(target_addr, 8));
                            }
                            bytes.push(target_addr as u8);
                        }
                        Mnemonic::JMP => {
                            // JMP uses 16-bit absolute address (big-endian)
                            if target_addr < 0 || target_addr > 65535 {
                                return Err(EncodeError::ImmediateOutOfRange(target_addr, 16));
                            }
                            bytes.extend_from_slice(&(target_addr as u16).to_be_bytes());
                        }
                        _ => {
                            // Default: use 16-bit absolute address for other label operands (big-endian)
                            if target_addr < 0 || target_addr > 65535 {
                                return Err(EncodeError::ImmediateOutOfRange(target_addr, 16));
                            }
                            bytes.extend_from_slice(&(target_addr as u16).to_be_bytes());
                        }
                    }
                }
                OperandType::None => {
                    // No operand encoding needed
                }
            }
        }

        Ok(bytes)
    }

    /// Resolve an operand to its numeric value (for immediates or labels)
    fn resolve_operand_value(&self, operand: &Operand, _instr_addr: u32, item_idx: usize) -> Result<i64, EncodeError> {
        match operand {
            Operand::Immediate(value) => {
                Ok(*value)
            }
            Operand::Label(name) => {
                // Use the scope of the current item, not the final global scope
                let item_scope = self.layout.item_scopes.get(item_idx).and_then(|s| s.as_ref());
                let addr = self
                    .layout
                    .symbols
                    .resolve_with_scope(name, item_scope)
                    .ok_or_else(|| {
                        EncodeError::UndefinedLabel(name.clone())
                    })?;
                Ok(addr as i64)
            }
            Operand::Register(_) => {
                // This should not be called for register operands
                Err(EncodeError::InvalidOperandType(0))
            }
        }
    }
}
