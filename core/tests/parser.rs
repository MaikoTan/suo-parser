use suo_parser_core::parser::Parser;
use suo_parser_core::{tokenizer::Tokenizer, types::semantic_ast::*};


fn parse(input: &str) -> Program {
    let tokenizer: Tokenizer<_> = input.into();
    let mut parser = Parser::new(tokenizer);
    parser.parse()
}

#[test]
fn test_parser_creation() {
    parse("hideall \"--sync--\"");
}

#[test]
fn test_hideall_statement() {
    let ast = parse("hideall \"--sync--\"");
    insta::assert_debug_snapshot!(ast, @r#"
        Program {
            defines: [],
            hide_alls: [
                HideAllStmt {
                    name: "--sync--",
                },
            ],
            alert_alls: [],
            entries: [],
        }
        "#);
}

#[test]
#[should_panic]
fn test_hideall_statement_panic() {
    parse("hideall 123");
}

#[test]
fn test_alert_all_statement() {
    let ast = parse("alertall \"name\" before 1 sound \"file\"");
    insta::assert_debug_snapshot!(ast, @r#"
        Program {
            defines: [],
            hide_alls: [],
            alert_alls: [
                AlertAllStmt {
                    name: "name",
                    before: Some(
                        Integer(
                            1,
                        ),
                    ),
                    sound: Some(
                        "file",
                    ),
                },
            ],
            entries: [],
        }
        "#);
}

#[test]
#[should_panic]
fn test_alert_all_panic_keywords() {
    parse("alertall \"name\" before sound before 1");
}

#[test]
fn test_define_statement() {
    let ast = parse("define alertsound \"name\" \"file\"");
    insta::assert_debug_snapshot!(ast, @r#"
        Program {
            defines: [
                DefineStmt {
                    define_type: AlertSound,
                    name: "name",
                    file: "file",
                },
            ],
            hide_alls: [],
            alert_alls: [],
            entries: [],
        }
        "#);
}

#[test]
fn test_timeline_entry() {
    let ast = parse(
        "0.0 \"--Reset--\" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0",
    );
    insta::assert_debug_snapshot!(ast, @r#"
        Program {
            defines: [],
            hide_alls: [],
            alert_alls: [],
            entries: [
                EntryStmt {
                    time: Float(
                        0.0,
                    ),
                    name: "--Reset--",
                    sync: Some(
                        SyncStmt(
                            SyncStmt {
                                regex: " 00:0839:.*is no longer sealed",
                            },
                        ),
                    ),
                    window: Some(
                        WindowStmt {
                            before: Integer(
                                10000,
                            ),
                            after: None,
                        },
                    ),
                    duration: Some(
                        DurationStmt {
                            time: Integer(
                                5,
                            ),
                        },
                    ),
                    jump: Some(
                        JumpStmt {
                            time: Integer(
                                0,
                            ),
                        },
                    ),
                },
            ],
        }
        "#);
}

//   it("timeline entry net sync", async () => {
//     const ast = await parseAsync('100.0 "test" Ability { id: "1000", name: "name" } window 10');
//     expect(ast.type).to.equal("Program");
//     expect(ast.body.length).to.equal(1);
//     const stmt = ast.body[0];
//     expect(stmt.type).to.equal("Entry");
//     expect(stmt).to.have.nested.property("name.type", "StringLiteral");
//     expect(stmt).to.have.nested.property("name.value", "test");
//     expect(stmt).to.have.nested.property("sync.syncType", "Ability");
//     expect(stmt).to.have.nested.property("sync.fields.id", "1000");
//     expect(stmt).to.have.nested.property("sync.fields.name", "name");
//     expect(stmt).to.have.nested.property("window.type", "WindowStatement");
//     expect(stmt).to.have.nested.property("window.before.type", "NumericLiteral");
//     expect(stmt).to.have.nested.property("window.before.value", 10);
//   });
// });

#[test]
fn test_timeline_entry_net_sync() {
    let ast = parse("100.0 \"test\" sync / 00:0839:.*is no longer sealed/ window 10");
    insta::assert_debug_snapshot!(ast, @r#"
        Program {
            defines: [],
            hide_alls: [],
            alert_alls: [],
            entries: [
                EntryStmt {
                    time: Float(
                        100.0,
                    ),
                    name: "test",
                    sync: Some(
                        SyncStmt(
                            SyncStmt {
                                regex: " 00:0839:.*is no longer sealed",
                            },
                        ),
                    ),
                    window: Some(
                        WindowStmt {
                            before: Integer(
                                10,
                            ),
                            after: None,
                        },
                    ),
                    duration: None,
                    jump: None,
                },
            ],
        }
        "#);
}
