use std::{fmt::Display, num::ParseIntError};

use ariadne::{Report, ReportKind, Source};
use logos::Logos;
use rust_decimal::prelude::*;

use crate::utils::strings::DotDisplay;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"\s")]
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

    #[regex(r"\d+(\.\d+)?", |lex| Decimal::from_f64(lex.slice().parse::<f64>().unwrap()), priority = 2)]
    Number(Decimal),

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

    #[regex(r"[\w_]+", |lex| lex.slice().to_string(), priority = 1)]
    Ident(String),

    #[token("{")]
    BlockStart,

    #[token("}")]
    BlockEnd,
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

                Self::LParen => "(".into(),
                Self::RParen => ")".into(),
                Self::Comma => ",".into(),

                Self::Number(n) => n.display(),
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
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    message: String,
    code: usize,
}

impl LexerError {
    pub fn new(message: &str, code: usize) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }

    pub fn other() -> Self {
        Self {
            message: "Unknown error.".into(),
            code: 0,
        }
    }

    pub fn print(&self, input: &str) {
        Report::build(ReportKind::Error, ("input", 12..12))
            .with_code(self.code)
            .with_message(&self.message)
            .finish()
            .print(("input", Source::from(input)))
            .unwrap();
    }
}

impl Default for LexerError {
    fn default() -> Self {
        Self::other()
    }
}

impl From<ParseIntError> for LexerError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind as E;
        match err.kind() {
            E::PosOverflow | E::NegOverflow => LexerError::new("Overflow error.", 1),
            E::InvalidDigit => LexerError::new("Invalid digit.", 2),

            _ => LexerError::other(),
        }
    }
}
