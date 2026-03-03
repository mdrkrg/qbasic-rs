use qbasic_rs::core::ast::*;
use qbasic_rs::core::eval::{Context, Interpreter};
use qbasic_rs::core::token::{Math, Relational};

// Helper to create a Context for testing
#[allow(dead_code)]
pub fn create_test_context() -> Context {
    Context::default()
}

// Helper to create an Interpreter for testing
#[allow(dead_code)]
pub fn create_test_interpreter() -> Interpreter {
    Interpreter::default()
}

// Helper to create a simple integer expression
#[allow(dead_code)]
pub fn int_expr(value: u32) -> Expr {
    Expr::Literal(LiteralValue::Integer(value))
}

// Helper to create a variable expression
#[allow(dead_code)]
pub fn var_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.to_string(),
    }
}

// Helper to create a binary arithmetic expression
#[allow(dead_code)]
pub fn arithmetic_expr(left: Expr, op: Math, right: Expr) -> Expr {
    Expr::Binary {
        operator: BinaryOp::Arithmetic(op),
        left: Box::new(left),
        right: Box::new(right),
    }
}

// Helper to create a binary arithmetic expression
#[allow(dead_code)]
pub fn binary_expr(left: Expr, op: Math, right: Expr) -> Expr {
    Expr::Binary {
        operator: BinaryOp::Arithmetic(op),
        left: Box::new(left),
        right: Box::new(right),
    }
}

// Helper to create a unary expression
#[allow(dead_code)]
pub fn unary_expr(op: UnaryOp, right: Expr) -> Expr {
    Expr::Unary {
        operator: op,
        right: Box::new(right),
    }
}

// Helper to create a binary relational expression
#[allow(dead_code)]
pub fn relational_expr(left: Expr, op: Relational, right: Expr) -> Expr {
    Expr::Binary {
        operator: BinaryOp::Relational(op),
        left: Box::new(left),
        right: Box::new(right),
    }
}
