/// The parser for QBasic. It should be input a sequence of Token and output a sequence of Line.
use crate::core::{
    ast::{BinaryOp, Expr, Line, LiteralValue, Stmt, UnaryOp},
    token::{Keyword, Math, Relational, Side, Token},
};
use anyhow::{Result, bail};
use std::collections::VecDeque;

/// A recursive decendent parser
pub struct Parser {
    pub tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Line>> {
        let mut lines = vec![];
        while self.peek().is_some() {
            let line = self.line();
            match line {
                Ok(line) => {
                    lines.push(line);
                }
                Err(err) => {
                    bail!("Error parsing: {err}")
                }
            }
        }
        Ok(lines)
    }

    pub fn line(&mut self) -> Result<Line> {
        let lineno = match self.advance() {
            Some(Token::Integer(lineno)) => lineno,
            Some(token) => bail!("Expected line number, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        let keyword = match self.advance() {
            // FIXME: Don't know whether suitable for a REM
            Some(Token::Comment(comment)) => Keyword::Rem(comment),
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
            Keyword::Rem(comment) => Stmt::Rem { comment },
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
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            Some(token) => bail!("Expected identifier, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        match self.advance() {
            Some(Token::Equal) => (),
            Some(token) => bail!("Expected assignment, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        let expr = self.expression()?;
        Ok(Stmt::Let { name, expr })
    }

    fn parse_goto(&mut self) -> Result<Stmt> {
        let lineno = match self.advance() {
            Some(Token::Integer(lineno)) => lineno,
            Some(token) => bail!("Expected line number, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        Ok(Stmt::Goto { lineno })
    }

    fn parse_if(&mut self) -> Result<Stmt> {
        let conditional = self.conditional()?;
        match self.advance() {
            Some(Token::Keyword(Keyword::Then)) => (),
            Some(token) => bail!("Expected THEN, got {token}"),
            None => bail!("Expected THEN, got EOF"),
        }
        let lineno = match self.advance() {
            Some(Token::Integer(lineno)) => lineno,
            Some(token) => bail!("Expected line number, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        Ok(Stmt::IfThen {
            conditional,
            lineno,
        })
    }

    fn parse_print(&mut self) -> Result<Stmt> {
        // Handle print a single new line
        if matches!(self.peek(), Some(Token::Newline) | None) {
            return Ok(Stmt::Print {
                expr: Expr::Literal(LiteralValue::None),
            });
        }
        let expr = self.expression()?;
        Ok(Stmt::Print { expr })
    }

    fn parse_input(&mut self) -> Result<Stmt> {
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            Some(token) => bail!("Expected identifier, got {token}"),
            None => bail!("Unexpected EOF"),
        };
        Ok(Stmt::Input { name })
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
        let left = self.expression()?;

        let op = match self.advance() {
            Some(Token::Equal) => Relational::Eq,
            Some(Token::Relational(relation)) => relation,
            Some(token) => bail!("Expected relational operators, got {token}"),
            None => bail!("Unexpected EOF"),
        };

        let right = self.expression()?;

        Ok(Expr::Binary {
            operator: BinaryOp::Relational(op),
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// expression -> term
    /// FIXME: it should not allow for 1 + -1
    fn expression(&mut self) -> Result<Expr> {
        self.term()
    }

    /// term -> factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        loop {
            let operator = match self.peek() {
                Some(Token::Operator(op))
                    if matches!(op, Math::Plus | Math::Minus | Math::Modulo) =>
                {
                    *op
                }
                _ => break,
            };
            let _ = self.advance();

            let right = self.factor()?;
            let prev_expr = expr;
            expr = Expr::Binary {
                operator: BinaryOp::Arithmetic(operator),
                left: Box::new(prev_expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    /// factor -> power ( ( "/" | "*" ) power )*
    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.power()?;

        loop {
            let operator = match self.peek() {
                Some(Token::Operator(op)) if matches!(op, Math::Times | Math::Division) => *op,
                _ => break,
            };
            let _ = self.advance();

            let right = self.power()?;
            let prev_expr = expr;
            expr = Expr::Binary {
                operator: BinaryOp::Arithmetic(operator),
                left: Box::new(prev_expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    /// power -> ( ( unary "**" )* unary ) // right associative
    fn power(&mut self) -> Result<Expr> {
        // Base
        let left = self.unary()?;

        match self.peek() {
            Some(Token::Operator(op)) if matches!(op, Math::Power) => {
                let operator = *op; // Math::Power
                // Consume
                self.advance();

                // Right recursion
                let right = self.power()?;

                Ok(Expr::Binary {
                    operator: BinaryOp::Arithmetic(operator),
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    /// unary -> "-" unary | primary
    fn unary(&mut self) -> Result<Expr> {
        if let Some(Token::Operator(Math::Minus)) = self.peek() {
            self.advance();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator: UnaryOp::Negate,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    /// primary -> NUMBER | STRING | "(" expression ")"
    fn primary(&mut self) -> Result<Expr> {
        let literal = match self.advance() {
            Some(Token::Integer(number)) => Expr::Literal(LiteralValue::Integer(number)),
            Some(Token::String(str)) => Expr::Literal(LiteralValue::String(str)),
            Some(Token::Identifier(name)) => Expr::Variable { name },
            Some(Token::Paran(Side::Left)) => {
                let expr = self.expression()?;
                if !matches!(self.advance(), Some(Token::Paran(Side::Right))) {
                    bail!("Expected ')' after expression");
                }
                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => bail!("Expected expression"),
        };

        Ok(literal)
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
