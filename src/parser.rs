// import { Tokenizer } from "./tokenizer";
// import {
//   AlertAllStatement,
//   BeforeStatement,
//   CommentLine,
//   DefineStatement,
//   DurationStatement,
//   Entry,
//   HideAllStatement,
//   JumpStatement,
//   NetSyncStatement,
//   Program,
//   SoundStatement,
//   Statement,
//   SyncStatement,
//   Token,
//   WindowStatement,
// } from "./types";
// import { Position, SourceLocation } from "./utils/location";
// import { netSyncLogType } from "./utils/logTypes";

// export interface ParserOptions {
//   sourceFile?: string;
//   sourceType?: "script" | "module";
// }

// export class Parser {
//   tokenizer: Tokenizer;
//   options: ParserOptions;

//   tokens: Array<Token>;
//   statements: Array<Statement | Entry>;
//   comments: Array<CommentLine>;

//   constructor(tokenizer: Tokenizer, options: ParserOptions = {}) {
//     this.tokenizer = tokenizer;

//     this.tokens = [];
//     this.statements = [];
//     this.comments = [];

//     this.options = {
//       sourceType: "module",
//       ...options,
//     };
//   }

//   parse(): Program {
//     while (this.tokenizer.hasNextToken()) {
//       const stmt = this.parseStatement();
//       if (stmt) {
//         if (stmt.type === "CommentLine") {
//           this.comments.push(stmt);
//         } else {
//           this.statements.push(stmt);
//         }
//       } else {
//         throw new Error("Unexpected token: " + this.tokenizer.peekToken());
//       }
//     }

//     return {
//       type: "Program",
//       body: this.statements,
//       sourceType: this.options.sourceType ?? "module",
//       sourceFile: this.options.sourceFile ?? "",
//       range: [0, this.tokenizer.index],
//       loc: new SourceLocation(new Position(1, 0), new Position(this.tokenizer.line, this.tokenizer.column)),
//       comments: this.comments,
//       tokens: this.tokens,
//     };
//   }

//   parseStatement(): Statement | CommentLine | Entry | null {
//     const token = this.tokenizer.peekToken();
//     switch (token.type) {
//       case "Comment":
//         this.tokenizer.nextToken();
//         return {
//           type: "CommentLine",
//           value: token.value,
//           loc: token.loc,
//           start: token.start,
//           end: token.end,
//         };

//       case "Keyword":
//         switch (token.value) {
//           case "hideall":
//             return this.parseHideAllStatement();
//           case "alertall":
//             return this.parseAlertAllStatement();
//           case "define":
//             return this.parseDefineStatement();
//           default:
//             return null;
//         }

//       case "NumericLiteral":
//         return this.parseEntry();

//       default:
//         if (token.type === "Unknown") {
//           throw new Error(`Unexpected token: ${token}`);
//         } else {
//           return null;
//         }
//     }
//   }

//   parseHideAllStatement(): HideAllStatement {
//     const token = this.tokenizer.nextToken();
//     const stringToken = this.tokenizer.nextToken();
//     if (stringToken.type !== "StringLiteral") {
//       console.log(stringToken);
//       throw new Error("Unexpected token type: " + stringToken.type);
//     }

//     return {
//       type: "HideAllStatement",
//       range: [token.start, stringToken.end],
//       loc: new SourceLocation(token.loc.start, stringToken.loc.end),
//       name: {
//         range: [stringToken.start, stringToken.end],
//         ...stringToken,
//       },
//     };
//   }

//   parseAlertAllStatement(): AlertAllStatement {
//     const token = this.tokenizer.nextToken();
//     const stringToken = this.tokenizer.nextToken();
//     if (stringToken.type !== "StringLiteral") {
//       console.log(stringToken);
//       throw new Error("Unexpected token type: " + token.type);
//     }

//     let before: BeforeStatement | null = null;
//     let sound: SoundStatement | null = null;
//     while (this.tokenizer.hasNextToken()) {
//       const nextToken = this.tokenizer.peekToken();
//       if (nextToken.type !== "Keyword") break;

//       this.tokenizer.nextToken();
//       const nextNextToken = this.tokenizer.nextToken();

