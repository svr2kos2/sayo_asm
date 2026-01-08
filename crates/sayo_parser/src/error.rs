use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token at line {line}, column {column}: {message}")]
    UnexpectedToken {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Invalid token at line {line}, column {column}: {token}")]
    InvalidToken {
        line: usize,
        column: usize,
        token: String,
    },

    #[error("Unrecognized token at line {line}, column {column}: {token}")]
    UnrecognizedToken {
        line: usize,
        column: usize,
        token: String,
    },

    #[error("Extra token at line {line}, column {column}: {token}")]
    ExtraToken {
        line: usize,
        column: usize,
        token: String,
    },

    #[error("Unexpected EOF")]
    UnexpectedEof,
}

impl ParseError {
    pub fn from_lalrpop<T>(err: lalrpop_util::ParseError<usize, T, &str>, input: &str) -> Self
    where
        T: std::fmt::Display,
    {
        match err {
            lalrpop_util::ParseError::InvalidToken { location } => {
                let (line, column) = Self::location_to_line_col(input, location);
                Self::InvalidToken {
                    line,
                    column,
                    token: Self::get_token_at(input, location),
                }
            }
            lalrpop_util::ParseError::UnrecognizedToken { token, expected: _ } => {
                let (line, column) = Self::location_to_line_col(input, token.0);
                Self::UnrecognizedToken {
                    line,
                    column,
                    token: token.1.to_string(),
                }
            }
            lalrpop_util::ParseError::UnrecognizedEof { .. } => Self::UnexpectedEof,
            lalrpop_util::ParseError::ExtraToken { token } => {
                let (line, column) = Self::location_to_line_col(input, token.0);
                Self::ExtraToken {
                    line,
                    column,
                    token: token.1.to_string(),
                }
            }
            lalrpop_util::ParseError::User { error } => {
                Self::UnexpectedToken {
                    line: 0,
                    column: 0,
                    message: error.to_string(),
                }
            }
        }
    }

    fn location_to_line_col(input: &str, location: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in input.chars().enumerate() {
            if i >= location {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    fn get_token_at(input: &str, location: usize) -> String {
        input
            .chars()
            .skip(location)
            .take(10)
            .collect::<String>()
            .trim()
            .to_string()
    }
}
