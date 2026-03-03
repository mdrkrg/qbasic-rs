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
    let mut tokens = vec![];
    let mut errors = vec![];

    // Line start byte offsets (for get_line_col)
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(input.match_indices('\n').map(|(i, _)| i + 1))
        .collect();

    while let Some(result) = lex.next() {
        let span = lex.span();

        match result {
            // Handle unterminated strings
            Ok(Token::UnterminatedString) => {
                let (line, column) = get_line_col(span.start, &line_starts);
                errors.push(LexicalError {
                    message: "String literal was not closed (missing \")".to_string(),
                    line,
                    column,
                    span,
                });
            }
            // Valid token
            Ok(token) => tokens.push(token),
            // Logos returns error
            Err(_) => {
                let (line, column) = get_line_col(span.start, &line_starts);
                errors.push(LexicalError {
                    message: format!("Unexpected character or sequence: '{}'", lex.slice()),
                    line,
                    column,
                    span,
                });
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(tokens)
}

/// Helper to convert byte index to (line, col)
fn get_line_col(byte_offset: usize, line_starts: &[usize]) -> (usize, usize) {
    // Find the line that contains this offset
    match line_starts.binary_search(&byte_offset) {
        Ok(line_idx) => (line_idx + 1, 1), // Matched, start of line
        Err(next_line_idx) => {
            // The index falls inside the previous line
            let line_idx = next_line_idx - 1;
            let line_start = line_starts[line_idx];
            let col = byte_offset - line_start + 1;
            (line_idx + 1, col)
        }
    }
}