//       switch (nextToken.value) {
//         case "before":
//           if (nextNextToken.type !== "NumericLiteral") {
//             throw new Error("Unexpected token type: " + nextNextToken.type);
//           }
//           before = {
//             type: "BeforeStatement",
//             range: [nextToken.start, nextNextToken.end],
//             loc: new SourceLocation(nextToken.loc.start, nextNextToken.loc.end),
//             time: {
//               type: "NumericLiteral",
//               value: parseFloat(nextNextToken.value),
//               range: [nextNextToken.start, nextNextToken.end],
//               loc: nextNextToken.loc,
//               raw: nextNextToken.raw,
//             },
//           };
//           break;
//         case "sound":
//           if (nextNextToken.type !== "StringLiteral") {
//             throw new Error("Unexpected token type: " + nextNextToken.type);
//           }
//           sound = {
//             type: "SoundStatement",
//             range: [nextToken.start, nextNextToken.end],
//             loc: new SourceLocation(nextToken.loc.start, nextNextToken.loc.end),
//             file: {
//               range: [nextNextToken.start, nextNextToken.end],
//               ...nextNextToken,
//             },
//           };
//           break;
//         default:
//           break;
//       }
//     }

//     const stmt: AlertAllStatement = {
//       type: "AlertAllStatement",
//       range: [token.start, stringToken.end],
//       loc: new SourceLocation(token.loc.start, stringToken.loc.end),
//       name: {
//         range: [stringToken.start, stringToken.end],
//         ...stringToken,
//       },
//     };

//     if (before) {
//       stmt.before = before;
//     }
//     if (sound) {
//       stmt.sound = sound;
//     }

//     return stmt;
//   }

//   parseDefineStatement(): DefineStatement {
//     const token = this.tokenizer.nextToken();
//     const identifier = this.tokenizer.nextToken();
//     if (identifier.type !== "Identifier" && identifier.value !== "alertsound") {
//       console.log(identifier);
//       throw new Error(`Unexpected token: { type: ${identifier.type},value: ${identifier.value} }`);
//     }

//     const nameToken = this.tokenizer.nextToken();
//     if (nameToken.type !== "StringLiteral") {
//       console.log(nameToken);
//       throw new Error("Unexpected token type: " + nameToken.type);
//     }

//     const fileToken = this.tokenizer.nextToken();
//     if (fileToken.type !== "StringLiteral") {
//       console.log(fileToken);
//       throw new Error("Unexpected token type: " + fileToken.type);
//     }

//     const stmt: DefineStatement = {
//       type: "DefineStatement",
//       range: [token.start, identifier.end],
//       loc: new SourceLocation(token.loc.start, identifier.loc.end),
//       defineType: "alertsound",
//       name: {
//         range: [nameToken.start, nameToken.end],
//         ...nameToken,
//       },
//       file: {
//         range: [fileToken.start, fileToken.end],
//         ...fileToken,
//       },
//     };
//     return stmt;
//   }

//   parseEntry(): Entry {
//     const token = this.tokenizer.nextToken();
//     const nameStrLit = this.tokenizer.nextToken();
//     if (token.type !== "NumericLiteral") {
//       console.log(token);
//       throw new Error("Unexpected token type: " + token.type);
//     }
//     if (nameStrLit.type !== "StringLiteral") {
//       console.log(nameStrLit);
//       throw new Error(`Unexpected token: { type: ${nameStrLit.type},value: ${nameStrLit.value} }`);
//     }

//     const stmt: Entry = {
//       type: "Entry",
//       range: [token.start, nameStrLit.end],
//       loc: new SourceLocation(token.loc.start, nameStrLit.loc.end),
//       time: {
//         type: "NumericLiteral",
//         value: parseFloat(token.value),
//         range: [token.start, token.end],
//         loc: token.loc,
//         raw: token.raw,
//       },
//       name: {
//         range: [nameStrLit.start, nameStrLit.end],
//         ...nameStrLit,
//       },
//     };

