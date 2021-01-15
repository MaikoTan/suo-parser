export interface BaseToken {
  type: string;
  value?: string;
  start: number;
  end: number;
  raw: string;
}

export interface EOFToken extends BaseToken {
  type: "EOF";
}
export interface KeywordToken extends BaseToken {
  type: "Keyword";
  value: string;
}
export interface IdentifierToken extends BaseToken {
  type: "Identifier";
  value: string;
}
export interface StringLiteralToken extends BaseToken {
  type: "StringLiteral";
  value: string;
}
export interface NumericLiteralToken extends BaseToken {
  type: "NumericLiteral";
  value: string;
}
export interface OperatorToken extends BaseToken {
  type: "Operator";
  value: string;
}
export interface PunctuatorToken extends BaseToken {
  type: "Punctuator";
  value: string;
}
export interface CommentToken extends BaseToken {
  type: "Comment";
  value: string;
}
export interface RegularExpressionToken extends BaseToken {
  type: "RegularExpression";
  value: string;
}
export interface WhitespaceToken extends BaseToken {
  type: "Whitespace";
}
export interface NewLineToken extends BaseToken {
  type: "NewLine";
}
export interface UnknownToken extends BaseToken {
  type: "Unknown";
  value: string;
}

export type Token =
  | EOFToken
  | KeywordToken
  | IdentifierToken
  | StringLiteralToken
  | NumericLiteralToken
  | OperatorToken
  | PunctuatorToken
  | CommentToken
  | RegularExpressionToken
  | WhitespaceToken
  | NewLineToken
  | UnknownToken;

const keywords = ["sync", "window", "jump", "duration", "hideall"] as const;

export type Keyword = typeof keywords[number];

export class Tokenizer {
  sourceCode: string;
  line: number;
  column: number;
  index: number;

  constructor(sourceCode: string) {
    this.sourceCode = sourceCode;
    this.line = 1;
    this.column = 0;
    this.index = 0;
  }

  next(): Token {
    if (this.index >= this.sourceCode.length) {
      return {
        type: "EOF",
        start: this.index,
        end: this.index,
        raw: this.sourceCode.substring(this.index),
      };
    }

    const char = this.peek();
    if (char === "#") {
      return this.parseComment();
    }
    if (this.isLineBreak(char)) {
      let raw = char;
      // Linux: "\n"  Windows: "\r\n"  Mac: "\r"
      if (char === "\r" && this.peek(1) === "\n")
        raw += this.peek(1);

      const token: NewLineToken = {
        type: "NewLine",
        start: this.index,
        end: this.index + raw.length,
        raw: raw,
      };
      this.index += raw.length;
      this.line++;
      this.column = 0;
      return token;
    }
    if (char === " " || char === "\t") {
      return this.parseWhitespace();
    }
    if (char === ",") {
      const token: PunctuatorToken = {
        type: "Punctuator",
        value: ",",
        start: this.index,
        end: this.index + 1,
        raw: char,
      };
      this.index++;
      return token;
    }
    if (char === "'" || char === '"') {
      return this.parseStringLiteral();
    }
    if (char === ".") {
      const next = this.peek(1);
      if (next >= "0" && next <= "9") {
        return this.parseNumericLiteral();
      }
      const token: UnknownToken = {
        type: "Unknown",
        start: this.index,
        end: this.index + 1,
        value: char,
        raw: char,
      };
      this.index++;
      return token;
    }
    if (char >= "0" && char <= "9") {
      return this.parseNumericLiteral();
    }

    // "/" in timeline only refer to a regexp now
    if (char === "/") {
      return this.parseRegularExpression();
    }

    return this.parseIdentifier();
  }

  private parseRegularExpression(): RegularExpressionToken {
    const start = this.index;
    this.index++; // skip "/"
    let value = "";
    let char = this.peek();
    while (char !== "\n" && char !== "") {
      if (char === "/") {
        this.index++;
        break;
      }
      if (char === "\\") {
        value += "\\" + this.peek(1);
        this.index++;
      }
      value += char;
      this.index++;
      char = this.peek();
    }
    return {
      type: "RegularExpression",
      value,
      start,
      end: this.index,
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseIdentifier(): KeywordToken | IdentifierToken | UnknownToken {
    const start = this.index;
    let char = this.peek();
    let value = "";
    while (char !== "\n" && char !== "") {
      if (char >= "a" && char <= "z") {
        value += char;
        this.index++;
      } else if (char >= "A" && char <= "Z") {
        value += char;
        this.index++;
      } else {
        break;
      }
      char = this.peek();
    }

    if (value === "") {
      return {
        type: "Unknown",
        value,
        start,
        end: this.index,
        raw: this.sourceCode.substring(start, this.index),
      };
    }
    // TODO: don't know why `includes` check function is strict typed
    if ((keywords as readonly string[]).includes(value)) {
      return {
        type: "Keyword",
        value,
        start,
        end: this.index,
        raw: this.sourceCode.substring(start, this.index),
      };
    }
    return {
      type: "Identifier",
      value,
      start,
      end: this.index,
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseWhitespace(): WhitespaceToken {
    const start = this.index;
    let char = this.peek();
    while (char === " " || char === "\t") {
      this.index++;
      char = this.peek();
    }
    return {
      type: "Whitespace",
      start,
      end: this.index,
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseComment(): CommentToken {
    const start = this.index;
    this.index++; // first char is "#", don't record in value
    let char = this.peek();
    let value = "";
    while (char !== "\r" && char !== "\n" && char !== "") {
      value += char;
      this.index++;
      this.line++;
      char = this.peek();
    }
    return {
      type: "Comment",
      value,
      start,
      end: this.index,
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseNumericLiteral(): NumericLiteralToken {
    const start = this.index;
    let char = this.peek();
    let value = "";
    while (char !== "\n" && char !== "") {
      if (char === ".") {
        value += char;
        this.index++;
      } else if (char >= "0" && char <= "9") {
        value += char;
        this.index++;
      } else {
        break;
      }
      char = this.peek();
    }
    return {
      type: "NumericLiteral",
      value,
      start,
      end: this.index,
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseStringLiteral(): StringLiteralToken {
    const start = this.index;
    let singleQuote = false;
    let char = this.peek();

    if (char === "'") {
      singleQuote = true;
    }
    this.index++;
    
    let value = "";
    while (char !== "\n" && char !== "") {
      char = this.peek();
      this.index++;
      // escape with backslash
      if (char === "\\") {
        const next = this.peek(1);
        if (next === "\n") {
          this.index++;
          continue;
        }
        if (next === "'" || next === '"') {
          value += next;
          this.index++;
          continue;
        }
        this.index++;
      }

      // allow non-escaped single quotes in double-quoted strings
      if (char === "'") {
        if (singleQuote) {
          break;
        }
        value += "'";
        continue;
      }

      // allow non-escaped double quotes in single-quoted strings
      if (char === '"') {
        if (!singleQuote) {
          break;
        }
        value += '"';
        continue;
      }

      value += char;
    }
    return {
      type: "StringLiteral",
      value,
      start,
      end: this.index,
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  hasNextToken(): boolean {
    return this.index < this.sourceCode.length;
  }

  private peek(next?: number): string {
    if (next) {
      return this.sourceCode.charAt(this.index + next);
    }
    return this.sourceCode.charAt(this.index);
  }

  private isLineBreak(char: string): boolean {
    return char === "\n" || char === "\r";
  }
}
