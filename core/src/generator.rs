use crate::types::semantic_ast::{AnySyncStmt, EntryStmt, HideAllStmt, Program, Time};

/// Generate timeline text from a parsed [Program].
pub struct Generator {
    ast: Program,
}

impl Generator {
    pub fn new(ast: Program) -> Self {
        Self { ast }
    }

    pub fn generate(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        for hide_all in &self.ast.hide_alls {
            lines.push(self.generate_statement(hide_all));
        }
        for entry in &self.ast.entries {
            lines.push(self.generate_entry(entry));
        }

        lines.join("\n")
    }

    fn generate_statement(&self, stmt: &HideAllStmt) -> String {
        let name = escape_string(&stmt.name);
        format!("hideall \"{}\"", name)
    }

    fn generate_entry(&self, stmt: &EntryStmt) -> String {
        let time = format_time(&stmt.time);
        let name = escape_string(&stmt.name);
        let mut ret = format!("{} \"{}\"", time, name);

        if let Some(sync) = &stmt.sync {
            match sync {
                AnySyncStmt::SyncStmt(s) => {
                    let regex = escape_regex(&s.regex);
                    ret.push_str(&format!(" sync /{}/", regex));
                }
                AnySyncStmt::NetSyncStmt(n) => {
                    let fields = n
                        .fields
                        .iter()
                        .map(|(key, value)| format!("{}: \"{}\"", key, value))
                        .collect::<Vec<_>>()
                        .join(", ");
                    ret.push_str(&format!(" {} {{ {} }}", n.sync_type, fields));
                }
            }
        }

        if let Some(duration) = &stmt.duration {
            ret.push_str(&format!(" duration {}", simplify_num(&duration.time)));
        }

        if let Some(window) = &stmt.window {
            let before = simplify_num(&window.before);
            if let Some(after) = &window.after {
                if !times_equal(&window.before, after) {
                    ret.push_str(&format!(" window {},{}", before, simplify_num(after)));
                } else {
                    ret.push_str(&format!(" window {}", before));
                }
            } else {
                ret.push_str(&format!(" window {}", before));
            }
        }

        if let Some(jump) = &stmt.jump {
            ret.push_str(&format!(" jump {}", simplify_num(&jump.time)));
        }

        ret
    }
}

/// Formats an entry time like JS toFixed(1): always one decimal digit.
fn format_time(time: &Time) -> String {
    match time {
        Time::Integer(value) => format!("{}.0", value),
        Time::Float(value) => format!("{:.1}", value),
    }
}

/// Formats a statement time like JS simplifyNum: integers without decimals,
/// floats with exactly one decimal digit.
fn simplify_num(time: &Time) -> String {
    match time {
        Time::Integer(value) => value.to_string(),
        Time::Float(value) => format!("{:.1}", value),
    }
}

fn times_equal(a: &Time, b: &Time) -> bool {
    match (a, b) {
        (Time::Integer(x), Time::Integer(y)) => x == y,
        (Time::Float(x), Time::Float(y)) => x == y,
        (Time::Integer(x), Time::Float(y)) => *x as f64 == *y,
        (Time::Float(x), Time::Integer(y)) => *x == *y as f64,
    }
}

/// Escapes a string literal body, mirroring the original TypeScript
/// Generator#escapeString behaviour (replaces the first occurrence of each
/// special character, in order).
fn escape_string(input: &str) -> String {
    let mut ret = input.to_string();
    for (from, to) in [('"', "\\\""), ('\n', "\\n"), ('\r', "\\r"), ('\t', "\\t")] {
        if let Some(pos) = ret.find(from) {
            ret.replace_range(pos..pos + from.len_utf8(), to);
        }
    }
    ret
}

/// Escapes a regular expression body: first all backslashes are doubled, then
/// every slash that is not already escaped is prefixed with a backslash.
fn escape_regex(input: &str) -> String {
    let doubled = input.replace('\\', "\\\\");
    let mut out = String::with_capacity(doubled.len());
    let mut prev_backslash = false;
    for c in doubled.chars() {
        if c == '/' && !prev_backslash {
            out.push('\\');
        }
        out.push(c);
        prev_backslash = c == '\\';
    }
    out
}
