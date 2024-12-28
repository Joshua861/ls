use std::fmt::Display;

use ariadne::{Label, Report, ReportKind, Source};
use logos::Logos;
use rust_decimal::prelude::*;
use strum::EnumIs;

use crate::utils::strings::DotDisplay;

#[derive(Logos, Debug, PartialEq, Clone, Eq, Hash, EnumIs)]
#[logos(skip r"\s+")]
#[logos(error = LexerError)]
pub enum Token {
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("%")]
    Modulo,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(",")]
    Comma,

    #[token(";")]
    Eol,

    #[token("let")]
    Let,

    #[token(":=")]
    AssignTo,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token(">=")]
    GreaterEqual,

    #[token("<=")]
    LessEqual,

    #[token("!")]
    Not,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("^")]
    Xor,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[regex(r"\d+(\.\d+)?", |lex| {
        let slice = lex.slice();
        match Decimal::from_str(slice) {
            Ok(n) => Ok(n),
            Err(e) => Err(LexerError::new(
                LexerErrorKind::ParseDecimalError(e.to_string()),
                lex.span()
            ))
        }
    }, priority = 2)]
    Number(Decimal),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice: &str = lex.slice();
        if slice.len() < 2 {
            return Err(LexerError::new(
                LexerErrorKind::UnterminatedString,
                lex.span()
            ));
        }
        Ok(rem_first_and_last(slice.to_string()))
    }, priority = 2)]
    String(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| {
        let slice: &str = lex.slice();
        if slice.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(slice.to_string())
        } else {
            Err(LexerError::new(
                LexerErrorKind::InvalidIdentifier(slice.to_string()),
                lex.span()
            ))
        }
    }, priority = 1)]
    Ident(String),

    #[token("{")]
    BlockStart,

    #[token("}")]
    BlockEnd,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[regex(r"//.*", |lex| {let slice: &str = lex.slice(); slice.to_string()}, priority = 1)]
    Comment(String),

    #[token("[")]
    ArrayStart,

    #[token("]")]
    ArrayEnd,

    #[token(".")]
    Dot,

    #[token("elif")]
    ElseIf,
}

fn rem_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.clone() {
                Self::Plus => "+".into(),
                Self::Minus => "-".into(),
                Self::Multiply => "*".into(),
                Self::Divide => "/".into(),
                Self::Modulo => "%".into(),

                Self::Number(n) => n.display(),
                Self::String(s) => s,
                Self::Ident(s) => s.clone(),
                Self::Eol => ";".into(),

                Self::Let => "let ".into(),
                Self::AssignTo => ":=".into(),

                Self::True => "true".into(),
                Self::False => "false".into(),

                Self::GreaterThan => ">".into(),
                Self::LessThan => "<".into(),
                Self::GreaterEqual => ">=".into(),
                Self::LessEqual => "<=".into(),
                Self::Equals => "==".into(),
                Self::NotEquals => "!=".into(),

                Self::Not => "!".into(),
                Self::And => "&&".into(),
                Self::Or => "||".into(),
                Self::Xor => "^".into(),

                Self::BlockStart => "{".into(),
                Self::BlockEnd => "}".into(),
                Self::ArrayStart => "[".into(),
                Self::ArrayEnd => "]".into(),
                Self::LParen => "(".into(),
                Self::RParen => ")".into(),
                Self::Comma => ",".into(),
                Self::Dot => ".".into(),

                Self::If => "if".into(),
                Self::Else => "else".into(),
                Self::ElseIf => "elif".into(),

                Self::Comment(s) => s,
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorKind {
    InvalidNumber(String),
    InvalidIdentifier(String),
    UnterminatedString,
    UnexpectedCharacter(char),
    InvalidOperator(String),
    UnmatchedDelimiter(char),
    InvalidFunction(String),
    UnknownToken(String),
    ParseDecimalError(String),
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    kind: LexerErrorKind,
    span: std::ops::Range<usize>,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, span: std::ops::Range<usize>) -> Self {
        Self { kind, span }
    }

    pub fn print(&self, input: &str) {
        let message = match &self.kind {
            LexerErrorKind::InvalidNumber(n) => format!("Invalid number: {}", n),
            LexerErrorKind::InvalidIdentifier(id) => format!("Invalid identifier: {}", id),
            LexerErrorKind::UnterminatedString => "Unterminated string literal".to_string(),
            LexerErrorKind::UnexpectedCharacter(c) => format!("Unexpected character: {}", c),
            LexerErrorKind::InvalidOperator(op) => format!("Invalid operator: {}", op),
            LexerErrorKind::UnmatchedDelimiter(d) => format!("Unmatched delimiter: {}", d),
            LexerErrorKind::InvalidFunction(f) => format!("Invalid function call: {}", f),
            LexerErrorKind::UnknownToken(t) => format!("Unknown token: {}", t),
            LexerErrorKind::ParseDecimalError(e) => format!("Failed to parse decimal: {}", e),
            LexerErrorKind::Other(msg) => msg.clone(),
        };

        let report = Report::build(ReportKind::Error, self.span.clone())
            .with_message(&message)
            .with_label(Label::new(self.span.clone()).with_message("Error occurred here"))
            .finish();

        report.print(Source::from(input)).unwrap();
    }
}

