use std::fmt::Display;

use strum_macros;

/// AST definitions.
/// Should not contain raw Tokens.
use crate::core::token::{Math, Relational};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Arithmetic(Math), // +, -, *, /, MOD
    Relational(Relational), // =, <, >, <=, >=
                      // Logic(Logic)
}

#[derive(strum_macros::Display, Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    #[strum(serialize = "-")]
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
#[derive(Debug, Clone)]
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
#[derive(Clone)]
pub struct Line {
    pub lineno: u32,
    pub statement: Stmt,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Arithmetic(math) => write!(f, "{math}"),
            BinaryOp::Relational(relational) => write!(f, "{relational}"),
        }
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Integer(i) => write!(f, "{i}"),
            LiteralValue::Number(d) => write!(f, "{d}"),
            LiteralValue::String(s) => write!(f, "{s}"),
            LiteralValue::None => write!(f, ""),
        }
    }
}
