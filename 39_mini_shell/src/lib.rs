// ============================================
// Level 10, Project 5: Mini Shell
// Learn: Command parsing, pipes, environment variables, glob patterns
// ============================================

use std::collections::HashMap;

// ============================================
// Topic 1: Command Parsing
// Learn: Splitting command strings into program + args
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        todo!()
    }

    pub fn arg_count(&self) -> usize { todo!() }

    pub fn to_string_repr(&self) -> String {
        todo!()
    }
}

/// Parse a command with quoted strings: echo "hello world" -> ["echo", "hello world"]
pub fn parse_quoted(input: &str) -> Vec<String> {
        todo!()
}

// ============================================
// Topic 2: Pipeline Parsing
// Learn: Pipe operator, command chaining
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

impl Pipeline {
    pub fn parse(input: &str) -> Option<Self> {
        todo!()
    }

    pub fn len(&self) -> usize { todo!() }
    pub fn is_empty(&self) -> bool { todo!() }

    pub fn is_pipeline(&self) -> bool { todo!() }
}

// ============================================
// Topic 3: Environment Variables
// Learn: Variable expansion, $VAR and ${VAR} syntax
// ============================================

pub struct Environment {
    vars: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        todo!()
    }

    pub fn set(&mut self, key: &str, value: &str) {
        todo!()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        todo!()
    }

    pub fn remove(&mut self, key: &str) -> bool {
        todo!()
    }

    /// Expand $VAR and ${VAR} in a string.
    pub fn expand(&self, input: &str) -> String {
        todo!()
    }

    pub fn len(&self) -> usize { todo!() }
    pub fn is_empty(&self) -> bool { todo!() }
}

impl Default for Environment {
    fn default() -> Self { Self::new() }
}

// ============================================
// Topic 4: Glob Pattern Matching
// Learn: Wildcard matching: *, ?, character classes
// ============================================

/// Match a string against a glob pattern (* = any chars, ? = one char).
pub fn glob_match(pattern: &str, text: &str) -> bool {
        todo!()
}

fn glob_match_inner(p: &[char], t: &[char], pi: usize, ti: usize) -> bool {
    if pi == p.len() { return ti == t.len(); }
    if p[pi] == '*' {
        // Try matching * as empty, one char, two chars, ...
        for skip in 0..=(t.len() - ti) {
            if glob_match_inner(p, t, pi + 1, ti + skip) { return true; }
        }
        false
    } else if p[pi] == '?' {
        ti < t.len() && glob_match_inner(p, t, pi + 1, ti + 1)
    } else {
        ti < t.len() && p[pi] == t[ti] && glob_match_inner(p, t, pi + 1, ti + 1)
    }
}

/// Filter filenames matching a glob pattern.
pub fn glob_filter<'a>(names: &[&'a str], pattern: &str) -> Vec<&'a str> {
        todo!()
}

// ============================================
// Topic 5: Redirect Parsing
// Learn: I/O redirect operators: >, >>, <, 2>
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub enum Redirect {
    StdoutWrite(String),   // >
    StdoutAppend(String),  // >>
    StdinRead(String),     // <
    StderrWrite(String),   // 2>
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandWithRedirects {
    pub command: Command,
    pub redirects: Vec<Redirect>,
}

impl CommandWithRedirects {
    pub fn parse(input: &str) -> Option<Self> {
        todo!()
    }

    pub fn has_redirects(&self) -> bool { todo!() }
}

// ============================================
// Topic 6: Shell Builtins
// Learn: Built-in command execution, history
// ============================================

pub struct ShellState {
    pub env: Environment,
    pub history: Vec<String>,
    pub cwd: String,
}

impl ShellState {
    pub fn new() -> Self {
        todo!()
    }

    pub fn execute_builtin(&mut self, input: &str) -> Option<String> {
        todo!()
    }

    pub fn history_count(&self) -> usize { todo!() }
}

impl Default for ShellState {
    fn default() -> Self { Self::new() }
}
