use sayo_ast::{Program, Item, Instruction, Operand, OperandType, Register, Spanned};

use crate::{SemanticError, check_immediate_range, LabelAnalyzer};

/// Semantic checker for Sayo assembly
pub struct SemanticChecker {
    errors: Vec<SemanticError>,
    label_analyzer: LabelAnalyzer,
}

impl SemanticChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            label_analyzer: LabelAnalyzer::new(),
        }
    }
    
    /// Check an entire program
    pub fn check(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        self.errors.clear();
        
        // First, perform label analysis (two-pass)
        if let Err(label_errors) = self.label_analyzer.analyze(program) {
            self.errors.extend(label_errors);
        }
        
        // Check for mixed directives and instructions in sections
        self.check_mixed_directives_and_instructions(program);
        
        // Then check instructions
        for item in &program.items {
            self.check_item(item);
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn check_item(&mut self, item: &Spanned<Item>) {
        match &item.node {
            Item::Instruction(instr) => {
                self.check_instruction(instr, item.span.start, item.span.end);
            }
            _ => {
                // Labels and directives don't need semantic checking
            }
        }
    }
    
    fn check_instruction(&mut self, instr: &Instruction, start: usize, _end: usize) {
        let metadata = instr.mnemonic.metadata();
        
        // Check operand count
        let expected_count = metadata.operands.len();
        let actual_count = instr.operands.len();
        if expected_count != actual_count {
            self.errors.push(SemanticError::OperandCountMismatch {
                instruction: format!("{:?}", instr.mnemonic),
                expected: expected_count,
                actual: actual_count,
                line: start,
                col: _end,
            });
            // Continue checking the operands we have
        }
        
        // Check each operand
        for (idx, operand) in instr.operands.iter().enumerate() {
            if idx >= metadata.operands.len() {
                continue; // More operands than expected, already reported above
            }
            
            let expected = &metadata.operands[idx];
            
            // Check operand type matches
            self.check_operand_type(&operand.node, &expected.op_type, &format!("{:?}", instr.mnemonic), operand.span.start, operand.span.end);
            
            // Check immediate value ranges
            if let Operand::Immediate(value) = operand.node {
                self.check_immediate_operand(value, &expected.op_type, operand.span.start, operand.span.end);
            }
            
            // Check register write permissions
            if let Operand::Register(reg) = operand.node {
                if expected.is_write {
                    self.check_register_write(reg, operand.span.start, operand.span.end);
                }
            }
        }
    }
    
    fn check_operand_type(&mut self, operand: &Operand, expected_type: &OperandType, instruction: &str, start: usize, end: usize) {
        let matches = match (operand, expected_type) {
            (Operand::Register(_), OperandType::Register) => true,
            (Operand::Immediate(_), OperandType::U8 | OperandType::I8 | OperandType::U16 | OperandType::I16 | OperandType::U32 | OperandType::I32) => true,
            (Operand::Label(_), OperandType::Label) => true,
            // Allow immediate (0-65535) where label is expected (for JMP 123)
            (Operand::Immediate(val), OperandType::Label) => *val >= 0 && *val <= 65535,
            // Allow label where immediate is expected (treat labels as u16 addresses)
            // Labels can be used in MOV8, MOV16, MOV32, ADD8, etc.
            (Operand::Label(_), OperandType::U8 | OperandType::I8 | OperandType::U16 | OperandType::I16 | OperandType::U32 | OperandType::I32) => true,
            (Operand::Immediate(_), OperandType::Rgb888) => true, // RGB888 is also an immediate
            _ => false,
        };
        
        if !matches {
            let actual = match operand {
                Operand::Register(_) => "register",
                Operand::Immediate(val) => {
                    // Special message for out-of-range address
                    if matches!(expected_type, OperandType::Label) {
                        if *val < 0 || *val > 65535 {
                            self.errors.push(SemanticError::ImmediateOutOfRange {
                                value: *val,
                                expected_type: "u16 (address)".to_string(),
                                line: start,
                                col: end,
                            });
                            return;
                        }
                    }
                    "immediate"
                }
                Operand::Label(_) => "label",
            };
            
            let expected = match expected_type {
                OperandType::Register => "register",
                OperandType::Label => "label or u16 address (0-65535)",
                OperandType::U8 => "u8 immediate",
                OperandType::I8 => "i8 immediate",
                OperandType::U16 => "u16 immediate",
                OperandType::I16 => "i16 immediate",
                OperandType::U32 => "u32 immediate",
                OperandType::I32 => "i32 immediate",
                OperandType::Rgb888 => "rgb888 immediate",
                OperandType::None => "none",
            };
            
            self.errors.push(SemanticError::InvalidOperandType {
                instruction: instruction.to_string(),
                expected: expected.to_string(),
                actual: actual.to_string(),
                line: start,
                col: end,
            });
        }
    }
    
    fn check_immediate_operand(&mut self, value: i64, expected_type: &OperandType, start: usize, end: usize) {
        let type_name = match expected_type {
            OperandType::U8 => "u8",
            OperandType::I8 => "i8",
            OperandType::U16 => "u16",
            OperandType::I16 => "i16",
            OperandType::U32 => "u32",
            OperandType::I32 => "i32",
            _ => return, // Not an immediate type
        };
        
        if !check_immediate_range(value, type_name) {
            self.errors.push(SemanticError::ImmediateOutOfRange {
                value,
                expected_type: type_name.to_string(),
                line: start,
                col: end,
            });
        }
    }
    
    fn check_register_write(&mut self, register: Register, start: usize, end: usize) {
        let metadata = register.metadata();
        
        if metadata.is_read_only() {
            self.errors.push(SemanticError::WriteToReadOnlyRegister {
                register: register.to_string(),
                line: start,
                col: end,
            });
        }
    }
    
    fn check_mixed_directives_and_instructions(&mut self, program: &Program) {
        // Split program into sections (delimited by labels)
        let mut sections = Vec::new();
        let mut current_section = Vec::new();
        
        for item in &program.items {
            match &item.node {
                Item::Label(_) => {
                    if !current_section.is_empty() {
                        sections.push(std::mem::take(&mut current_section));
                    }
                }
                _ => {
                    current_section.push(item);
                }
            }
        }
        
        // Add the last section if it has items
        if !current_section.is_empty() {
            sections.push(current_section);
        }
        
        // Check each section for mixed directives and instructions
        for section in sections {
            let mut has_directive = false;
            let mut has_instruction = false;
            let mut error_span = (0, 0);
            
            for item in section {
                match &item.node {
                    Item::Directive(_) => {
                        has_directive = true;
                        if !has_instruction {
                            error_span = (item.span.start, item.span.end);
                        }
                    }
                    Item::Instruction(_) => {
                        has_instruction = true;
                        if !has_directive {
                            error_span = (item.span.start, item.span.end);
                        }
                    }
                    _ => {}
                }
                
                // If both are present, report error
                if has_directive && has_instruction {
                    self.errors.push(SemanticError::MixedDirectivesAndInstructions {
                        line: error_span.0,
                        col: error_span.1,
                    });
                    break; // Only report once per section
                }
            }
        }
    }
}

impl Default for SemanticChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sayo_ast::{Program, Item, Instruction, Operand, Mnemonic, Register, Spanned, Span};
    
    #[test]
    fn test_immediate_in_range() {
        let mut checker = SemanticChecker::new();
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::SLEEP,
                        operands: vec![Spanned::new(Operand::Immediate(200), Span::new(0, 10))],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        assert!(checker.check(&program).is_ok());
    }
    
    #[test]
    fn test_immediate_out_of_range() {
        let mut checker = SemanticChecker::new();
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::SLEEP,
                        operands: vec![Spanned::new(Operand::Immediate(300), Span::new(0, 10))],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        let result = checker.check(&program);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(matches!(errors[0], SemanticError::ImmediateOutOfRange { .. }));
        }
    }
    
    #[test]
    fn test_write_to_read_only_register() {
        let mut checker = SemanticChecker::new();
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::MOV8,
                        operands: vec![
                            Spanned::new(Operand::Register(Register::Zero), Span::new(0, 5)),
                            Spanned::new(Operand::Immediate(42), Span::new(6, 10)),
                        ],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        let result = checker.check(&program);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(matches!(errors[0], SemanticError::WriteToReadOnlyRegister { .. }));
        }
    }
    
    #[test]
    fn test_write_to_writable_register() {
        let mut checker = SemanticChecker::new();
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::MOV8,
                        operands: vec![
                            Spanned::new(Operand::Register(Register::R0), Span::new(0, 5)),
                            Spanned::new(Operand::Immediate(42), Span::new(6, 10)),
                        ],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        assert!(checker.check(&program).is_ok());
    }
    
    #[test]
    fn test_operand_count_mismatch() {
        let mut checker = SemanticChecker::new();
        // ADD_R requires 3 register operands, but we only provide 2
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::ADD_R,
                        operands: vec![
                            Spanned::new(Operand::Register(Register::R0), Span::new(0, 5)),
                            Spanned::new(Operand::Register(Register::R1), Span::new(6, 10)),
                        ],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        let result = checker.check(&program);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(matches!(errors[0], SemanticError::OperandCountMismatch { .. }));
        }
    }
    
    #[test]
    fn test_invalid_operand_type_register_expected() {
        let mut checker = SemanticChecker::new();
        // MOV expects register, register but we provide register, label
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::MOV,
                        operands: vec![
                            Spanned::new(Operand::Register(Register::R0), Span::new(0, 5)),
                            Spanned::new(Operand::Label("loop_start".to_string()), Span::new(6, 10)),
                        ],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        let result = checker.check(&program);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert!(errors.len() > 0);
            assert!(errors.iter().any(|e| matches!(e, SemanticError::InvalidOperandType { .. })));
        }
    }
    
    #[test]
    fn test_invalid_operand_type_immediate_expected() {
        let mut checker = SemanticChecker::new();
        // SLEEP expects u8 immediate but we provide a register
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::SLEEP,
                        operands: vec![
                            Spanned::new(Operand::Register(Register::R0), Span::new(0, 5)),
                        ],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        let result = checker.check(&program);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert!(errors.len() > 0);
            assert!(errors.iter().any(|e| matches!(e, SemanticError::InvalidOperandType { .. })));
        }
    }
    
    #[test]
    fn test_multiple_errors() {
        let mut checker = SemanticChecker::new();
        // Multiple errors: wrong operand count AND trying to write to read-only register
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::MOV8,
                        operands: vec![
                            Spanned::new(Operand::Register(Register::Zero), Span::new(0, 5)),
                        ],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                )
            ],
        };
        
        let result = checker.check(&program);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert!(errors.len() >= 2); // Should have both operand count and read-only errors
            assert!(errors.iter().any(|e| matches!(e, SemanticError::OperandCountMismatch { .. })));
            assert!(errors.iter().any(|e| matches!(e, SemanticError::WriteToReadOnlyRegister { .. })));
        }
    }
}

