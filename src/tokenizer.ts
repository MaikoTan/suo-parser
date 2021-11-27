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

export const Spec: [
  regex: RegExp,
  tokenType: Token["type"] | null,
  transformer: ((code: string, matches: RegExpExecArray) => string) | null,
][] = [
  [/^\s+/, null, null],
  [/^#(.*)/, "Comment", (_code, matches) => matches[1]],
  [/^,/, "Punctuator", () => ","],
  [/^([1-9]\d*(?:\.\d+)?|0?\.\d+|0)/, "NumericLiteral", (_code, matches) => matches[0]],
  [
    /^(".*?(?<!\\)")|('.*?(?<!\\)')/,
    "StringLiteral",
    (_code, matches) => {
      return (
        matches[0]
          // omit quotes
          .substring(1, matches[0].length - 1)
          // escaped characters
          // TODO: make a correspond map?
          .replace('\\"', '"')
          .replace("\\'", "'")
          .replace("\\n", "\n")
          .replace("\\t", "\t")
      );
    },
  ],
  [
    /^\/((?![*+?])(?:[^\r\n\[/\\]|\\.|\[(?:[^\r\n\]\\]|\\.)*\])+)\//,
    "RegularExpression",
    (_code, matches) => matches[1],
  ],
  [new RegExp("^(" + keywords.join("|") + ")\\b"), "Keyword", (_code, matches) => matches[0]],
  [/^\w+/, "Identifier", (_code, matches) => matches[0]],
];

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
    sourceCode = sourceCode
      // strip any UTF-8 BOM off of the start of `str`, if it exists.
      .replace(/^\uFEFF/, "")
      // replace all line terminators with `\n`
      .replace(/\r\n|\r/g, "\n");
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

    for (const [regex, kind, transformer] of Spec) {
      const sourceCode = this.sourceCode.substring(this.index);
      const matches = regex.exec(sourceCode);

      if (!matches) {
        continue;
      }

      const newEnd = {
        // As benchmark says, the split method is the fastest (faster than for-loop a lot)
        // see http://jsbench.github.io/#4cc806b4507ae063efc81900cdfb9b02
        line: this.line + (matches[0].split("\n").length - 1),
        column: matches[0].includes("\n")
          ? matches[0].split("\n").pop()?.length ?? matches[0].length
          : this.column + matches[0].length,
      };

      if (kind === null || transformer === null) {
        // whitespaces, like "\t" "\n" or " "
        this.line = newEnd.line;
        this.column = newEnd.column;
        this.index += matches[0].length;
        continue;
      }

      const loc = {
        raw: matches[0],
        start: this.index,
        end: this.index + matches[0].length,
        loc: {
          start: {
            line: this.line,
            column: this.column,
          },
          end: newEnd,
        },
      };

      this.index += matches[0].length;
      this.line = newEnd.line;
      this.column = newEnd.column;

      return {
        ...loc,
        type: kind,
        value: transformer(sourceCode, matches),
      };
    }
    return {
      type: "Unknown",
      value: "",
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
    return char === "\n" || char === "\t" || char === " ";
  }
}
