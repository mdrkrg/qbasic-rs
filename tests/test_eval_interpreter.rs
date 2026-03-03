use qbasic_rs::core::ast::*;
use qbasic_rs::core::eval::interpreter::{Interpreter, InterpreterState};
use qbasic_rs::core::eval::value::Value;
use qbasic_rs::core::token::Relational;

mod utils;
use utils::{int_expr, relational_expr, var_expr};

#[test]
fn test_interpreter_basic_execution() {
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "X".to_string(),
                expr: int_expr(42),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::Print {
                expr: var_expr("X"),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);
    interpreter.run_test();

    assert_eq!(interpreter.state(), &InterpreterState::Finished);
    assert_eq!(
        interpreter.context().variables.get("X"),
        Some(&Value::Integer(42))
    );
}

#[test]
fn test_interpreter_goto() {
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Goto { lineno: 30 },
        },
        Line {
            lineno: 20,
            statement: Stmt::Let {
                name: "X".to_string(),
                expr: int_expr(1), // Should be skipped
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "X".to_string(),
                expr: int_expr(2), // Should be executed
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);
    interpreter.run_test();

    assert_eq!(interpreter.state(), &InterpreterState::Finished);
    assert_eq!(
        interpreter.context().variables.get("X"),
        Some(&Value::Integer(2))
    );
}

#[test]
fn test_interpreter_if_then() {
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "X".to_string(),
                expr: int_expr(5),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::IfThen {
                conditional: relational_expr(var_expr("X"), Relational::Lt, int_expr(10)),
                lineno: 40,
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "Y".to_string(),
                expr: int_expr(1), // Should be skipped
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::Let {
                name: "Y".to_string(),
                expr: int_expr(2), // Should be executed
            },
        },
        Line {
            lineno: 50,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);
    interpreter.run_test();

    assert_eq!(interpreter.state(), &InterpreterState::Finished);
    assert_eq!(
        interpreter.context().variables.get("Y"),
        Some(&Value::Integer(2))
    );
}
