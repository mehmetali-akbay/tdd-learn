// ============================================
// Level 4, Project 1: Calculator — Full Mini Project
// Learn: Tokenizer, parser, evaluator, end-to-end design
// ============================================

use std::collections::HashMap;
use std::fmt;

// ============================================
// Topic 1: Tokens — Lexical Analysis
// Learn: Enums for tokens, character scanning, token classification
// ============================================

/// Token types for our calculator
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{n}"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

/// Tokenize an expression string into a list of tokens
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                // Negative number: if '-' is at the start or after an operator/LParen
                let is_unary = tokens.is_empty()
                    || matches!(
                        tokens.last(),
                        Some(Token::Plus)
                            | Some(Token::Minus)
                            | Some(Token::Star)
                            | Some(Token::Slash)
                            | Some(Token::Percent)
                            | Some(Token::Caret)
                            | Some(Token::LParen)
                    );
                if is_unary {
                    chars.next();
                    let num = read_number(&mut chars)?;
                    tokens.push(Token::Number(-num));
                } else {
                    tokens.push(Token::Minus);
                    chars.next();
                }
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '%' => {
                tokens.push(Token::Percent);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Caret);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            c if c.is_ascii_digit() || c == '.' => {
                let num = read_number(&mut chars)?;
                tokens.push(Token::Number(num));
            }
            c => return Err(format!("unexpected character: '{c}'")),
        }
    }
    Ok(tokens)
}

fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
    let mut num_str = String::new();
    let mut has_dot = false;
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num_str.push(ch);
            chars.next();
        } else if ch == '.' && !has_dot {
            has_dot = true;
            num_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    if num_str.is_empty() {
        return Err("expected a number".to_string());
    }
    num_str
        .parse::<f64>()
        .map_err(|e| format!("invalid number '{num_str}': {e}"))
}

/// Count the total number of tokens
pub fn token_count(tokens: &[Token]) -> usize {
    tokens.len()
}

/// Count operator tokens (+, -, *, /, %, ^)
pub fn count_operators(tokens: &[Token]) -> usize {
    tokens
        .iter()
        .filter(|t| {
            matches!(
                t,
                Token::Plus
                    | Token::Minus
                    | Token::Star
                    | Token::Slash
                    | Token::Percent
                    | Token::Caret
            )
        })
        .count()
}

/// Count number tokens
pub fn count_numbers(tokens: &[Token]) -> usize {
    tokens
        .iter()
        .filter(|t| matches!(t, Token::Number(_)))
        .count()
}

/// Check if the token stream contains any parentheses
pub fn has_parentheses(tokens: &[Token]) -> bool {
    tokens
        .iter()
        .any(|t| matches!(t, Token::LParen | Token::RParen))
}

/// Reconstruct expression string from tokens
pub fn tokens_to_string(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

// ============================================
// Topic 2: AST — Abstract Syntax Tree
// Learn: Recursive data structures, tree traversal, node operations
// ============================================

/// Expression AST node
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryMinus(Box<Expr>),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::Pow => write!(f, "^"),
        }
    }
}

impl Expr {
    /// Pretty-print an expression with parentheses
    pub fn to_string_expr(&self) -> String {
        match self {
            Expr::Number(n) => format!("{n}"),
            Expr::BinOp { op, left, right } => {
                format!(
                    "({} {} {})",
                    left.to_string_expr(),
                    op,
                    right.to_string_expr()
                )
            }
            Expr::UnaryMinus(e) => format!("(-{})", e.to_string_expr()),
        }
    }

    /// Count the depth of nesting
    pub fn depth(&self) -> usize {
        match self {
            Expr::Number(_) => 0,
            Expr::UnaryMinus(e) => 1 + e.depth(),
            Expr::BinOp { left, right, .. } => 1 + left.depth().max(right.depth()),
        }
    }

    /// Count total nodes in the AST
    pub fn node_count(&self) -> usize {
        match self {
            Expr::Number(_) => 1,
            Expr::UnaryMinus(e) => 1 + e.node_count(),
            Expr::BinOp { left, right, .. } => 1 + left.node_count() + right.node_count(),
        }
    }

    /// Count leaf nodes (numbers only)
    pub fn leaf_count(&self) -> usize {
        match self {
            Expr::Number(_) => 1,
            Expr::UnaryMinus(e) => e.leaf_count(),
            Expr::BinOp { left, right, .. } => left.leaf_count() + right.leaf_count(),
        }
    }

