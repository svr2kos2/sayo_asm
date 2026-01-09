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
        "bss" => Ok(Directive::Bss),
        "globl" | "global" => {
            // Parse label name (can be identifier or directive-like .L.str.1)
            match lexer.peek_token() {
                Some((_, Token::Identifier(label), _)) => {
                    lexer.next_token();
                    Ok(Directive::Globl(label))
                }
                Some((_, Token::Directive(label), _)) => {
                    lexer.next_token();
                    Ok(Directive::Globl(format!(".{}", label)))
                }
                _ => Ok(Directive::Globl(String::new()))
            }
        }
        "local" => {
            match lexer.peek_token() {
                Some((_, Token::Identifier(label), _)) => {
                    lexer.next_token();
                    Ok(Directive::Local(label))
                }
                Some((_, Token::Directive(label), _)) => {
                    lexer.next_token();
                    Ok(Directive::Local(format!(".{}", label)))
                }
                _ => Ok(Directive::Local(String::new()))
            }
        }
        "section" => {
            // Parse section name - can be .rodata.str1.1 or quoted string
            let section_name = consume_rest_of_line_as_string(lexer);
            Ok(Directive::Section(section_name))
        }
        "align" => {
            if let Some((_, Token::Integer(val), _)) = lexer.next_token() {
                Ok(Directive::Align(val as u32))
            } else {
                Ok(Directive::Align(1))
            }
        }
        "p2align" => {
            if let Some((_, Token::Integer(val), _)) = lexer.next_token() {
                Ok(Directive::P2align(val as u32))
            } else {
                Ok(Directive::P2align(0))
            }
        }
        "byte" => {
            let values = parse_data_values(lexer)?;
            Ok(Directive::Byte(values))
        }
        "word" | "short" => {
            let values = parse_data_values(lexer)?;
            if name == "short" {
                Ok(Directive::Short(values))
            } else {
                Ok(Directive::Word(values))
            }
        }
        "long" => {
            let values = parse_data_values(lexer)?;
            Ok(Directive::Long(values))
        }
        "quad" => {
            let values = parse_data_values(lexer)?;
            Ok(Directive::Quad(values))
        }
        "ascii" => {
            // Parse a string literal (without null terminator)
            let s = parse_string_value(lexer)?;
            Ok(Directive::Ascii(s))
        }
        "asciz" | "string" => {
            // Parse a string literal (with null terminator)
            let s = parse_string_value(lexer)?;
            Ok(Directive::Asciz(s))
        }
        "zero" => {
            if let Some((_, Token::Integer(val), _)) = lexer.next_token() {
                Ok(Directive::Zero(val))
            } else {
                Ok(Directive::Zero(0))
            }
        }
        "skip" => {
            if let Some((_, Token::Integer(val), _)) = lexer.next_token() {
                Ok(Directive::Skip(val))
            } else {
                Ok(Directive::Skip(0))
            }
        }
        "org" => {
            if let Some((_, Token::Integer(val), _)) = lexer.next_token() {
                Ok(Directive::Org(val))
            } else if let Some((_, Token::HexInteger(val), _)) = lexer.next_token() {
                Ok(Directive::Org(val as i64))
            } else {
                Ok(Directive::Org(0))
            }
        }
        "file" => {
            let content = consume_rest_of_line_as_string(lexer);
            Ok(Directive::File(content))
        }
        "ident" => {
            let content = consume_rest_of_line_as_string(lexer);
            Ok(Directive::Ident(content))
        }
        "loc" => {
            let content = consume_rest_of_line_as_string(lexer);
            Ok(Directive::Loc(content))
        }
        "type" => {
            // Parse "name,@type" or "name, @type" - consume everything until newline
            let mut sym_name = String::new();
            let mut sym_type = String::new();
            loop {
                match lexer.peek_token() {
                    None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
                    Some((_, Token::Identifier(ref s), _)) => {
                        lexer.next_token();
                        if sym_name.is_empty() {
                            sym_name = s.clone();
                        } else {
                            sym_type = s.clone();
                        }
                    }
                    Some((_, Token::Directive(ref s), _)) => {
                        lexer.next_token();
                        if sym_name.is_empty() {
                            sym_name = format!(".{}", s);
                        }
                    }
                    _ => {
                        lexer.next_token(); // consume commas, etc.
                    }
                }
            }
            Ok(Directive::Type(sym_name, sym_type))
        }
        "size" => {
            // Parse "name, expr" - consume everything until newline
            let mut sym_name = String::new();
            let mut size_expr = String::new();
            loop {
                match lexer.peek_token() {
                    None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
                    Some((_, Token::Identifier(ref s), _)) => {
                        lexer.next_token();
                        if sym_name.is_empty() {
                            sym_name = s.clone();
                        } else if size_expr.is_empty() {
                            size_expr = s.clone();
                        }
                    }
                    Some((_, Token::Directive(ref s), _)) => {
                        lexer.next_token();
                        if sym_name.is_empty() {
                            sym_name = format!(".{}", s);
                        }
                    }
                    Some((_, Token::Integer(n), _)) => {
                        lexer.next_token();
                        if size_expr.is_empty() {
                            size_expr = n.to_string();
                        }
                    }
                    _ => {
                        lexer.next_token(); // consume commas, minus signs, etc.
                    }
                }
            }
            Ok(Directive::Size(sym_name, size_expr))
        }
        "addrsig" => {
            // .addrsig has no arguments, just consume rest of line
            consume_rest_of_line(lexer);
            Ok(Directive::Addrsig)
        }
        "addrsig_sym" => {
            // Parse symbol name (can be identifier or directive-like .L.str.1)
            match lexer.peek_token() {
                Some((_, Token::Identifier(label), _)) => {
                    lexer.next_token();
                    Ok(Directive::AddrsigSym(label))
                }
                Some((_, Token::Directive(label), _)) => {
                    lexer.next_token();
                    Ok(Directive::AddrsigSym(format!(".{}", label)))
                }
                _ => {
                    // No symbol provided, treat as empty
                    Ok(Directive::AddrsigSym(String::new()))
                }
            }
        }
        _ => {
            // Unknown directive - check if it looks like a label
            // (starts with uppercase like .LBB or .L.str.1)
            if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                // It's actually a local label, not a directive
                // Return a placeholder, the caller should handle this
                consume_rest_of_line(lexer);
                Ok(Directive::Text) // This case should be handled specially by caller
            } else {
                // Unknown directive - consume rest of line
                consume_rest_of_line(lexer);
                Ok(Directive::Text)
            }
        }
    }
}

