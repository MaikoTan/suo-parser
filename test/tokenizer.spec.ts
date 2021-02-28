import { expect } from "chai";
import { Tokenizer } from "../src/tokenizer";

describe("Tokenizer", () => {
  it("should tokenize a simple timeline entry", () => {
    const input = '100 "name"';
    const tokens = new Tokenizer(input);
    let next = tokens.nextToken();
    expect(next.type).equals("NumericLiteral");
    expect(next.value).equals("100");
    expect(next.start).equals(0);
    expect(next.end).equals(3);
    next = tokens.nextToken();
    expect(next.type).equals("StringLiteral");
    expect(next.value).equals("name");
    expect(next.start).equals(4);
    expect(next.end).equals(10);
    next = tokens.nextToken(); // EOF
    expect(next.type).equals("EOF");
  });

  it("should tokenize string with escape", () => {
    const input = '"\\""';
    const tokens = new Tokenizer(input);
    let next = tokens.nextToken();
    expect(next.type).equals("StringLiteral");
    expect(next.value).equals('"');
  });

  it("should tokenize string mixup with single and double quotes", () => {
    const input = "\"I'm\" 'str\"ing'";
    const tokens = new Tokenizer(input);
    let next = tokens.nextToken();
    expect(next.type).equals("StringLiteral");
    expect(next.value).equals("I'm");
    next = tokens.nextToken();
    expect(next.type).equals("StringLiteral");
    expect(next.value).equals('str"ing');
  });

  it("should tokenize a timeline entry with a comment", () => {
    const input = '10.0 "name" # comment';
    const tokens = new Tokenizer(input);
    let next = tokens.nextToken();
    expect(next.type).equals("NumericLiteral");
    expect(next.value).equals("10.0");
    next = tokens.nextToken();
    expect(next.type).equals("StringLiteral");
    expect(next.value).equals("name");
    next = tokens.nextToken(); // comment
    expect(next.type).equals("Comment");
    expect(next.value).equals(" comment");
    expect(next.raw).equals("# comment");
  });

  it("should tokenize sync command with regex", () => {
    const input = "sync /regexp/";
    const tokens = new Tokenizer(input);
    let next = tokens.nextToken();
    expect(next.type).equals("Keyword");
    expect(next.value).equals("sync");
    expect(next.start).equals(0);
    expect(next.end).equals(4);
    next = tokens.nextToken();
    expect(next.type).equals("RegularExpression");
    expect(next.value).equals("regexp");
    expect(next.raw).equals("/regexp/");
    expect(next.start).equals(5);
    expect(next.end).equals(13);
  });

  it("should tokenize window command", () => {
    const input = "window 10.0\nwindow 1,1";
    const tokens = new Tokenizer(input);
    const tokenTypes = [
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
    ].map((token) => token.type);
    expect(tokenTypes).deep.equals([
      "Keyword",
      "NumericLiteral",
      "Keyword",
      "NumericLiteral",
      "Punctuator",
      "NumericLiteral",
      "EOF",
    ]);
  });

  it("should tokenize jump command", () => {
    const input = "jump 10.0\njump 10";
    const tokens = new Tokenizer(input);
    const tokenTypes = [
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
      tokens.nextToken(),
    ].map((token) => token.type);
    expect(tokenTypes).deep.equals(["Keyword", "NumericLiteral", "Keyword", "NumericLiteral", "EOF"]);
  });
});
