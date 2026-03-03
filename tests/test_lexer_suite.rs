#[cfg(test)]
mod test_lexer_suite {
    use qbasic_rs::lexer::*;
    use qbasic_rs::token::*;
    use std::fs;

    // Helper function to read test file
    fn read_test_file(filename: &str) -> String {
        let path = format!("tests/suite/{}", filename);
        Result::expect(
            fs::read_to_string(&path),
            &format!("Failed to read test file: {}", path),
        )
    }

    #[test]
    fn test_countdown_program() {
        let input = read_test_file("countdown.bas");
        let result = tokenize(&input).unwrap();

        // Expected tokens for countdown.bas
        // Line 1: 10 REM Program to simulate a countdown
        assert!(matches!(result[0], Token::Integer(10)));
        assert!(matches!(result[1], Token::Keyword(Keyword::Rem(_)))); // TODO: comment
        assert!(matches!(result[2], Token::Newline));

        // Line 2: 20 LET T = 10
        assert!(matches!(result[3], Token::Integer(20)));
        assert!(matches!(result[4], Token::Keyword(Keyword::Let)));
        assert!(matches!(result[5], Token::Identifier(_)));
        assert!(matches!(result[6], Token::Equal));
        assert!(matches!(result[7], Token::Integer(10)));
        assert!(matches!(result[8], Token::Newline));

        // Line 3: 30 IF T < 0 THEN 70
        assert!(matches!(result[9], Token::Integer(30)));
        assert!(matches!(result[10], Token::Keyword(Keyword::If)));
        assert!(matches!(result[11], Token::Identifier(_)));
        assert!(matches!(result[12], Token::Relational(Relational::Lt)));
        assert!(matches!(result[13], Token::Integer(0)));
        assert!(matches!(result[14], Token::Keyword(Keyword::Then)));
        assert!(matches!(result[15], Token::Integer(70)));
        assert!(matches!(result[16], Token::Newline));

        // Line 4: 40 PRINT T
        assert!(matches!(result[17], Token::Integer(40)));
        assert!(matches!(result[18], Token::Keyword(Keyword::Print)));
        assert!(matches!(result[19], Token::Identifier(_)));
        assert!(matches!(result[20], Token::Newline));

        // Line 5: 50 LET T = T - 1
        assert!(matches!(result[21], Token::Integer(50)));
        assert!(matches!(result[22], Token::Keyword(Keyword::Let)));
        assert!(matches!(result[23], Token::Identifier(_)));
        assert!(matches!(result[24], Token::Equal));
        assert!(matches!(result[25], Token::Identifier(_)));
        assert!(matches!(result[26], Token::Operator(Math::Minus)));
        assert!(matches!(result[27], Token::Integer(1)));
        assert!(matches!(result[28], Token::Newline));

        // Line 6: 60 GOTO 30
        assert!(matches!(result[29], Token::Integer(60)));
        assert!(matches!(result[30], Token::Keyword(Keyword::Goto)));
        assert!(matches!(result[31], Token::Integer(30)));
        assert!(matches!(result[32], Token::Newline));

        // Line 7: 65 PRINT "hi"
        assert!(matches!(result[33], Token::Integer(65)));
        assert!(matches!(result[34], Token::Keyword(Keyword::Print)));
        assert!(matches!(result[35], Token::String(_)));
        assert!(matches!(result[36], Token::Newline));

        // Line 8: 70 END
        assert!(matches!(result[37], Token::Integer(70)));
        assert!(matches!(result[38], Token::Keyword(Keyword::End)));

        println!("Countdown program token count: {}", result.len());
    }