/// Parse data values (integers or label references)
fn parse_data_values(lexer: &mut Lexer) -> Result<Vec<DataValue>, ParseError> {
    let mut values = Vec::new();
    
    loop {
        match lexer.peek_token() {
            None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
            Some((_, Token::Integer(n), _)) => {
                lexer.next_token();
                values.push(DataValue::Immediate(n));
            }
            Some((_, Token::HexInteger(n), _)) => {
                lexer.next_token();
                values.push(DataValue::Immediate(n as i64));
            }
            Some((_, Token::Identifier(ref label), _)) => {
                let label = label.clone();
                lexer.next_token();
                values.push(DataValue::Label(label));
            }
            Some((_, Token::Directive(ref label), _)) => {
                // Handle labels like .L.str.1
                let label = format!(".{}", label);
                lexer.next_token();
                values.push(DataValue::Label(label));
            }
            Some((_, Token::Comma, _)) => {
                lexer.next_token();
                continue;
            }
            _ => {
                // Skip unknown tokens
                lexer.next_token();
            }
        }
    }
    
    Ok(values)
}

/// Parse a string value (handles escape sequences)
fn parse_string_value(lexer: &mut Lexer) -> Result<String, ParseError> {
    match lexer.peek_token() {
        Some((_, Token::String(s), _)) => {
            lexer.next_token();
            Ok(parse_string_escapes(&s))
        }
        _ => {
            // No string found, consume rest of line
            let content = consume_rest_of_line_as_string(lexer);
            Ok(content)
        }
    }
}

/// Parse escape sequences in a string
fn parse_string_escapes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('0') => result.push('\0'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('e') | Some('E') => result.push('\x1b'), // ESC character
                Some(d) if d.is_ascii_digit() => {
                    // Octal escape sequence: \NNN
                    let mut octal = String::new();
                    octal.push(d);
                    // Read up to 2 more octal digits
                    for _ in 0..2 {
                        if let Some(&next) = chars.peek() {
                            if next.is_ascii_digit() && next <= '7' {
                                octal.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if let Ok(code) = u8::from_str_radix(&octal, 8) {
                        result.push(code as char);
                    } else {
                        result.push_str(&format!("\\{}", octal));
                    }
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Consume rest of line, returning as string
fn consume_rest_of_line_as_string(lexer: &mut Lexer) -> String {
    let mut content = String::new();
    loop {
        match lexer.peek_token() {
            None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
            Some((_, Token::Identifier(ref s), _)) => {
                lexer.next_token();
                if !content.is_empty() { content.push(' '); }
                content.push_str(s);
            }
            Some((_, Token::Directive(ref s), _)) => {
                lexer.next_token();
                if !content.is_empty() { content.push(' '); }
                content.push('.');
                content.push_str(s);
            }
            Some((_, Token::String(ref s), _)) => {
                lexer.next_token();
                if !content.is_empty() { content.push(' '); }
                content.push_str(s);
            }
            Some((_, Token::Integer(n), _)) => {
                lexer.next_token();
                if !content.is_empty() { content.push(' '); }
                content.push_str(&n.to_string());
            }
            _ => {
                lexer.next_token(); // consume and ignore
            }
        }
    }
    content
}

/// Consume rest of line (ignoring content)
fn consume_rest_of_line(lexer: &mut Lexer) {
    loop {
        match lexer.peek_token() {
            None | Some((_, Token::Newline, _)) | Some((_, Token::Comment(_), _)) => break,
            _ => { lexer.next_token(); }
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
