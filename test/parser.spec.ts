import { expect } from "chai";
import { Parser } from "../src/parser";
import { Tokenizer } from "../src/tokenizer";

describe("Parser", () => {
  /**
   * This shape should be like:
   *
   * HideAllStatement:
   *   name: StringLiteral
   */
  it("hideall statement", () => {
    const tokenizer = new Tokenizer('hideall "--sync--"');
    const parser = new Parser(tokenizer);
    const ast = parser.parse();
    expect(ast.type).to.equal("Program");
    expect(ast.body.length).to.equal(1);
    const stmt = ast.body[0];
    expect(stmt.type).to.equal("HideAllStatement");
    expect(stmt).to.have.nested.property("name.type", "StringLiteral");
    expect(stmt).to.have.nested.property("name.value", "--sync--");
  });

  /**
   * This shape should be like:
   *
   * AlertAllStatement:
   *   name: StringLiteral
   *   before?: BeforeStatement
   *     time: NumericLiteral
   *   sound?: SoundStatement
   *     file: StringLiteral
   */
  it("alertall statement", () => {
    const tokenizer = new Tokenizer('alertall "name" before 1 sound "file"');
    const parser = new Parser(tokenizer);
    const ast = parser.parse();
    expect(ast.type).to.equal("Program");
    expect(ast.body.length).to.equal(1);
    const stmt = ast.body[0];
    expect(stmt.type).to.equal("AlertAllStatement");
    expect(stmt).to.have.nested.property("name.type", "StringLiteral");
    expect(stmt).to.have.nested.property("name.value", "name");
    expect(stmt).to.have.nested.property("before.type", "BeforeStatement");
    expect(stmt).to.have.nested.property("before.time.type", "NumericLiteral");
    expect(stmt).to.have.nested.property("before.time.value", 1);
    expect(stmt).to.have.nested.property("sound.type", "SoundStatement");
    expect(stmt).to.have.nested.property("sound.file.type", "StringLiteral");
    expect(stmt).to.have.nested.property("sound.file.value", "file");
  });

  /**
   * This shape should be like:
   *
   * DefineStatement:
   *   defineType: "alertsound"
   *   name: StringLiteral
   *   file: StringLiteral
   */
  it("define statement", () => {
    const tokenizer = new Tokenizer('define alertsound "name" "file"');
    const parser = new Parser(tokenizer);
    const ast = parser.parse();
    expect(ast.type).to.equal("Program");
    expect(ast.body.length).to.equal(1);
    const stmt = ast.body[0];
    expect(stmt.type).to.equal("DefineStatement");
    expect(stmt).to.have.own.property("defineType", "alertsound");
    expect(stmt).to.have.nested.property("name.type", "StringLiteral");
    expect(stmt).to.have.nested.property("name.value", "name");
    expect(stmt).to.have.nested.property("file.type", "StringLiteral");
    expect(stmt).to.have.nested.property("file.value", "file");
  });

  /**
   * This shape should be like:
   *
   * Entry:
   *   name: StringLiteral
   *   sync?: SyncStatement
   *     regex: RegExpLiteral
   *   window?: WindowStatement
   *     before: NumericLiteral
   *     after: NumericLiteral
   *   jump?: JumpStatement
   *     time: NumericLiteral
   */
  it("timeline entry", () => {
    const tokenizer = new Tokenizer('0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ window 10000 jump 0');
    const parser = new Parser(tokenizer);
    const ast = parser.parse();
    expect(ast.type).to.equal("Program");
    expect(ast.body.length).to.equal(1);
    const stmt = ast.body[0];
    expect(stmt.type).to.equal("Entry");
    expect(stmt).to.have.nested.property("name.type", "StringLiteral");
    expect(stmt).to.have.nested.property("name.value", "--Reset--");
    expect(stmt).to.have.nested.property("window.type", "WindowStatement");
    expect(stmt).to.have.nested.property("window.before.type", "NumericLiteral");
    expect(stmt).to.have.nested.property("window.before.value", 10000);
    expect(stmt).to.have.nested.property("window.type", "WindowStatement");
    expect(stmt).to.have.nested.property("window.after.type", "NumericLiteral");
    expect(stmt).to.have.nested.property("window.after.value", 10000);
    expect(stmt).to.have.nested.property("jump.type", "JumpStatement");
    expect(stmt).to.have.nested.property("jump.time.type", "NumericLiteral");
    expect(stmt).to.have.nested.property("jump.time.value", 0);
  });
});