    #[test]
    fn test_arithmetic_program() {
        let input = read_test_file("arithmetic.bas");
        let result = tokenize(&input).unwrap();

        // Basic validation - just check it tokenizes without errors
        assert!(!result.is_empty());

        // Count specific tokens
        let let_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Let)))
            .count();
        let print_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Print)))
            .count();
        let operator_count = result
            .iter()
            .filter(|t| matches!(t, Token::Operator(_)))
            .count();

        assert_eq!(let_count, 8); // Lines 20-90
        assert_eq!(print_count, 13); // Lines 100-220 (more PRINT statements now)
        assert!(operator_count >= 6); // +, -, *, /, **, MOD

        println!("Arithmetic program token count: {}", result.len());
    }

    #[test]
    fn test_relational_program() {
        let input = read_test_file("relational.bas");
        let result = tokenize(&input).unwrap();

        // Check for relational operators
        let relational_count = result
            .iter()
            .filter(|t| matches!(t, Token::Relational(_)))
            .count();
        let if_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::If)))
            .count();
        let then_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Then)))
            .count();

        assert_eq!(relational_count, 5); // <, <=, >, >=, <>
        assert_eq!(if_count, 5); // 5 IF statements
        assert_eq!(then_count, 5); // 5 THEN keywords

        println!("Relational program token count: {}", result.len());
    }

    #[test]
    fn test_expressions_program() {
        let input = read_test_file("expressions.bas");
        let result = tokenize(&input).unwrap();

        // Check for parentheses
        let paren_count = result
            .iter()
            .filter(|t| matches!(t, Token::Paran(_)))
            .count();
        let power_count = result
            .iter()
            .filter(|t| matches!(t, Token::Operator(Math::Power)))
            .count();
        let mod_count = result
            .iter()
            .filter(|t| matches!(t, Token::Operator(Math::Modulo)))
            .count();

        assert!(paren_count >= 10); // Multiple parentheses in expressions
        assert!(power_count >= 2); // ** operators
        assert!(mod_count >= 2); // MOD operators

        println!("Expressions program token count: {}", result.len());
    }

    #[test]
    fn test_strings_program() {
        let input = read_test_file("strings.bas");
        let result = tokenize(&input).unwrap();

        // Check for string tokens and INPUT keyword
        let string_count = result
            .iter()
            .filter(|t| matches!(t, Token::String(_)))
            .count();
        let input_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Input)))
            .count();
        let print_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Print)))
            .count();

        assert!(string_count >= 5); // Multiple string literals
        assert_eq!(input_count, 2); // INPUT in comment not count
        assert!(print_count >= 8); // Multiple PRINT statements

        println!("Strings program token count: {}", result.len());
    }

    #[test]
    fn test_edge_cases_program() {
        let input = read_test_file("edge_cases.bas");
        let result = tokenize(&input).unwrap();

        // Check mixed case keywords work
        let let_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Let)))
            .count();
        let print_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::Print)))
            .count();
        let if_count = result
            .iter()
            .filter(|t| matches!(t, Token::Keyword(Keyword::If)))
            .count();

        // Should find all keywords regardless of case
        assert!(let_count >= 8);
        assert!(print_count >= 4); // Reduced from 6 since we removed some PRINT statements
        assert!(if_count >= 1);

        // Check comments
        let comment_count = result
            .iter()
            .filter(|t| matches!(t, Token::Comment(_)))
            .count();
        assert!(comment_count >= 2); // REM and apostrophe comments

        println!("Edge cases program token count: {}", result.len());
    }

    #[test]
    fn test_error_cases_program() {
        let input = read_test_file("error_cases.bas");
        let result = tokenize(&input);

        // This should fail due to invalid characters
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(!errors.is_empty());

        // Check for specific error messages
        let has_invalid_char = errors.iter().any(|err| err.message.contains("@"));
        let has_unterminated_string = errors.iter().any(|err| err.message.contains("not closed"));

        assert!(
            has_invalid_char,
            "Should have error for invalid character @"
        );
        assert!(
            has_unterminated_string,
            "Should have error for unterminated string"
        );

        println!("Error cases program found {} errors", errors.len());
    }

    #[test]
    fn test_simple_program_exact_tokens() {
        let input = read_test_file("simple_verify.bas");
        let result = tokenize(&input).unwrap();

        // Exact token sequence verification
        assert_eq!(result.len(), 13);

        // Line 1: 10 LET X = 5
        assert!(matches!(result[0], Token::Integer(10)));
        assert!(matches!(result[1], Token::Keyword(Keyword::Let)));
        if let Token::Identifier(id) = &result[2] {
            assert_eq!(id, "X");
        } else {
            panic!("Expected Identifier token");
        }
        assert!(matches!(result[3], Token::Equal));
        assert!(matches!(result[4], Token::Integer(5)));
        assert!(matches!(result[5], Token::Newline));

        // Line 2: 20 PRINT X
        assert!(matches!(result[6], Token::Integer(20)));
        assert!(matches!(result[7], Token::Keyword(Keyword::Print)));
        if let Token::Identifier(id) = &result[8] {
            assert_eq!(id, "X");
        } else {
            panic!("Expected Identifier token");
        }
        assert!(matches!(result[9], Token::Newline));

        // Line 3: 30 END
        assert!(matches!(result[10], Token::Integer(30)));
        assert!(matches!(result[11], Token::Keyword(Keyword::End)));
        assert!(matches!(result[12], Token::Newline));

        println!("Simple program token count: {}", result.len());
    }
}
