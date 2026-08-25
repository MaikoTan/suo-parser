#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub line: u16,
    pub column: u16,
    pub offset: u32,
}

impl Position {
    pub fn new(line: u16, column: u16, offset: u32) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

impl SourceLocation {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
