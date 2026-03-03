use crate::{
    ast::{Expr, Line, Stmt},
    token::{Keyword, Token},
};
use anyhow::{Result, bail};
use std::collections::VecDeque;

/// A recursive decendent parser
pub struct Parser {
    pub tokens: VecDeque<Token>,
}

impl Parser {
    pub fn line(&mut self) -> Result<Line> {
        let lineno = match self.advance() {
            Some(Token::Integer(lineno)) => lineno,
            Some(token) => bail!("Expected line number, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        let keyword = match self.advance() {
            Some(Token::Keyword(keyword)) => keyword,
            Some(token) => bail!("Expected keyword, got {token}"),
            None => bail!("Unexpected EOF"),
        };

        let statement = match keyword {
            Keyword::Let => self.parse_let()?,
            Keyword::Goto => self.parse_goto()?,
            Keyword::If => self.parse_if()?,
            Keyword::Print => self.parse_print()?,
            Keyword::Input => self.parse_input()?,
            Keyword::End => self.parse_end()?,
            Keyword::Rem(comment) => Stmt::Rem {
                comment: Token::Comment(comment),
            },
            Keyword::Then => bail!("Unexpected THEN at start of statement"),
        };

        // Handle end of line
        match self.peek() {
            Some(Token::Newline) => {
                self.advance();
            }
            None => {
                // EOF is a valid EOL for the last line
            }
            Some(token) => bail!("Expected end of line, found {}", token),
        }

        Ok(Line { lineno, statement })
    }

    fn parse_let(&mut self) -> Result<Stmt> {
        todo!()
    }

    fn parse_goto(&mut self) -> Result<Stmt> {
        let lineno = match self.advance() {
            Some(Token::Integer(lineno)) => lineno,
            Some(token) => bail!("Expected line number, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        Ok(Stmt::Goto {
            lineno: Token::Integer(lineno),
        })
    }

    fn parse_if(&mut self) -> Result<Stmt> {
        todo!()
    }

    fn parse_print(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        Ok(Stmt::Print { expr })
    }

    fn parse_input(&mut self) -> Result<Stmt> {
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            Some(token) => bail!("Expected identifier, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        Ok(Stmt::Input {
            name: Token::Identifier(name),
        })
    }

    fn parse_end(&mut self) -> Result<Stmt> {
        loop {
            let token = self.advance();
            match token {
                Some(Token::Newline) => continue,
                Some(_) => bail!("Unexpected statement after END"),
                None => break,
            };
        }
        Ok(Stmt::End)
    }

    // Syntax definitions

    /// In QBasic, a conditional is only a conditional operator
    /// with expressions on both sides
    fn conditional(&mut self) -> Result<Expr> {
        todo!()
    }

    /// expression -> term
    fn expression(&mut self) -> Result<Expr> {
        self.term()
    }

    /// term -> factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Result<Expr> {
        todo!()
    }
    /// factor -> power ( ( "/" | "*" ) power )*
    fn factor(&mut self) -> Result<Expr> {
        todo!()
    }

    /// power -> ( ( unary "**" )* unary ) // right associative
    fn power(&mut self) -> Result<Expr> {
        todo!()
    }

    /// unary -> "-" unary | primary
    fn unary(&mut self) -> Result<Expr> {
        todo!()
    }

    /// primary -> NUMBER | STRING | "(" expression ")"
    fn primary(&mut self) -> Result<Expr> {
        todo!()
    }

    /// Consume a token and return it
    fn advance(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    /// Peek at the next token
    fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }
}
