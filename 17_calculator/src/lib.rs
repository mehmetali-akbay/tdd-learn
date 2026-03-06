// ============================================
// Level 4, Project 1: Calculator — Full Mini Project
// Learn: Tokenizer, parser, evaluator, end-to-end design
// ============================================

use std::fmt;

// ============================================
// Topic 1: Tokens — Lexical Analysis
// Learn: Enums for tokens, character scanning
// ============================================

/// Token types for our calculator
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
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
            c => return Err(format!("unexpected character: '{}'", c)),
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
        .map_err(|e| format!("invalid number '{}': {}", num_str, e))
}

// ============================================
// Topic 2: AST — Abstract Syntax Tree
// Learn: Recursive data structure for expressions
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
        }
    }
}

impl Expr {
    /// Pretty-print an expression
    pub fn to_string_expr(&self) -> String {
        match self {
            Expr::Number(n) => format!("{}", n),
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
}

// ============================================
// Topic 3: Parser — Recursive Descent
// Learn: Operator precedence, recursive descent parsing
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

    /// Parse a full expression
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_add_sub()?;
        if self.pos < self.tokens.len() {
            return Err(format!("unexpected token: {}", self.tokens[self.pos]));
        }
        Ok(expr)
    }

    // Addition and subtraction (lowest precedence)
    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div()?;
        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_mul_div()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // Multiplication and division (higher precedence)
    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;
        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
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
            Some(tok) => Err(format!("unexpected token: {}", tok)),
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

// ============================================
// Topic 4: Evaluator
// Learn: Recursive evaluation, error handling
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
            CalcError::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

/// Evaluate an AST
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
            }
        }
    }
}

// ============================================
// Topic 5: End-to-End — calc()
// Learn: Composing tokenizer + parser + evaluator
// ============================================

/// Calculate the result of an expression string
pub fn calc(input: &str) -> Result<f64, CalcError> {
    let expr = parse(input).map_err(CalcError::ParseError)?;
    eval_expr(&expr)
}

/// Format a result nicely (remove trailing .0 for integers)
pub fn format_result(value: f64) -> String {
    if value == value.floor() && value.is_finite() {
        format!("{}", value as i64)
    } else {
        format!("{:.6}", value)
    }
}

// ============================================
// Topic 6: Advanced — Variables & Functions
// Learn: HashMap for variable storage, extensibility
// ============================================

use std::collections::HashMap;

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

    pub fn set(&mut self, name: &str, value: f64) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.vars.get(name).copied()
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
    /// (simple text replacement, variables must be single lowercase letters)
    pub fn substitute(&self, expr: &str) -> String {
        let mut result = expr.to_string();
        for (name, value) in &self.vars {
            result = result.replace(name, &format!("{}", value));
        }
        result
    }

    /// Evaluate with variable substitution
    pub fn eval_with_vars(&mut self, expr: &str) -> Result<f64, CalcError> {
        let substituted = self.substitute(expr);
        calc(&substituted)
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 7: Extra Practice
// Learn: More expression handling and evaluation
// ============================================

/// Evaluate a simple postfix expression: "3 4 +" -> 7
pub fn eval_postfix(expr: &str) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or("underflow")?;
                let a = stack.pop().ok_or("underflow")?;
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => { if b == 0.0 { return Err("div by zero".into()); } a / b },
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => {
                let num: f64 = token.parse().map_err(|_| format!("bad token: {}", token))?;
                stack.push(num);
            }
        }
    }
    stack.pop().ok_or("empty expression".into())
}

/// Check if parentheses are balanced: "(())" -> true, "(()" -> false
pub fn balanced_parens(s: &str) -> bool {
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => { depth -= 1; if depth < 0 { return false; } }
            _ => {}
        }
    }
    depth == 0
}

/// Convert infix single-digit "3+4*2" to postfix "3 4 2 * +"
pub fn infix_to_postfix(expr: &str) -> String {
    let mut output = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let precedence = |op: char| -> i32 {
        match op { '+' | '-' => 1, '*' | '/' => 2, _ => 0 }
    };
    for c in expr.chars() {
        if c.is_ascii_digit() || c == '.' {
            output.push(c.to_string());
        } else if "+-*/".contains(c) {
            while let Some(&top) = ops.last() {
                if top != '(' && precedence(top) >= precedence(c) {
                    output.push(ops.pop().unwrap().to_string());
                } else { break; }
            }
            ops.push(c);
        } else if c == '(' {
            ops.push(c);
        } else if c == ')' {
            while let Some(op) = ops.pop() {
                if op == '(' { break; }
                output.push(op.to_string());
            }
        }
    }
    while let Some(op) = ops.pop() { output.push(op.to_string()); }
    output.join(" ")
}