impl Default for LexerError {
    fn default() -> Self {
        Self {
            kind: LexerErrorKind::Other("Unknown error".to_string()),
            span: std::ops::Range { start: 0, end: 0 },
        }
    }
}

// impl logos::Logos for Token {
//     type Error = LexerError;
//
//     fn error(lex: &mut logos::Lexer<Self>) -> Self::Error {
//         let slice = lex.slice();
//         let span = lex.span();
//
//         if let Some(c) = slice.chars().next() {
//             if c.is_numeric() {
//                 LexerError::new(LexerErrorKind::InvalidNumber(slice.to_string()), span)
//             } else if c.is_alphabetic() {
//                 LexerError::new(LexerErrorKind::InvalidIdentifier(slice.to_string()), span)
//             } else {
//                 LexerError::new(LexerErrorKind::UnexpectedCharacter(c), span)
//             }
//         } else {
//             LexerError::new(LexerErrorKind::Other("Empty token".to_string()), span)
//         }
//     }
// }
//
// Function to find function calls in the input
fn detect_function_calls(input: &str) -> Result<(), LexerError> {
    let function_pattern = regex::Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    let valid_functions = ["sqrt", "rand", "rand_between", "max", "print", "join"];

    for capture in function_pattern.captures_iter(input) {
        let function_name = capture.get(1).unwrap().as_str();
        if !valid_functions.contains(&function_name) {
            return Err(LexerError::new(
                LexerErrorKind::InvalidFunction(function_name.to_string()),
                capture.get(1).unwrap().range(),
            ));
        }
    }
    Ok(())
}

// Helper function to validate the input before lexing
pub fn validate_input(input: &str) -> Result<(), LexerError> {
    // Check for function calls
    detect_function_calls(input)?;

    // Check for balanced delimiters
    let mut stack = Vec::new();
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' | '{' | '[' => stack.push((c, i)),
            ')' | '}' | ']' => {
                if let Some((open, _)) = stack.pop() {
                    let is_match = match (open, c) {
                        ('(', ')') | ('{', '}') | ('[', ']') => true,
                        _ => false,
                    };
                    if !is_match {
                        return Err(LexerError::new(
                            LexerErrorKind::UnmatchedDelimiter(c),
                            i..i + 1,
                        ));
                    }
                } else {
                    return Err(LexerError::new(
                        LexerErrorKind::UnmatchedDelimiter(c),
                        i..i + 1,
                    ));
                }
            }
            _ => {}
        }
    }

    if let Some((c, i)) = stack.pop() {
        return Err(LexerError::new(
            LexerErrorKind::UnmatchedDelimiter(c),
            i..i + 1,
        ));
    }

    Ok(())
}

// Example usage in your main lexing function
pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    // Validate input first
    validate_input(input)?;

    // Then proceed with lexing
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(input);

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => tokens.push(token),
            Err(error) => return Err(error),
        }
    }

    Ok(tokens)
}
