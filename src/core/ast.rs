/// AST definitions.
/// Should not contain raw Tokens.
use crate::core::token::{Math, Relational};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Arithmetic(Math), // +, -, *, /, MOD
    Relational(Relational), // =, <, >, <=, >=
                      // Logic(Logic)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate, // -
            // Not,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        operator: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(LiteralValue),
    Variable {
        name: String,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(u32),
    Number(f64),
    String(String),
    None, // for printing a new line
}

/// A QBasic statement
#[derive(Debug)]
pub enum Stmt {
    Rem { comment: String },
    Let { name: String, expr: Expr },
    Print { expr: Expr },
    Input { name: String },
    Goto { lineno: u32 },
    IfThen { conditional: Expr, lineno: u32 },
    End,
}

/// A line of QBasic code
pub struct Line {
    pub lineno: u32,
    pub statement: Stmt,
}
