use qbasic_rs::core::ast::*;
use qbasic_rs::core::display::StatsProvider;
use qbasic_rs::core::eval::interpreter::{Interpreter, InterpreterState};
use qbasic_rs::core::token::{Math, Relational};

mod utils;
use utils::{binary_expr, int_expr, relational_expr, var_expr};

struct MockStats;
impl StatsProvider for MockStats {
    fn execution_count(&self, _: u32) -> u32 {
        0
    }

    fn if_branch_counts(&self, _: u32) -> (u32, u32) {
        (0, 0)
    }

    fn variable_use_count(&self, _: &str) -> u32 {
        0
    }
}

#[test]
fn test_syntax_tree_rem() {
    let line = Line {
        lineno: 10,
        statement: Stmt::Rem {
            comment: "test comment".to_string(),
        },
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 REM 0
    test comment"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_input() {
    let line = Line {
        lineno: 10,
        statement: Stmt::Input {
            name: "x".to_string(),
        },
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 INPUT 0
    x"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_goto() {
    let line = Line {
        lineno: 10,
        statement: Stmt::Goto { lineno: 100 },
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 GOTO 0
    100"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_end() {
    let line = Line {
        lineno: 10,
        statement: Stmt::End,
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 END 0"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_let_with_expression() {
    use Math::{Plus, Times};
    // LET m = p + q * t
    let expr = binary_expr(
        var_expr("p"),
        Plus,
        binary_expr(var_expr("q"), Times, var_expr("t")),
    );
    let line = Line {
        lineno: 10,
        statement: Stmt::Let {
            name: "m".to_string(),
            expr,
        },
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 LET = 0
    m 0
    +
        p
        *
            q
            t"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_print_with_expression() {
    use Math::{Plus, Times};
    let expr = binary_expr(
        var_expr("p"),
        Plus,
        binary_expr(var_expr("q"), Times, var_expr("t")),
    );
    let line = Line {
        lineno: 10,
        statement: Stmt::Print { expr },
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 PRINT 0
    +
        p
        *
            q
            t"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_if_with_relational() {
    let conditional = relational_expr(var_expr("m"), Relational::Gt, var_expr("max"));
    let line = Line {
        lineno: 10,
        statement: Stmt::IfThen {
            conditional,
            lineno: 50,
        },
    };
    let tree = line.format_syntax_tree(&MockStats);
    let expected = r#"10 IF THEN 0 0
    >
        m
        max
    50"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_let() {
    // Program: LET x = 5, PRINT x, END
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "x".to_string(),
                expr: int_expr(5),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::Print {
                expr: var_expr("x"),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute the program
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Now generate syntax tree for line 10 (LET x = 5)
    let line = Line {
        lineno: 10,
        statement: Stmt::Let {
            name: "x".to_string(),
            expr: int_expr(5),
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 1, variable usage count 1 (x used once in PRINT)
    let expected = r#"10 LET = 1
    x 1
    5"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_if() {
    // Program: LET x = 5, IF x > 3 THEN 50, PRINT 99, END, 50: PRINT 100, END
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "x".to_string(),
                expr: int_expr(5),
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
            statement: Stmt::Print { expr: int_expr(99) },
        },
        Line {
            lineno: 40,
            statement: Stmt::End,
        },
        Line {
            lineno: 50,
            statement: Stmt::Print {
                expr: int_expr(100),
            },
        },
        Line {
            lineno: 60,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 20 (IF x > 3 THEN 50)
    let line = Line {
        lineno: 20,
        statement: Stmt::IfThen {
            conditional: relational_expr(var_expr("x"), Relational::Gt, int_expr(3)),
            lineno: 50,
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 1, true branch count 1, false branch count 0
    let expected = r#"20 IF THEN 1 0
    >
        x
        3
    50"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_print() {
    // Program: PRINT 42, END
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Print { expr: int_expr(42) },
        },
        Line {
            lineno: 20,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 10 (PRINT 42)
    let line = Line {
        lineno: 10,
        statement: Stmt::Print { expr: int_expr(42) },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 1
    let expected = r#"10 PRINT 1
    42"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_goto() {
    // Program: LET i = 0, 10: IF i >= 3 THEN 50, LET i = i + 1, GOTO 10, 50: END
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
                lineno: 50,
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: binary_expr(var_expr("i"), Math::Plus, int_expr(1)),
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::Goto { lineno: 20 },
        },
        Line {
            lineno: 50,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 40 (GOTO 20)
    let line = Line {
        lineno: 40,
        statement: Stmt::Goto { lineno: 20 },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // GOTO executed 3 times (i=0,1,2)
    let expected = r#"40 GOTO 3
    20"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_variable_multiple_uses() {
    // Program: LET x = 5, PRINT x * x + x, END
    // x used 3 times in expression x * x + x
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "x".to_string(),
                expr: int_expr(5),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::Print {
                expr: binary_expr(
                    binary_expr(var_expr("x"), Math::Times, var_expr("x")),
                    Math::Plus,
                    var_expr("x"),
                ),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 10 (LET x = 5)
    let line = Line {
        lineno: 10,
        statement: Stmt::Let {
            name: "x".to_string(),
            expr: int_expr(5),
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 1, variable usage count 3 (x used 3 times in PRINT)
    let expected = r#"10 LET = 1
    x 3
    5"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_if_false() {
    // Program: LET x = 1, IF x > 3 THEN 50, PRINT 99, END, 50: PRINT 100, END
    // Condition false, so false branch taken
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
            statement: Stmt::Print { expr: int_expr(99) },
        },
        Line {
            lineno: 40,
            statement: Stmt::End,
        },
        Line {
            lineno: 50,
            statement: Stmt::Print {
                expr: int_expr(100),
            },
        },
        Line {
            lineno: 60,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 20 (IF x > 3 THEN 50)
    let line = Line {
        lineno: 20,
        statement: Stmt::IfThen {
            conditional: relational_expr(var_expr("x"), Relational::Gt, int_expr(3)),
            lineno: 50,
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 1, true branch count 0, false branch count 1
    let expected = r#"20 IF THEN 0 1
    >
        x
        3
    50"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_rem() {
    // Program: REM comment, PRINT 42, END
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Rem {
                comment: "This is a comment".to_string(),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::Print { expr: int_expr(42) },
        },
        Line {
            lineno: 30,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 10 (REM comment)
    let line = Line {
        lineno: 10,
        statement: Stmt::Rem {
            comment: "This is a comment".to_string(),
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 1
    let expected = r#"10 REM 1
    This is a comment"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_reset() {
    // Program: LET x = 5, PRINT x, END
    let lines = vec![
        Line {
            lineno: 10,
            statement: Stmt::Let {
                name: "x".to_string(),
                expr: int_expr(5),
            },
        },
        Line {
            lineno: 20,
            statement: Stmt::Print {
                expr: var_expr("x"),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Reset statistics
    interpreter.reset_statistics();

    // Generate syntax tree for line 10 (LET x = 5) after reset
    let line = Line {
        lineno: 10,
        statement: Stmt::Let {
            name: "x".to_string(),
            expr: int_expr(5),
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // After reset, should show: execution count 0, variable usage count 0
    let expected = r#"10 LET = 0
    x 0
    5"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_if_multiple_executions() {
    // Program: LET i = 0, 20: IF i < 3 THEN 30, PRINT 99, END, 30: LET i = i + 1, GOTO 20, 40: END
    // This creates a loop where IF is executed 4 times: 3 times true (i=0,1,2), 1 time false (i=3)
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
                conditional: relational_expr(var_expr("i"), Relational::Lt, int_expr(3)),
                lineno: 30,
            },
        },
        Line {
            lineno: 25,
            statement: Stmt::Print { expr: int_expr(99) },
        },
        Line {
            lineno: 27,
            statement: Stmt::End,
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: binary_expr(var_expr("i"), Math::Plus, int_expr(1)),
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

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 20 (IF i < 3 THEN 30)
    let line = Line {
        lineno: 20,
        statement: Stmt::IfThen {
            conditional: relational_expr(var_expr("i"), Relational::Lt, int_expr(3)),
            lineno: 30,
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 4, true branch count 3, false branch count 1
    let expected = r#"20 IF THEN 3 1
    <
        i
        3
    30"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_goto_multiple_executions() {
    // Program: LET i = 0, 20: IF i >= 3 THEN 50, LET i = i + 1, GOTO 20, 50: END
    // GOTO executed 3 times (i=0,1,2)
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
                lineno: 50,
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: binary_expr(var_expr("i"), Math::Plus, int_expr(1)),
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::Goto { lineno: 20 },
        },
        Line {
            lineno: 50,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 40 (GOTO 20)
    let line = Line {
        lineno: 40,
        statement: Stmt::Goto { lineno: 20 },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // GOTO executed 3 times
    let expected = r#"40 GOTO 3
    20"#;
    assert_eq!(tree, expected);

    // Also test line 20 (IF statement)
    let line_if = Line {
        lineno: 20,
        statement: Stmt::IfThen {
            conditional: relational_expr(var_expr("i"), Relational::GtEq, int_expr(3)),
            lineno: 50,
        },
    };
    let tree_if = line_if.format_syntax_tree(&interpreter);

    // IF executed 4 times: 1 true (i=3), 3 false (i=0,1,2)
    let expected_if = r#"20 IF THEN 1 3
    >=
        i
        3
    50"#;
    assert_eq!(tree_if, expected_if);
}

#[test]
fn test_syntax_tree_statistics_print_multiple_executions() {
    // Program: LET i = 0, 20: PRINT i, LET i = i + 1, IF i < 3 THEN 20, END
    // PRINT executed 3 times (i=0,1,2)
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
            statement: Stmt::Print {
                expr: var_expr("i"),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: binary_expr(var_expr("i"), Math::Plus, int_expr(1)),
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::IfThen {
                conditional: relational_expr(var_expr("i"), Relational::Lt, int_expr(3)),
                lineno: 20,
            },
        },
        Line {
            lineno: 50,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 20 (PRINT i)
    let line = Line {
        lineno: 20,
        statement: Stmt::Print {
            expr: var_expr("i"),
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // PRINT executed 3 times
    let expected = r#"20 PRINT 3
    i"#;
    assert_eq!(tree, expected);
}

#[test]
fn test_syntax_tree_statistics_let_multiple_executions_with_variable_use() {
    // Program: LET i = 0, 20: LET j = i * 2, LET i = i + 1, IF i < 3 THEN 20, END
    // LET j = i * 2 executed 3 times (i=0,1,2)
    // Variable i used 3 times in expression i * 2 (once per execution)
    // Variable j assigned 3 times, but usage count should be 0 (not used anywhere)
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
            statement: Stmt::Let {
                name: "j".to_string(),
                expr: binary_expr(var_expr("i"), Math::Times, int_expr(2)),
            },
        },
        Line {
            lineno: 30,
            statement: Stmt::Let {
                name: "i".to_string(),
                expr: binary_expr(var_expr("i"), Math::Plus, int_expr(1)),
            },
        },
        Line {
            lineno: 40,
            statement: Stmt::IfThen {
                conditional: relational_expr(var_expr("i"), Relational::Lt, int_expr(3)),
                lineno: 20,
            },
        },
        Line {
            lineno: 50,
            statement: Stmt::End,
        },
    ];

    let mut interpreter = Interpreter::new(lines);

    // Execute until finished
    while !matches!(*interpreter.state(), InterpreterState::Finished) {
        interpreter.step();
    }

    // Generate syntax tree for line 20 (LET j = i * 2)
    let line = Line {
        lineno: 20,
        statement: Stmt::Let {
            name: "j".to_string(),
            expr: binary_expr(var_expr("i"), Math::Times, int_expr(2)),
        },
    };
    let tree = line.format_syntax_tree(&interpreter);

    // Should show: execution count 3, variable j usage count 0 (j not used)
    // Variable i usage count is tracked separately and shown in LET i syntax tree
    let expected = r#"20 LET = 3
    j 0
    *
        i
        2"#;
    assert_eq!(tree, expected);

    // Also test line 10 (LET i = 0)
    let line_i = Line {
        lineno: 10,
        statement: Stmt::Let {
            name: "i".to_string(),
            expr: int_expr(0),
        },
    };
    let tree_i = line_i.format_syntax_tree(&interpreter);

    // Should show: execution count 1, variable i usage count 9
    // Breakdown of i usage:
    // - Line 20: i * 2 uses i once per execution (3 executions = 3 uses)
    // - Line 30: i + 1 uses i once per execution (3 executions = 3 uses)
    // - Line 40: IF i < 3 uses i once per execution (3 executions = 3 uses)
    // Total: 9 uses
    let expected_i = r#"10 LET = 1
    i 9
    0"#;
    assert_eq!(tree_i, expected_i);
}
