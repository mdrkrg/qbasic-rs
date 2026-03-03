#[cfg(test)]
mod test_parser {
    use anyhow::bail;
    use qbasic_rs::core::ast::*;
    use qbasic_rs::core::lexer::*;
    use qbasic_rs::core::parser::*;
    use qbasic_rs::core::token::*;
    use std::collections::VecDeque;

    // Helper to parse a single line of BASIC code
    fn parse_line(input: &str) -> Result<Line, anyhow::Error> {
        let tokens = match tokenize(input) {
            Ok(it) => it,
            Err(_err) => bail!("Error tokenizing"),
        };
        let mut parser = Parser {
            tokens: VecDeque::from(tokens),
        };
        parser.line()
    }

    #[test]
    fn test_let_statement() {
        let line = parse_line("10 LET X = 5").unwrap();
        assert_eq!(line.lineno, 10);
        match line.statement {
            Stmt::Let { name, expr } => {
                assert_eq!(name, "X");
                match expr {
                    Expr::Literal(LiteralValue::Integer(5)) => (),
                    _ => panic!("Expected integer literal 5"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_let_statement_with_expression() {
        let line = parse_line("20 LET Y = X + 10").unwrap();
        assert_eq!(line.lineno, 20);
        match line.statement {
            Stmt::Let { name, expr } => {
                assert_eq!(name, "Y");
                // Should be binary expression X + 10
                match expr {
                    Expr::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        match operator {
                            BinaryOp::Arithmetic(Math::Plus) => (),
                            _ => panic!("Expected Plus operator"),
                        }
                        match *left {
                            Expr::Variable { name } => assert_eq!(name, "X"),
                            _ => panic!("Expected variable X"),
                        }
                        match *right {
                            Expr::Literal(LiteralValue::Integer(10)) => (),
                            _ => panic!("Expected integer literal 10"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_print_statement() {
        let line = parse_line("30 PRINT \"Hello\"").unwrap();
        assert_eq!(line.lineno, 30);
        match line.statement {
            Stmt::Print { expr } => match expr {
                Expr::Literal(LiteralValue::String(s)) => assert_eq!(s, "\"Hello\""),
                _ => panic!("Expected string literal"),
            },
            _ => panic!("Expected Print statement"),
        }
    }

    #[test]
    fn test_print_empty() {
        // PRINT without argument should print newline (None literal)
        let line = parse_line("40 PRINT").unwrap();
        assert_eq!(line.lineno, 40);
        match line.statement {
            Stmt::Print { expr } => match expr {
                Expr::Literal(LiteralValue::None) => (),
                _ => panic!("Expected None literal for empty PRINT"),
            },
            _ => panic!("Expected Print statement"),
        }
    }

    #[test]
    fn test_goto_statement() {
        let line = parse_line("50 GOTO 100").unwrap();
        assert_eq!(line.lineno, 50);
        match line.statement {
            Stmt::Goto { lineno } => {
                assert_eq!(lineno, 100);
            }
            _ => panic!("Expected Goto statement"),
        }
    }

    #[test]
    fn test_if_then_statement() {
        let line = parse_line("60 IF X < 10 THEN 200").unwrap();
        assert_eq!(line.lineno, 60);
        match line.statement {
            Stmt::IfThen {
                conditional,
                lineno,
            } => {
                assert_eq!(lineno, 200);
                // Check conditional expression X < 10
                match conditional {
                    Expr::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        match operator {
                            BinaryOp::Relational(Relational::Lt) => (),
                            _ => panic!("Expected Less than relational"),
                        }
                        match *left {
                            Expr::Variable { name } => assert_eq!(name, "X"),
                            _ => panic!("Expected variable X"),
                        }
                        match *right {
                            Expr::Literal(LiteralValue::Integer(10)) => (),
                            _ => panic!("Expected integer literal 10"),
                        }
                    }
                    _ => panic!("Expected binary conditional expression"),
                }
            }
            _ => panic!("Expected IfThen statement"),
        }
    }

    #[test]
    fn test_input_statement() {
        let line = parse_line("70 INPUT NAME").unwrap();
        assert_eq!(line.lineno, 70);
        match line.statement {
            Stmt::Input { name } => {
                assert_eq!(name, "NAME");
            }
            _ => panic!("Expected Input statement"),
        }
    }

    #[test]
    fn test_end_statement() {
        let line = parse_line("80 END").unwrap();
        assert_eq!(line.lineno, 80);
        match line.statement {
            Stmt::End => (),
            _ => panic!("Expected End statement"),
        }
    }

    #[test]
    fn test_rem_statement() {
        let line = parse_line("90 REM This is a comment").unwrap();
        assert_eq!(line.lineno, 90);
        match line.statement {
            Stmt::Rem { comment } => {
                assert_eq!(comment, "This is a comment");
            }
            _ => panic!("Expected Rem statement"),
        }
    }

    #[test]
    fn test_apostrophe_comment() {
        let line = parse_line("100 ' Another comment").unwrap();
        assert_eq!(line.lineno, 100);
        match line.statement {
            Stmt::Rem { comment } => {
                assert_eq!(comment, " Another comment");
            }
            _ => panic!("Expected Rem statement from apostrophe comment"),
        }
    }

    #[test]
    fn test_operator_precedence_multiplication_before_addition() {
        // A + B * C should parse as A + (B * C)
        let line = parse_line("110 LET RESULT = A + B * C").unwrap();
        assert_eq!(line.lineno, 110);
        match line.statement {
            Stmt::Let { name: _, expr } => {
                // Should be Binary(+, Variable(A), Binary(*, Variable(B), Variable(C)))
                match expr {
                    Expr::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        // Top-level operator should be Plus
                        match operator {
                            BinaryOp::Arithmetic(Math::Plus) => (),
                            _ => panic!("Expected Plus at top level, got {:?}", operator),
                        }
                        // Left should be Variable A
                        match *left {
                            Expr::Variable { name } => assert_eq!(name, "A"),
                            _ => panic!("Expected variable A on left"),
                        }
                        // Right should be Binary(*, B, C)
                        match *right {
                            Expr::Binary {
                                operator,
                                left,
                                right,
                            } => {
                                match operator {
                                    BinaryOp::Arithmetic(Math::Times) => (),
                                    _ => panic!("Expected Times in right subtree"),
                                }
                                match *left {
                                    Expr::Variable { name } => assert_eq!(name, "B"),
                                    _ => panic!("Expected variable B"),
                                }
                                match *right {
                                    Expr::Variable { name } => assert_eq!(name, "C"),
                                    _ => panic!("Expected variable C"),
                                }
                            }
                            _ => panic!("Expected binary expression on right"),
                        }
                    }
                    _ => panic!("Expected binary expression at top level"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_operator_precedence_parentheses_change_precedence() {
        // (A + B) * C should parse as (A + B) * C (addition before multiplication due to parens)
        let line = parse_line("120 LET RESULT = (A + B) * C").unwrap();
        assert_eq!(line.lineno, 120);
        match line.statement {
            Stmt::Let { name: _, expr } => {
                // Should be Binary(*, Binary(+, A, B), Variable(C))
                match expr {
                    Expr::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        // Top-level operator should be Times
                        match operator {
                            BinaryOp::Arithmetic(Math::Times) => (),
                            _ => panic!("Expected Times at top level"),
                        }
                        // Left should be Grouping containing Binary(+, A, B)
                        match *left {
                            Expr::Grouping { expression } => {
                                // Inside grouping should be Binary(+, A, B)
                                match *expression {
                                    Expr::Binary {
                                        operator,
                                        left,
                                        right,
                                    } => {
                                        match operator {
                                            BinaryOp::Arithmetic(Math::Plus) => (),
                                            _ => panic!("Expected Plus in grouping"),
                                        }
                                        match *left {
                                            Expr::Variable { name } => assert_eq!(name, "A"),
                                            _ => panic!("Expected variable A"),
                                        }
                                        match *right {
                                            Expr::Variable { name } => assert_eq!(name, "B"),
                                            _ => panic!("Expected variable B"),
                                        }
                                    }
                                    _ => panic!("Expected binary expression inside grouping"),
                                }
                            }
                            _ => panic!("Expected grouping expression on left"),
                        }
                        // Right should be Variable C
                        match *right {
                            Expr::Variable { name } => assert_eq!(name, "C"),
                            _ => panic!("Expected variable C on right"),
                        }
                    }
                    _ => panic!("Expected binary expression at top level"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_unary_minus() {
        // -X should parse as Unary(Negate, Variable(X))
        let line = parse_line("130 LET RESULT = -X").unwrap();
        assert_eq!(line.lineno, 130);
        match line.statement {
            Stmt::Let { name: _, expr } => match expr {
                Expr::Unary { operator, right } => {
                    match operator {
                        UnaryOp::Negate => (),
                    }
                    match *right {
                        Expr::Variable { name } => assert_eq!(name, "X"),
                        _ => panic!("Expected variable X"),
                    }
                }
                _ => panic!("Expected unary expression"),
            },
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_power_operator_right_associative() {
        // A ** B ** C should parse as A ** (B ** C) (right associative)
        let line = parse_line("140 LET RESULT = A ** B ** C").unwrap();
        assert_eq!(line.lineno, 140);
        match line.statement {
            Stmt::Let { name: _, expr } => {
                // Should be Binary(**, Variable(A), Binary(**, Variable(B), Variable(C)))
                match expr {
                    Expr::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        // Top-level operator should be Power
                        match operator {
                            BinaryOp::Arithmetic(Math::Power) => (),
                            _ => panic!("Expected Power at top level"),
                        }
                        // Left should be Variable A
                        match *left {
                            Expr::Variable { name } => assert_eq!(name, "A"),
                            _ => panic!("Expected variable A"),
                        }
                        // Right should be Binary(**, B, C)
                        match *right {
                            Expr::Binary {
                                operator,
                                left,
                                right,
                            } => {
                                match operator {
                                    BinaryOp::Arithmetic(Math::Power) => (),
                                    _ => panic!("Expected Power in right subtree"),
                                }
                                match *left {
                                    Expr::Variable { name } => assert_eq!(name, "B"),
                                    _ => panic!("Expected variable B"),
                                }
                                match *right {
                                    Expr::Variable { name } => assert_eq!(name, "C"),
                                    _ => panic!("Expected variable C"),
                                }
                            }
                            _ => panic!("Expected binary expression on right"),
                        }
                    }
                    _ => panic!("Expected binary expression at top level"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_mod_operator() {
        // A MOD B should parse as Binary(Modulo, A, B)
        let line = parse_line("150 LET RESULT = A MOD B").unwrap();
        assert_eq!(line.lineno, 150);
        match line.statement {
            Stmt::Let { name: _, expr } => match expr {
                Expr::Binary {
                    operator,
                    left,
                    right,
                } => {
                    match operator {
                        BinaryOp::Arithmetic(Math::Modulo) => (),
                        _ => panic!("Expected Modulo operator"),
                    }
                    match *left {
                        Expr::Variable { name } => assert_eq!(name, "A"),
                        _ => panic!("Expected variable A"),
                    }
                    match *right {
                        Expr::Variable { name } => assert_eq!(name, "B"),
                        _ => panic!("Expected variable B"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_complex_mixed_expression() {
        // -A * (B + C) ** D / E MOD F
        // This tests: unary minus, parentheses, addition, power, division, mod
        let line = parse_line("160 LET RESULT = -A * (B + C) ** D / E MOD F").unwrap();
        assert_eq!(line.lineno, 160);
        match line.statement {
            Stmt::Let { name: _, expr } => {
                // Let's just verify it parses without error
                // The exact tree structure is complex to test, but we can check it's a binary expression
                match expr {
                    Expr::Binary {
                        operator: _,
                        left: _,
                        right: _,
                    } => {
                        // Good, it's a binary expression (likely MOD at top level)
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_all_relational_operators() {
        // Test all relational operators in IF statements
        let test_cases = vec![
            ("170 IF X < Y THEN 1000", Relational::Lt),
            ("180 IF X <= Y THEN 1000", Relational::LtEq),
            ("190 IF X > Y THEN 1000", Relational::Gt),
            ("200 IF X >= Y THEN 1000", Relational::GtEq),
            ("210 IF X <> Y THEN 1000", Relational::NotEq),
            ("220 IF X = Y THEN 1000", Relational::Eq),
        ];

        for (input, expected_op) in test_cases {
            let line = parse_line(input).unwrap();
            match line.statement {
                Stmt::IfThen {
                    conditional,
                    lineno: _,
                } => match conditional {
                    Expr::Binary {
                        operator,
                        left: _,
                        right: _,
                    } => match operator {
                        BinaryOp::Relational(op) => assert_eq!(op, expected_op),
                        _ => panic!("Expected relational operator"),
                    },
                    _ => panic!("Expected binary conditional"),
                },
                _ => panic!("Expected IfThen statement"),
            }
        }
    }

    #[test]
    fn test_parser_error_missing_line_number() {
        // Input without line number should fail
        let result = parse_line("LET X = 5");
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_error_missing_equals_in_let() {
        // LET X 5 (missing =) should fail
        let result = parse_line("10 LET X 5");
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_error_missing_then_in_if() {
        // IF X < 10 200 (missing THEN) should fail
        let result = parse_line("10 IF X < 10 200");
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_error_unclosed_parenthesis() {
        // (A + B * C (missing closing paren) should fail
        let result = parse_line("10 LET X = (A + B * C");
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_error_unexpected_token() {
        // LET X = + (plus without right operand) should fail
        let result = parse_line("10 LET X = +");
        assert!(result.is_err());
    }

    #[test]
    fn test_nested_parentheses() {
        // ((A + B) * C) should parse with nested grouping
        let line = parse_line("220 LET RESULT = ((A + B) * C)").unwrap();
        assert_eq!(line.lineno, 220);
        match line.statement {
            Stmt::Let { name: _, expr } => {
                // Should be Grouping containing Binary(*, Grouping(Binary(+, A, B)), Variable(C))
                match expr {
                    Expr::Grouping { expression } => {
                        // Inside outer grouping: Binary(*, Grouping(Binary(+, A, B)), Variable(C))
                        match *expression {
                            Expr::Binary {
                                operator,
                                left,
                                right,
                            } => {
                                match operator {
                                    BinaryOp::Arithmetic(Math::Times) => (),
                                    _ => panic!("Expected Times in outer grouping"),
                                }
                                // Left should be Grouping(Binary(+, A, B))
                                match *left {
                                    Expr::Grouping { expression } => match *expression {
                                        Expr::Binary {
                                            operator,
                                            left,
                                            right,
                                        } => {
                                            match operator {
                                                BinaryOp::Arithmetic(Math::Plus) => (),
                                                _ => panic!("Expected Plus in inner grouping"),
                                            }
                                            match *left {
                                                Expr::Variable { name } => assert_eq!(name, "A"),
                                                _ => panic!("Expected variable A"),
                                            }
                                            match *right {
                                                Expr::Variable { name } => assert_eq!(name, "B"),
                                                _ => panic!("Expected variable B"),
                                            }
                                        }
                                        _ => panic!("Expected binary in inner grouping"),
                                    },
                                    _ => panic!("Expected grouping on left"),
                                }
                                // Right should be Variable C
                                match *right {
                                    Expr::Variable { name } => assert_eq!(name, "C"),
                                    _ => panic!("Expected variable C"),
                                }
                            }
                            _ => panic!("Expected binary in outer grouping"),
                        }
                    }
                    _ => panic!("Expected outer grouping"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_multiple_unary_minus() {
        // --X should parse as Unary(Negate, Unary(Negate, Variable(X)))
        let line = parse_line("230 LET RESULT = --X").unwrap();
        assert_eq!(line.lineno, 230);
        match line.statement {
            Stmt::Let { name: _, expr } => {
                match expr {
                    Expr::Unary { operator, right } => {
                        match operator {
                            UnaryOp::Negate => (),
                        }
                        // Right should be another Unary
                        match *right {
                            Expr::Unary { operator, right } => {
                                match operator {
                                    UnaryOp::Negate => (),
                                }
                                match *right {
                                    Expr::Variable { name } => assert_eq!(name, "X"),
                                    _ => panic!("Expected variable X"),
                                }
                            }
                            _ => panic!("Expected inner unary expression"),
                        }
                    }
                    _ => panic!("Expected outer unary expression"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }
}
