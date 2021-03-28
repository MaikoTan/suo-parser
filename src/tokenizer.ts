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
  | UnknownToken;

const keywords = [
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
] as const;

export type Keyword = typeof keywords[number];

export class Tokenizer {
  sourceCode: string;
  line: number;
  column: number;
  index: number;

  /**
   * cache the next token
   */
  private currentToken: Token | null;

  constructor(sourceCode: string) {
    this.sourceCode = sourceCode;
    this.line = 1;
    this.column = 0;
    this.index = 0;

    this.currentToken = null;
  }

  peekToken(): Token {
    if (this.currentToken) {
      return this.currentToken;
    }

    this.currentToken = this.nextToken();
    return this.currentToken;
  }

  nextToken(): Token {
    if (this.currentToken) {
      const token = this.currentToken;
      this.currentToken = null;
      return token;
    }

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
        raw: "",
      };
    }

    const char = this.peek();
    if (char === "#") {
      return this.parseComment();
    }

    if (this.isWhitespace(char)) {
      this.eatWhitespace();
      return this.nextToken();
    }

    if (char === ",") {
      const token: PunctuatorToken = {
        type: "Punctuator",
        value: ",",
        start: this.index,
        end: this.index + 1,
        loc: {
          start: {
            line: this.line,
            column: this.column,
          },
          end: {
            line: this.line,
            column: this.column + 1,
          },
        },
        raw: char,
      };
      this.index++;
      this.column++;
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
        loc: {
          start: {
            line: this.line,
            column: this.column,
          },
          end: {
            line: this.line,
            column: this.column + 1,
          },
        },
        raw: char,
      };
      this.index++;
      this.column++;
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
    this.column++;

    let value = "";
    let char = this.peek();
    while (char !== "\n" && char !== "") {
      if (char === "/") {
        this.index++;
        this.column++;
        break;
      }
      if (char === "\\") {
        value += "\\" + this.peek(1);
        this.index++;
        this.column++;
      }
      value += char;
      this.index++;
      this.column++;
      char = this.peek();
    }
    return {
      type: "RegularExpression",
      value,
      start,
      end: this.index,
      loc: {
        start: {
          line: this.line,
          column: start,
        },
        end: {
          line: this.line,
          column: this.column,
        },
      },
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseIdentifier(): KeywordToken | IdentifierToken | UnknownToken {
    const start = this.index;
    const columnStart = this.column;

    let char = this.peek();
    let value = "";
    while (char !== "\n" && char !== "") {
      if (char >= "a" && char <= "z") {
        value += char;
        this.index++;
        this.column++;
      } else if (char >= "A" && char <= "Z") {
        value += char;
        this.index++;
        this.column++;
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
        loc: {
          start: {
            line: this.line,
            column: columnStart,
          },
          end: {
            line: this.line,
            column: this.column,
          },
        },
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
        loc: {
          start: {
            line: this.line,
            column: columnStart,
          },
          end: {
            line: this.line,
            column: this.column,
          },
        },
        raw: this.sourceCode.substring(start, this.index),
      };
    }
    return {
      type: "Identifier",
      value,
      start,
      end: this.index,
      loc: {
        start: {
          line: this.line,
          column: columnStart,
        },
        end: {
          line: this.line,
          column: this.column,
        },
      },
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private eatWhitespace(): void {
    let char = this.peek();
    while (this.isWhitespace(char)) {
      this.index++;
      this.column++;
      if (char === "\r" && this.peek() === "\n") {
        this.line++;
        this.column = 0;
        this.index++;
      } else if (char === "\n" || char === "\r") {
        this.line++;
        this.column = 0;
      }
      char = this.peek();
    }
  }

  private parseComment(): CommentToken {
    const start = this.index;
    const columnStart = this.column;

    this.index++; // first char is "#", don't record in value
    this.column++;

    let char = this.peek();
    let value = "";
    while (char !== "\r" && char !== "\n" && char !== "") {
      value += char;
      this.index++;
      this.column++;
      char = this.peek();
    }
    return {
      type: "Comment",
      value,
      start,
      end: this.index,
      loc: {
        start: {
          line: this.line,
          column: columnStart,
        },
        end: {
          line: this.line,
          column: this.column,
        },
      },
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseNumericLiteral(): NumericLiteralToken {
    const start = this.index;
    const columnStart = this.column;

    let char = this.peek();
    let value = "";
    while (char !== "\n" && char !== "") {
      if (char === ".") {
        value += char;
        this.index++;
        this.column++;
      } else if (char >= "0" && char <= "9") {
        value += char;
        this.index++;
        this.column++;
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
      loc: {
        start: {
          line: this.line,
          column: columnStart,
        },
        end: {
          line: this.line,
          column: this.column,
        },
      },
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  private parseStringLiteral(): StringLiteralToken {
    const start = this.index;
    const lineStart = this.line;
    const columnStart = this.column;

    let singleQuote = false;
    let char = this.peek();

    if (char === "'") {
      singleQuote = true;
    }
    this.index++;
    this.column++;

    let value = "";
    while (char !== "\n" && char !== "") {
      char = this.peek();
      this.index++;
      this.column++;
      // escape with backslash
      if (char === "\\") {
        const next = this.peek(1);
        if (next === "\n") {
          this.index++;
          this.column++;
          continue;
        }
        if (next === "'" || next === '"') {
          value += next;
          this.index++;
          this.column++;
          continue;
        }
        this.index++;
        this.column++;
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
      loc: {
        start: {
          line: lineStart,
          column: columnStart,
        },
        end: {
          line: this.line,
          column: this.column,
        },
      },
      raw: this.sourceCode.substring(start, this.index),
    };
  }

  hasNextToken(): boolean {
    return (this.currentToken !== null && this.currentToken.type !== "EOF") || this.index < this.sourceCode.length;
  }

  /**
   * This is a helper function to get the next character without consuming it.
   */
  peek(next?: number): string {
    if (next) {
      return this.sourceCode.charAt(this.index + next);
    }
    return this.sourceCode.charAt(this.index);
  }

  /**
   * This function would move the cursor forward
   */
  next(): string {
    return this.sourceCode.charAt(++this.index);
  }

  private isWhitespace(char: string): boolean {
    return char === "\n" || char === "\r" || char === "\t" || char === " ";
  }
}