//     while (this.tokenizer.hasNextToken()) {
//       const nextToken = this.tokenizer.peekToken();
//       if (
//         nextToken.type !== "Keyword" ||
//         !(
//           nextToken.value === "sync" ||
//           nextToken.value === "window" ||
//           nextToken.value === "jump" ||
//           nextToken.value === "duration" ||
//           netSyncLogType.includes(nextToken.value)
//         )
//       ) {
//         break;
//       }

//       if (netSyncLogType.includes(nextToken.value)) {
//         stmt.sync = this.parseNetSyncStatement();
//       }
//       if (nextToken.value === "sync") {
//         stmt.sync = this.parseSyncStatement();
//       }
//       if (nextToken.value === "window") {
//         stmt.window = this.parseWindowStatement();
//       }
//       if (nextToken.value === "jump") {
//         stmt.jump = this.parseJumpStatement();
//       }
//       if (nextToken.value === "duration") {
//         stmt.duration = this.parseDurationStatement();
//       }
//     }
//     return stmt;
//   }

//   parseNetSyncStatement(): NetSyncStatement {
//     const typeToken = this.tokenizer.nextToken();
//     if (typeToken.type !== "Keyword") {
//       console.log(typeToken);
//       throw new Error(`Unexpected token type: ${typeToken.type}`);
//     }

//     const stmt: NetSyncStatement = {
//       type: "NetSyncStatement",
//       syncType: typeToken.value,
//       fields: {},
//       range: [typeToken.start, typeToken.end],
//       loc: new SourceLocation(typeToken.loc.start, typeToken.loc.end),
//     };

//     const leftBrace = this.tokenizer.nextToken();
//     if (leftBrace.type !== "Brace" || leftBrace.value !== "{") {
//       console.log(leftBrace);
//       throw new Error(`Unexpected token: ${leftBrace.value}`);
//     }

//     while (this.tokenizer.hasNextToken()) {
//       const nextToken = this.tokenizer.peekToken();
//       if (nextToken.type === "Identifier") {
//         const keyToken = this.tokenizer.nextToken();
//         if (keyToken.type !== "Identifier") {
//           console.log(keyToken);
//           throw new Error(`Unexpected token: ${keyToken.value}`);
//         }
//         const colonToken = this.tokenizer.nextToken();
//         if (colonToken.type !== "Punctuator" || colonToken.value !== ":") {
//           console.log(colonToken);
//           throw new Error(`Unexpected token: ${colonToken.value}`);
//         }
//         const valueToken = this.tokenizer.nextToken();
//         if (valueToken.type !== "StringLiteral" && valueToken.type !== "NumericLiteral") {
//           console.log(valueToken);
//           throw new Error(`Unexpected token: ${valueToken.value}`);
//         }
//         stmt.fields[keyToken.value] = valueToken.value;
//       }
//       const rightBrace = this.tokenizer.peekToken();
//       if (rightBrace.type === "Brace" && rightBrace.value === "}") {
//         this.tokenizer.nextToken();
//         break;
//       }
//       if (rightBrace.type === "Punctuator" && rightBrace.value === ",") {
//         this.tokenizer.nextToken();
//         continue;
//       }
//       console.log(rightBrace);
//       throw new Error(`Unexpected token: ${rightBrace.value}`);
//     }
//     return stmt;
//   }

//   parseSyncStatement(): SyncStatement {
//     const token = this.tokenizer.nextToken();
//     const regexLit = this.tokenizer.nextToken();
//     if (regexLit.type !== "RegularExpression") {
//       console.log(token);
//       throw new Error(`Unexpected token type: ${token.type}`);
//     }

//     const stmt: SyncStatement = {
//       type: "SyncStatement",
//       range: [token.start, token.end],
//       loc: token.loc,
//       regex: {
//         type: "RegExpLiteral",
//         flags: "",
//         pattern: regexLit.value,
//         range: [regexLit.start, regexLit.end],
//         loc: regexLit.loc,
//         raw: regexLit.raw,
//       },
//     };
//     return stmt;
//   }

//   parseWindowStatement(): WindowStatement {
//     const token = this.tokenizer.nextToken();
//     const numLit = this.tokenizer.nextToken();
//     if (numLit.type !== "NumericLiteral") {
//       console.log(token);
//       throw new Error(`Unexpected token type: ${token.type}`);
//     }

