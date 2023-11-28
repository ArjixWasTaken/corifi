use std::{fmt, error};

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

use self::{types::{Token, Range}, utils::SpanManager};

mod types;
mod utils;


pub fn lex<S: AsRef<str>>(input: S) -> Vec<Token> {
    let chars = input.as_ref().chars().collect::<Vec<_>>();

    let mut errors: Vec<Range<usize>> = vec![];
    let mut tokens: Vec<Token> = vec![];
    let mut spanman = SpanManager::new();

    let mut idx: usize = 0;

    while idx < chars.len() {
        let mut tok: Option<Token> = None;

        macro_rules! single_char {
            ($tokType: expr) => {
                tok = Some($tokType(spanman.consume(vec![chars[idx]])))
            };
        }

        match chars[idx].into() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                let mut span = spanman.state();

                spanman.waste(chars[idx]);

                while idx+1 < chars.len() && chars[idx+1].is_numeric() {
                    spanman.waste(chars[idx+1]);
                    idx += 1;
                }

                span.len = spanman.col - span.col;

                tok = Some(Token::Int(span))
            },

            '#' => single_char!(Token::Pound),
            ',' => single_char!(Token::Comma),
            ';' => single_char!(Token::SemiColon),
            '+' => single_char!(Token::Operator),
            '-' => single_char!(Token::Operator),

            ' ' | '\n' | '\r' | '\t' => spanman.waste(chars[idx]), 
            
            _ => {
                errors.push(spanman.state().range);
                spanman.waste(chars[idx]);
            }
        }

        if let Some(token) = tok {
            tokens.push(token);
        }
        
        idx += 1;
    }

    if !errors.is_empty() {
        // TODO: Combine sibling errors.

        let mut i = 1;
        while i < errors.len() {
            if errors[i-1].end == errors[i].start - 1 {
                errors[i-1] = Range {
                    start: errors[i-1].start,
                    end: errors[i].end
                };

                errors.remove(i);
                i -= 1;
            }

            i += 1;
        }

        let mut files = SimpleFiles::new();
        let file_id = files.add("[stdin]", input.as_ref());

        for error in errors {
            let diagnostic = Diagnostic::error()
                .with_message("Unexpected character.")
                .with_labels(vec![
                    Label::primary(file_id, error.start..error.end+1)
                        .with_message(format!("'{}' was unexpected", chars[error.start..error.end+1].into_iter().collect::<String>())),
                ]);

            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();

            term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
        }
    }

    tokens
}
