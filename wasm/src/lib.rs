//! WASM bindings for suo-parser-core.
//!
//! Build with: wasm-pack build wasm --target nodejs

use serde_wasm_bindgen::{from_value, to_value};
use suo_parser_core::generator::Generator;
use suo_parser_core::parser::Parser;
use suo_parser_core::tokenizer::Tokenizer;
use suo_parser_core::types::semantic_ast::Program;
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
