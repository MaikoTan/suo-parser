//! WASM bindings for suo-parser-core.
//!
//! Build with: wasm-pack build wasm --target nodejs

use serde_wasm_bindgen::{from_value, to_value};
use suo_parser_core::generator::Generator;
use suo_parser_core::parser::Parser;
use suo_parser_core::tokenizer::Tokenizer;
use suo_parser_core::types::semantic_ast::{
    AlertAllStmt, DefineStmt, DefineType, DurationStmt, EntryStmt, HideAllStmt, JumpStmt,
    NetSyncStmt, Program, Statement, SyncStmt, Time, WindowStmt,
};
use wasm_bindgen::prelude::*;

/// Install the panic hook so Rust panics print a readable message to the JS
/// console instead of a bare `unreachable` trap.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Parse a timeline string into a JSON-serializable [Program].
#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    let tokenizer: Tokenizer<_> = input.into();
    let mut parser = Parser::new(tokenizer);
    let program: Program = parser.parse();
    to_value(&program).map_err(|err| JsValue::from_str(&err.to_string()))
}

/// Generate timeline text from a JSON-serialized [Program].
#[wasm_bindgen]
pub fn generate(program: JsValue) -> Result<String, JsValue> {
    let program: Program =
        from_value(program).map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(Generator::new(program).generate())
}

/// Parse a timeline string and generate it back (round-trip).
#[wasm_bindgen]
pub fn transform(input: &str) -> Result<String, JsValue> {
    let tokenizer: Tokenizer<_> = input.into();
    let mut parser = Parser::new(tokenizer);
    let program: Program = parser.parse();
    Ok(Generator::new(program).generate())
}

/// Parse a timeline string and return the raw AST as a JSON object.
/// This is an alias for `parse` but with clearer naming for debugging.
#[wasm_bindgen(js_name = parseAST)]
pub fn parse_ast(input: &str) -> Result<JsValue, JsValue> {
    parse(input)
}

/// Tokenize a timeline string into a list of tokens for debugging.
#[wasm_bindgen]
pub fn tokenize(input: &str) -> Result<JsValue, JsValue> {
    let mut tokenizer: Tokenizer<_> = input.into();
    let mut tokens = Vec::new();
    while tokenizer.has_next_token() {
        if let Some(token) = tokenizer.next_token() {
            tokens.push(token);
        }
    }
    to_value(&tokens).map_err(|err| JsValue::from_str(&err.to_string()))
}

/// Version of the parser.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Re-export types for TypeScript users via serde-wasm-bindgen
#[wasm_bindgen(typescript_custom_section)]
const TS_PROGRAM: &'static str = r#"
export interface Program {
    defines: DefineStmt[];
    hide_alls: HideAllStmt[];
    alert_alls: AlertAllStmt[];
    entries: EntryStmt[];
}

export type Statement =
    | DefineStmt
    | HideAllStmt
    | AlertAllStmt
    | EntryStmt;

export interface DefineStmt {
    define_type: DefineType;
    name: string;
    file: string;
}

export enum DefineType {
    Include,
    Library,
}

export interface HideAllStmt {
    name: string;
}

export interface AlertAllStmt {
    name: string;
    before?: Time;
    sound?: string;
}

export interface EntryStmt {
    time: Time;
    name: string;
    sync?: AnySyncStmt;
    window?: WindowStmt;
    duration?: DurationStmt;
    jump?: JumpStmt;
}

export type AnySyncStmt = SyncStmt | NetSyncStmt;

export interface SyncStmt {
    regex: string;
}

export interface NetSyncStmt {
    sync_type: string;
    fields: [string, string][];
}

export interface WindowStmt {
    before: Time;
    after?: Time;
}

export interface DurationStmt {
    time: Time;
}

export interface JumpStmt {
    time: Time;
}

export type Time =
    | { type: "Relative"; value: number }
    | { type: "Absolute"; value: string }
    | { type: "Current"; };
"#;

// Make types serializable for TypeScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Program")]
    pub type ProgramTs;

    #[wasm_bindgen(typescript_type = "Statement")]
    pub type StatementTs;

    #[wasm_bindgen(typescript_type = "DefineStmt")]
    pub type DefineStmtTs;

    #[wasm_bindgen(typescript_type = "DefineType")]
    pub type DefineTypeTs;

    #[wasm_bindgen(typescript_type = "HideAllStmt")]
    pub type HideAllStmtTs;

    #[wasm_bindgen(typescript_type = "AlertAllStmt")]
    pub type AlertAllStmtTs;

    #[wasm_bindgen(typescript_type = "EntryStmt")]
    pub type EntryStmtTs;

    #[wasm_bindgen(typescript_type = "AnySyncStmt")]
    pub type AnySyncStmtTs;

    #[wasm_bindgen(typescript_type = "SyncStmt")]
    pub type SyncStmtTs;

    #[wasm_bindgen(typescript_type = "NetSyncStmt")]
    pub type NetSyncStmtTs;

    #[wasm_bindgen(typescript_type = "WindowStmt")]
    pub type WindowStmtTs;

    #[wasm_bindgen(typescript_type = "DurationStmt")]
    pub type DurationStmtTs;

    #[wasm_bindgen(typescript_type = "JumpStmt")]
    pub type JumpStmtTs;

    #[wasm_bindgen(typescript_type = "Time")]
    pub type TimeTs;
}
