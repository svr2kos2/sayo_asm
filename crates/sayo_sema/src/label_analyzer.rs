use std::collections::HashMap;
use sayo_ast::{Program, Item, Instruction, Operand};
use crate::SemanticError;

/// Label table for tracking label definitions
#[derive(Debug, Clone)]
pub struct LabelTable {
    /// Global labels map: label_name -> address
    global_labels: HashMap<String, u16>,
    /// Local labels map: global_context -> (local_label_name -> address)
    local_labels: HashMap<String, HashMap<String, u16>>,
    /// Current global context for local label binding
    current_global: Option<String>,
}

impl LabelTable {
    pub fn new() -> Self {
        Self {
            global_labels: HashMap::new(),
            local_labels: HashMap::new(),
            current_global: None,
        }
    }
    
    /// Add a global label (no leading '.')
    pub fn add_global_label(&mut self, name: String, address: u16) -> Result<(), ()> {
        if self.global_labels.contains_key(&name) {
            return Err(());
        }
        self.global_labels.insert(name.clone(), address);
        self.current_global = Some(name);
        Ok(())
    }
    
    /// Add a local label (starts with '.')
    pub fn add_local_label(&mut self, name: String, address: u16) -> Result<(), ()> {
        let global_context = match &self.current_global {
            Some(ctx) => ctx.clone(),
            None => return Err(()), // No global label context
        };
        
        let local_map = self.local_labels.entry(global_context).or_insert_with(HashMap::new);
        
        if local_map.contains_key(&name) {
            return Err(());
        }
        
        local_map.insert(name, address);
        Ok(())
    }
    
    /// Resolve a label reference
    pub fn resolve(&self, name: &str) -> Result<u16, ResolveError> {
        if name.starts_with('.') {
            // Local label - must have current global context
            let global_context = match &self.current_global {
                Some(ctx) => ctx,
                None => return Err(ResolveError::NoGlobalContext),
            };
            
            if let Some(local_map) = self.local_labels.get(global_context) {
                if let Some(&address) = local_map.get(name) {
                    return Ok(address);
                }
            }
            
            Err(ResolveError::Undefined)
        } else {
            // Global label
            self.global_labels.get(name)
                .copied()
                .ok_or(ResolveError::Undefined)
        }
    }
    
