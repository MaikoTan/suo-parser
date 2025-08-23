use crate::tokenizer::Tokenizer;
use std::fs::File;
use std::io::Cursor;

impl<'a> From<&'a str> for Tokenizer<Cursor<&'a str>> {
    fn from(input: &'a str) -> Self {
        Tokenizer::new(Cursor::new(input))
    }
}

impl From<String> for Tokenizer<Cursor<String>> {
    fn from(input: String) -> Self {
        Tokenizer::new(Cursor::new(input))
    }
}

impl From<File> for Tokenizer<File> {
    fn from(file: File) -> Self {
        Tokenizer::new(file)
    }
}
