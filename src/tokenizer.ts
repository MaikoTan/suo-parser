export interface BaseToken {
  type: string;
  value?: string;
  start: number;
  end: number;
  loc: {
    start: {
      line: number;
      column: number;
    };
    end: {
      line: number;
      column: number;
    };
  };
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

  peek(): Token {
    if (this.index >= this.sourceCode.length) {
      return {
        type: "EOF",
        start: this.index,
        end: this.index,
        loc: {
          start: {
            line: this.line,
            column: this.column,
          },
          end: {
            line: this.line,
            column: this.column,
          },
        },
        raw: this.sourceCode.substring(this.index),
      };
    }

    let line = this.line;
    let column = this.column;

    let offset = 0;

    const char = this.peekChar(offset);
    if (char === "#") {
      return this.parseComment();
    }
    if (this.isLineBreak(char)) {
      let raw = char;
      // Linux: "\n"  Windows: "\r\n"  Mac: "\r"
      if (char === "\r" && this.peekChar(1) === "\n") raw += this.peekChar(offset + 1);

      const token: NewLineToken = {
        type: "NewLine",
        start: this.index,
        end: this.index + raw.length,
        loc: {
          start: {
            line: line,
            column: column,
          },
          end: {
            line: line,
            column: column + raw.length,
          },
        },
        raw: raw,
      };
      line++;
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
        loc: {
          start: {
            line: line,
            column: column,
          },
          end: {
            line: line,
            column: column + 1,
          },
        },
        raw: char,
      };
      return token;
    }
    if (char === "'" || char === '"') {
      return this.parseStringLiteral();
    }
    if (char === ".") {
      const next = this.peekChar(1);
      if (next >= "0" && next <= "9") {
        return this.parseNumericLiteral();
      }
      const token: UnknownToken = {
        type: "Unknown",
        start: this.index,
        end: this.index + 1,
        value: char,
        loc: {
          start: {
            line: line,
            column: column,
          },
          end: {
            line: line,
            column: column + 1,
          },
        },
        raw: char,
      };
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
    let line = this.line;
    let column = this.column;

    let offset = 1; // skip "/"

    let value = "";
    let char = this.peekChar(offset);
    while (char !== "\n" && char !== "") {
      if (char === "/") {
        offset++;
        column++;
        break;
      }
      if (char === "\\") {
        value += "\\" + this.peekChar(offset + 1);
        offset++;
        column++;
      }
      value += char;
      offset++;
      column++;
      char = this.peekChar(offset);
    }
    return {
      type: "RegularExpression",
      value,
      start,
      end: this.index + offset,
      loc: {
        start: {
          line: line,
          column: column,
        },
        end: {
          line: line,
          column: column + value.length,
        },
      },
      raw: this.sourceCode.substring(start, this.index + offset),
    };
  }

  private parseIdentifier(): KeywordToken | IdentifierToken | UnknownToken {
    const start = this.index;
    let line = this.line;
    let column = this.column;

    let offset = 0;

    let char = this.peekChar(offset);
    let value = "";
    while (char !== "\n" && char !== "") {
      if (char >= "a" && char <= "z") {
        value += char;
        offset++;
        column++;
      } else if (char >= "A" && char <= "Z") {
        value += char;
        offset++;
        column++;
      } else {
        break;
      }
      char = this.peekChar(offset);
    }

    if (value === "") {
      return {
        type: "Unknown",
        value,
        start,
        end: this.index + offset,
        loc: {
          start: {
            line: line,
            column: column,
          },
          end: {
            line: line,
            column: column + value.length,
          },
        },
        raw: this.sourceCode.substring(start, this.index + offset),
      };
    }
    // TODO: don't know why `includes` check function is strict typed
    if ((keywords as readonly string[]).includes(value)) {
      return {
        type: "Keyword",
        value,
        start,
        end: this.index + offset,
        loc: {
          start: {
            line: line,
            column: column,
          },
          end: {
            line: line,
            column: column + value.length,
          },
        },
        raw: this.sourceCode.substring(start, this.index + offset),
      };
    }

    // although there is no literal `identifier` in a timeline
    return {
      type: "Identifier",
      value,
      start,
      end: this.index + offset,
      loc: {
        start: {
          line: line,
          column: column,
        },
        end: {
          line: line,
          column: column + value.length,
        },
      },
      raw: this.sourceCode.substring(start, this.index + offset),
    };
  }

  private parseWhitespace(): WhitespaceToken {
    const start = this.index;
    const columnStart = this.column;
    let line = this.line;
    let column = this.column;

    let offset = 0;

    let char = this.peekChar(offset);
    while (char === " " || char === "\t") {
      offset++;
      column++;
      char = this.peekChar(offset);
    }
    return {
      type: "Whitespace",
      start,
      end: this.index + offset,
      loc: {
        start: {
          line: line,
          column: columnStart,
        },
        end: {
          line: line,
          column: column,
        },
      },
      raw: this.sourceCode.substring(start, this.index + offset),
    };
  }

  private parseComment(): CommentToken {
    const start = this.index;
    const columnStart = this.column;
    let line = this.line;
    let column = this.column;

    let offset = 1; // first char is "#", don't record in value

    let char = this.peekChar(offset);
    let value = "";
    while (char !== "\r" && char !== "\n" && char !== "") {
      value += char;
      offset++;
      column++;
      char = this.peekChar(offset);
    }
    return {
      type: "Comment",
      value,
      start,
      end: this.index + offset,
      loc: {
        start: {
          line: line,
          column: columnStart,
        },
        end: {
          line: line,
          column: column,
        },
      },
      raw: this.sourceCode.substring(start, this.index + offset),
    };
  }

  private parseNumericLiteral(): NumericLiteralToken {
    const start = this.index;
    const columnStart = this.column;
    let line = this.line;
    let column = this.column;

    let offset = 0;

    let char = this.peekChar(offset);
    let value = "";
    while (char !== "\n" && char !== "") {
      if (char === ".") {
        value += char;
        offset++;
        column++;
      } else if (char >= "0" && char <= "9") {
        value += char;
        offset++;
        column++;
      } else {
        break;
      }
      char = this.peekChar(offset);
    }
    return {
      type: "NumericLiteral",
      value,
      start,
      end: this.index + offset,
      loc: {
        start: {
          line: line,
          column: columnStart,
        },
        end: {
          line: line,
          column: column,
        },
      },
      raw: this.sourceCode.substring(start, this.index + offset),
    };
  }

  private parseStringLiteral(): StringLiteralToken {
    const start = this.index;
    const columnStart = this.column;
    let line = this.line;
    let column = this.column;

    let offset = 0;

    let singleQuote = false;
    let char = this.peekChar(offset);

    if (char === "'") {
      singleQuote = true;
    }
    offset++;
    column++;

    let value = "";
    while (char !== "\n" && char !== "") {
      char = this.peekChar(offset);
      offset++;
      column++;

      // escape with backslash
      if (char === "\\") {
        const next = this.peekChar(offset + 1);
        if (next === "\n") {
          line++;
          column = 0;
          offset++;
          continue;
        }
        if (next === "'" || next === '"') {
          value += next;
          offset++;
          column++;
          continue;
        }
        offset++;
        column++;
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
      end: this.index + offset,
      loc: {
        start: {
          line: line,
          column: columnStart,
        },
        end: {
          line: line,
          column: column,
        },
      },
      raw: this.sourceCode.substring(start, this.index + offset),
    };
  }

  next(): Token {
    // read next token
    const token = this.peek();
    // restore state
    this.index += token.raw.length;
    this.column = token.loc.end.column;
    this.line = token.loc.end.line;
    return token;
  }

  skip(type: Token["type"]): void {
    const token = this.peek();
    if (type === token.type) {
      this.next();
    }
  }

  hasNextToken(): boolean {
    return this.index < this.sourceCode.length;
  }

  private peekChar(next?: number): string {
    if (next) {
      return this.sourceCode.charAt(this.index + next);
    }
    return this.sourceCode.charAt(this.index);
  }

  private isLineBreak(char: string): boolean {
    return char === "\n" || char === "\r";
  }
}
