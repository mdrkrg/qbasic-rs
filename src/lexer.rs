use crate::token::Token;
use anyhow::Result;
use logos::Logos;
use std::ops::Range;

#[derive(Debug)]
pub struct LexicalError {
    /// The error message
    pub message: String,
    /// The line of the error token
    pub line: usize,
    /// The column of the error token
    pub column: usize,
    /// The range of the error token, from `Lexer.span`
    pub span: Range<usize>,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Vec<LexicalError>> {
    let mut lex = Token::lexer(input);
    todo!()
}
