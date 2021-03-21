import { KeywordToken, Tokenizer } from "./tokenizer";
import {
  AlertAllStatement,
  BeforeStatement,
  CommentLine,
  HideAllStatement,
  Program,
  SoundStatement,
  Statement,
  Token,
} from "./types";

export interface ParserOptions {
  sourceFile?: string;
  sourceType?: "script" | "module";
}

export class Parser {
  tokenizer: Tokenizer;
  options: ParserOptions;

  tokens: Array<Token>;
  statements: Array<Statement>;
  comments: Array<CommentLine>;

  constructor(tokenizer: Tokenizer, options: ParserOptions = {}) {
    this.tokenizer = tokenizer;

    this.tokens = [];
    this.statements = [];
    this.comments = [];

    this.options = {
      sourceType: "module",
      ...options,
    };
  }

  parse(): Program {
    while (this.tokenizer.hasNextToken()) {
      const stmt = this.parseStatement();
      if (stmt) {
        if (stmt.type === "CommentLine") {
          this.comments.push(stmt);
        } else {
          this.statements.push(stmt);
        }
      }
    }

    return {
      type: "Program",
      body: this.statements,
      sourceType: this.options.sourceType ?? "module",
      sourceFile: this.options.sourceFile ?? "",
      range: [0, this.tokenizer.index],
      loc: {
        start: { line: 1, column: 0 },
        end: { line: this.tokenizer.line, column: this.tokenizer.column },
      },
      comments: this.comments,
      tokens: this.tokens,
    };
  }

  parseStatement(): Statement | CommentLine | null {
    const token = this.tokenizer.peekToken();
    switch (token.type) {
      case "Comment":
        this.tokenizer.nextToken();
        return {
          type: "CommentLine",
          value: token.value,
          range: [token.start, token.end],
          loc: token.loc,
          raw: token.raw,
        };

      case "Keyword":
        switch (token.value) {
          case "hideall":
            return this.parseHideAllStatement();
          case "alertall":
            return this.parseAlertAllStatement();
          default:
            return null;
        }

      default:
        return null;
      // throw new Error("Unexpected token type: " + token.type);
    }
  }

  parseHideAllStatement(): HideAllStatement {
    const token = this.tokenizer.nextToken();
    const stringToken = this.tokenizer.nextToken();
    if (stringToken.type !== "StringLiteral") {
      console.log(stringToken);
      throw new Error("Unexpected token type: " + stringToken.type);
    }

    return {
      type: "HideAllStatement",
      range: [token.start, stringToken.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: stringToken.loc.end.line,
          column: stringToken.loc.end.column,
        },
      },
      name: {
        range: [stringToken.start, stringToken.end],
        ...stringToken,
      },
    };
  }

  parseAlertAllStatement(): AlertAllStatement {
    const token = this.tokenizer.nextToken();
    const stringToken = this.tokenizer.nextToken();
    if (stringToken.type !== "StringLiteral") {
      console.log(stringToken);
      throw new Error("Unexpected token type: " + token.type);
    }

    let before: BeforeStatement | null = null;
    let sound: SoundStatement | null = null;
    while (this.tokenizer.hasNextToken()) {
      const nextToken = this.tokenizer.peekToken();
      if (nextToken.type !== "Keyword") break;

      this.tokenizer.nextToken();
      const nextNextToken = this.tokenizer.nextToken();

      switch (nextToken.value) {
        case "before":
          if (nextNextToken.type !== "NumericLiteral") {
            throw new Error("Unexpected token type: " + nextNextToken.type);
          }
          before = {
            type: "BeforeStatement",
            range: [nextToken.start, nextNextToken.end],
            loc: {
              start: {
                line: nextToken.loc.start.line,
                column: nextToken.loc.start.column,
              },
              end: {
                line: nextNextToken.loc.end.line,
                column: nextNextToken.loc.end.column,
              },
            },
            time: {
              type: "NumericLiteral",
              value: parseFloat(nextNextToken.value),
              range: [nextNextToken.start, nextNextToken.end],
              loc: nextNextToken.loc,
              raw: nextNextToken.raw,
            },
          };
          break;
        case "sound":
          if (nextNextToken.type !== "StringLiteral") {
            throw new Error("Unexpected token type: " + nextNextToken.type);
          }
          sound = {
            type: "SoundStatement",
            range: [nextToken.start, nextNextToken.end],
            loc: {
              start: {
                line: nextToken.loc.start.line,
                column: nextToken.loc.start.column,
              },
              end: {
                line: nextNextToken.loc.end.line,
                column: nextNextToken.loc.end.column,
              },
            },
            file: {
              range: [nextNextToken.start, nextNextToken.end],
              ...nextNextToken,
            },
          };
          break;
        default:
          break;
      }
    }

    const stmt: AlertAllStatement = {
      type: "AlertAllStatement",
      range: [token.start, stringToken.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: stringToken.loc.end.line,
          column: stringToken.loc.end.column,
        },
      },
      name: {
        range: [stringToken.start, stringToken.end],
        ...stringToken,
      },
    };

    if (before) {
      stmt.before = before;
    }
    if (sound) {
      stmt.sound = sound;
    }

    return stmt;
  }
}
