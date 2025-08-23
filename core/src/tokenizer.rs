use std::io::{BufRead, BufReader, Read};

use super::utils::location::{Position, SourceLocation};
use super::utils::log_types::NetSyncLogType;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Including keywords, NetSyncLogType variants
    Keyword,
    /// Double-quoted string literal
    StringLiteral,
    /// Numeric literal (integer and float)
    NumericLiteral,
    /// Regular expression literal (without /)
    RegularExpression,
    /// ","
    Punctuator,
    /// ":"
    Colon,
    /// "{" or "}"
    Brace,
    /// Any identifier (variable names, function names, etc.)
    Identifier,
    /// Single-line comment
    Comment,
    /// Whitespace (spaces, tabs, newlines)
    Whitespace,
    /// Unknown token
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub loc: SourceLocation,
    pub range: (u32, u32),
}

pub enum Keyword {
    Sync,
    Window,
    Jump,
    Duration,
    HideAll,
    AlertAll,
    Before,
    Sound,
    Define,
    InfoText,
    AlertText,
    AlarmText,
}

pub struct Tokenizer<R: Read> {
    reader: BufReader<R>,
    position: u32,
    line: u16,
    column: u16,

    pub current_token: Option<Token>,
}

impl<R: Read> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            position: 0,
            line: 1,
            column: 1,
            current_token: None,
        }
    }
}

impl<R: Read> Tokenizer<R> {
    pub fn position(&mut self) -> Position {
        Position::new(self.line, self.column, self.position)
    }

    pub fn peek_chars(&mut self, size: usize) -> Result<&[u8], std::io::Error> {
        self.reader.peek(size)
    }

    pub fn read_chars(&mut self, size: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0; size];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn has_next_token(&mut self) -> bool {
        self.current_token.is_some() || self.reader.has_data_left().ok().unwrap()
    }