//     const stmt: WindowStatement = {
//       type: "WindowStatement",
//       range: [token.start, numLit.end],
//       loc: new SourceLocation(token.loc.start, numLit.loc.end),
//       // assign the same value temporarily
//       before: {
//         type: "NumericLiteral",
//         value: parseFloat(numLit.value),
//         range: [numLit.start, numLit.end],
//         loc: numLit.loc,
//         raw: numLit.raw,
//       },
//     };

//     if (this.tokenizer.hasNextToken() && this.tokenizer.peekToken().type === "Punctuator") {
//       const nextToken = this.tokenizer.nextToken();
//       if (nextToken.value !== ",") {
//         console.log(nextToken);
//         throw new Error(`Unexpected token: ${nextToken.value}`);
//       }

//       const numLit2 = this.tokenizer.nextToken();
//       if (numLit2.type !== "NumericLiteral") {
//         console.log(numLit2);
//         throw new Error(`Unexpected token type: ${numLit2.type}`);
//       }

//       stmt.after = {
//         type: "NumericLiteral",
//         value: parseFloat(numLit2.value),
//         range: [numLit2.start, numLit2.end],
//         loc: numLit2.loc,
//         raw: numLit2.raw,
//       };
//     }

//     return stmt;
//   }

//   parseJumpStatement(): JumpStatement {
//     const token = this.tokenizer.nextToken();
//     const numLit = this.tokenizer.nextToken();
//     if (numLit.type !== "NumericLiteral") {
//       console.log(numLit);
//       throw new Error(`Unexpected token type: ${numLit.type}`);
//     }

//     const stmt: JumpStatement = {
//       type: "JumpStatement",
//       range: [token.start, numLit.end],
//       loc: new SourceLocation(token.loc.start, numLit.loc.end),
//       time: {
//         type: "NumericLiteral",
//         value: parseFloat(numLit.value),
//         range: [numLit.start, numLit.end],
//         loc: numLit.loc,
//         raw: numLit.raw,
//       },
//     };

//     return stmt;
//   }

//   parseDurationStatement(): DurationStatement {
//     const token = this.tokenizer.nextToken();
//     const numLit = this.tokenizer.nextToken();
//     if (numLit.type !== "NumericLiteral") {
//       console.log(numLit);
//       throw new Error(`Unexpected token type: ${numLit.type}`);
//     }

//     const stmt: DurationStatement = {
//       type: "DurationStatement",
//       range: [token.start, numLit.end],
//       loc: new SourceLocation(token.loc.start, numLit.loc.end),
//       time: {
//         type: "NumericLiteral",
//         value: parseFloat(numLit.value),
//         range: [numLit.start, numLit.end],
//         loc: numLit.loc,
//         raw: numLit.raw,
//       },
//     };

//     return stmt;
//   }
// }

#[cfg(test)]
mod tests {
  // import { expect } from "chai";
  // import { parseAsync } from "../src";

  // describe("Parser", () => {
  //   /**
  //    * This shape should be like:
  //    *
  //    * HideAllStatement:
  //    *   name: StringLiteral
  //    */
  //   it("hideall statement", async () => {
  //     const ast = await parseAsync('hideall "--sync--"');
  //     expect(ast.type).to.equal("Program");
  //     expect(ast.body.length).to.equal(1);
  //     const stmt = ast.body[0];
  //     expect(stmt.type).to.equal("HideAllStatement");
  //     expect(stmt).to.have.nested.property("name.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("name.value", "--sync--");
  //   });

  //   /**
  //    * This shape should be like:
  //    *
  //    * AlertAllStatement:
  //    *   name: StringLiteral
  //    *   before?: BeforeStatement
  //    *     time: NumericLiteral
  //    *   sound?: SoundStatement
  //    *     file: StringLiteral
  //    */
  //   it("alertall statement", async () => {
  //     const ast = await parseAsync('alertall "name" before 1 sound "file"');
  //     expect(ast.type).to.equal("Program");
  //     expect(ast.body.length).to.equal(1);
  //     const stmt = ast.body[0];
  //     expect(stmt.type).to.equal("AlertAllStatement");
  //     expect(stmt).to.have.nested.property("name.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("name.value", "name");
  //     expect(stmt).to.have.nested.property("before.type", "BeforeStatement");
  //     expect(stmt).to.have.nested.property("before.time.type", "NumericLiteral");
  //     expect(stmt).to.have.nested.property("before.time.value", 1);
  //     expect(stmt).to.have.nested.property("sound.type", "SoundStatement");
  //     expect(stmt).to.have.nested.property("sound.file.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("sound.file.value", "file");
  //   });

