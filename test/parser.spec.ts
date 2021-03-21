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
});