    pub fn peek_token(&mut self) -> Option<Token> {
        if let Some(token) = &self.current_token {
            return Some(token.clone());
        }

        self.current_token = self.next_token();
        self.current_token.clone()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.current_token.take() {
            return Some(token);
        }

        self.skip_whitespace();

        if !self.reader.has_data_left().ok().unwrap() {
            return None;
        }

        let start = self.position();
        let char = self.peek_chars(1).unwrap()[0];

        let token = if char.is_ascii_alphabetic() {
            self.consume_keyword()
        } else if char.is_ascii_digit() {
            self.consume_numeric_literal()
        } else if char == b'"' || char == b'\'' {
            self.consume_string_literal()
        } else if char == b'#' {
            self.consume_comment()
        } else if char == b'/' {
            self.consume_regular_expression()
        } else if char == b':' {
            self.position += 1;
            self.column += 1;
            self.read_chars(1).unwrap(); // Consume the ':'
            Token {
                kind: TokenKind::Colon,
                value: Some(":".to_string()),
                loc: SourceLocation::new(start.clone(), self.position()),
                range: (start.offset, self.position),
            }
        } else if char == b'{' || char == b'}' {
            self.position += 1;
            self.column += 1;
            self.read_chars(1).unwrap(); // Consume the '{' or '}'
            Token {
                kind: TokenKind::Brace,
                value: Some(char as char).map(|c| c.to_string()),
                loc: SourceLocation::new(start.clone(), self.position()),
                range: (start.offset, self.position),
            }
        } else if char == b',' {
            self.position += 1;
            self.column += 1;
            self.read_chars(1).unwrap(); // Consume the ','
            Token {
                kind: TokenKind::Punctuator,
                value: Some(",".to_string()),
                loc: SourceLocation::new(start.clone(), self.position()),
                range: (start.offset, self.position),
            }
        } else {
            self.consume_unknown()
        };

        let end = self.position();

        Some(Token {
            kind: token.kind,
            value: token.value,
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.clone().offset, self.position),
        })
    }

    fn skip_whitespace(&mut self) {
        while self.reader.has_data_left().ok().unwrap()
            && self.peek_chars(1).unwrap()[0].is_ascii_whitespace()
        {
            let char = self.peek_chars(1).unwrap()[0];
            if char == b'\n' {
                self.line += 1;
                self.column = 1;
            } else if char == b'\r' {
                if self.peek_chars(2).unwrap()[1] == b'\n' {
                    self.read_chars(1).unwrap(); // Consume the '\r'
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.line += 1;
                }
            } else {
                self.column += 1;
            }
            self.position += 1;

            self.read_chars(1).unwrap(); // Consume the whitespace character
        }
    }

    fn consume_keyword(&mut self) -> Token {
        let keywords = [
            "label",
            "forcejump",
            "sync",
            "window",
            "jump",
            "duration",
            "hideall",
            "alertall",
            "before",
            "sound",
            "define",
            "infotext",
            "alerttext",
            "alarmtext",
        ];

        for keyword in keywords.iter() {
            if self.peek_chars(keyword.len()).unwrap() == keyword.as_bytes() {
                let start = self.position();
                self.position += keyword.len() as u32;
                self.column += keyword.len() as u16;

                let end = self.position();

                self.read_chars(keyword.len()).unwrap(); // Consume the keyword

                return Token {
                    kind: TokenKind::Keyword,
                    value: Some(keyword.to_string()),
                    loc: SourceLocation::new(start.clone(), end.clone()),
                    range: (start.offset, end.offset),
                };
            }
        }

        // If no keyword matches, try parsing it as NetSyncLogType
        for key in NetSyncLogType::all_keys() {
            if self.peek_chars(key.len()).unwrap() == key.as_bytes() {
                let start = self.position();
                self.position += key.len() as u32;
                self.column += key.len() as u16;

                let end = self.position();

                self.read_chars(key.len()).unwrap(); // Consume the keyword

                return Token {
                    kind: TokenKind::Keyword,
                    value: Some(key),
                    loc: SourceLocation::new(start.clone(), end.clone()),
                    range: (start.offset, end.offset),
                };
            }
        }

        let start = self.position();
        {
            // Otherwise, return an identifier token
            let mut name = String::new();
            while self.reader.has_data_left().ok().unwrap() {
                let char = self.peek_chars(1).unwrap()[0];
                if char.is_ascii_alphanumeric() || char == b'_' {
                    name.push(self.read_chars(1).unwrap()[0] as char);
                    self.position += 1;
                    self.column += 1;
                } else {
                    break;
                }
            }

            Token {
                kind: TokenKind::Identifier,
                value: Some(name),
                loc: SourceLocation::new(start.clone(), self.position()),
                range: (start.offset, self.position().offset),
            }
        }
    }

    fn consume_numeric_literal(&mut self) -> Token {
        let start = self.position();
        let mut end = start.clone();
        let mut value = String::new();

        while self.reader.has_data_left().ok().unwrap() {
            let current_char = self.peek_chars(1).unwrap()[0] as char;

            if current_char.is_ascii_digit() || current_char == '.' {
                self.read_chars(1).unwrap();
                value.push(current_char);
                self.position += 1;
                self.column += 1;
                end = self.position();
            } else {
                break;
            }
        }

        Token {
            kind: TokenKind::NumericLiteral,
            value: Some(value),
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
        }
    }

    fn consume_string_literal(&mut self) -> Token {
        let start = self.position();
        let mut end = start.clone();
        let mut value = String::new();
        let quote_char = self.read_chars(1).unwrap()[0] as char; // Either '"' or '\''

        // Ensure the first character is a quote
        if quote_char != '"' && quote_char != '\'' {
            return Token {
                kind: TokenKind::Unknown,
                value: None,
                loc: SourceLocation::new(start.clone(), start.clone()),
                range: (start.offset, start.offset),
            };
        }

        self.position += 1; // Consume the opening quote
        self.column += 1;

        while self.reader.has_data_left().ok().unwrap() {
            let current_char = self.read_chars(1).unwrap()[0] as char;

            if current_char == quote_char {
                // Closing quote found
                self.position += 1;
                self.column += 1;
                end = self.position();
                break;
            } else if current_char == '\\' {
                // Handle escape sequences
                self.position += 1;
                self.column += 1;

                if self.reader.has_data_left().ok().unwrap() {
                    let escaped_char = self.read_chars(1).unwrap()[0] as char;
                    match escaped_char {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        '\'' => value.push('\''),
                        _ => value.push(escaped_char), // Unknown escape sequence
                    }
                    self.position += 1;
                    self.column += 1;
                }
            } else {
                // Regular character
                value.push(current_char);
                self.position += 1;
                self.column += 1;
            }
        }

        Token {
            kind: TokenKind::StringLiteral,
            value: Some(value),
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
        }
    }

    fn consume_regular_expression(&mut self) -> Token {
        let start = self.position();
        let mut end = start.clone();
        let mut value = String::new();

        if self.peek_chars(1).unwrap()[0] != b'/' {
            return self.consume_unknown();
        }

        self.read_chars(1).unwrap(); // Consume the opening '/'
        self.position += 1;
        self.column += 1;

        let mut escaped = false;
        while self.reader.has_data_left().ok().unwrap() {
            let current_char = self.peek_chars(1).unwrap()[0] as char;

            if escaped {
                value.push(current_char);
                self.read_chars(1).unwrap(); // Consume the escaped character
                escaped = false;
            } else if current_char == '\\' {
                value.push(current_char);
                self.read_chars(1).unwrap(); // Consume the backslash
                escaped = true;
            } else if current_char == '/' {
                // Closing slash found
                self.read_chars(1).unwrap(); // Consume the closing '/'
                self.position += 1;
                self.column += 1;
                end = self.position();
                break;
            } else {
                self.read_chars(1).unwrap(); // Consume the regular character
                value.push(current_char);
            }

            self.position += 1;
            self.column += 1;
            end = self.position();
        }

        Token {
            kind: TokenKind::RegularExpression,
            value: Some(value),
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
        }
    }

    fn consume_comment(&mut self) -> Token {
        let start = self.position();
        let mut end = start.clone();
        let mut value = String::new();

        while self.reader.has_data_left().ok().unwrap() {
            let current_char = self.peek_chars(1).unwrap()[0] as char;

            if current_char == '\n' || current_char == '\r' {
                break;
            } else {
                self.read_chars(1).unwrap(); // Consume the character
                value.push(current_char);
                self.position += 1;
                self.column += 1;
                end = self.position();
            }
        }

        Token {
            kind: TokenKind::Comment,
            value: Some(value),
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
        }
    }

    fn consume_unknown(&mut self) -> Token {
        let start = self.position();
        let current_char = self.peek_chars(1).unwrap()[0] as char;

        self.position += 1;
        self.column += 1;
        self.read_chars(1).unwrap(); // Consume the character

        let end = self.position();

        Token {
            kind: TokenKind::Unknown,
            value: Some(current_char.to_string()),
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
        }
    }
}
