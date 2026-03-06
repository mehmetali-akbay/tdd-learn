use calculator::*;

// ===== Topic 1: Tokenizer =====

#[test]
fn test_tokenize_number() {
    let tokens = tokenize("42").unwrap();
    assert_eq!(tokens, vec![Token::Number(42.0)]);
}

#[test]
fn test_tokenize_decimal() {
    let tokens = tokenize("3.15").unwrap();
    assert_eq!(tokens, vec![Token::Number(3.15)]);
}

#[test]
fn test_tokenize_expression() {
    let tokens = tokenize("2 + 3").unwrap();
    assert_eq!(
        tokens,
        vec![Token::Number(2.0), Token::Plus, Token::Number(3.0)]
    );
}

#[test]
fn test_tokenize_all_operators() {
    let tokens = tokenize("1 + 2 - 3 * 4 / 5").unwrap();
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[1], Token::Plus);
    assert_eq!(tokens[3], Token::Minus);
    assert_eq!(tokens[5], Token::Star);
    assert_eq!(tokens[7], Token::Slash);
}

#[test]
fn test_tokenize_parentheses() {
    let tokens = tokenize("(1 + 2)").unwrap();
    assert_eq!(tokens[0], Token::LParen);
    assert_eq!(tokens[4], Token::RParen);
}

#[test]
fn test_tokenize_negative() {
    let tokens = tokenize("-5").unwrap();
    assert_eq!(tokens, vec![Token::Number(-5.0)]);
}

#[test]
fn test_tokenize_error() {
    assert!(tokenize("2 + @").is_err());
}

// ===== Topic 2: AST =====

#[test]
fn test_expr_display() {
    let expr = Expr::BinOp {
        op: BinOp::Add,
        left: Box::new(Expr::Number(2.0)),
        right: Box::new(Expr::Number(3.0)),
    };
    assert_eq!(expr.to_string_expr(), "(2 + 3)");
}

#[test]
fn test_expr_depth() {
    assert_eq!(Expr::Number(5.0).depth(), 0);
    let nested = Expr::BinOp {
        op: BinOp::Add,
        left: Box::new(Expr::BinOp {
            op: BinOp::Mul,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        }),
        right: Box::new(Expr::Number(3.0)),
    };
    assert_eq!(nested.depth(), 2);
}

// ===== Topic 3: Parser =====

#[test]
fn test_parse_number() {
    let expr = parse("42").unwrap();
    assert_eq!(expr, Expr::Number(42.0));
}

#[test]
fn test_parse_addition() {
    let expr = parse("2 + 3").unwrap();
    assert_eq!(
        expr,
        Expr::BinOp {
            op: BinOp::Add,
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        }
    );
}

#[test]
fn test_parse_precedence() {
    // 2 + 3 * 4 should parse as 2 + (3 * 4), not (2 + 3) * 4
    let expr = parse("2 + 3 * 4").unwrap();
    let result = eval_expr(&expr).unwrap();
    assert!((result - 14.0).abs() < 0.001);
}

#[test]
fn test_parse_parentheses() {
    // (2 + 3) * 4 = 20
    let expr = parse("(2 + 3) * 4").unwrap();
    let result = eval_expr(&expr).unwrap();
    assert!((result - 20.0).abs() < 0.001);
}

#[test]
fn test_parse_error_empty() {
    assert!(parse("").is_err());
}

#[test]
fn test_parse_error_unmatched_paren() {
    assert!(parse("(2 + 3").is_err());
}

// ===== Topic 4: Evaluator =====

#[test]
fn test_eval_number() {
    assert_eq!(eval_expr(&Expr::Number(42.0)), Ok(42.0));
}

#[test]
fn test_eval_add() {
    let expr = Expr::BinOp {
        op: BinOp::Add,
        left: Box::new(Expr::Number(2.0)),
        right: Box::new(Expr::Number(3.0)),
    };
    assert_eq!(eval_expr(&expr), Ok(5.0));
}

#[test]
fn test_eval_div_by_zero() {
    let expr = Expr::BinOp {
        op: BinOp::Div,
        left: Box::new(Expr::Number(1.0)),
        right: Box::new(Expr::Number(0.0)),
    };
    assert_eq!(eval_expr(&expr), Err(CalcError::DivisionByZero));
}

#[test]
fn test_calc_error_display() {
    assert_eq!(format!("{}", CalcError::DivisionByZero), "division by zero");
}

// ===== Topic 5: End-to-End calc() =====

#[test]
fn test_calc_simple() {
    assert_eq!(calc("2 + 3"), Ok(5.0));
    assert_eq!(calc("10 - 4"), Ok(6.0));
    assert_eq!(calc("3 * 7"), Ok(21.0));
    assert_eq!(calc("20 / 4"), Ok(5.0));
}

#[test]
fn test_calc_complex() {
    assert!((calc("(2 + 3) * (4 - 1)").unwrap() - 15.0).abs() < 0.001);
    assert!((calc("100 / (5 * 2)").unwrap() - 10.0).abs() < 0.001);
}

#[test]
fn test_calc_negative() {
    assert_eq!(calc("-5 + 3"), Ok(-2.0));
}

#[test]
fn test_calc_decimal() {
    assert!((calc("3.15 * 2").unwrap() - 6.30).abs() < 0.001);
}

#[test]
fn test_calc_div_by_zero() {
    assert_eq!(calc("5 / 0"), Err(CalcError::DivisionByZero));
}

#[test]
fn test_calc_parse_error() {
    assert!(matches!(calc("2 +"), Err(CalcError::ParseError(_))));
}

#[test]
fn test_format_result() {
    assert_eq!(format_result(5.0), "5");
    assert_eq!(format_result(3.15), "3.150000");
    assert_eq!(format_result(-10.0), "-10");
}

// ===== Topic 6: Environment =====

#[test]
fn test_env_set_get() {
    let mut env = Env::new();
    env.set("x", 42.0);
    assert_eq!(env.get("x"), Some(42.0));
    assert_eq!(env.get("y"), None);
}

#[test]
fn test_env_eval_line_expr() {
    let mut env = Env::new();
    assert_eq!(env.eval_line("2 + 3"), Ok(5.0));
}

#[test]
fn test_env_eval_line_assignment() {
    let mut env = Env::new();
    env.eval_line("x = 10").unwrap();
    assert_eq!(env.get("x"), Some(10.0));
}

#[test]
fn test_env_substitute() {
    let mut env = Env::new();
    env.set("x", 5.0);
    let result = env.substitute("x + 3");
    assert_eq!(result, "5 + 3");
}

#[test]
fn test_env_eval_with_vars() {
    let mut env = Env::new();
    env.set("x", 10.0);
    assert_eq!(env.eval_with_vars("x + 5"), Ok(15.0));
}


// ===== Topic 7: Extra Practice =====

#[test]
fn test_eval_postfix() {
    assert_eq!(eval_postfix("3 4 +"), Ok(7.0));
    assert_eq!(eval_postfix("5 3 - 2 *"), Ok(4.0));
    assert!(eval_postfix("1 0 /").is_err());
}

#[test]
fn test_balanced_parens() {
    assert!(balanced_parens("(())"));
    assert!(balanced_parens("()()"));
    assert!(!balanced_parens("(()"));
    assert!(!balanced_parens(")("));
    assert!(balanced_parens(""));
}

#[test]
fn test_infix_to_postfix() {
    assert_eq!(infix_to_postfix("3+4"), "3 4 +");
    assert_eq!(infix_to_postfix("3+4*2"), "3 4 2 * +");
}
