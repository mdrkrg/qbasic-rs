use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
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
}

/// A QBasic statement
pub enum Stmt {
    Rem {
        comment: Token, // String?
    },
    Let {
        name: String,
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    Input {
        name: Token, // String?
    },
    Goto {
        lineno: Token, // u32?
    },
    IfThen {
        conditional: Expr,
        lineno: Token, // u32?
    },
    End,
}

/// A line of QBasic code
pub struct Line {
    pub lineno: u32,
    pub statement: Stmt,
}
