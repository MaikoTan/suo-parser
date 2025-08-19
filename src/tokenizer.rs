use super::utils::location::{Position, SourceLocation};
use super::utils::log_types::NetSyncLogType;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Keyword,
    StringLiteral,
    NumericLiteral,
    RegularExpression,
    Punctuator,
    Comment,
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

// const keywords = [
//   "sync",
//   "window",
//   "jump",
//   "duration",
//   "hideall",
//   "alertall",
//   "before",
//   "sound",
//   "define",
//   "infotext",
//   "alerttext",
//   "alarmtext",
// ] as const;

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

// export type Keyword = typeof keywords[number];

// export const Spec: [
//   regex: RegExp,
//   tokenType: Token["type"],
//   transformer: (code: string, matches: RegExpExecArray) => string,
// ][] = [
//   [/^\s+/, "Whitespace", (_code, matches) => matches[0]],
//   [/^#(.*)/, "Comment", (_code, matches) => matches[1]],
//   [/^(,|:)/, "Punctuator", (_code, matches) => matches[0]],
//   [/^(\{|\})/, "Brace", (_code, matches) => matches[0]],
//   [/^([1-9]\d*(?:\.\d+)?|0?\.\d+|0)/, "NumericLiteral", (_code, matches) => matches[0]],
//   [
//     /^(".*?(?<!\\)")|('.*?(?<!\\)')/,
//     "StringLiteral",
//     (_code, matches) => {
//       return (
//         matches[0]
//           // omit quotes
//           .substring(1, matches[0].length - 1)
//           // escaped characters
//           // TODO: make a correspond map?
//           .replace('\\"', '"')
//           .replace("\\'", "'")
//           .replace("\\n", "\n")
//           .replace("\\t", "\t")
//       );
//     },
//   ],
//   [
//     /^\/((?![*+?])(?:[^\r\n\[/\\]|\\.|\[(?:[^\r\n\]\\]|\\.)*\])+)\//,
//     "RegularExpression",
//     (_code, matches) => matches[1],
//   ],
//   [new RegExp("^(" + [...netSyncLogType, ...keywords].join("|") + ")\\b"), "Keyword", (_code, matches) => matches[0]],
//   [/^\w+/, "Identifier", (_code, matches) => matches[0]],
// ];

// export class Tokenizer {
//   sourceCode: string;
//   line: number;
//   column: number;
//   index: number;

//   /**
//    * cache the next token
//    */
//   private currentToken: Token | null;

//   constructor(sourceCode: string) {
//     sourceCode = sourceCode
//       // strip any UTF-8 BOM off of the start of `str`, if it exists.
//       .replace(/^\uFEFF/, "")
//       // replace all line terminators with `\n`
//       .replace(/\r\n|\r/g, "\n");
//     this.sourceCode = sourceCode;
//     this.line = 1;
//     this.column = 0;
//     this.index = 0;

//     this.currentToken = null;
//   }

//   peekToken(): Token {
//     if (this.currentToken) {
//       return this.currentToken;
//     }

//     this.currentToken = this.nextToken();
//     return this.currentToken;
//   }

//   nextToken(): Token {
//     const token = this.nextTokenWithWhiteSpaces();
//     if (token.type === "Whitespace") {
//       return this.nextToken();
//     }
//     return token;
//   }

//   nextTokenWithWhiteSpaces(): Token {
//     if (this.currentToken) {
//       const token = this.currentToken;
//       this.currentToken = null;
//       return token;
//     }

//     if (this.index >= this.sourceCode.length) {
//       return {
//         type: "EOF",
//         start: this.index,
//         end: this.index,
//         loc: new SourceLocation(new Position(this.line, this.column), new Position(this.line, this.column)),
//         raw: "",
//       };
//     }

//     for (const [regex, kind, transformer] of Spec) {
//       const sourceCode = this.sourceCode.substring(this.index);
//       const matches = regex.exec(sourceCode);

//       if (!matches) {
//         continue;
//       }

//       const newEnd = {
//         // As benchmark says, the split method is the fastest (faster than for-loop a lot)
//         // see http://jsbench.github.io/#4cc806b4507ae063efc81900cdfb9b02
//         line: this.line + (matches[0].split("\n").length - 1),
//         column: matches[0].includes("\n")
//           ? matches[0].split("\n").pop()?.length ?? matches[0].length
//           : this.column + matches[0].length,
//       };

//       const loc = {
//         raw: matches[0],
//         start: this.index,
//         end: this.index + matches[0].length,
//         loc: new SourceLocation(new Position(this.line, this.column), newEnd),
//       };

//       this.index += matches[0].length;
//       this.line = newEnd.line;
//       this.column = newEnd.column;

//       return {
//         ...loc,
//         type: kind,
//         value: transformer(sourceCode, matches),
//       };
//     }
//     return {
//       type: "Unknown",
//       value: "",
//       start: this.index,
//       end: this.index,
//       loc: new SourceLocation(new Position(this.line, this.column), new Position(this.line, this.column)),
//       raw: this.sourceCode.substring(this.index),
//     };
//   }

//   hasNextToken(): boolean {
//     return (this.currentToken !== null && this.currentToken.type !== "EOF") || this.index < this.sourceCode.length;
//   }

//   /**
//    * This is a helper function to get the next character without consuming it.
//    */
//   peek(next?: number): string {
//     if (next) {
//       return this.sourceCode.charAt(this.index + next);
//     }
//     return this.sourceCode.charAt(this.index);
//   }

//   /**
//    * This function would move the cursor forward
//    */
//   next(): string {
//     return this.sourceCode.charAt(++this.index);
//   }

//   get allTokens(): Token[] {
//     if (this.line !== 1 || this.column !== 0 || this.index !== 0) {
//       throw new Error("Tokenizer is not at the beginning of the source code");
//     }
//     const tokens: Token[] = [];
//     while (this.hasNextToken()) {
//       tokens.push(this.nextTokenWithWhiteSpaces());
//     }
//     return tokens;
//   }
// }

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    line: u32,
    column: u32,

    current_token: Option<Token>,
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

    pub fn peek_token(&mut self) -> Token {
        if let Some(token) = &self.current_token {
            return token.clone();
        }

        self.current_token = self.next_token();
        self.current_token.clone().unwrap()
    }

    pub fn next_token(&mut self) -> Option<Token> {
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

        // If no keyword matches, return an unknown token
        let start = self.position();
        self.position += 1;
        self.column += 1;

        let end = self.position();

        Token {
            kind: TokenKind::Unknown,
            value: None,
            loc: SourceLocation::new(start.clone(), end.clone()),
            range: (start.offset, end.offset),
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

    // #[test]
    // fn test_tokenizer_sync_netsync_command() {
    //     let vec= all_tokens("Ability { id: \"1000\", name: \"name\" }");

    //     insta::assert_debug_snapshot!("Sync Netsync Command", vec);
    // }

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
}
