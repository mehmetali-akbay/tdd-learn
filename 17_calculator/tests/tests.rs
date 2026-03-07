use calculator::*;
use std::cmp::Ordering;

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
    let tokens = tokenize("1 + 2 - 3 * 4 / 5 % 6 ^ 7").unwrap();
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[1], Token::Plus);
    assert_eq!(tokens[3], Token::Minus);
    assert_eq!(tokens[5], Token::Star);
    assert_eq!(tokens[7], Token::Slash);
    assert_eq!(tokens[9], Token::Percent);
    assert_eq!(tokens[11], Token::Caret);
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
fn test_tokenize_unary_after_operator() {
    let tokens = tokenize("3 * -2").unwrap();
    assert_eq!(
        tokens,
        vec![Token::Number(3.0), Token::Star, Token::Number(-2.0)]
    );
}

#[test]
fn test_tokenize_error() {
    assert!(tokenize("2 + @").is_err());
}

#[test]
fn test_token_display() {
    assert_eq!(format!("{}", Token::Plus), "+");
    assert_eq!(format!("{}", Token::Percent), "%");
    assert_eq!(format!("{}", Token::Caret), "^");
    assert_eq!(format!("{}", Token::Number(3.5)), "3.5");
}

#[test]
fn test_token_count() {
    let tokens = tokenize("2 + 3 * 4").unwrap();
    assert_eq!(token_count(&tokens), 5);
    assert_eq!(token_count(&[]), 0);
}

#[test]
fn test_count_operators() {
    let tokens = tokenize("2 + 3 * 4 - 1").unwrap();
    assert_eq!(count_operators(&tokens), 3);
}

#[test]
fn test_count_numbers() {
    let tokens = tokenize("2 + 3 * 4").unwrap();
    assert_eq!(count_numbers(&tokens), 3);
}

#[test]
fn test_has_parentheses() {
    let with = tokenize("(2 + 3)").unwrap();
    let without = tokenize("2 + 3").unwrap();
    assert!(has_parentheses(&with));
    assert!(!has_parentheses(&without));
}

