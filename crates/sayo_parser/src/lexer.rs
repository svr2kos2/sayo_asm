use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords and identifiers
    Identifier(String),
    Label(String),
    
    // Mnemonics (we'll rely on identifier parsing and check later)
    
    // Literals
    Integer(i64),
    HexInteger(u64),
    
    // Registers (identified from identifier)
    Register(String),
    
    // Punctuation
    Comma,
    Colon,
    Dot,
    
    // Directives
    Directive(String),
    
    // Comments
    Comment(String),
    
    // Newline/Whitespace (for line tracking)
    Newline,
    
    // Encoding annotation
    Encoding(Vec<u8>),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "identifier '{}'", s),
            Token::Label(s) => write!(f, "label '{}'", s),
            Token::Integer(n) => write!(f, "integer {}", n),
            Token::HexInteger(n) => write!(f, "hex 0x{:x}", n),
            Token::Register(s) => write!(f, "register {}", s),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),
            Token::Directive(s) => write!(f, "directive .{}", s),
            Token::Comment(_s) => write!(f, "comment"),
            Token::Newline => write!(f, "newline"),
            Token::Encoding(_) => write!(f, "encoding"),
        }
    }
}

/// Simple lexer for Sayo assembly
pub struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, pos: 0, line: 1, column: 1 }
    }
    
    pub fn current_line(&self) -> usize {
        self.line
    }
    
    pub fn current_column(&self) -> usize {
        self.column
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if predicate(ch) {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    pub fn peek_token(&mut self) -> Option<(usize, Token, usize)> {
        let saved_pos = self.pos;
        let saved_line = self.line;
        let saved_column = self.column;
        let token = self.next_token();
        self.pos = saved_pos;
        self.line = saved_line;
        self.column = saved_column;
        token
    }

    pub fn next_token(&mut self) -> Option<(usize, Token, usize)> {
        self.skip_whitespace();

        let start = self.pos;
        let ch = self.peek_char()?;

        let token = match ch {
            '\n' => {
                self.advance();
                Token::Newline
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            ';' => {
                // Comment
                self.advance();
                let comment = self.read_while(|c| c != '\n');
                Token::Comment(comment.trim().to_string())
            }
            '.' => {
                self.advance();
                let ident = self.read_while(|c| c.is_alphanumeric() || c == '_');
                // Check if it's a directive (common directives start with known keywords)
                if ident.is_empty() {
                    // Just a dot, might be part of something else
                    return self.next_token();
                }
                // If starts with lowercase and is a known directive, treat as directive
                let first_char = ident.chars().next().unwrap();
                if first_char.is_lowercase() && matches!(ident.as_str(), 
                    "text" | "data" | "bss" | "section" | "globl" | "global" | "local" |
                    "type" | "size" | "byte" | "word" | "long" | "quad" | 
                    "ascii" | "asciz" | "zero" | "align" | "p2align" | 
                    "org" | "skip" | "file" | "ident" | "loc" | "addrsig" | "string") {
                    Token::Directive(ident)
                } else {
                    // It's a local label like .LBB14_25 or .loop
                    Token::Identifier(format!(".{}", ident))
                }
            }
            '0'..='9' => {
                let num_str = self.read_while(|c| c.is_alphanumeric() || c == 'x' || c == 'X');
                if num_str.starts_with("0x") || num_str.starts_with("0X") {
                    let hex = u64::from_str_radix(&num_str[2..], 16).unwrap_or(0);
                    Token::HexInteger(hex)
                } else {
                    let num = num_str.parse::<i64>().unwrap_or(0);
                    Token::Integer(num)
                }
            }
            '-' => {
                self.advance();
                let num_str = self.read_while(|c| c.is_numeric());
                let num = num_str.parse::<i64>().unwrap_or(0);
                Token::Integer(-num)
            }
            'a'..='z' | 'A'..='Z' | '_' | '*' => {
                let ident = self.read_while(|c| {
                    c.is_alphanumeric() || c == '_' || c == '*'
                });
                Token::Identifier(ident)
            }
            _ => {
                self.advance();
                return self.next_token();
            }
        };

        Some((start, token, self.pos))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = (usize, Token, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