    /// Check if the tree contains a specific binary operator
    pub fn contains_op(&self, target: BinOp) -> bool {
        match self {
            Expr::Number(_) => false,
            Expr::UnaryMinus(e) => e.contains_op(target),
            Expr::BinOp { op, left, right } => {
                *op == target || left.contains_op(target) || right.contains_op(target)
            }
        }
    }

    /// Check if the expression is a single literal number
    pub fn is_literal(&self) -> bool {
        matches!(self, Expr::Number(_))
    }

    /// Create a negated copy of this expression
    pub fn negate(&self) -> Expr {
        Expr::UnaryMinus(Box::new(self.clone()))
    }

    /// Collect all distinct binary operators used in the expression
    pub fn operators_used(&self) -> Vec<BinOp> {
        let mut ops = Vec::new();
        self.collect_ops(&mut ops);
        ops
    }

    fn collect_ops(&self, ops: &mut Vec<BinOp>) {
        match self {
            Expr::Number(_) => {}
            Expr::UnaryMinus(e) => e.collect_ops(ops),
            Expr::BinOp { op, left, right } => {
                if !ops.contains(op) {
                    ops.push(*op);
                }
                left.collect_ops(ops);
                right.collect_ops(ops);
            }
        }
    }
}

// ============================================
// Topic 3: Parser — Recursive Descent
// Learn: Operator precedence, right-associativity, error recovery
// ============================================

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    /// Parse a full expression (consumes all tokens)
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_add_sub()?;
        if self.pos < self.tokens.len() {
            return Err(format!("unexpected token: {}", self.tokens[self.pos]));
        }
        Ok(expr)
    }

    // Addition and subtraction (lowest precedence)
    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div_mod()?;
        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_mul_div_mod()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // Multiplication, division, and modulo
    fn parse_mul_div_mod(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // Power (right-associative, highest binary precedence)
    fn parse_power(&mut self) -> Result<Expr, String> {
        let base = self.parse_primary()?;
        if let Some(Token::Caret) = self.peek() {
            self.advance();
            let exp = self.parse_power()?; // right-recursive for right-associativity
            Ok(Expr::BinOp {
                op: BinOp::Pow,
                left: Box::new(base),
                right: Box::new(exp),
            })
        } else {
            Ok(base)
        }
    }

    // Primary: numbers and parenthesized expressions
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_add_sub()?;
                match self.advance() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("expected closing ')'".to_string()),
                }
            }
            Some(tok) => Err(format!("unexpected token: {tok}")),
            None => Err("unexpected end of expression".to_string()),
        }
    }
}

/// Parse an expression string into an AST
pub fn parse(input: &str) -> Result<Expr, String> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err("empty expression".to_string());
    }
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Parse directly from a token vector
pub fn parse_tokens(tokens: Vec<Token>) -> Result<Expr, String> {
    if tokens.is_empty() {
        return Err("empty expression".to_string());
    }
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Try to parse an expression, returning None on failure
pub fn try_parse(input: &str) -> Option<Expr> {
    parse(input).ok()
}

/// Parse multiple expressions at once
pub fn parse_many(exprs: &[&str]) -> Vec<Result<Expr, String>> {
    exprs.iter().map(|e| parse(e)).collect()
}

/// Describe an expression in human-readable form
pub fn describe_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => format!("literal {n}"),
        Expr::UnaryMinus(e) => format!("negation of {}", describe_expr(e)),
        Expr::BinOp { op, left, right } => {
            let op_name = match op {
                BinOp::Add => "addition",
                BinOp::Sub => "subtraction",
                BinOp::Mul => "multiplication",
                BinOp::Div => "division",
                BinOp::Mod => "modulo",
                BinOp::Pow => "power",
            };
            format!(
                "{} of {} and {}",
                op_name,
                describe_expr(left),
                describe_expr(right)
            )
        }
    }
}

// ============================================
// Topic 4: Evaluator
// Learn: Recursive evaluation, error handling, numeric utilities
// ============================================

/// Calculator error
#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    ParseError(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "division by zero"),
            CalcError::ParseError(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

/// Evaluate an AST expression
pub fn eval_expr(expr: &Expr) -> Result<f64, CalcError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::UnaryMinus(e) => Ok(-eval_expr(e)?),
        Expr::BinOp { op, left, right } => {
            let l = eval_expr(left)?;
            let r = eval_expr(right)?;
            match op {
                BinOp::Add => Ok(l + r),
                BinOp::Sub => Ok(l - r),
                BinOp::Mul => Ok(l * r),
                BinOp::Div => {
                    if r == 0.0 {
                        Err(CalcError::DivisionByZero)
                    } else {
                        Ok(l / r)
                    }
                }
                BinOp::Mod => {
                    if r == 0.0 {
                        Err(CalcError::DivisionByZero)
                    } else {
                        Ok(l % r)
                    }
                }
                BinOp::Pow => Ok(l.powf(r)),
            }
        }
    }
}

