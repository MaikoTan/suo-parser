export interface BaseNode {
  type: Node["type"];
  loc: SourceLocation;
  range: [number, number];
}

export type Node =
  | Program
  | CommentLine
  | StringLiteral
  | NumericLiteral
  | RegExpLiteral
  | SyncStatement
  | WindowStatement
  | JumpStatement
  | DurationStatement
  | BeforeStatement
  | SoundStatement
  | HideAllStatement
  | AlertAllStatement
  | Entry;

interface SourceLocation {
  start: Position;
  end: Position;
}

interface Position {
  line: number; // 1-indexed
  column: number; // 0-indexed
}

export interface CommentLine extends BaseNode {
  type: "CommentLine";
  value: string;
  raw: string;
}

export interface StringLiteral extends BaseNode {
  type: "StringLiteral";
  value: string;
  raw: string;
}

export interface NumericLiteral extends BaseNode {
  type: "NumericLiteral";
  value: number;
  raw: string;
}

export interface RegExpLiteral extends BaseNode {
  type: "RegExpLiteral";
  pattern: string;
  flags: string;
  raw: string;
}

export interface SyncStatement extends BaseNode {
  type: "SyncStatement";
  regex: RegExpLiteral;
}

export interface WindowStatement extends BaseNode {
  type: "WindowStatement";
  before: NumericLiteral;
  after: NumericLiteral;
}

export interface JumpStatement extends BaseNode {
  type: "JumpStatement";
  time: NumericLiteral;
}

export interface DurationStatement extends BaseNode {
  type: "DurationStatement";
  time: NumericLiteral;
}

export interface BeforeStatement extends BaseNode {
  type: "BeforeStatement";
  time: NumericLiteral;
}

export interface SoundStatement extends BaseNode {
  type: "SoundStatement";
  file: StringLiteral;
}

export interface HideAllStatement extends BaseNode {
  type: "HideAllStatement";
  name: StringLiteral;
}

export interface AlertAllStatement extends BaseNode {
  type: "AlertAllStatement";
  name: StringLiteral;
  before?: BeforeStatement;
  sound?: SoundStatement;
}

export interface Entry extends BaseNode {
  type: "Entry";
  time: NumericLiteral;
  name: StringLiteral;
  sync?: SyncStatement;
  window?: WindowStatement;
  duration?: DurationStatement;
  jump?: JumpStatement;
}

export type Literal = StringLiteral | NumericLiteral | RegExpLiteral;
export type Statement =
  | SyncStatement
  | WindowStatement
  | JumpStatement
  | DurationStatement
  | HideAllStatement
  | AlertAllStatement;

export interface Token {
  type: string;
  loc: SourceLocation;
  range: [number, number];
  value: string;
}

export interface Program extends BaseNode {
  type: "Program";
  body: Array<Statement>;
  sourceType: "script" | "module";
  sourceFile: string;
  tokens: Array<Token>;
  comments: Array<Token>;
}
