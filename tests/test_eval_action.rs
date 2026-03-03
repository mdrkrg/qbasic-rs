use qbasic_rs::core::ast::*;
use qbasic_rs::core::eval::action::Action;
use qbasic_rs::core::eval::value::{Context, Value};
use qbasic_rs::core::token::{Math, Relational};

mod utils;
use utils::{binary_expr, int_expr, relational_expr, var_expr};

// Helper to create a Context for testing
fn create_test_context() -> Context {
    Context::default()
}

#[test]
fn test_let_statement_execution() {
    let ctx = create_test_context();
    let stmt = Stmt::Let {
        name: "X".to_string(),
        expr: int_expr(42),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Assign(name, value) => {
            assert_eq!(name, "X");
            assert_eq!(value, Value::Integer(42));
        }
        _ => panic!("Expected Assign action"),
    }
}

#[test]
fn test_let_statement_with_expression() {
    let ctx = create_test_context();
    // LET Y = 10 + 5
    let expr = binary_expr(int_expr(10), Math::Plus, int_expr(5));
    let stmt = Stmt::Let {
        name: "Y".to_string(),
        expr,
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Assign(name, value) => {
            assert_eq!(name, "Y");
            assert_eq!(value, Value::Integer(15));
        }
        _ => panic!("Expected Assign action"),
    }
}

#[test]
fn test_print_statement_integer() {
    let ctx = create_test_context();
    let stmt = Stmt::Print {
        expr: int_expr(123),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Output(output) => {
            assert_eq!(output, "123");
        }
        _ => panic!("Expected Output action"),
    }
}

#[test]
fn test_print_statement_string() {
    let ctx = create_test_context();
    let stmt = Stmt::Print {
        expr: Expr::Literal(LiteralValue::String("\"Hello\"".to_string())),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Output(output) => {
            assert_eq!(output, "\"Hello\"");
        }
        _ => panic!("Expected Output action"),
    }
}

#[test]
fn test_print_statement_empty() {
    let ctx = create_test_context();
    let stmt = Stmt::Print {
        expr: Expr::Literal(LiteralValue::None),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Output(output) => {
            assert_eq!(output, "\n");
        }
        _ => panic!("Expected Output action"),
    }
}

#[test]
fn test_print_statement_variable() {
    let mut ctx = create_test_context();
    ctx.variables.insert("A".to_string(), Value::Integer(99));

    let stmt = Stmt::Print {
        expr: var_expr("A"),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Output(output) => {
            assert_eq!(output, "99");
        }
        _ => panic!("Expected Output action"),
    }
}

#[test]
fn test_input_statement() {
    let ctx = create_test_context();
    let stmt = Stmt::Input {
        name: "NAME".to_string(),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Input(var_name) => {
            assert_eq!(var_name, "NAME");
        }
        _ => panic!("Expected Input action"),
    }
}

#[test]
fn test_goto_statement() {
    let ctx = create_test_context();
    let stmt = Stmt::Goto { lineno: 50 };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Jump(lineno) => {
            assert_eq!(lineno, 50);
        }
        _ => panic!("Expected Jump action"),
    }
}

#[test]
fn test_if_then_statement_true_condition() {
    let ctx = create_test_context();
    // IF 5 < 10 THEN 100
    let conditional = relational_expr(int_expr(5), Relational::Lt, int_expr(10));
    let stmt = Stmt::IfThen {
        conditional,
        lineno: 100,
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Jump(lineno) => {
            assert_eq!(lineno, 100);
        }
        _ => panic!("Expected Jump action for true condition"),
    }
}

#[test]
fn test_if_then_statement_false_condition() {
    let ctx = create_test_context();
    // IF 10 < 5 THEN 100
    let conditional = relational_expr(int_expr(10), Relational::Lt, int_expr(5));
    let stmt = Stmt::IfThen {
        conditional,
        lineno: 100,
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Continue => (), // Should not jump
        _ => panic!("Expected Continue action for false condition"),
    }
}

#[test]
fn test_end_statement() {
    let ctx = create_test_context();
    let stmt = Stmt::End;

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::End => (),
        _ => panic!("Expected End action"),
    }
}

#[test]
fn test_rem_statement() {
    let ctx = create_test_context();
    let stmt = Stmt::Rem {
        comment: "This is a comment".to_string(),
    };

    let result = stmt.execute(&ctx).unwrap();
    match result {
        Action::Continue => (),
        _ => panic!("Expected Continue action"),
    }
}
