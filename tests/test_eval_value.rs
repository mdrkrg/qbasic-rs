use qbasic_rs::ast::*;
use qbasic_rs::eval::value::{Context, Value};
use qbasic_rs::token::{Math, Relational};
use std::collections::HashMap;
use std::str::FromStr;

// Helper to create a Context for testing
fn create_test_context() -> Context {
    Context {
        variables: HashMap::new(),
    }
}

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
fn binary_expr(left: Expr, op: Math, right: Expr) -> Expr {
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

// Helper to create a unary expression
fn unary_expr(op: UnaryOp, right: Expr) -> Expr {
    Expr::Unary {
        operator: op,
        right: Box::new(right),
    }
}

#[test]
fn test_expression_evaluation_integer_literal() {
    let ctx = create_test_context();
    let expr = int_expr(42);
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(42));
}

#[test]
fn test_expression_evaluation_string_literal() {
    let ctx = create_test_context();
    let expr = Expr::Literal(LiteralValue::String("\"Hello\"".to_string()));
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::String("\"Hello\"".to_string()));
}

#[test]
fn test_expression_evaluation_none_literal() {
    let ctx = create_test_context();
    let expr = Expr::Literal(LiteralValue::None);
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::None);
}

#[test]
fn test_expression_evaluation_variable_exists() {
    let mut ctx = create_test_context();
    ctx.variables.insert("X".to_string(), Value::Integer(100));

    let expr = var_expr("X");
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(100));
}

#[test]
fn test_expression_evaluation_variable_missing() {
    let ctx = create_test_context();
    // Missing variables default to 0 (BASIC standard)
    let expr = var_expr("NONEXISTENT");
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(0));
}

#[test]
fn test_arithmetic_operations() {
    let ctx = create_test_context();

    // Test all arithmetic operations
    let test_cases = vec![
        (Math::Plus, 10, 5, 15),
        (Math::Minus, 10, 5, 5),
        (Math::Times, 10, 5, 50),
        (Math::Division, 10, 5, 2),
        (Math::Modulo, 10, 3, 1),
        (Math::Power, 2, 3, 8), // 2**3 = 8
    ];

    for (op, left, right, expected) in test_cases {
        let expr = binary_expr(int_expr(left), op, int_expr(right));
        let result = expr.evaluate(&ctx).unwrap();
        assert_eq!(result, Value::Integer(expected), "Failed for {:?}", op);
    }
}

#[test]
fn test_division_by_zero() {
    let ctx = create_test_context();
    let expr = binary_expr(int_expr(10), Math::Division, int_expr(0));
    let result = expr.evaluate(&ctx);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Division by zero"));
}

#[test]
fn test_relational_operations_integers() {
    let ctx = create_test_context();

    let test_cases = vec![
        (Relational::Lt, 5, 10, true),
        (Relational::Lt, 10, 5, false),
        (Relational::LtEq, 5, 5, true),
        (Relational::LtEq, 5, 10, true),
        (Relational::Gt, 10, 5, true),
        (Relational::Gt, 5, 10, false),
        (Relational::GtEq, 5, 5, true),
        (Relational::GtEq, 10, 5, true),
        (Relational::Eq, 5, 5, true),
        (Relational::Eq, 5, 10, false),
        (Relational::NotEq, 5, 10, true),
        (Relational::NotEq, 5, 5, false),
    ];

    for (op, left, right, expected) in test_cases {
        let expr = relational_expr(int_expr(left), op, int_expr(right));
        let result = expr.evaluate(&ctx).unwrap();
        assert_eq!(result, Value::Boolean(expected), "Failed for {:?}", op);
    }
}

#[test]
fn test_relational_operations_strings() {
    let ctx = create_test_context();

    let test_cases = vec![
        (Relational::Eq, "abc", "abc", true),
        (Relational::Eq, "abc", "def", false),
        (Relational::NotEq, "abc", "def", true),
        (Relational::NotEq, "abc", "abc", false),
        (Relational::Lt, "abc", "def", true), // "abc" < "def"
        (Relational::Lt, "def", "abc", false),
        (Relational::Gt, "def", "abc", true), // "def" > "abc"
        (Relational::Gt, "abc", "def", false),
    ];

    for (op, left, right, expected) in test_cases {
        let left_expr = Expr::Literal(LiteralValue::String(format!("\"{}\"", left)));
        let right_expr = Expr::Literal(LiteralValue::String(format!("\"{}\"", right)));
        let expr = relational_expr(left_expr, op, right_expr);
        let result = expr.evaluate(&ctx).unwrap();
        assert_eq!(result, Value::Boolean(expected), "Failed for {:?}", op);
    }
}