  //   /**
  //    * This shape should be like:
  //    *
  //    * DefineStatement:
  //    *   defineType: "alertsound"
  //    *   name: StringLiteral
  //    *   file: StringLiteral
  //    */
  //   it("define statement", async () => {
  //     const ast = await parseAsync('define alertsound "name" "file"');
  //     expect(ast.type).to.equal("Program");
  //     expect(ast.body.length).to.equal(1);
  //     const stmt = ast.body[0];
  //     expect(stmt.type).to.equal("DefineStatement");
  //     expect(stmt).to.have.own.property("defineType", "alertsound");
  //     expect(stmt).to.have.nested.property("name.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("name.value", "name");
  //     expect(stmt).to.have.nested.property("file.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("file.value", "file");
  //   });

  //   /**
  //    * This shape should be like:
  //    *
  //    * Entry:
  //    *   name: StringLiteral
  //    *   sync?: SyncStatement
  //    *     regex: RegExpLiteral
  //    *   duration?: DurationStatement
  //    *     time: NumericLiteral
  //    *   window?: WindowStatement
  //    *     before: NumericLiteral
  //    *     after?: NumericLiteral
  //    *   jump?: JumpStatement
  //    *     time: NumericLiteral
  //    */
  //   it("timeline entry", async () => {
  //     const ast = await parseAsync(
  //       '0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0',
  //     );
  //     expect(ast.type).to.equal("Program");
  //     expect(ast.body.length).to.equal(1);
  //     const stmt = ast.body[0];
  //     expect(stmt.type).to.equal("Entry");
  //     expect(stmt).to.have.nested.property("name.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("name.value", "--Reset--");
  //     expect(stmt).to.have.nested.property("sync.type", "SyncStatement");
  //     expect(stmt).to.have.nested.property("sync.regex.type", "RegExpLiteral");
  //     expect(stmt).to.have.nested.property("sync.regex.pattern", " 00:0839:.*is no longer sealed");
  //     expect(stmt).to.have.nested.property("duration.type", "DurationStatement");
  //     expect(stmt).to.have.nested.property("duration.time.type", "NumericLiteral");
  //     expect(stmt).to.have.nested.property("duration.time.value", 5);
  //     expect(stmt).to.have.nested.property("window.type", "WindowStatement");
  //     expect(stmt).to.have.nested.property("window.before.type", "NumericLiteral");
  //     expect(stmt).to.have.nested.property("window.before.value", 10000);
  //     expect(stmt).to.have.nested.property("jump.type", "JumpStatement");
  //     expect(stmt).to.have.nested.property("jump.time.type", "NumericLiteral");
  //     expect(stmt).to.have.nested.property("jump.time.value", 0);
  //   });

  //   it("timeline entry net sync", async () => {
  //     const ast = await parseAsync('100.0 "test" Ability { id: "1000", name: "name" } window 10');
  //     expect(ast.type).to.equal("Program");
  //     expect(ast.body.length).to.equal(1);
  //     const stmt = ast.body[0];
  //     expect(stmt.type).to.equal("Entry");
  //     expect(stmt).to.have.nested.property("name.type", "StringLiteral");
  //     expect(stmt).to.have.nested.property("name.value", "test");
  //     expect(stmt).to.have.nested.property("sync.syncType", "Ability");
  //     expect(stmt).to.have.nested.property("sync.fields.id", "1000");
  //     expect(stmt).to.have.nested.property("sync.fields.name", "name");
  //     expect(stmt).to.have.nested.property("window.type", "WindowStatement");
  //     expect(stmt).to.have.nested.property("window.before.type", "NumericLiteral");
  //     expect(stmt).to.have.nested.property("window.before.value", 10);
  //   });
  // });
}
