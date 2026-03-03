#[cfg(test)]
mod lexer_basic {
    use qbasic_rs::core::lexer::*;
    use qbasic_rs::core::token::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "LET x = 10\nPRINT x";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 7);
        assert!(matches!(result[0], Token::Keyword(Keyword::Let)));
        assert!(matches!(result[1], Token::Identifier(_)));
        assert!(matches!(result[2], Token::Equal));
        assert!(matches!(result[3], Token::Integer(10)));
        assert!(matches!(result[4], Token::Newline));
        assert!(matches!(result[5], Token::Keyword(Keyword::Print)));
        assert!(matches!(result[6], Token::Identifier(_)));
    }

    #[test]
    fn test_operators() {
        let input = "1 + 2 * 3 / 4 - 5";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 9);
        assert!(matches!(result[1], Token::Operator(Math::Plus)));
        assert!(matches!(result[3], Token::Operator(Math::Times)));
        assert!(matches!(result[5], Token::Operator(Math::Division)));
        assert!(matches!(result[7], Token::Operator(Math::Minus)));
    }

    #[test]
    fn test_relational_operators() {
        let input = "x < y <= z > w >= v <>";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 10);
        assert!(matches!(result[1], Token::Relational(Relational::Lt)));
        assert!(matches!(result[3], Token::Relational(Relational::LtEq)));
        assert!(matches!(result[5], Token::Relational(Relational::Gt)));
        assert!(matches!(result[7], Token::Relational(Relational::GtEq)));
        assert!(matches!(result[9], Token::Relational(Relational::NotEq)));
    }

    #[test]
    fn test_parentheses() {
        let input = "(x + y) * (a - b)";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 11);
        assert!(matches!(result[0], Token::Paran(Side::Left)));
        assert!(matches!(result[4], Token::Paran(Side::Right)));
        assert!(matches!(result[6], Token::Paran(Side::Left)));
        assert!(matches!(result[10], Token::Paran(Side::Right)));
    }

    #[test]
    fn test_comments() {
        let input = "REM This is a comment\n' This is also a comment\nPRINT x";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 6);
        assert!(matches!(result[0], Token::Keyword(Keyword::Rem(_))));
        assert!(matches!(result[1], Token::Newline));
        assert!(matches!(result[2], Token::Comment(_)));
        assert!(matches!(result[3], Token::Newline));
        assert!(matches!(result[4], Token::Keyword(Keyword::Print)));
        assert!(matches!(result[5], Token::Identifier(_)));
    }

    #[test]
    fn test_string_literals() {
        let input = r#"PRINT "Hello, World!""#;
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 2);
        assert!(matches!(result[0], Token::Keyword(Keyword::Print)));
        if let Token::String(s) = &result[1] {
            assert_eq!(s, "\"Hello, World!\"");
        } else {
            panic!("Expected String token");
        }
    }

    #[test]
    fn test_case_insensitive_keywords() {
        let input = "let print if then input goto end";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 7);
        assert!(matches!(result[0], Token::Keyword(Keyword::Let)));
        assert!(matches!(result[1], Token::Keyword(Keyword::Print)));
        assert!(matches!(result[2], Token::Keyword(Keyword::If)));
        assert!(matches!(result[3], Token::Keyword(Keyword::Then)));
        assert!(matches!(result[4], Token::Keyword(Keyword::Input)));
        assert!(matches!(result[5], Token::Keyword(Keyword::Goto)));
        assert!(matches!(result[6], Token::Keyword(Keyword::End)));
    }

    #[test]
    fn test_invalid_characters() {
        let input = "LET x = @invalid";
        let result = tokenize(input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error
                .iter()
                .any(|err| err.message.contains("Unexpected character"))
        );
        assert!(
            error
                .iter()
                .any(|err| err.message.to_string().contains("@"))
        );
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#"PRINT "hello"#;
        let result = tokenize(input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error
                .iter()
                .any(|err| err.message.contains("String literal was not closed"))
        );
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_whitespace_only() {
        let input = "   \t\n  ";
        let result = tokenize(input).unwrap();

        // Should contain only one Newline
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Token::Newline));
    }

    #[test]
    /// Multiple new lines should be counted as one token.
    fn test_multiple_new_lines() {
        let input = "REM   \n\n\n REM ";
        let result = tokenize(input).unwrap();

        assert_eq!(result.len(), 3);
        assert!(matches!(result[1], Token::Newline));
    }
}
