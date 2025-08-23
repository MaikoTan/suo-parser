use insta;
use std::fs::File;
use suo_parser_core::tokenizer::{Token, Tokenizer};

#[test]
fn test_tokenizer_read_char() {
    let mut tokenizer: Tokenizer<_> = "100 \"name\" # comment".into();
    assert_eq!(tokenizer.has_next_token(), true);
    assert_eq!(tokenizer.peek_chars(3).unwrap(), b"100");
    assert_eq!(tokenizer.read_chars(3).unwrap(), b"100");
    assert_eq!(tokenizer.peek_chars(1).unwrap(), b" ");
    assert_eq!(tokenizer.read_chars(1).unwrap(), b" ");
    assert_eq!(tokenizer.peek_chars(6).unwrap(), b"\"name\"");
    assert_eq!(tokenizer.read_chars(6).unwrap(), b"\"name\"");
    assert_eq!(tokenizer.peek_chars(1).unwrap(), b" ");
    assert_eq!(tokenizer.read_chars(1).unwrap(), b" ");
    assert_eq!(tokenizer.peek_chars(9).unwrap(), b"# comment");
    assert_eq!(tokenizer.read_chars(9).unwrap(), b"# comment");
    assert_eq!(tokenizer.has_next_token(), false);
}

fn all_tokens(input: &str) -> Vec<Token> {
    let mut tokenizer: Tokenizer<_> = input.into();
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }
    tokens
}

#[test]
fn test_tokenizer_string() {
    let vec = all_tokens("\"name\"");

    insta::assert_debug_snapshot!("Quoted String", vec);
}

#[test]
fn test_tokenizer_identifier() {
    let vec = all_tokens("identifier");

    insta::assert_debug_snapshot!("Identifier", vec);
}

#[test]
fn test_tokenizer_identifier2() {
    let vec = all_tokens("identifier\nidentifier");

    insta::assert_debug_snapshot!("Identifier2", vec);
}

#[test]
fn test_tokenizer_simple_entry() {
    let vec = all_tokens("100 \"name\"");

    insta::assert_debug_snapshot!("Simple Entry", vec);
}

#[test]
fn test_tokenizer_string_with_escape() {
    let vec = all_tokens("\"\\\"\"");

    insta::assert_debug_snapshot!("Escaped String", vec);
}

#[test]
fn test_tokenizer_string_mixup() {
    let vec = all_tokens("\"I'm\" 'str\"ing'");

    insta::assert_debug_snapshot!("String Mixup", vec);
}

#[test]
fn test_tokenizer_timeline_entry_with_comment() {
    let vec = all_tokens("10.0 \"name\" # comment");

    insta::assert_debug_snapshot!("Timeline Entry with Comment", vec);
}

#[test]
fn test_tokenizer_sync_command_with_regex() {
    let vec = all_tokens("sync /regexp/");

    insta::assert_debug_snapshot!("Sync Command with Regex", vec);
}

#[test]
fn test_tokenizer_sync_netsync_command() {
    let vec = all_tokens("Ability { id: \"1000\", name: \"name\" }");

    insta::assert_debug_snapshot!("Sync Netsync Command", vec);
}

#[test]
fn test_tokenizer_window_command() {
    let vec = all_tokens("window 10.0\nwindow 1,1.0");

    insta::assert_debug_snapshot!("Window Command", vec);
}

#[test]
fn test_tokenizer_jump_command() {
    let vec = all_tokens("jump 10.0\njump 10");

    insta::assert_debug_snapshot!("Jump Command", vec);
}

#[test]
fn test_tokenizer_full_timeline() {
    // Taken from https://github.com/OverlayPlugin/cactbot/blob/main/ui/raidboss/data/00-misc/test.txt
    let vec = {
        let file = File::open("./tests/data/test.txt").expect("Failed to open test file");
        let mut tokenizer: Tokenizer<_> = file.into();

        let mut vec = Vec::new();
        while let Some(token) = tokenizer.next_token() {
            vec.push(token);
        }
        vec
    };

    insta::assert_debug_snapshot!("Full Timeline", vec);
}

#[test]
fn test_tokenizer_peek_token() {
    let mut tokenizer: Tokenizer<_> = "0 \"test\" sync /regex/".into();
    let v = {
        let mut vec = Vec::new();
        // NumericLiteral
        vec.push(tokenizer.peek_token().unwrap());
        vec.push(tokenizer.peek_token().unwrap());
        vec.push(tokenizer.next_token().unwrap());
        // StringLiteral
        vec.push(tokenizer.peek_token().unwrap());
        vec.push(tokenizer.next_token().unwrap());
        // Keyword
        vec.push(tokenizer.peek_token().unwrap());
        vec.push(tokenizer.next_token().unwrap());
        // RegexLiteral
        vec.push(tokenizer.peek_token().unwrap());
        vec.push(tokenizer.next_token().unwrap());

        vec
    };

    insta::assert_debug_snapshot!("Peek Tokens", v);
}
