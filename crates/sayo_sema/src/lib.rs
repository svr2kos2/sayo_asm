
use thiserror::Error;

pub mod checker;
pub mod label_analyzer;

pub use checker::SemanticChecker;
pub use label_analyzer::{LabelAnalyzer, LabelTable};

/// Semantic error types
#[derive(Debug, Error, Clone, PartialEq)]
pub enum SemanticError {
    #[error("Immediate value {value} out of range for type {expected_type} at position {line}:{col}")]
    ImmediateOutOfRange {
        value: i64,
        expected_type: String,
        line: usize,
        col: usize,
    },
    
    #[error("Cannot write to read-only register {register} at position {line}:{col}")]
    WriteToReadOnlyRegister {
        register: String,
        line: usize,
        col: usize,
    },
    
    #[error("{instruction} requires {expected} operand(s), but {actual} provided at position {line}:{col}")]
    OperandCountMismatch {
        instruction: String,
        expected: usize,
        actual: usize,
        line: usize,
        col: usize,
    },
    
    #[error("Invalid operand type for {instruction}: expected {expected}, got {actual} at position {line}:{col}")]
    InvalidOperandType {
        instruction: String,
        expected: String,
        actual: String,
        line: usize,
        col: usize,
    },
    
    #[error("Undefined label '{label}' at position {line}:{col}")]
    UndefinedLabel {
        label: String,
        line: usize,
        col: usize,
    },
    
    #[error("Directives and instructions cannot be mixed in the same section (between labels) at position {line}:{col}")]
    MixedDirectivesAndInstructions {
        line: usize,
        col: usize,
    },
    
    #[error("Duplicate label definition '{label}' at position {line}:{col}")]
    DuplicateLabel {
        label: String,
        line: usize,
        col: usize,
    },
    
    #[error("Local label '{label}' used without a preceding global label at position {line}:{col}")]
    LocalLabelWithoutGlobal {
        label: String,
        line: usize,
        col: usize,
    },
    
    #[error("Unsupported alignment .align {value} at position {line}:{col}. Only .align 1 or no alignment is supported.")]
    UnsupportedAlignment {
        value: i64,
        line: usize,
        col: usize,
    },
}

/// Result type for semantic analysis
pub type SemanticResult<T> = Result<T, Vec<SemanticError>>;

/// Check if an immediate value is within range for the given type
pub fn check_immediate_range(value: i64, type_name: &str) -> bool {
    match type_name {
        "u8" => value >= 0 && value <= 255,
        "i8" => value >= -128 && value <= 127,
        "u16" => value >= 0 && value <= 65535,
        "i16" => value >= -32768 && value <= 32767,
        "u32" => value >= 0 && value <= 4294967295,
        "i32" => value >= -2147483648 && value <= 2147483647,
        _ => true, // Unknown type, assume valid
    }
}
