import { SourceLocation } from "./utils/location";

export interface NodeBase {
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
  | DefineStatement
  | Entry;

export interface CommentLine extends NodeBase {
  type: "CommentLine";
  value: string;
  raw: string;
}

export interface StringLiteral extends NodeBase {
  type: "StringLiteral";
  value: string;
  raw: string;
}

export interface NumericLiteral extends NodeBase {
  type: "NumericLiteral";
  value: number;
  raw: string;
}

export interface RegExpLiteral extends NodeBase {
  type: "RegExpLiteral";
  pattern: string;
  flags: string;
  raw: string;
}

export interface SyncStatement extends NodeBase {
  type: "SyncStatement";
  regex: RegExpLiteral;
}

export interface WindowStatement extends NodeBase {
  type: "WindowStatement";
  before: NumericLiteral;
  after?: NumericLiteral;
}

export interface JumpStatement extends NodeBase {
  type: "JumpStatement";
  time: NumericLiteral;
}

export interface DurationStatement extends NodeBase {
  type: "DurationStatement";
  time: NumericLiteral;
}

export interface BeforeStatement extends NodeBase {
  type: "BeforeStatement";
  time: NumericLiteral;
}

export interface SoundStatement extends NodeBase {
  type: "SoundStatement";
  file: StringLiteral;
}

export interface HideAllStatement extends NodeBase {
  type: "HideAllStatement";
  name: StringLiteral;
}

export interface AlertAllStatement extends NodeBase {
  type: "AlertAllStatement";
  name: StringLiteral;
  before?: BeforeStatement;
  sound?: SoundStatement;
}

export interface DefineStatement extends NodeBase {
  type: "DefineStatement";
  /**
   * currently only alertsound supported (?)
   *
   * @see https://github.com/grindingcoil/act_timeline/blob/master/doc/TimelineSyntax.md#%E8%AD%A6%E5%91%8A%E9%9F%B3%E3%81%AE%E5%88%A5%E5%90%8D%E8%A8%AD%E5%AE%9A
   * @see https://github.com/grindingcoil/act_timeline/blob/d1c82613dfc9ef5136986cafe0fb96bf42cff3be/src/TimelineLoader.cs#L133
   */
  defineType: "alertsound";
  name: StringLiteral;
  file: StringLiteral;
}

export interface Entry extends NodeBase {
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
  | AlertAllStatement
  | DefineStatement;

export interface Token {
  type: string;
  loc: SourceLocation;
  range: [number, number];
  value: string;
}

export interface Program extends NodeBase {
  type: "Program";
  body: Array<Statement | Entry>;
  sourceType: "script" | "module";
  sourceFile: string;
  tokens: Array<Token>;
  comments: Array<Token>;
}
