import { expect } from "chai";
import { parseAsync } from "../src";

describe("Parser", () => {
  /**
   * This shape should be like:
   *
   * HideAllStatement:
   *   name: StringLiteral
   */
  it("hideall statement", async () => {
    const ast = await parseAsync('hideall "--sync--"');
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
  it("alertall statement", async () => {
    const ast = await parseAsync('alertall "name" before 1 sound "file"');
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
  it("define statement", async () => {
    const ast = await parseAsync('define alertsound "name" "file"');
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
   *   duration?: DurationStatement
   *     time: NumericLiteral
   *   window?: WindowStatement
   *     before: NumericLiteral
   *     after: NumericLiteral
   *   jump?: JumpStatement
   *     time: NumericLiteral
   */
  it("timeline entry", async () => {
    const ast = await parseAsync(
      '0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0',
    );
    expect(ast.type).to.equal("Program");
    expect(ast.body.length).to.equal(1);
    const stmt = ast.body[0];
    expect(stmt.type).to.equal("Entry");
    expect(stmt).to.have.nested.property("name.type", "StringLiteral");
    expect(stmt).to.have.nested.property("name.value", "--Reset--");
    expect(stmt).to.have.nested.property("sync.type", "SyncStatement");
    expect(stmt).to.have.nested.property("sync.regex.type", "RegExpLiteral");
    expect(stmt).to.have.nested.property("sync.regex.pattern", " 00:0839:.*is no longer sealed");
    expect(stmt).to.have.nested.property("duration.type", "DurationStatement");
    expect(stmt).to.have.nested.property("duration.time.type", "NumericLiteral");
    expect(stmt).to.have.nested.property("duration.time.value", 5);
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
