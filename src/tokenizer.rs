use super::utils::location::{Position, SourceLocation};
use super::utils::log_types::NetSyncLogType;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  EOF,
  Whitespace,
  Keyword,
  Identifier,
  StringLiteral,
  NumericLiteral,
  RegularExpression,
  Operator,
  Punctuator,
  Brace,
  Comment,
  /// Unknown token
  Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub type_: TokenType,
  pub value: Option<String>,
  pub start: u32,
  pub end: u32,
  pub loc: SourceLocation,
  pub raw: String,
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

pub struct Tokenizer {
  source_code: String,
  line: u32,
  column: u32,
  index: u32,

  /// cache the next token
  current_token: Option<Token>,
}

impl Tokenizer {
  pub fn new(source_code: String) -> Self {
    let source_code = source_code
      // strip any UTF-8 BOM off of the start of `str`, if it exists.
      .replace("\u{feff}", "")
      // replace all line terminators with `\n`
      .replace("\r\n", "\n")
      .replace("\r", "\n");
    Tokenizer {
      source_code,
      line: 1,
      column: 0,
      index: 0,
      current_token: None,
    }
  }

  pub fn peek_token(&mut self) -> Token {
    if let Some(token) = &self.current_token {
      return token.clone();
    }

    self.current_token = Some(self.next_token());
    self.current_token.clone().unwrap()
  }

  pub fn next_token(&mut self) -> Token {
    let token = self.next_token_with_white_spaces();
    if token.type_ == TokenType::Whitespace {
      return self.next_token();
    }
    token
  }

  pub fn next_token_with_white_spaces(&mut self) -> Token {
    if let Some(token) = &self.current_token {
      let token = token.clone();
      self.current_token = None;
      return token;
    }

    if self.index >= self.source_code.len() as u32 {
      return Token {
        type_: TokenType::EOF,
        value: None,
        start: self.index,
        end: self.index,
        loc: SourceLocation::new(Position::new(self.line, self.column), Position::new(self.line, self.column)),
        raw: "".to_string(),
      };
    }

    return Token {
      type_: TokenType::Unknown,
      value: None,
      start: self.index,
      end: self.index,
      loc: SourceLocation::new(Position::new(self.line, self.column), Position::new(self.line, self.column)),
      raw: self.source_code[self.index as usize..].to_string(),
    };
  }

  pub fn has_next_token(&self) -> bool {
    (self.current_token.is_some() && self.current_token.as_ref().unwrap().type_ != TokenType::EOF)
      || self.index < self.source_code.len() as u32
  }

  pub fn peek(&self, next: Option<u32>) -> char {
    if let Some(next) = next {
      return self.source_code.chars().nth((self.index + next) as usize).unwrap();
    }
    self.source_code.chars().nth(self.index as usize).unwrap()
  }

  pub fn next(&mut self) -> char {
    self.index += 1;
    self.source_code.chars().nth(self.index as usize).unwrap()
  }

  pub fn all_tokens(&mut self) -> impl Iterator<Item = Token> + '_ {
    if self.line != 1 || self.column != 0 || self.index != 0 {
      panic!("Tokenizer is not at the beginning of the source code");
    }
    (0..).map(move |_| self.next_token_with_white_spaces())
  }
}

#[cfg(test)]
mod tests {
  use super::{Tokenizer, TokenType};

  #[test]
  fn test_tokenizer_simple_entry() {
    let input = "100 \"name\"";
    let mut tokens = Tokenizer::new(input.to_string());
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::NumericLiteral);
    assert_eq!(next.value, Some("100".to_string()));
    assert_eq!(next.start, 0);
    assert_eq!(next.end, 3);
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("name".to_string()));
    assert_eq!(next.start, 4);
    assert_eq!(next.end, 10);
    let next = tokens.next_token(); // EOF
    assert_eq!(next.type_, TokenType::EOF);
  }

  #[test]
  fn test_tokenizer_string_with_escape() {
    let input = "\"\\\"\"";
    let mut tokens = Tokenizer::new(input.to_string());
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("\"".to_string()));
  }

  #[test]
  fn test_tokenizer_string_mixup() {
    let input = "\"I'm\" 'str\"ing'";
    let mut tokens = Tokenizer::new(input.to_string());
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("I'm".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("str\"ing".to_string()));
  }

  #[test]
  fn test_tokenizer_timeline_entry_with_comment() {
    let input = "10.0 \"name\" # comment";
    let mut tokens = Tokenizer::new(input.to_string());
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::NumericLiteral);
    assert_eq!(next.value, Some("10.0".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("name".to_string()));
    let next = tokens.next_token(); // comment
    assert_eq!(next.type_, TokenType::Comment);
    assert_eq!(next.value, Some(" comment".to_string()));
    assert_eq!(next.raw, "# comment");
  }

  #[test]
  fn test_tokenizer_sync_command_with_regex() {
    let input = "sync /regexp/";
    let mut tokens = Tokenizer::new(input.to_string());
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Keyword);
    assert_eq!(next.value, Some("sync".to_string()));
    assert_eq!(next.start, 0);
    assert_eq!(next.end, 4);
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::RegularExpression);
    assert_eq!(next.value, Some("regexp".to_string()));
    assert_eq!(next.raw, "/regexp/");
    assert_eq!(next.start, 5);
    assert_eq!(next.end, 13);
  }

  #[test]
  fn test_tokenizer_sync_netsync_command() {
    let input = "Ability { id: \"1000\", name: \"name\" }";
    let mut tokens = Tokenizer::new(input.to_string());
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Keyword);
    assert_eq!(next.value, Some("Ability".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Brace);
    assert_eq!(next.value, Some("{".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Identifier);
    assert_eq!(next.value, Some("id".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Punctuator);
    assert_eq!(next.value, Some(":".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("1000".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Punctuator);
    assert_eq!(next.value, Some(",".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Identifier);
    assert_eq!(next.value, Some("name".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::Punctuator);
    assert_eq!(next.value, Some(":".to_string()));
    let next = tokens.next_token();
    assert_eq!(next.type_, TokenType::StringLiteral);
    assert_eq!(next.value, Some("name".to_string()));
  }

  #[test]
  fn test_tokenizer_window_command() {
    let input = "window 10.0\nwindow 1,1";
    let mut tokens = Tokenizer::new(input.to_string());
    let token_types: Vec<_> = tokens.all_tokens().map(|token| token.type_).collect();
    assert_eq!(
      token_types,
      vec![
        TokenType::Keyword,
        TokenType::Whitespace,
        TokenType::NumericLiteral,
        TokenType::Whitespace,
        TokenType::Keyword,
        TokenType::Whitespace,
        TokenType::NumericLiteral,
        TokenType::Punctuator,
        TokenType::NumericLiteral,
      ]
    );
  }

  #[test]
  fn test_tokenizer_jump_command() {
    let input = "jump 10.0\njump 10";
    let mut tokens = Tokenizer::new(input.to_string());
    let token_types: Vec<TokenType> = tokens.all_tokens().map(|token| token.type_).collect();
    assert_eq!(
      token_types,
      vec![
        TokenType::Keyword,
        TokenType::Whitespace,
        TokenType::NumericLiteral,
        TokenType::Whitespace,
        TokenType::Keyword,
        TokenType::Whitespace,
        TokenType::NumericLiteral,
      ]
    );
  }
}