/// Evaluate an expression, returning None on error
pub fn eval_safe(expr: &Expr) -> Option<f64> {
    eval_expr(expr).ok()
}

/// Evaluate an expression, returning a default on error
pub fn eval_or_default(expr: &Expr, default: f64) -> f64 {
    eval_expr(expr).unwrap_or(default)
}

/// Check if a result is a valid finite number
pub fn is_valid_result(value: f64) -> bool {
    value.is_finite()
}

/// Compare two floats with a given epsilon tolerance
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Clamp a value within a range
pub fn clamp_result(value: f64, min: f64, max: f64) -> f64 {
    value.clamp(min, max)
}

// ============================================
// Topic 5: End-to-End — calc()
// Learn: Composing tokenizer + parser + evaluator, batch processing
// ============================================

/// Calculate the result of an expression string
pub fn calc(input: &str) -> Result<f64, CalcError> {
    let expr = parse(input).map_err(CalcError::ParseError)?;
    eval_expr(&expr)
}

/// Calculate with a fallback default value
pub fn calc_or_default(input: &str, default: f64) -> f64 {
    calc(input).unwrap_or(default)
}

/// Evaluate multiple expressions at once
pub fn calc_many(exprs: &[&str]) -> Vec<Result<f64, CalcError>> {
    exprs.iter().map(|e| calc(e)).collect()
}

/// Format a result nicely (remove trailing .0 for integers)
pub fn format_result(value: f64) -> String {
    if value == value.floor() && value.is_finite() {
        format!("{}", value as i64)
    } else {
        format!("{value:.6}")
    }
}

/// Format a result with a specific number of decimal places
pub fn format_result_precision(value: f64, decimals: usize) -> String {
    if decimals == 0 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.decimals$}")
    }
}

/// Calculate and format the result in one step
pub fn calc_and_format(input: &str) -> String {
    match calc(input) {
        Ok(value) => format_result(value),
        Err(e) => format!("Error: {e}"),
    }
}

/// Compare two expression results, returning their ordering
pub fn compare_expressions(a: &str, b: &str) -> Option<std::cmp::Ordering> {
    let va = calc(a).ok()?;
    let vb = calc(b).ok()?;
    va.partial_cmp(&vb)
}

// ============================================
// Topic 6: Variables & Environment
// Learn: HashMap state management, substitution, merging
// ============================================

/// An environment holding variable bindings
#[derive(Debug, Clone)]
pub struct Env {
    vars: HashMap<String, f64>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
        }
    }

    /// Set a variable value
    pub fn set(&mut self, name: &str, value: f64) {
        self.vars.insert(name.to_string(), value);
    }

    /// Get a variable value
    pub fn get(&self, name: &str) -> Option<f64> {
        self.vars.get(name).copied()
    }

    /// Check if a variable exists
    pub fn has_var(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    /// Remove a variable, returning its value if it existed
    pub fn remove(&mut self, name: &str) -> Option<f64> {
        self.vars.remove(name)
    }

    /// Clear all variables
    pub fn clear(&mut self) {
        self.vars.clear();
    }

    /// Get the number of defined variables
    pub fn var_count(&self) -> usize {
        self.vars.len()
    }

    /// List all variable names sorted alphabetically
    pub fn list_vars(&self) -> Vec<String> {
        let mut names: Vec<String> = self.vars.keys().cloned().collect();
        names.sort();
        names
    }

    /// Evaluate a simple "var = expr" assignment or an expression
    pub fn eval_line(&mut self, line: &str) -> Result<f64, CalcError> {
        if let Some(eq_pos) = line.find('=') {
            let name = line[..eq_pos].trim();
            let expr_str = line[eq_pos + 1..].trim();
            let value = calc(expr_str)?;
            self.set(name, value);
            Ok(value)
        } else {
            calc(line)
        }
    }

    /// Substitute known variables into an expression string
    /// (sorts by name length descending to avoid partial replacements)
    pub fn substitute(&self, expr: &str) -> String {
        let mut result = expr.to_string();
        let mut pairs: Vec<_> = self.vars.iter().collect();
        pairs.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        for (name, value) in pairs {
            result = result.replace(name.as_str(), &format!("{value}"));
        }
        result
    }

    /// Evaluate with variable substitution
    pub fn eval_with_vars(&mut self, expr: &str) -> Result<f64, CalcError> {
        let substituted = self.substitute(expr);
        calc(&substituted)
    }

    /// Create an environment from an array of name-value pairs
    pub fn from_pairs(pairs: &[(&str, f64)]) -> Self {
        let mut env = Env::new();
        for &(name, value) in pairs {
            env.set(name, value);
        }
        env
    }

    /// Merge another environment into this one (other's values take precedence)
    pub fn merge(&mut self, other: &Env) {
        for (name, &value) in &other.vars {
            self.vars.insert(name.clone(), value);
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 7: Postfix, Prefix & Expression Utilities
// Learn: Stack-based evaluation, notation conversion, validation
// ============================================

/// Evaluate a postfix (RPN) expression: "3 4 +" -> 7.0
pub fn eval_postfix(expr: &str) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err("division by zero".into());
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => {
                let num: f64 = token
                    .parse()
                    .map_err(|_| format!("bad token: {token}"))?;
                stack.push(num);
            }
        }
    }
    if stack.len() != 1 {
        return Err("invalid expression".into());
    }
    Ok(stack.pop().unwrap())
}

