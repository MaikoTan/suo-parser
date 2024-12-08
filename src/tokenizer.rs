// import { Position, SourceLocation } from "./utils/location";
// import { netSyncLogType } from "./utils/logTypes";

// export interface BaseToken {
//   type: string;
//   value?: string;
//   start: number;
//   end: number;
//   loc: SourceLocation;
//   raw: string;
// }

// export interface EOFToken extends BaseToken {
//   type: "EOF";
// }
// export interface WhitespaceToken extends BaseToken {
//   type: "Whitespace";
//   value: string;
// }
// export interface KeywordToken extends BaseToken {
//   type: "Keyword";
//   value: string;
// }
// export interface IdentifierToken extends BaseToken {
//   type: "Identifier";
//   value: string;
// }
// export interface StringLiteralToken extends BaseToken {
//   type: "StringLiteral";
//   value: string;
// }
// export interface NumericLiteralToken extends BaseToken {
//   type: "NumericLiteral";
//   value: string;
// }
// export interface OperatorToken extends BaseToken {
//   type: "Operator";
//   value: string;
// }
// export interface PunctuatorToken extends BaseToken {
//   type: "Punctuator";
//   value: string;
// }
// export interface BraceToken extends BaseToken {
//   type: "Brace";
//   value: string;
// }
// export interface CommentToken extends BaseToken {
//   type: "Comment";
//   value: string;
// }
// export interface RegularExpressionToken extends BaseToken {
//   type: "RegularExpression";
//   value: string;
// }
// export interface UnknownToken extends BaseToken {
//   type: "Unknown";
//   value: string;
// }

// export type Token =
//   | EOFToken
//   | WhitespaceToken
//   | KeywordToken
//   | IdentifierToken
//   | StringLiteralToken
//   | NumericLiteralToken
//   | OperatorToken
//   | PunctuatorToken
//   | BraceToken
//   | CommentToken
//   | RegularExpressionToken
//   | UnknownToken;

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

#[cfg(test)]
mod tests {
  // import { expect } from "chai";
  // import { Tokenizer } from "../src/tokenizer";

  // describe("Tokenizer", () => {
  //   it("should tokenize a simple timeline entry", () => {
  //     const input = '100 "name"';
  //     const tokens = new Tokenizer(input);
  //     let next = tokens.nextToken();
  //     expect(next.type).equals("NumericLiteral");
  //     expect(next.value).equals("100");
  //     expect(next.start).equals(0);
  //     expect(next.end).equals(3);
  //     next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals("name");
  //     expect(next.start).equals(4);
  //     expect(next.end).equals(10);
  //     next = tokens.nextToken(); // EOF
  //     expect(next.type).equals("EOF");
  //   });

  //   it("should tokenize string with escape", () => {
  //     const input = '"\\""';
  //     const tokens = new Tokenizer(input);
  //     let next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals('"');
  //   });

  //   it("should tokenize string mixup with single and double quotes", () => {
  //     const input = "\"I'm\" 'str\"ing'";
  //     const tokens = new Tokenizer(input);
  //     let next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals("I'm");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals('str"ing');
  //   });

  //   it("should tokenize a timeline entry with a comment", () => {
  //     const input = '10.0 "name" # comment';
  //     const tokens = new Tokenizer(input);
  //     let next = tokens.nextToken();
  //     expect(next.type).equals("NumericLiteral");
  //     expect(next.value).equals("10.0");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals("name");
  //     next = tokens.nextToken(); // comment
  //     expect(next.type).equals("Comment");
  //     expect(next.value).equals(" comment");
  //     expect(next.raw).equals("# comment");
  //   });

  //   it("should tokenize sync command with regex", () => {
  //     const input = "sync /regexp/";
  //     const tokens = new Tokenizer(input);
  //     let next = tokens.nextToken();
  //     expect(next.type).equals("Keyword");
  //     expect(next.value).equals("sync");
  //     expect(next.start).equals(0);
  //     expect(next.end).equals(4);
  //     next = tokens.nextToken();
  //     expect(next.type).equals("RegularExpression");
  //     expect(next.value).equals("regexp");
  //     expect(next.raw).equals("/regexp/");
  //     expect(next.start).equals(5);
  //     expect(next.end).equals(13);
  //   });

  //   it("should tokenize sync netsync command", () => {
  //     const input = 'Ability { id: "1000", name: "name" }';
  //     const tokens = new Tokenizer(input);
  //     let next = tokens.nextToken();
  //     expect(next.type).equals("Keyword");
  //     expect(next.value).equals("Ability");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("Brace");
  //     expect(next.value).equals("{");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("Identifier");
  //     expect(next.value).equals("id");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("Punctuator");
  //     expect(next.value).equals(":");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals("1000");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("Punctuator");
  //     expect(next.value).equals(",");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("Identifier");
  //     expect(next.value).equals("name");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("Punctuator");
  //     expect(next.value).equals(":");
  //     next = tokens.nextToken();
  //     expect(next.type).equals("StringLiteral");
  //     expect(next.value).equals("name");
  //   });

  //   it("should tokenize window command", () => {
  //     const input = "window 10.0\nwindow 1,1";
  //     const tokens = new Tokenizer(input);
  //     const tokenTypes = tokens.allTokens.map((token) => token.type);
  //     expect(tokenTypes).deep.equals([
  //       "Keyword",
  //       "Whitespace",
  //       "NumericLiteral",
  //       "Whitespace",
  //       "Keyword",
  //       "Whitespace",
  //       "NumericLiteral",
  //       "Punctuator",
  //       "NumericLiteral",
  //     ]);
  //   });

  //   it("should tokenize jump command", () => {
  //     const input = "jump 10.0\njump 10";
  //     const tokens = new Tokenizer(input);
  //     const tokenTypes = tokens.allTokens.map((token) => token.type);
  //     expect(tokenTypes).deep.equals([
  //       "Keyword",
  //       "Whitespace",
  //       "NumericLiteral",
  //       "Whitespace",
  //       "Keyword",
  //       "Whitespace",
  //       "NumericLiteral",
  //     ]);
  //   });
  // });
}
