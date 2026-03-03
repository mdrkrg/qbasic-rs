use qbasic_rs::core::ast::*;
use qbasic_rs::core::eval::interpreter::{Interpreter, InterpreterState};
use qbasic_rs::core::token::{Math, Relational};

// Helper to create a simple integer expression
fn int_expr(value: u32) -> Expr {
    Expr::Literal(LiteralValue::Integer(value))
}

// Helper to create a variable expression
fn var_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.to_string(),
    }
}

// Helper to create a binary arithmetic expression
fn arithmetic_expr(left: Expr, op: Math, right: Expr) -> Expr {
    Expr::Binary {
        operator: BinaryOp::Arithmetic(op),
        left: Box::new(left),
        right: Box::new(right),
    }
}

// Helper to create a binary relational expression
fn relational_expr(left: Expr, op: Relational, right: Expr) -> Expr {
    Expr::Binary {
        operator: BinaryOp::Relational(op),
        left: Box::new(left),
        right: Box::new(right),
    }
}

/// Helper function to create a simple interpreter with test program
fn create_test_interpreter() -> Interpreter {
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Print { expr: int_expr(42) },
        },
        Line {
            lineno: 20,
            statement: Stmt::Let {
                name: "x".to_string(),
                expr: int_expr(5),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::IfThen {
                conditional: relational_expr(var_expr("x"), Relational::Gt, int_expr(3)),
                lineno: 50,
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::Print {
                expr: var_expr("x"),
            },
        },
        Line {
            lineno: 50,
            statement: Stmt::End,
        },
    ];

    Interpreter::new(lines)
}

#[test]
fn test_execution_count_tracking() {
    let mut interpreter = create_test_interpreter();

    // Initially, no statistics
    assert!(interpreter.line_stats().is_empty());

    // Execute first line (PRINT 42)
    interpreter.step();
    let stats = interpreter.line_stats();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats.get(&10).unwrap().execution_count, 1);
    assert_eq!(stats.get(&10).unwrap().if_true_count, 0);
    assert_eq!(stats.get(&10).unwrap().if_false_count, 0);

    // Execute second line (LET x = 5)
    // Note: Assign now auto-continues, so we need to manually step again
    // Actually, after PRINT, we need to call next() to continue
    interpreter.next(); // Continue after PRINT
    interpreter.step(); // Execute LET x = 5
    let stats = interpreter.line_stats();
    assert_eq!(stats.len(), 2);
    assert_eq!(stats.get(&20).unwrap().execution_count, 1);

    // Execute third line (IF x > 3 THEN 50)
    interpreter.step(); // Execute IF statement
    let stats = interpreter.line_stats();
    assert_eq!(stats.len(), 3);
    assert_eq!(stats.get(&30).unwrap().execution_count, 1);
    // Since x = 5 > 3, condition is true, should jump to line 50
    assert_eq!(stats.get(&30).unwrap().if_true_count, 1);
    assert_eq!(stats.get(&30).unwrap().if_false_count, 0);

    // Should have jumped to line 50 (END)
    assert_eq!(*interpreter.pc(), 50);
}