/// Check if parentheses are balanced
pub fn balanced_parens(s: &str) -> bool {
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

/// Convert infix to postfix using the Shunting-Yard algorithm
/// Supports single-char digits and operators: +, -, *, /, %, ^
pub fn infix_to_postfix(expr: &str) -> String {
    let mut output = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let precedence = |op: char| -> i32 {
        match op {
            '+' | '-' => 1,
            '*' | '/' | '%' => 2,
            '^' => 3,
            _ => 0,
        }
    };
    let is_right_assoc = |op: char| -> bool { op == '^' };
    for c in expr.chars() {
        if c.is_ascii_digit() || c == '.' {
            output.push(c.to_string());
        } else if "+-*/%^".contains(c) {
            while let Some(&top) = ops.last() {
                if top != '('
                    && (precedence(top) > precedence(c)
                        || (precedence(top) == precedence(c) && !is_right_assoc(c)))
                {
                    output.push(ops.pop().unwrap().to_string());
                } else {
                    break;
                }
            }
            ops.push(c);
        } else if c == '(' {
            ops.push(c);
        } else if c == ')' {
            while let Some(op) = ops.pop() {
                if op == '(' {
                    break;
                }
                output.push(op.to_string());
            }
        }
    }
    while let Some(op) = ops.pop() {
        output.push(op.to_string());
    }
    output.join(" ")
}

/// Find the maximum parenthesis nesting depth
pub fn max_paren_depth(s: &str) -> usize {
    let mut depth: usize = 0;
    let mut max_depth: usize = 0;
    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                max_depth = max_depth.max(depth);
            }
            ')' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    max_depth
}

/// Count the number of arithmetic operations in an expression string
pub fn count_operations(expr: &str) -> usize {
    tokenize(expr)
        .map(|tokens| count_operators(&tokens))
        .unwrap_or(0)
}

/// Check if an expression string is syntactically valid
pub fn validate_expression(expr: &str) -> bool {
    parse(expr).is_ok()
}

/// Convert a postfix expression to infix with parentheses
pub fn postfix_to_infix(expr: &str) -> Result<String, String> {
    let mut stack: Vec<String> = Vec::new();
    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" | "%" | "^" => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(format!("({a} {token} {b})"));
            }
            _ => {
                let _: f64 = token
                    .parse()
                    .map_err(|_| format!("bad token: {token}"))?;
                stack.push(token.to_string());
            }
        }
    }
    if stack.len() != 1 {
        return Err("invalid expression".into());
    }
    Ok(stack.pop().unwrap())
}

/// Evaluate a prefix (Polish notation) expression: "+ 3 4" -> 7.0
pub fn eval_prefix(expr: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut pos = 0;
    let result = eval_prefix_helper(&tokens, &mut pos)?;
    if pos != tokens.len() {
        return Err("extra tokens in expression".into());
    }
    Ok(result)
}

fn eval_prefix_helper(tokens: &[&str], pos: &mut usize) -> Result<f64, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of expression".into());
    }
    let token = tokens[*pos];
    *pos += 1;
    match token {
        "+" | "-" | "*" | "/" => {
            let a = eval_prefix_helper(tokens, pos)?;
            let b = eval_prefix_helper(tokens, pos)?;
            match token {
                "+" => Ok(a + b),
                "-" => Ok(a - b),
                "*" => Ok(a * b),
                "/" => {
                    if b == 0.0 {
                        return Err("division by zero".into());
                    }
                    Ok(a / b)
                }
                _ => unreachable!(),
            }
        }
        _ => token
            .parse::<f64>()
            .map_err(|_| format!("bad token: {token}")),
    }
}
