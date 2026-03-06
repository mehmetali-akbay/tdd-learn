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
        todo!()
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
        todo!()
    }

    /// Count the depth of nesting
    pub fn depth(&self) -> usize {
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
}

// ============================================
// Topic 5: End-to-End — calc()
// Learn: Composing tokenizer + parser + evaluator
// ============================================

/// Calculate the result of an expression string
pub fn calc(input: &str) -> Result<f64, CalcError> {
        todo!()
}

/// Format a result nicely (remove trailing .0 for integers)
pub fn format_result(value: f64) -> String {
        todo!()
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
        todo!()
    }

    pub fn set(&mut self, name: &str, value: f64) {
        todo!()
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        todo!()
    }

    /// Evaluate a simple "var = expr" assignment or an expression
    pub fn eval_line(&mut self, line: &str) -> Result<f64, CalcError> {
        todo!()
    }

    /// Substitute known variables into an expression string
    /// (simple text replacement, variables must be single lowercase letters)
    pub fn substitute(&self, expr: &str) -> String {
        todo!()
    }

    /// Evaluate with variable substitution
    pub fn eval_with_vars(&mut self, expr: &str) -> Result<f64, CalcError> {
        todo!()
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
        todo!()
}

/// Check if parentheses are balanced: "(())" -> true, "(()" -> false
pub fn balanced_parens(s: &str) -> bool {
        todo!()
}

/// Convert infix single-digit "3+4*2" to postfix "3 4 2 * +"
pub fn infix_to_postfix(expr: &str) -> String {
        todo!()
}