    /// Check if a label is defined
    pub fn is_defined(&self, name: &str) -> bool {
        self.resolve(name).is_ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveError {
    Undefined,
    NoGlobalContext,
}

/// Label analyzer - performs two-pass analysis
pub struct LabelAnalyzer {
    table: LabelTable,
    errors: Vec<SemanticError>,
}

impl LabelAnalyzer {
    pub fn new() -> Self {
        Self {
            table: LabelTable::new(),
            errors: Vec::new(),
        }
    }
    
    /// Perform two-pass label analysis
    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        self.errors.clear();
        
        // First pass: collect all label definitions
        self.first_pass(program);
        
        // Second pass: validate all label references
        self.second_pass(program);
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    /// First pass: collect label definitions and their addresses
    fn first_pass(&mut self, program: &Program) {
        let mut address: u16 = 0;
        
        for item in &program.items {
            match &item.node {
                Item::Label(label_name) => {
                    let is_local = label_name.starts_with('.');
                    
                    if is_local {
                        // Local label
                        if let Err(()) = self.table.add_local_label(label_name.clone(), address) {
                            if self.table.current_global.is_none() {
                                self.errors.push(SemanticError::LocalLabelWithoutGlobal {
                                    label: label_name.clone(),
                                    line: item.span.start,
                                    col: item.span.end,
                                });
                            } else {
                                self.errors.push(SemanticError::DuplicateLabel {
                                    label: label_name.clone(),
                                    line: item.span.start,
                                    col: item.span.end,
                                });
                            }
                        }
                    } else {
                        // Global label
                        if let Err(()) = self.table.add_global_label(label_name.clone(), address) {
                            self.errors.push(SemanticError::DuplicateLabel {
                                label: label_name.clone(),
                                line: item.span.start,
                                col: item.span.end,
                            });
                        }
                    }
                }
                Item::Instruction(instr) => {
                    // Calculate instruction length
                    let length = instr.mnemonic.metadata().length as u16;
                    address = address.saturating_add(length);
                }
                _ => {
                    // Directives don't contribute to address
                }
            }
        }
    }
    
    /// Second pass: validate label references
    fn second_pass(&mut self, program: &Program) {
        // Reset current global for second pass
        self.table.current_global = None;
        
        for item in &program.items {
            match &item.node {
                Item::Label(label_name) => {
                    // Track current global context for local label resolution
                    if !label_name.starts_with('.') {
                        self.table.current_global = Some(label_name.clone());
                    }
                }
                Item::Instruction(instr) => {
                    self.check_instruction_labels(instr);
                }
                _ => {}
            }
        }
    }
    
    /// Check all label references in an instruction
    fn check_instruction_labels(&mut self, instr: &Instruction) {
        for operand in &instr.operands {
            if let Operand::Label(label_name) = &operand.node {
                match self.table.resolve(label_name) {
                    Ok(_address) => {
                        // Label is defined, valid
                    }
                    Err(ResolveError::Undefined) => {
                        self.errors.push(SemanticError::UndefinedLabel {
                            label: label_name.clone(),
                            line: operand.span.start,
                            col: operand.span.end,
                        });
                    }
                    Err(ResolveError::NoGlobalContext) => {
                        self.errors.push(SemanticError::LocalLabelWithoutGlobal {
                            label: label_name.clone(),
                            line: operand.span.start,
                            col: operand.span.end,
                        });
                    }
                }
            }
        }
    }
}

impl Default for LabelAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sayo_ast::{Instruction, Operand, Mnemonic, Span, Spanned};
    
    #[test]
    fn test_global_label() {
        let mut analyzer = LabelAnalyzer::new();
        let program = Program {
            items: vec![
                Spanned::new(Item::Label("start".to_string()), Span::new(0, 5)),
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::JMP,
                        operands: vec![Spanned::new(Operand::Label("start".to_string()), Span::new(10, 15))],
                        encoding: None,
                    }),
                    Span::new(6, 20),
                ),
            ],
        };
        
        assert!(analyzer.analyze(&program).is_ok());
    }
    
    #[test]
    fn test_local_label() {
        let mut analyzer = LabelAnalyzer::new();
        let program = Program {
            items: vec![
                Spanned::new(Item::Label("main".to_string()), Span::new(0, 4)),
                Spanned::new(Item::Label(".loop".to_string()), Span::new(5, 10)),
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::JMP,
                        operands: vec![Spanned::new(Operand::Label(".loop".to_string()), Span::new(15, 20))],
                        encoding: None,
                    }),
                    Span::new(11, 25),
                ),
            ],
        };
        
        assert!(analyzer.analyze(&program).is_ok());
    }
    
    #[test]
    fn test_undefined_label() {
        let mut analyzer = LabelAnalyzer::new();
        let program = Program {
            items: vec![
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::JMP,
                        operands: vec![Spanned::new(Operand::Label("undefined".to_string()), Span::new(0, 10))],
                        encoding: None,
                    }),
                    Span::new(0, 10),
                ),
            ],
        };
        
        let result = analyzer.analyze(&program);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], SemanticError::UndefinedLabel { .. }));
    }
    
    #[test]
    fn test_duplicate_global_label() {
        let mut analyzer = LabelAnalyzer::new();
        let program = Program {
            items: vec![
                Spanned::new(Item::Label("start".to_string()), Span::new(0, 5)),
                Spanned::new(Item::Label("start".to_string()), Span::new(10, 15)),
            ],
        };
        
        let result = analyzer.analyze(&program);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], SemanticError::DuplicateLabel { .. }));
    }
    
    #[test]
    fn test_local_label_without_global() {
        let mut analyzer = LabelAnalyzer::new();
        let program = Program {
            items: vec![
                Spanned::new(Item::Label(".loop".to_string()), Span::new(0, 5)),
            ],
        };
        
        let result = analyzer.analyze(&program);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], SemanticError::LocalLabelWithoutGlobal { .. }));
    }
    
    #[test]
    fn test_local_label_scope_isolation() {
        let mut analyzer = LabelAnalyzer::new();
        let program = Program {
            items: vec![
                // Function 1
                Spanned::new(Item::Label("func1".to_string()), Span::new(0, 5)),
                Spanned::new(Item::Label(".loop".to_string()), Span::new(6, 11)),
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::JMP,
                        operands: vec![Spanned::new(Operand::Label(".loop".to_string()), Span::new(12, 17))],
                        encoding: None,
                    }),
                    Span::new(12, 20),
                ),
                // Function 2 - same local label name, different scope
                Spanned::new(Item::Label("func2".to_string()), Span::new(21, 26)),
                Spanned::new(Item::Label(".loop".to_string()), Span::new(27, 32)),
                Spanned::new(
                    Item::Instruction(Instruction {
                        mnemonic: Mnemonic::JMP,
                        operands: vec![Spanned::new(Operand::Label(".loop".to_string()), Span::new(33, 38))],
                        encoding: None,
                    }),
                    Span::new(33, 41),
                ),
            ],
        };
        
        // Both functions can use the same local label name without conflict
        assert!(analyzer.analyze(&program).is_ok());
    }
}
