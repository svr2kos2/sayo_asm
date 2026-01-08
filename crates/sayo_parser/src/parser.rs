/// Simple manual parser for Sayo assembly
/// This bypasses LALRPOP to avoid LR(1) conflicts during initial development
use crate::error::ParseError;
use crate::lexer::{Token, Lexer};
use sayo_ast::*;

pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let mut lexer = Lexer::new(input);
    let mut items = Vec::new();
    
    while let Some((start, token, end)) = lexer.next_token() {
        let span = Span::new(start, end);
        
        match token {
            Token::Directive(name) => {
                let directive = parse_directive(&name, &mut lexer)?;
                items.push(Spanned::new(Item::Directive(directive), span));
            }
            Token::Identifier(name) => {
                // Check if next is colon (label) or operands (instruction)
                if let Some((_, Token::Colon, end2)) = lexer.peek_token() {
                    lexer.next_token(); // consume colon
                    items.push(Spanned::new(Item::Label(name), Span::new(start, end2)));
                } else {
                    // It's an instruction
                    let inst = parse_instruction(&name, &mut lexer)?;
                    items.push(Spanned::new(Item::Instruction(inst), span));
                }
            }
            Token::Newline => {
                // Skip newlines
                continue;
            }
            Token::Comment(_) => {
                // Skip comments
                continue;
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    line: 0,
                    column: 0,
                    message: format!("Unexpected token: {:?}", token),
                });
            }
        }
    }
    
    Ok(Program { items })
}

fn parse_directive(name: &str, lexer: &mut Lexer) -> Result<Directive, ParseError> {
    match name {
        "text" => Ok(Directive::Text),
        "data" => Ok(Directive::Data),
        "globl" => {
            // Parse label name
            if let Some((_, Token::Identifier(label), _)) = lexer.next_token() {
                Ok(Directive::Globl(label))
            } else {
                Ok(Directive::Globl(String::new()))
            }
        }
        "align" => {
            // Parse alignment value
            if let Some((_, Token::Integer(val), _)) = lexer.next_token() {
                Ok(Directive::Align(val as u32))
            } else {
                Ok(Directive::Align(1))
            }
        }
        "file" => {
            // Parse filename - consume rest of line
            let mut filename = String::new();
            loop {
                match lexer.peek_token() {
                    None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
                    Some((_, Token::Identifier(ref s), _)) => {
                        lexer.next_token();
                        filename = s.clone();
                    }
                    _ => {
                        lexer.next_token(); // consume and ignore
                    }
                }
            }
            Ok(Directive::File(filename))
        }
        "type" => {
            // Parse "name,@type" or "name, @type" - consume everything until newline
            let mut name = String::new();
            let mut typ = String::new();
            loop {
                match lexer.peek_token() {
                    None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
                    Some((_, Token::Identifier(ref s), _)) => {
                        lexer.next_token();
                        if name.is_empty() {
                            name = s.clone();
                        } else {
                            typ = s.clone();
                        }
                    }
                    _ => {
                        lexer.next_token(); // consume commas, etc.
                    }
                }
            }
            Ok(Directive::Type(name, typ))
        }
        "size" => {
            // Parse "name, expr" - consume everything until newline
            let mut name = String::new();
            let mut size = String::new();
            loop {
                match lexer.peek_token() {
                    None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
                    Some((_, Token::Identifier(ref s), _)) => {
                        lexer.next_token();
                        if name.is_empty() {
                            name = s.clone();
                        } else if size.is_empty() {
                            size = s.clone();
                        }
                    }
                    _ => {
                        lexer.next_token(); // consume commas, minus signs, etc.
                    }
                }
            }
            Ok(Directive::Size(name, size))
        }
        _ => {
            // Unknown directive - consume rest of line
            loop {
                match lexer.peek_token() {
                    None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
                    _ => {
                        lexer.next_token();
                    }
                }
            }
            Ok(Directive::Text)
        }
    }
}

fn parse_instruction(mnemonic_str: &str, lexer: &mut Lexer) -> Result<Instruction, ParseError> {
    // Check if mnemonic is valid
    let mnemonic = match Mnemonic::from_str(mnemonic_str) {
        Some(m) => m,
        None => {
            return Err(ParseError::InvalidToken {
                line: lexer.current_line(),
                column: lexer.current_column(),
                token: format!("Unknown instruction: {}", mnemonic_str),
            });
        }
    };
    
    let mut operands = Vec::new();
    
    // Parse operands until newline or comment
    loop {
        match lexer.peek_token() {
            None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => {
                break;
            }
            Some((_start, _, _)) => {
                if let Some((s, tok, e)) = lexer.next_token() {
                    match tok {
                        Token::Identifier(id) => {
                            // Try to parse as register first (handles *, R, V, GL_, SYS_, etc.)
                            let operand = if let Some(reg) = Register::from_str(&id) {
                                Operand::Register(reg)
                            } else {
                                // It's a label (can start with . for local labels)
                                Operand::Label(id)
                            };
                            operands.push(Spanned::new(operand, Span::new(s, e)));
                        }
                        Token::Integer(n) => {
                            operands.push(Spanned::new(Operand::Immediate(n), Span::new(s, e)));
                        }
                        Token::HexInteger(n) => {
                            operands.push(Spanned::new(Operand::Immediate(n as i64), Span::new(s, e)));
                        }
                        Token::Comma => {
                            // Skip commas
                            continue;
                        }
                        _ => break,
                    }
                }
            }
        }
    }
    
    Ok(Instruction {
        mnemonic,
        operands,
        encoding: None,
    })
}
