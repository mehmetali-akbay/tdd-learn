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
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() { return None; }
        Some(Command {
            program: parts[0].to_string(),
            args: parts[1..].iter().map(|s| s.to_string()).collect(),
        })
    }

    pub fn arg_count(&self) -> usize { self.args.len() }

    pub fn to_string_repr(&self) -> String {
        if self.args.is_empty() {
            self.program.clone()
        } else {
            format!("{} {}", self.program, self.args.join(" "))
        }
    }
}

/// Parse a command with quoted strings: echo "hello world" -> ["echo", "hello world"]
pub fn parse_quoted(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    for ch in input.chars() {
        if in_quotes {
            if ch == quote_char {
                in_quotes = false;
            } else {
                current.push(ch);
            }
        } else if ch == '"' || ch == '\'' {
            in_quotes = true;
            quote_char = ch;
        } else if ch.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() { tokens.push(current); }
    tokens
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
        let cmds: Result<Vec<_>, _> = input
            .split('|')
            .map(|s| Command::parse(s.trim()).ok_or(()))
            .collect();
        let commands = cmds.ok()?;
        if commands.is_empty() { return None; }
        Some(Pipeline { commands })
    }

    pub fn len(&self) -> usize { self.commands.len() }
    pub fn is_empty(&self) -> bool { self.commands.is_empty() }

    pub fn is_pipeline(&self) -> bool { self.commands.len() > 1 }
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
        Environment { vars: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(|s| s.as_str())
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.vars.remove(key).is_some()
    }

    /// Expand $VAR and ${VAR} in a string.
    pub fn expand(&self, input: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '$' && i + 1 < chars.len() {
                if chars[i+1] == '{' {
                    // ${VAR} syntax
                    if let Some(end) = chars[i+2..].iter().position(|&c| c == '}') {
                        let name: String = chars[i+2..i+2+end].iter().collect();
                        result.push_str(self.vars.get(&name).map(|s| s.as_str()).unwrap_or(""));
                        i = i + 3 + end;
                    } else {
                        result.push(chars[i]);
                        i += 1;
                    }
                } else {
                    // $VAR syntax
                    let start = i + 1;
                    let mut end = start;
                    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                        end += 1;
                    }
                    let name: String = chars[start..end].iter().collect();
                    result.push_str(self.vars.get(&name).map(|s| s.as_str()).unwrap_or(""));
                    i = end;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
    }

    pub fn len(&self) -> usize { self.vars.len() }
    pub fn is_empty(&self) -> bool { self.vars.is_empty() }
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
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    glob_match_inner(&p, &t, 0, 0)
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
    names.iter().filter(|n| glob_match(pattern, n)).copied().collect()
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
        let mut redirects = Vec::new();
        let mut cmd_parts = Vec::new();
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i] {
                ">>" => {
                    i += 1;
                    if i < tokens.len() { redirects.push(Redirect::StdoutAppend(tokens[i].to_string())); }
                    i += 1;
                }
                ">" => {
                    i += 1;
                    if i < tokens.len() { redirects.push(Redirect::StdoutWrite(tokens[i].to_string())); }
                    i += 1;
                }
                "<" => {
                    i += 1;
                    if i < tokens.len() { redirects.push(Redirect::StdinRead(tokens[i].to_string())); }
                    i += 1;
                }
                "2>" => {
                    i += 1;
                    if i < tokens.len() { redirects.push(Redirect::StderrWrite(tokens[i].to_string())); }
                    i += 1;
                }
                _ => {
                    cmd_parts.push(tokens[i]);
                    i += 1;
                }
            }
        }
        let command = Command::parse(&cmd_parts.join(" "))?;
        Some(CommandWithRedirects { command, redirects })
    }

    pub fn has_redirects(&self) -> bool { !self.redirects.is_empty() }
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
        ShellState {
            env: Environment::new(),
            history: Vec::new(),
            cwd: "/home/user".to_string(),
        }
    }

    pub fn execute_builtin(&mut self, input: &str) -> Option<String> {
        self.history.push(input.to_string());
        let cmd = Command::parse(input)?;
        match cmd.program.as_str() {
            "echo" => Some(cmd.args.join(" ")),
            "pwd" => Some(self.cwd.clone()),
            "cd" => {
                if let Some(dir) = cmd.args.first() {
                    self.cwd = dir.clone();
                }
                Some(String::new())
            }
            "export" => {
                for arg in &cmd.args {
                    if let Some((k, v)) = arg.split_once('=') {
                        self.env.set(k, v);
                    }
                }
                Some(String::new())
            }
            "history" => Some(self.history.iter().enumerate()
                .map(|(i, h)| format!("  {} {}", i + 1, h))
                .collect::<Vec<_>>()
                .join("\n")),
            _ => None, // not a builtin
        }
    }

    pub fn history_count(&self) -> usize { self.history.len() }
}

impl Default for ShellState {
    fn default() -> Self { Self::new() }
}
