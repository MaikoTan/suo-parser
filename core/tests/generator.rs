use suo_parser_core::generator::Generator;
use suo_parser_core::parser::Parser;
use suo_parser_core::tokenizer::Tokenizer;
use suo_parser_core::types::semantic_ast::Program;

fn transform(input: &str) -> String {
    let tokenizer: Tokenizer<_> = input.into();
    let mut parser = Parser::new(tokenizer);
    let program: Program = parser.parse();
    Generator::new(program).generate()
}

#[test]
fn test_generate_simple_timeline_entry() {
    let timeline_text =
        "0.0 \"--Reset--\" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0";
    assert_eq!(transform(timeline_text), timeline_text);
}

#[test]
fn test_generate_hideall_statement() {
    let timeline_text = "hideall \"--sync--\"";
    assert_eq!(transform(timeline_text), timeline_text);
}

#[test]
fn test_transform_timeline() {
    let timeline_text =
        "0.0 \"--Reset--\" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0";
    assert_eq!(transform(timeline_text), timeline_text);
}

#[test]
fn test_transform_timeline_with_net_sync() {
    let timeline_text = "0.0 \"name\" Ability { id: \"1000\", name: \"name\" } window 10";
    assert_eq!(transform(timeline_text), timeline_text);
}

#[test]
fn test_transform_hideall_statement() {
    let timeline_text = "hideall \"--sync--\"";
    assert_eq!(transform(timeline_text), timeline_text);
}
