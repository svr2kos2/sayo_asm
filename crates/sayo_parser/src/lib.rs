// Temporarily remove the lalrpop grammar, use a simple manual parser instead
// lalrpop_mod!(pub grammar);

pub mod lexer;
pub mod error;
pub mod parser;

pub use error::ParseError;
use sayo_ast::{Program};

/// Parse assembly source code into an AST
pub fn parse(input: &str) -> Result<Program, ParseError> {
    parser::parse_program(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = r#"
            .text
            .globl main
        main:
            mov R0, 10
            RET
        "#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    }
}
