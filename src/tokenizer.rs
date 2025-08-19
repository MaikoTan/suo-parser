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
    pub range: (usize, usize),
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

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    line: u32,
    column: u32,

    pub current_token: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
            current_token: None,
        }
    }

    pub fn position(&mut self) -> Position {
        Position::new(self.line, self.column, self.position)
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

        if self.position >= self.input.len() {
            return None;
        }

        let start = self.position();
        let char = self.input.as_bytes()[self.position];

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
            Token {
                kind: TokenKind::Colon,
                value: Some(":".to_string()),
                loc: SourceLocation::new(start.clone(), self.position()),
                range: (start.offset, self.position),
            }
        } else if char == b'{' || char == b'}' {
            self.position += 1;
            self.column += 1;
            Token {
                kind: TokenKind::Brace,
                value: Some(char as char).map(|c| c.to_string()),
                loc: SourceLocation::new(start.clone(), self.position()),
                range: (start.offset, self.position),
            }
        } else if char == b',' {
            self.position += 1;
            self.column += 1;
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
        while self.position < self.input.len()
            && self.input[self.position..].starts_with(|c: char| c.is_whitespace())
        {
            if self.input[self.position..].starts_with('\n') {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
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
            if self.input[self.position..].starts_with(keyword) {
                let start = self.position();
                self.position += keyword.len();
                self.column += keyword.len() as u32;

                let end = self.position();

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
            if self.input[self.position..].starts_with(&key) {
                let start = self.position();
                self.position += key.len();
                self.column += key.len() as u32;

                let end = self.position();

                return Token {
                    kind: TokenKind::Keyword,
                    value: Some(key),
                    loc: SourceLocation::new(start.clone(), end.clone()),
                    range: (start.offset, end.offset),
                };
            }
        }

        // Otherwise, return an identifier token
        let start = self.position();
        while self.position < self.input.len()
            && self.input[self.position..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
        {
            self.position += 1;
            self.column += 1;
        }

        Token {
            kind: TokenKind::Identifier,
            value: Some(self.input[start.offset..self.position].to_string()),
            loc: SourceLocation::new(start.clone(), self.position()),
            range: (start.offset, self.position().offset),
        }
    }

    fn consume_numeric_literal(&mut self) -> Token {
        let start = self.position();
        let mut end = start.clone();
        let mut value = String::new();

        while self.position < self.input.len() {
            let current_char = self.input.as_bytes()[self.position] as char;

            if current_char.is_ascii_digit() || current_char == '.' {
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
        let quote_char = self.input.as_bytes()[self.position] as char; // Either '"' or '\''

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

        while self.position < self.input.len() {
            let current_char = self.input.as_bytes()[self.position] as char;

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

                if self.position < self.input.len() {
                    let escaped_char = self.input.as_bytes()[self.position] as char;
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

        if self.input.as_bytes()[self.position] != b'/' {
            return self.consume_unknown();
        }

        self.position += 1; // Consume the opening '/'
        self.column += 1;

        let mut escaped = false;
        while self.position < self.input.len() {
            let current_char = self.input.as_bytes()[self.position] as char;

            if escaped {
                value.push(current_char);
                escaped = false;
            } else if current_char == '\\' {
                value.push(current_char);
                escaped = true;
            } else if current_char == '/' {
                // Closing slash found
                self.position += 1;
                self.column += 1;
                end = self.position();
                break;
            } else {
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

        while self.position < self.input.len() {
            let current_char = self.input.as_bytes()[self.position] as char;

            if current_char == '\n' {
                break;
            } else {
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
        let current_char = self.input.as_bytes()[self.position] as char;

        self.position += 1; // Consume the character
        self.column += 1;

        let end = self.position();

        Token {
            kind: TokenKind::Unknown,
            value: Some(current_char.to_string()),
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenKind, Tokenizer};
    use insta;

    fn all_tokens(input: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next_token() {
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_tokenizer_string() {
        let vec = all_tokens("\"name\"");

        insta::assert_debug_snapshot!("Quoted String", vec);
    }

    #[test]
    fn test_tokenizer_simple_entry() {
        let vec = all_tokens("100 \"name\"");

        insta::assert_debug_snapshot!("Simple Entry", vec);
    }

    #[test]
    fn test_tokenizer_string_with_escape() {
        let vec = all_tokens("\"\\\"\"");

        insta::assert_debug_snapshot!("Escaped String", vec);
    }

    #[test]
    fn test_tokenizer_string_mixup() {
        let vec = all_tokens("\"I'm\" 'str\"ing'");

        insta::assert_debug_snapshot!("String Mixup", vec);
    }

    #[test]
    fn test_tokenizer_timeline_entry_with_comment() {
        let vec = all_tokens("10.0 \"name\" # comment");

        insta::assert_debug_snapshot!("Timeline Entry with Comment", vec);
    }

    #[test]
    fn test_tokenizer_sync_command_with_regex() {
        let vec = all_tokens("sync /regexp/");

        insta::assert_debug_snapshot!("Sync Command with Regex", vec);
    }

    #[test]
    fn test_tokenizer_sync_netsync_command() {
        let vec = all_tokens("Ability { id: \"1000\", name: \"name\" }");

        insta::assert_debug_snapshot!("Sync Netsync Command", vec);
    }

    #[test]
    fn test_tokenizer_window_command() {
        let vec = all_tokens("window 10.0\nwindow 1,1.0");

        insta::assert_debug_snapshot!("Window Command", vec);
    }

    #[test]
    fn test_tokenizer_jump_command() {
        let vec = all_tokens("jump 10.0\njump 10");

        insta::assert_debug_snapshot!("Jump Command", vec);
    }

    #[test]
    fn test_tokenizer_full_timeline() {
        // Taken from https://github.com/OverlayPlugin/cactbot/blob/main/ui/raidboss/data/00-misc/test.txt
        let vec = all_tokens(r#"# I am but a wee little test timeline
#
# Teleport your way to Summerford Farms, ancestral home of striking dummies.
#
# Make sure emotes show up in your log, such that /bow generates a line such
# as "You bow courteously to the striking dummy." Once that is turned on,
# /bow to the striking dummy to start the timeline.
#
# Alternatively, do a countdown, such as "/countdown 5" which will also
# start it.
#
# /poke, /pysch, or /laugh at the striking dummy for trigger examples.
#
# /goodbye to the striking dummy to stop the timeline.

hideall "--Reset--"
hideall "--sync--"

0 "--Reset--" GameLog { code: "001D", line: "You bid farewell to the striking dummy.*?" } window 0,10000 jump 0

# two examples with different quoting which should both be supported
0 "--sync--" GameLog { line: 'testNetRegexTimeline' } window 100000,100000
0 "--sync--" GameLog { "line": "testNetRegexTimeline" } window 100000,100000

0 "--sync--" GameLog { code: "0039", line: "Engage!.*?" } window 100000,100000
0 "--sync--" GameLog { code: "001D", line: "You bow courteously to the striking dummy.*?" } window 0,1
3 "Almagest"
6 "Angry Dummy"
10 "Long Castbar" duration 10
15 "Final Sting"
18 "Pentacle Sac (DPS)"
25 "Super Tankbuster" GameLog { code: "0038", line: "test sync1.*?" } window 30,30
30 "Dummy Stands Still"
40 "Death"

50 "--sync--" GameLog { code: "0038", line: "test sync2.*?" } window 100,1 forcejump "loop"

# Loop test!
100 label "loop"
102 "Two"
103 label "three"
103 "Three"
104 "Four"
106 "Six"
110 "Ten" #duration 100
115 "Fifteen"
118 "Force Jump Three" GameLog { code: "0038", line: "test sync3.*?" } window 10,10 forcejump "three"
120 "Invisible" GameLog { code: "0038", line: "test sync4.*?" } forcejump 1000
"#);

        insta::assert_debug_snapshot!("Full Timeline", vec);
    }

    #[test]
    fn test_tokenizer_peek_token() {
        let mut tokenizer = Tokenizer::new("0 \"test\" sync /regex/");
        let v = {
            let mut vec = Vec::new();
            // NumericLiteral
            vec.push(tokenizer.peek_token().unwrap());
            vec.push(tokenizer.peek_token().unwrap());
            vec.push(tokenizer.next_token().unwrap());
            // StringLiteral
            vec.push(tokenizer.peek_token().unwrap());
            vec.push(tokenizer.next_token().unwrap());
            // Keyword
            vec.push(tokenizer.peek_token().unwrap());
            vec.push(tokenizer.next_token().unwrap());
            // RegexLiteral
            vec.push(tokenizer.peek_token().unwrap());
            vec.push(tokenizer.next_token().unwrap());

            vec
        };

        insta::assert_debug_snapshot!("Peek Tokens", v);
    }
}
