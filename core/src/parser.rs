use std::io::Read;

use crate::tokenizer::{TokenKind, Tokenizer};
use crate::types::semantic_ast::*;
use crate::utils::log_types::NetSyncLogType;

impl From<String> for Time {
    fn from(value: String) -> Self {
        if let Ok(int_value) = value.parse::<u64>() {
            Time::Integer(int_value)
        } else if let Ok(float_value) = value.parse::<f64>() {
            Time::Float(float_value)
        } else {
            panic!("Invalid time format: {}", value);
        }
    }
}

pub struct Parser<R: Read> {
    tokenizer: Tokenizer<R>,
    program: Program,
}

impl<R: Read> Parser<R> {
    pub fn new(tokenizer: Tokenizer<R>) -> Self {
        Parser {
            tokenizer,
            program: Program {
                defines: Vec::new(),
                hide_alls: Vec::new(),
                alert_alls: Vec::new(),
                entries: Vec::new(),
            },
        }
    }
}

impl<R: Read> Parser<R> {
    pub fn parse(&mut self) -> Program {
        while self.tokenizer.has_next_token() {
            let stmt = self.parse_statement().unwrap();
            match stmt {
                Statement::Define(DefineStmt {
                    define_type,
                    name,
                    file,
                }) => {
                    self.program.defines.push(DefineStmt {
                        define_type,
                        name,
                        file,
                    });
                }
                Statement::HideAll(HideAllStmt { name }) => {
                    self.program.hide_alls.push(HideAllStmt { name });
                }
                Statement::AlertAll(AlertAllStmt {
                    name,
                    before,
                    sound,
                }) => {
                    self.program.alert_alls.push(AlertAllStmt {
                        name,
                        before,
                        sound,
                    });
                }
                Statement::Entry(EntryStmt {
                    time,
                    name,
                    sync,
                    window,
                    duration,
                    jump,
                }) => {
                    self.program.entries.push(EntryStmt {
                        time,
                        name,
                        sync,
                        window,
                        duration,
                        jump,
                    });
                }
                _ => {
                    panic!("Unexpected statement: {:?}", stmt);
                }
            }
        }

        self.program.clone()
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let token = self.tokenizer.peek_token().unwrap();
        match token.kind {
            TokenKind::Comment => {
                // TODO: Implement comment parsing
                None
            }
            TokenKind::Keyword => {
                let value = token.value?;
                let keyword = value.as_str();
                match keyword {
                    "hideall" => self.parse_hide_all_statement(),
                    "alertall" => self.parse_alert_all_statement(),
                    "define" => self.parse_define_statement(),
                    _ => panic!("Unexpected keyword: {}", keyword),
                }
            }
            TokenKind::NumericLiteral => self.parse_entry_statement(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn parse_hide_all_statement(&mut self) -> Option<Statement> {
        let token = self.tokenizer.next_token().unwrap(); // keyword hideall
        let name = self.tokenizer.next_token().unwrap(); // string literal

        if token.kind != TokenKind::Keyword || name.kind != TokenKind::StringLiteral {
            panic!("Unexpected token types: {:?}, {:?}", token, name);
        }
        Some(Statement::HideAll(HideAllStmt { name: name.value? }))
    }

    fn parse_alert_all_statement(&mut self) -> Option<Statement> {
        let token = self.tokenizer.next_token().unwrap(); // keyword alertall
        let name = self.tokenizer.next_token().unwrap(); // string literal

        if token.kind != TokenKind::Keyword || name.kind != TokenKind::StringLiteral {
            panic!("Unexpected token types: {:?}, {:?}", token, name);
        }

        let mut before: Option<Time> = None;
        let mut sound: Option<String> = None;

        while self.tokenizer.has_next_token() {
            let next_token = self.tokenizer.peek_token().unwrap();
            if next_token.kind != TokenKind::Keyword {
                break;
            }

            let keyword = next_token.value.as_ref().unwrap();
            self.tokenizer.next_token(); // consume keyword

            match keyword.as_str() {
                "before" => {
                    let time_token = self.tokenizer.next_token().unwrap();
                    if time_token.kind != TokenKind::NumericLiteral {
                        panic!("Unexpected token type for before: {:?}", time_token);
                    }
                    let time_literal = time_token.value?;
                    let value = Time::from(time_literal);
                    before = Some(value);
                }
                "sound" => {
                    let sound_token = self.tokenizer.next_token().unwrap();
                    if sound_token.kind != TokenKind::StringLiteral {
                        panic!("Unexpected token type for sound: {:?}", sound_token);
                    }
                    sound = Some(sound_token.value?);
                }
                _ => panic!("Unexpected keyword in alertall: {}", keyword),
            }
        }

        Some(Statement::AlertAll(AlertAllStmt {
            name: name.value?,
            before,
            sound,
        }))
    }

    fn parse_define_statement(&mut self) -> Option<Statement> {
        let _token = self.tokenizer.next_token().unwrap(); // keyword define
        let define_type = self.tokenizer.next_token().unwrap(); // identifier (should be "alertsound")
        if define_type.kind != TokenKind::Identifier
            || define_type.value.as_ref() != Some(&"alertsound".to_string())
        {
            panic!("Unexpected token type for define: {:?}", define_type);
        }

        let name = self.tokenizer.next_token().unwrap(); // string literal
        if name.kind != TokenKind::StringLiteral {
            panic!("Unexpected token type for name: {:?}", name);
        }

        let file = self.tokenizer.next_token().unwrap(); // string literal
        if file.kind != TokenKind::StringLiteral {
            panic!("Unexpected token type for file: {:?}", file);
        }

        Some(Statement::Define(DefineStmt {
            define_type: DefineType::AlertSound,
            name: name.value?,
            file: file.value?,
        }))
    }

    fn parse_entry_statement(&mut self) -> Option<Statement> {
        let token = self.tokenizer.next_token().unwrap(); // numeric literal
        let name = self.tokenizer.next_token().unwrap(); // string literal

        if token.kind != TokenKind::NumericLiteral || name.kind != TokenKind::StringLiteral {
            panic!("Unexpected token types: {:?}, {:?}", token, name);
        }

        let time = Time::from(token.value.unwrap());
        let name_value = name.value.unwrap();

        let mut sync: Option<AnySyncStmt> = None;
        let mut window: Option<WindowStmt> = None;
        let mut duration: Option<DurationStmt> = None;
        let mut jump: Option<JumpStmt> = None;

        while self.tokenizer.has_next_token() {
            let next_token = self.tokenizer.peek_token().unwrap();
            if next_token.kind != TokenKind::Keyword {
                break;
            }

            let keyword = next_token.value.as_ref().unwrap();
            self.tokenizer.peek_token();

            match keyword.as_str() {
                "sync" => {
                    sync = Some(self.parse_sync_statement()?);
                }
                "window" => {
                    window = Some(self.parse_window_statement()?);
                }
                "duration" => {
                    duration = Some(self.parse_duration_statement()?);
                }
                "jump" => {
                    jump = Some(self.parse_jump_statement()?);
                }
                _ if NetSyncLogType::all_keys().contains(&keyword.to_string()) => {
                    sync = Some(self.parse_net_sync_statement()?);
                }
                _ => panic!("Unexpected keyword in entry statement: {}", keyword),
            }
        }

        Some(Statement::Entry(EntryStmt {
            time,
            name: name_value,
            sync,
            window,
            duration,
            jump,
        }))
    }

    fn parse_sync_statement(&mut self) -> Option<AnySyncStmt> {
        let token = self.tokenizer.next_token().unwrap(); // keyword sync
        if token.kind != TokenKind::Keyword || token.value.as_ref() != Some(&"sync".to_string()) {
            panic!("Unexpected token type for sync: {:?}", token);
        }

        let regex_token = self.tokenizer.next_token().unwrap(); // regex literal
        if regex_token.kind != TokenKind::RegularExpression {
            panic!("Unexpected token type for regex: {:?}", regex_token);
        }

        Some(AnySyncStmt::SyncStmt(SyncStmt {
            regex: regex_token.value.unwrap(),
        }))
    }

    fn parse_net_sync_statement(&mut self) -> Option<AnySyncStmt> {
        let type_token = self.tokenizer.next_token().unwrap(); // keyword, e.g. "Ability"
        if type_token.kind != TokenKind::Keyword {
            panic!("Unexpected token type: {:?}", type_token);
        }

        let mut fields: Vec<(String, String)> = Vec::new();

        let left_brace = self.tokenizer.next_token().unwrap();
        if left_brace.kind != TokenKind::Brace || left_brace.value.as_deref() != Some("{") {
            panic!("Unexpected token: {:?}", left_brace);
        }

        while self.tokenizer.has_next_token() {
            let next_token = self.tokenizer.peek_token().unwrap();
            if next_token.kind == TokenKind::Identifier {
                let key_token = self.tokenizer.next_token().unwrap();
                if key_token.kind != TokenKind::Identifier {
                    panic!("Unexpected token: {:?}", key_token);
                }

                let colon_token = self.tokenizer.next_token().unwrap();
                if colon_token.kind != TokenKind::Colon {
                    panic!("Unexpected token: {:?}", colon_token);
                }

                let value_token = self.tokenizer.next_token().unwrap();
                if value_token.kind != TokenKind::StringLiteral
                    && value_token.kind != TokenKind::NumericLiteral
                {
                    panic!("Unexpected token: {:?}", value_token);
                }
                fields.push((key_token.value.unwrap(), value_token.value.unwrap()));
            }

            let right_brace = self.tokenizer.peek_token().unwrap();
            if right_brace.kind == TokenKind::Brace && right_brace.value.as_deref() == Some("}") {
                self.tokenizer.next_token();
                break;
            }
            if right_brace.kind == TokenKind::Punctuator
                && right_brace.value.as_deref() == Some(",")
            {
                self.tokenizer.next_token();
                continue;
            }
            panic!("Unexpected token: {:?}", right_brace);
        }

        Some(AnySyncStmt::NetSyncStmt(NetSyncStmt {
            sync_type: type_token.value.unwrap(),
            fields,
        }))
    }

    fn parse_jump_statement(&mut self) -> Option<JumpStmt> {
        let token = self.tokenizer.next_token().unwrap(); // keyword jump
        if token.kind != TokenKind::Keyword || token.value.as_ref() != Some(&"jump".to_string()) {
            panic!("Unexpected token type for jump: {:?}", token);
        }

        let time = self.tokenizer.next_token().unwrap(); // numeric literal
        if time.kind != TokenKind::NumericLiteral {
            panic!("Unexpected token type for time: {:?}", time);
        }

        Some(JumpStmt {
            time: Time::from(time.value.unwrap()),
        })
    }

    fn parse_duration_statement(&mut self) -> Option<DurationStmt> {
        let token = self.tokenizer.next_token().unwrap(); // keyword duration
        if token.kind != TokenKind::Keyword || token.value.as_ref() != Some(&"duration".to_string())
        {
            panic!("Unexpected token type for duration: {:?}", token);
        }

        let time = self.tokenizer.next_token().unwrap(); // numeric literal
        if time.kind != TokenKind::NumericLiteral {
            panic!("Unexpected token type for time: {:?}", time);
        }

        Some(DurationStmt {
            time: Time::from(time.value.unwrap()),
        })
    }

    fn parse_window_statement(&mut self) -> Option<WindowStmt> {
        let token = self.tokenizer.next_token().unwrap(); // keyword window
        if token.kind != TokenKind::Keyword || token.value.as_ref() != Some(&"window".to_string()) {
            panic!("Unexpected token type for window: {:?}", token);
        }

        let before = self.tokenizer.next_token().unwrap(); // numeric literal
        if before.kind != TokenKind::NumericLiteral {
            panic!("Unexpected token type for time: {:?}", before);
        }

        if self
            .tokenizer
            .peek_token()
            .is_some_and(|t| t.kind == TokenKind::Punctuator)
        {
            let comma = self.tokenizer.next_token().unwrap(); // comma
            if comma.kind != TokenKind::Punctuator || comma.value.as_ref() != Some(&",".to_string())
            {
                panic!("Expected a comma after before time, found: {:?}", comma);
            }

            let after = self.tokenizer.next_token().unwrap(); // numeric literal
            if after.kind != TokenKind::NumericLiteral {
                panic!("Unexpected token type for after time: {:?}", after);
            }

            return Some(WindowStmt {
                before: Time::from(before.value.unwrap()),
                after: Some(Time::from(after.value.unwrap())),
            });
        }

        Some(WindowStmt {
            before: Time::from(before.value.unwrap()),
            after: None,
        })
    }
}