#[test]
fn test_tokens_to_string() {
    let tokens = tokenize("2 + 3").unwrap();
    assert_eq!(tokens_to_string(&tokens), "2 + 3");
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
fn test_expr_display_unary() {
    let expr = Expr::UnaryMinus(Box::new(Expr::Number(5.0)));
    assert_eq!(expr.to_string_expr(), "(-5)");
}

#[test]
fn test_binop_display() {
    assert_eq!(format!("{}", BinOp::Add), "+");
    assert_eq!(format!("{}", BinOp::Mod), "%");
    assert_eq!(format!("{}", BinOp::Pow), "^");
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

#[test]
fn test_node_count() {
    assert_eq!(Expr::Number(5.0).node_count(), 1);
    let expr = Expr::BinOp {
        op: BinOp::Add,
        left: Box::new(Expr::Number(2.0)),
        right: Box::new(Expr::Number(3.0)),
    };
    assert_eq!(expr.node_count(), 3); // 1 binop + 2 numbers
}

#[test]
fn test_leaf_count() {
    let expr = Expr::BinOp {
        op: BinOp::Add,
        left: Box::new(Expr::BinOp {
            op: BinOp::Mul,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        }),
        right: Box::new(Expr::Number(3.0)),
    };
    assert_eq!(expr.leaf_count(), 3);
}

#[test]
fn test_contains_op() {
    let expr = parse("2 + 3 * 4").unwrap();
    assert!(expr.contains_op(BinOp::Add));
    assert!(expr.contains_op(BinOp::Mul));
    assert!(!expr.contains_op(BinOp::Div));
}

#[test]
fn test_is_literal() {
    assert!(Expr::Number(42.0).is_literal());
    let expr = parse("2 + 3").unwrap();
    assert!(!expr.is_literal());
}

#[test]
fn test_negate() {
    let expr = Expr::Number(5.0);
    let neg = expr.negate();
    assert_eq!(neg, Expr::UnaryMinus(Box::new(Expr::Number(5.0))));
}

#[test]
fn test_operators_used() {
    let expr = parse("2 + 3 * 4 + 1").unwrap();
    let ops = expr.operators_used();
    assert!(ops.contains(&BinOp::Add));
    assert!(ops.contains(&BinOp::Mul));
    assert!(!ops.contains(&BinOp::Div));
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
fn test_parse_power() {
    let expr = parse("2 ^ 3").unwrap();
    let result = eval_expr(&expr).unwrap();
    assert!((result - 8.0).abs() < 0.001);
}

#[test]
fn test_parse_power_right_assoc() {
    // 2 ^ 3 ^ 2 should be 2 ^ (3 ^ 2) = 2 ^ 9 = 512
    let expr = parse("2 ^ 3 ^ 2").unwrap();
    let result = eval_expr(&expr).unwrap();
    assert!((result - 512.0).abs() < 0.001);
}

#[test]
fn test_parse_modulo() {
    let expr = parse("10 % 3").unwrap();
    let result = eval_expr(&expr).unwrap();
    assert!((result - 1.0).abs() < 0.001);
}

#[test]
fn test_parse_error_empty() {
    assert!(parse("").is_err());
}

#[test]
fn test_parse_error_unmatched_paren() {
    assert!(parse("(2 + 3").is_err());
}

#[test]
fn test_parse_tokens_fn() {
    let tokens = vec![Token::Number(5.0), Token::Plus, Token::Number(3.0)];
    let expr = parse_tokens(tokens).unwrap();
    assert_eq!(eval_expr(&expr).unwrap(), 8.0);
    assert!(parse_tokens(vec![]).is_err());
}

#[test]
fn test_try_parse() {
    assert!(try_parse("2 + 3").is_some());
    assert!(try_parse("").is_none());
    assert!(try_parse("@@@").is_none());
}

#[test]
fn test_parse_many() {
    let results = parse_many(&["2 + 3", "bad!", "10"]);
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    assert!(results[2].is_ok());
}

#[test]
fn test_describe_expr() {
    let expr = parse("2 + 3").unwrap();
    assert_eq!(
        describe_expr(&expr),
        "addition of literal 2 and literal 3"
    );
    let neg = Expr::UnaryMinus(Box::new(Expr::Number(5.0)));
    assert_eq!(describe_expr(&neg), "negation of literal 5");
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
fn test_eval_sub_mul() {
    let sub = Expr::BinOp {
        op: BinOp::Sub,
        left: Box::new(Expr::Number(10.0)),
        right: Box::new(Expr::Number(4.0)),
    };
    assert_eq!(eval_expr(&sub), Ok(6.0));

    let mul = Expr::BinOp {
        op: BinOp::Mul,
        left: Box::new(Expr::Number(3.0)),
        right: Box::new(Expr::Number(7.0)),
    };
    assert_eq!(eval_expr(&mul), Ok(21.0));
}

#[test]
fn test_eval_div() {
    let expr = Expr::BinOp {
        op: BinOp::Div,
        left: Box::new(Expr::Number(20.0)),
        right: Box::new(Expr::Number(4.0)),
    };
    assert_eq!(eval_expr(&expr), Ok(5.0));
}

#[test]
fn test_eval_mod() {
    let expr = Expr::BinOp {
        op: BinOp::Mod,
        left: Box::new(Expr::Number(10.0)),
        right: Box::new(Expr::Number(3.0)),
    };
    assert_eq!(eval_expr(&expr), Ok(1.0));
}

#[test]
fn test_eval_pow() {
    let expr = Expr::BinOp {
        op: BinOp::Pow,
        left: Box::new(Expr::Number(2.0)),
        right: Box::new(Expr::Number(10.0)),
    };
    assert_eq!(eval_expr(&expr), Ok(1024.0));
}

#[test]
fn test_eval_unary_minus() {
    let expr = Expr::UnaryMinus(Box::new(Expr::Number(7.0)));
    assert_eq!(eval_expr(&expr), Ok(-7.0));
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
fn test_eval_mod_by_zero() {
    let expr = Expr::BinOp {
        op: BinOp::Mod,
        left: Box::new(Expr::Number(5.0)),
        right: Box::new(Expr::Number(0.0)),
    };
    assert_eq!(eval_expr(&expr), Err(CalcError::DivisionByZero));
}

#[test]
fn test_eval_safe() {
    let good = Expr::Number(42.0);
    let bad = Expr::BinOp {
        op: BinOp::Div,
        left: Box::new(Expr::Number(1.0)),
        right: Box::new(Expr::Number(0.0)),
    };
    assert_eq!(eval_safe(&good), Some(42.0));
    assert_eq!(eval_safe(&bad), None);
}

#[test]
fn test_eval_or_default() {
    let bad = Expr::BinOp {
        op: BinOp::Div,
        left: Box::new(Expr::Number(1.0)),
        right: Box::new(Expr::Number(0.0)),
    };
    assert_eq!(eval_or_default(&Expr::Number(5.0), 0.0), 5.0);
    assert_eq!(eval_or_default(&bad, -1.0), -1.0);
}

#[test]
fn test_is_valid_result() {
    assert!(is_valid_result(42.0));
    assert!(is_valid_result(-3.14));
    assert!(!is_valid_result(f64::INFINITY));
    assert!(!is_valid_result(f64::NAN));
}

#[test]
fn test_approx_equal() {
    assert!(approx_equal(1.0, 1.0000001, 0.001));
    assert!(!approx_equal(1.0, 2.0, 0.001));
}

#[test]
fn test_clamp_result() {
    assert_eq!(clamp_result(5.0, 0.0, 10.0), 5.0);
    assert_eq!(clamp_result(-5.0, 0.0, 10.0), 0.0);
    assert_eq!(clamp_result(15.0, 0.0, 10.0), 10.0);
}

#[test]
fn test_calc_error_display() {
    assert_eq!(format!("{}", CalcError::DivisionByZero), "division by zero");
    assert_eq!(
        format!("{}", CalcError::ParseError("bad".into())),
        "parse error: bad"
    );
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
fn test_calc_power_and_mod() {
    assert!((calc("2 ^ 8").unwrap() - 256.0).abs() < 0.001);
    assert!((calc("17 % 5").unwrap() - 2.0).abs() < 0.001);
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
fn test_calc_or_default_fn() {
    assert_eq!(calc_or_default("2 + 3", 0.0), 5.0);
    assert_eq!(calc_or_default("bad!", 0.0), 0.0);
    assert_eq!(calc_or_default("1 / 0", -1.0), -1.0);
}

#[test]
fn test_calc_many_fn() {
    let results = calc_many(&["2 + 3", "10 / 0", "7 * 2"]);
    assert_eq!(results[0], Ok(5.0));
    assert_eq!(results[1], Err(CalcError::DivisionByZero));
    assert_eq!(results[2], Ok(14.0));
}

#[test]
fn test_format_result() {
    assert_eq!(format_result(5.0), "5");
    assert_eq!(format_result(3.15), "3.150000");
    assert_eq!(format_result(-10.0), "-10");
}

#[test]
fn test_format_result_precision() {
    assert_eq!(format_result_precision(3.14159, 2), "3.14");
    assert_eq!(format_result_precision(3.14159, 4), "3.1416");
    assert_eq!(format_result_precision(5.0, 0), "5");
}

#[test]
fn test_calc_and_format_fn() {
    assert_eq!(calc_and_format("2 + 3"), "5");
    assert_eq!(calc_and_format("1 / 0"), "Error: division by zero");
    assert!(calc_and_format("bad!").starts_with("Error:"));
}

#[test]
fn test_compare_expressions_fn() {
    assert_eq!(
        compare_expressions("2 + 3", "10 / 2"),
        Some(Ordering::Equal)
    );
    assert_eq!(compare_expressions("10", "5"), Some(Ordering::Greater));
    assert_eq!(compare_expressions("1", "2"), Some(Ordering::Less));
    assert_eq!(compare_expressions("bad", "1"), None);
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
fn test_env_has_var() {
    let mut env = Env::new();
    env.set("x", 1.0);
    assert!(env.has_var("x"));
    assert!(!env.has_var("y"));
}

#[test]
fn test_env_remove() {
    let mut env = Env::new();
    env.set("x", 42.0);
    assert_eq!(env.remove("x"), Some(42.0));
    assert_eq!(env.remove("x"), None);
    assert!(!env.has_var("x"));
}

#[test]
fn test_env_clear() {
    let mut env = Env::new();
    env.set("a", 1.0);
    env.set("b", 2.0);
    env.clear();
    assert_eq!(env.var_count(), 0);
    assert!(!env.has_var("a"));
}

#[test]
fn test_env_var_count() {
    let mut env = Env::new();
    assert_eq!(env.var_count(), 0);
    env.set("x", 1.0);
    env.set("y", 2.0);
    assert_eq!(env.var_count(), 2);
}

#[test]
fn test_env_list_vars() {
    let mut env = Env::new();
    env.set("z", 3.0);
    env.set("a", 1.0);
    env.set("m", 2.0);
    assert_eq!(env.list_vars(), vec!["a", "m", "z"]);
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

#[test]
fn test_env_from_pairs() {
    let env = Env::from_pairs(&[("x", 1.0), ("y", 2.0)]);
    assert_eq!(env.get("x"), Some(1.0));
    assert_eq!(env.get("y"), Some(2.0));
    assert_eq!(env.var_count(), 2);
}

#[test]
fn test_env_merge() {
    let mut env1 = Env::from_pairs(&[("x", 1.0), ("y", 2.0)]);
    let env2 = Env::from_pairs(&[("y", 99.0), ("z", 3.0)]);
    env1.merge(&env2);
    assert_eq!(env1.get("x"), Some(1.0));
    assert_eq!(env1.get("y"), Some(99.0)); // overwritten
    assert_eq!(env1.get("z"), Some(3.0));
    assert_eq!(env1.var_count(), 3);
}

#[test]
fn test_env_default() {
    let env = Env::default();
    assert_eq!(env.var_count(), 0);
}

// ===== Topic 7: Postfix, Prefix & Expression Utilities =====

#[test]
fn test_eval_postfix() {
    assert_eq!(eval_postfix("3 4 +"), Ok(7.0));
    assert_eq!(eval_postfix("5 3 - 2 *"), Ok(4.0));
    assert_eq!(eval_postfix("10 2 /"), Ok(5.0));
}

#[test]
fn test_eval_postfix_errors() {
    assert!(eval_postfix("1 0 /").is_err());
    assert!(eval_postfix("+").is_err());
    assert!(eval_postfix("abc").is_err());
}

#[test]
fn test_balanced_parens() {
    assert!(balanced_parens("(())"));
    assert!(balanced_parens("()()"));
    assert!(balanced_parens(""));
    assert!(balanced_parens("no parens here"));
    assert!(!balanced_parens("(()"));
    assert!(!balanced_parens(")("));
    assert!(!balanced_parens("((())"));
}

#[test]
fn test_infix_to_postfix() {
    assert_eq!(infix_to_postfix("3+4"), "3 4 +");
    assert_eq!(infix_to_postfix("3+4*2"), "3 4 2 * +");
    assert_eq!(infix_to_postfix("(3+4)*2"), "3 4 + 2 *");
}

#[test]
fn test_infix_to_postfix_power() {
    // ^ is right-associative: 2^3^2 = 2^(3^2)
    assert_eq!(infix_to_postfix("2^3^2"), "2 3 2 ^ ^");
}

#[test]
fn test_max_paren_depth() {
    assert_eq!(max_paren_depth("no parens"), 0);
    assert_eq!(max_paren_depth("(a)"), 1);
    assert_eq!(max_paren_depth("((a))"), 2);
    assert_eq!(max_paren_depth("((a) + (b))"), 2);
    assert_eq!(max_paren_depth("(((a)))"), 3);
}

#[test]
fn test_count_operations() {
    assert_eq!(count_operations("2 + 3"), 1);
    assert_eq!(count_operations("2 + 3 * 4"), 2);
    assert_eq!(count_operations("42"), 0);
    assert_eq!(count_operations("bad!"), 0);
}

#[test]
fn test_validate_expression() {
    assert!(validate_expression("2 + 3"));
    assert!(validate_expression("(1 + 2) * 3"));
    assert!(!validate_expression(""));
    assert!(!validate_expression("2 +"));
    assert!(!validate_expression("@@@"));
}

#[test]
fn test_postfix_to_infix() {
    assert_eq!(postfix_to_infix("3 4 +"), Ok("(3 + 4)".to_string()));
    assert_eq!(
        postfix_to_infix("3 4 + 2 *"),
        Ok("((3 + 4) * 2)".to_string())
    );
    assert!(postfix_to_infix("+").is_err());
    assert!(postfix_to_infix("abc").is_err());
}

#[test]
fn test_eval_prefix() {
    assert_eq!(eval_prefix("+ 3 4"), Ok(7.0));
    assert_eq!(eval_prefix("* + 3 4 2"), Ok(14.0));
    assert_eq!(eval_prefix("- 10 3"), Ok(7.0));
}

#[test]
fn test_eval_prefix_errors() {
    assert!(eval_prefix("/ 1 0").is_err());
    assert!(eval_prefix("+ 1").is_err());
    assert!(eval_prefix("abc").is_err());
}

#[test]
fn test_eval_prefix_extra_tokens() {
    assert!(eval_prefix("+ 3 4 5").is_err());
}
