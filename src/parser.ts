import { Tokenizer } from "./tokenizer";
import {
  AlertAllStatement,
  BeforeStatement,
  CommentLine,
  DefineStatement,
  DurationStatement,
  Entry,
  HideAllStatement,
  JumpStatement,
  Program,
  SoundStatement,
  Statement,
  SyncStatement,
  Token,
  WindowStatement,
} from "./types";

export interface ParserOptions {
  sourceFile?: string;
  sourceType?: "script" | "module";
}

export class Parser {
  tokenizer: Tokenizer;
  options: ParserOptions;

  tokens: Array<Token>;
  statements: Array<Statement | Entry>;
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

  parseStatement(): Statement | CommentLine | Entry | null {
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
          case "define":
            return this.parseDefineStatement();
          default:
            return null;
        }

      case "NumericLiteral":
        return this.parseEntry();

      default:
        if (token.type === "Unknown") {
          throw new Error(`Unexpected token: ${token}`);
        } else {
          return null;
        }
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

  parseDefineStatement(): DefineStatement {
    const token = this.tokenizer.nextToken();
    const identifier = this.tokenizer.nextToken();
    if (identifier.type !== "Identifier" && identifier.value !== "alertsound") {
      console.log(identifier);
      throw new Error(`Unexpected token: { type: ${identifier.type},value: ${identifier.value} }`);
    }

    const nameToken = this.tokenizer.nextToken();
    if (nameToken.type !== "StringLiteral") {
      console.log(nameToken);
      throw new Error("Unexpected token type: " + nameToken.type);
    }

    const fileToken = this.tokenizer.nextToken();
    if (fileToken.type !== "StringLiteral") {
      console.log(fileToken);
      throw new Error("Unexpected token type: " + fileToken.type);
    }

    const stmt: DefineStatement = {
      type: "DefineStatement",
      range: [token.start, identifier.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: identifier.loc.end.line,
          column: identifier.loc.end.column,
        },
      },
      defineType: "alertsound",
      name: {
        range: [nameToken.start, nameToken.end],
        ...nameToken,
      },
      file: {
        range: [fileToken.start, fileToken.end],
        ...fileToken,
      },
    };
    return stmt;
  }

  parseEntry(): Entry {
    const token = this.tokenizer.nextToken();
    const nameStrLit = this.tokenizer.nextToken();
    if (token.type !== "NumericLiteral") {
      console.log(token);
      throw new Error("Unexpected token type: " + token.type);
    }
    if (nameStrLit.type !== "StringLiteral") {
      console.log(nameStrLit);
      throw new Error(`Unexpected token: { type: ${nameStrLit.type},value: ${nameStrLit.value} }`);
    }

    const stmt: Entry = {
      type: "Entry",
      range: [token.start, nameStrLit.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: nameStrLit.loc.end.line,
          column: nameStrLit.loc.end.column,
        },
      },
      time: {
        type: "NumericLiteral",
        value: parseFloat(token.value),
        range: [token.start, token.end],
        loc: token.loc,
        raw: token.raw,
      },
      name: {
        range: [nameStrLit.start, nameStrLit.end],
        ...nameStrLit,
      },
    };

    while (this.tokenizer.hasNextToken()) {
      const nextToken = this.tokenizer.peekToken();
      if (
        nextToken.type !== "Keyword" ||
        !(
          nextToken.value === "sync" ||
          nextToken.value === "window" ||
          nextToken.value === "jump" ||
          nextToken.value === "duration"
        )
      ) {
        break;
      }

      if (nextToken.value === "sync") {
        stmt.sync = this.parseSyncStatement();
      }
      if (nextToken.value === "window") {
        stmt.window = this.parseWindowStatement();
      }
      if (nextToken.value === "jump") {
        stmt.jump = this.parseJumpStatement();
      }
      if (nextToken.value === "duration") {
        stmt.duration = this.parseDurationStatement();
      }
    }
    return stmt;
  }

  parseSyncStatement(): SyncStatement {
    const token = this.tokenizer.nextToken();
    const regexLit = this.tokenizer.nextToken();
    if (regexLit.type !== "RegularExpression") {
      console.log(token);
      throw new Error(`Unexpected token type: ${token.type}`);
    }

    const stmt: SyncStatement = {
      type: "SyncStatement",
      range: [token.start, token.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: token.loc.end.line,
          column: token.loc.end.column,
        },
      },
      regex: {
        type: "RegExpLiteral",
        flags: "",
        pattern: regexLit.value,
        range: [regexLit.start, regexLit.end],
        loc: regexLit.loc,
        raw: regexLit.raw,
      },
    };
    return stmt;
  }

  parseWindowStatement(): WindowStatement {
    const token = this.tokenizer.nextToken();
    const numLit = this.tokenizer.nextToken();
    if (numLit.type !== "NumericLiteral") {
      console.log(token);
      throw new Error(`Unexpected token type: ${token.type}`);
    }

    const stmt: WindowStatement = {
      type: "WindowStatement",
      range: [token.start, numLit.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: numLit.loc.end.line,
          column: numLit.loc.end.column,
        },
      },
      // assign the same value temporarily
      before: {
        type: "NumericLiteral",
        value: parseFloat(numLit.value),
        range: [numLit.start, numLit.end],
        loc: numLit.loc,
        raw: numLit.raw,
      },
    };

    if (this.tokenizer.hasNextToken() && this.tokenizer.peekToken().type === "Punctuator") {
      const nextToken = this.tokenizer.nextToken();
      if (nextToken.value !== ",") {
        console.log(nextToken);
        throw new Error(`Unexpected token: ${nextToken.value}`);
      }

      const numLit2 = this.tokenizer.nextToken();
      if (numLit2.type !== "NumericLiteral") {
        console.log(numLit2);
        throw new Error(`Unexpected token type: ${numLit2.type}`);
      }

      stmt.after = {
        type: "NumericLiteral",
        value: parseFloat(numLit2.value),
        range: [numLit2.start, numLit2.end],
        loc: numLit2.loc,
        raw: numLit2.raw,
      };
    }

    return stmt;
  }

  parseJumpStatement(): JumpStatement {
    const token = this.tokenizer.nextToken();
    const numLit = this.tokenizer.nextToken();
    if (numLit.type !== "NumericLiteral") {
      console.log(numLit);
      throw new Error(`Unexpected token type: ${numLit.type}`);
    }

    const stmt: JumpStatement = {
      type: "JumpStatement",
      range: [token.start, numLit.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: numLit.loc.end.line,
          column: numLit.loc.end.column,
        },
      },
      time: {
        type: "NumericLiteral",
        value: parseFloat(numLit.value),
        range: [numLit.start, numLit.end],
        loc: numLit.loc,
        raw: numLit.raw,
      },
    };

    return stmt;
  }

  parseDurationStatement(): DurationStatement {
    const token = this.tokenizer.nextToken();
    const numLit = this.tokenizer.nextToken();
    if (numLit.type !== "NumericLiteral") {
      console.log(numLit);
      throw new Error(`Unexpected token type: ${numLit.type}`);
    }

    const stmt: DurationStatement = {
      type: "DurationStatement",
      range: [token.start, numLit.end],
      loc: {
        start: {
          line: token.loc.start.line,
          column: token.loc.start.column,
        },
        end: {
          line: numLit.loc.end.line,
          column: numLit.loc.end.column,
        },
      },
      time: {
        type: "NumericLiteral",
        value: parseFloat(numLit.value),
        range: [numLit.start, numLit.end],
        loc: numLit.loc,
        raw: numLit.raw,
      },
    };

    return stmt;
  }
}
