#[cfg(test)]
mod test_parser {
    use anyhow::bail;
    use qbasic_rs::ast::*;
    use qbasic_rs::lexer::*;
    use qbasic_rs::parser::*;
    use qbasic_rs::token::*;
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
}
