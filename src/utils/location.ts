export class Position {
  line: number;
  column: number;

  constructor(line: number, col: number) {
    this.line = line;
    this.column = col;
  }
}

export class SourceLocation {
  start: Position;
  end?: Position;

  constructor(start: Position, end?: Position) {
    this.start = start;
    this.end = end;
  }
}