#[test]
fn test_if_branch_tracking() {
    // Test IF statement with false condition
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "x".to_string(),
                expr: int_expr(1),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::IfThen {
                conditional: relational_expr(var_expr("x"), Relational::Gt, int_expr(3)),
                lineno: 50,
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Print {
                expr: var_expr("x"),
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::End,
        },
        Line {
            lineno: 50,
            statement: Stmt::Print {
                expr: int_expr(999),
            },
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute LET x = 1
    interpreter.step();
    // WARN: No explicit next() after LET
    // interpreter.next(); // Continue after assignment

    // Execute IF x > 3 THEN 50 (false, should continue to line 30)
    interpreter.step();
    let stats = interpreter.line_stats();
    assert_eq!(stats.get(&20).unwrap().execution_count, 1);
    assert_eq!(stats.get(&20).unwrap().if_true_count, 0);
    assert_eq!(stats.get(&20).unwrap().if_false_count, 1);

    // Should be at line 30 (not 50)
    assert_eq!(*interpreter.pc(), 30);
}

#[test]
fn test_reset_statistics() {
    let mut interpreter = create_test_interpreter();

    // Execute some lines
    interpreter.step(); // Line 10
    interpreter.next(); // Continue
    interpreter.step(); // Line 20
    interpreter.step(); // Line 30

    // Verify statistics exist
    let stats = interpreter.line_stats();
    assert!(!stats.is_empty());
    assert!(stats.get(&10).is_some());
    assert!(stats.get(&20).is_some());
    assert!(stats.get(&30).is_some());

    // Reset statistics
    interpreter.reset_statistics();

    // Verify statistics are cleared
    let stats = interpreter.line_stats();
    assert!(stats.is_empty());

    // Execute again and verify fresh counts
    // WARN: should be at line 50 after 30 is executed
    interpreter.step(); // Should still be at line 30 (IF statement)
    let stats = interpreter.line_stats();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats.get(&50).unwrap().execution_count, 1);
}

#[test]
fn test_multiple_executions() {
    // Test program with a loop
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: int_expr(0),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::IfThen {
                conditional: relational_expr(var_expr("i"), Relational::GtEq, int_expr(3)),
                lineno: 40, // Jump to END when i >= 3
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: arithmetic_expr(var_expr("i"), Math::Plus, int_expr(1)),
            },
        },
        Line {
            lineno: 35,
            statement: Stmt::Goto { lineno: 20 },
        },
        Line {
            lineno: 40,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Manually execute the loop
    // Line 10: LET i = 0
    interpreter.step(); // Execute line 10
    // WARN: No explicit next() after LET
    // interpreter.next(); // Continue to line 20

    // Loop 3 times
    for _ in 0..3 {
        // Line 20: IF i < 3 THEN 40 (false, continue to line 30)
        interpreter.step(); // Execute line 20
        // WARN: No explicit next() after LET
        // interpreter.next(); // Continue to line 30 (since condition false)

        // Line 30: LET i = i + 1
        interpreter.step(); // Execute line 30
        // WARN: No explicit next() after LET
        // interpreter.next(); // Continue to line 35

        // Line 35: GOTO 20
        interpreter.step(); // Execute line 35 (jumps to line 20)
    }

    // Final iteration: i = 3, condition is false
    interpreter.step(); // Execute line 20 (condition false)
    // WARN: No explicit next() after LET
    // interpreter.next(); // Continue to line 30

    // Check statistics
    let stats = interpreter.line_stats();
    eprintln!("stats={stats:#?}");

    // Line 10 executed once
    assert_eq!(stats.get(&10).unwrap().execution_count, 1);

    // Line 20 executed 4 times (3 true, 1 false)
    assert_eq!(stats.get(&20).unwrap().execution_count, 4);
    assert_eq!(stats.get(&20).unwrap().if_true_count, 1); // i >= 3 true for i=3
    assert_eq!(stats.get(&20).unwrap().if_false_count, 3); // i >= 3 false for i=0,1,2

    // Line 30 executed 3 times
    assert_eq!(stats.get(&30).unwrap().execution_count, 3);

    // Line 35 executed 3 times
    assert_eq!(stats.get(&35).unwrap().execution_count, 3);
}

#[test]
fn test_variable_use_counts() {
    todo!()
}

#[test]
fn test_statistics_with_error() {
    // Test division by zero error
    let lines = vec![Line {
        lineno: 10,
        statement: Stmt::Print {
            expr: arithmetic_expr(int_expr(5), Math::Division, int_expr(0)),
        },
    }];

    let mut interpreter = Interpreter::new(lines);

    // Execute line with division by zero
    interpreter.step();

    // Should be in error state
    assert!(matches!(*interpreter.state(), InterpreterState::Error(_)));

    // Statistics should still be tracked for the attempted execution
    let stats = interpreter.line_stats();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats.get(&10).unwrap().execution_count, 1);
}

#[test]
fn test_statistics_reset_during_execution() {
    // Test program that prints multiple times
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Print { expr: int_expr(1) },
        },
        Line {
            lineno: 20,
            statement: Stmt::Print { expr: int_expr(2) },
        },
        Line {
            lineno: 30,
            statement: Stmt::Print { expr: int_expr(3) },
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute first line
    interpreter.step(); // Line 10
    interpreter.next(); // Continue after output

    // Check statistics
    let stats = interpreter.line_stats();
    assert_eq!(stats.get(&10).unwrap().execution_count, 1);
    assert_eq!(stats.len(), 1);

    // Reset statistics
    interpreter.reset_statistics();

    // Verify statistics are cleared
    let stats = interpreter.line_stats();
    assert!(stats.is_empty());

    // Execute second line
    interpreter.step(); // Line 20
    interpreter.next(); // Continue after output

    // Check fresh statistics
    let stats = interpreter.line_stats();
    assert_eq!(stats.get(&20).unwrap().execution_count, 1);
    assert_eq!(stats.len(), 1);
    assert!(stats.get(&10).is_none()); // Line 10 should not be in stats after reset
}
