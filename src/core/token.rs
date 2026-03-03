use logos::{Lexer, Logos};
use std::str::FromStr;
use strum::Display;
use strum_macros::EnumString;

#[derive(EnumString, Debug, PartialEq, Clone)]
pub enum Keyword {
    #[strum(ascii_case_insensitive)]
    Rem(String),
    #[strum(ascii_case_insensitive)]
    Let,
    #[strum(ascii_case_insensitive)]
    Print,
    #[strum(ascii_case_insensitive)]
    Input,
    #[strum(ascii_case_insensitive)]
    If,
    #[strum(ascii_case_insensitive)]
    Then,
    #[strum(ascii_case_insensitive)]
    Goto,
    #[strum(ascii_case_insensitive)]
    End,
}

#[derive(EnumString, Debug, PartialEq, Clone, Copy)]
/// Math expression operators
pub enum Math {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Times,
    #[strum(serialize = "/")]
    Division,
    #[strum(serialize = "**")]
    Power,
    #[strum(serialize = "MOD", ascii_case_insensitive)]
    Modulo,
}

#[derive(EnumString, Debug, PartialEq, Clone, Copy)]
/// Math expression groupings
pub enum Side {
    #[strum(serialize = "(")]
    Left,
    #[strum(serialize = ")")]
    Right,
}

#[derive(EnumString, Debug, PartialEq, Clone, Copy)]
/// Relational operators
pub enum Relational {
    #[strum(serialize = "<")]
    Lt,
    #[strum(serialize = "<=")]
    LtEq,
    #[strum(serialize = ">")]
    Gt,
    #[strum(serialize = ">=")]
    GtEq,
    #[strum(serialize = "<>")]
    NotEq,
    #[strum(serialize = "=")]
    Eq, // need to convert Token::Equal to this in conditional expressions
}

#[derive(Debug, Logos, PartialEq, Clone, Display)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    // Handle the Apostrophe shorthand
    #[regex(r"'[^\n]*", extract_comment_text, allow_greedy = true)]
    Comment(String),
    #[regex(r"(?i)REM[ \t\f][^\n]*", extract_rem_text, allow_greedy = true)]
    #[regex(
        "(?i)(LET|PRINT|INPUT|IF|THEN|GOTO|END)",
        |lex| Keyword::from_str(lex.slice()).unwrap(),
    )]
    /// Keywords
    Keyword(Keyword),

    #[token("+",  |_| Math::Plus)]
    #[token("-", |_| Math::Minus)]
    #[token("*",  |_| Math::Times)]
    #[token("/", |_| Math::Division)]
    #[token("**", |_| Math::Power)]
    #[token("MOD", |_| Math::Modulo)]
    Operator(Math),

    #[token("<",  |_| Relational::Lt)]
    #[token("<=", |_| Relational::LtEq)]
    #[token(">",  |_| Relational::Gt)]
    #[token(">=", |_| Relational::GtEq)]
    #[token("<>", |_| Relational::NotEq)]
    Relational(Relational),

    #[token("(", |_| Side::Left)]
    #[token(")", |_| Side::Right)]
    Paran(Side),

    /// Equal sign
    /// Special: can be assign and equals
    #[token("=")]
    Equal,

    #[regex(r"[a-zA-Z][a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // WARN: This will panic if cannot parse u32, and what if we want i32?
    #[regex(
        r"[0-9]+",
        |lex| lex.slice().parse::<u32>().unwrap(),
        priority = 8,
    )]
    Integer(u32),

    // currently floating points not supported
    // #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    // Number(f64),
    //
    #[regex(
        r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#,
        // trim starting and ending '"'
        |lex| lex.slice()[1..lex.slice().len()-1].to_owned(),
    )]
    String(String),

    /// Unterminated string
    /// Matches a quote followed by anything until EOF, should be an error (in lexer or parser)
    #[regex(r#""[^"]*"#, priority = 0)]
    UnterminatedString,

    #[regex(r"\n+")]
    // Newlines are statement separators
    Newline,
}

/// Helper to clean up the comment, trims "'" comment identifier
fn extract_comment_text(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    slice.trim_start_matches("'").to_string()
}

/// Helper to clean up the REM comment
fn extract_rem_text(lex: &mut Lexer<Token>) -> Keyword {
    let slice = lex.slice().trim_start_matches("REM ").to_string();
    Keyword::Rem(slice)
}