#[test]
fn test_relational_operations_mixed_types() {
    let ctx = create_test_context();

    // Integer vs String comparisons
    let int_expr = int_expr(123);
    let str_expr = Expr::Literal(LiteralValue::String("\"abc\"".to_string()));

    // Eq and NotEq should work (return false/true respectively)
    let eq_expr = relational_expr(int_expr.clone(), Relational::Eq, str_expr.clone());
    let result = eq_expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Boolean(false));

    let ne_expr = relational_expr(int_expr, Relational::NotEq, str_expr);
    let result = ne_expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_relational_operations_mixed_types_error() {
    let ctx = create_test_context();

    // Integer vs String with < should error
    let int_expr = int_expr(123);
    let str_expr = Expr::Literal(LiteralValue::String("\"abc\"".to_string()));

    let lt_expr = relational_expr(int_expr, Relational::Lt, str_expr);
    let result = lt_expr.evaluate(&ctx);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Cannot compare integer to string"));
}

#[test]
fn test_unary_negate() {
    let ctx = create_test_context();
    let expr = unary_expr(UnaryOp::Negate, int_expr(5));
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(-5));
}

#[test]
fn test_unary_negate_twice() {
    let ctx = create_test_context();
    let inner = unary_expr(UnaryOp::Negate, int_expr(5));
    let expr = unary_expr(UnaryOp::Negate, inner);
    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(5)); // -(-5) = 5
}

#[test]
fn test_grouping_expression() {
    let ctx = create_test_context();
    // (5 + 3) * 2
    let inner = binary_expr(int_expr(5), Math::Plus, int_expr(3));
    let grouping = Expr::Grouping {
        expression: Box::new(inner),
    };
    let expr = binary_expr(grouping, Math::Times, int_expr(2));

    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(16)); // (5+3)*2 = 16
}

#[test]
fn test_complex_expression() {
    let mut ctx = create_test_context();
    ctx.variables.insert("A".to_string(), Value::Integer(10));
    ctx.variables.insert("B".to_string(), Value::Integer(3));

    // A * 2 + B ** 2
    let a_times_2 = binary_expr(var_expr("A"), Math::Times, int_expr(2));
    let b_squared = binary_expr(var_expr("B"), Math::Power, int_expr(2));
    let expr = binary_expr(a_times_2, Math::Plus, b_squared);

    let result = expr.evaluate(&ctx).unwrap();
    assert_eq!(result, Value::Integer(29)); // 10*2 + 3**2 = 20 + 9 = 29
}

#[test]
fn test_type_conversion_errors() {
    let ctx = create_test_context();

    // String in arithmetic should fail
    let str_expr = Expr::Literal(LiteralValue::String("\"hello\"".to_string()));
    let int_expr = int_expr(5);
    let add_expr = binary_expr(str_expr, Math::Plus, int_expr);

    let result = add_expr.evaluate(&ctx);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Type Mismatch"));
}

#[test]
fn test_value_display() {
    assert_eq!(Value::Integer(42).to_string(), "42");
    assert_eq!(Value::Boolean(true).to_string(), "true");
    assert_eq!(Value::Boolean(false).to_string(), "false");
    assert_eq!(Value::String("hello".to_string()).to_string(), "hello");
    assert_eq!(Value::None.to_string(), "\n");
}

#[test]
fn test_value_from_str() {
    assert_eq!(Value::from_str("42").unwrap(), Value::Integer(42));
    assert_eq!(Value::from_str("  -123  ").unwrap(), Value::Integer(-123));
    assert_eq!(
        Value::from_str("hello").unwrap(),
        Value::String("hello".to_string())
    );
    assert_eq!(
        Value::from_str("  test  ").unwrap(),
        Value::String("test".to_string())
    );
}