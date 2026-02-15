use mini_shell::*;

// ===== Topic 1: Command Parsing =====

#[test]
fn test_parse_command() {
    let cmd = Command::parse("ls -la /tmp").unwrap();
    assert_eq!(cmd.program, "ls");
    assert_eq!(cmd.args, vec!["-la", "/tmp"]);
    assert_eq!(cmd.arg_count(), 2);
}

#[test]
fn test_parse_empty() {
    assert!(Command::parse("").is_none());
    assert!(Command::parse("   ").is_none());
}

#[test]
fn test_parse_quoted() {
    let tokens = parse_quoted(r#"echo "hello world" foo"#);
    assert_eq!(tokens, vec!["echo", "hello world", "foo"]);
}

#[test]
fn test_command_to_string() {
    let cmd = Command::parse("git commit -m msg").unwrap();
    assert_eq!(cmd.to_string_repr(), "git commit -m msg");
}

// ===== Topic 2: Pipeline =====

#[test]
fn test_parse_pipeline() {
    let p = Pipeline::parse("cat file | grep foo | wc -l").unwrap();
    assert_eq!(p.len(), 3);
    assert!(p.is_pipeline());
    assert_eq!(p.commands[0].program, "cat");
    assert_eq!(p.commands[1].program, "grep");
    assert_eq!(p.commands[2].program, "wc");
}

#[test]
fn test_single_command_pipeline() {
    let p = Pipeline::parse("ls -la").unwrap();
    assert_eq!(p.len(), 1);
    assert!(!p.is_pipeline());
}

// ===== Topic 3: Environment Variables =====

#[test]
fn test_env_set_get() {
    let mut env = Environment::new();
    env.set("HOME", "/home/user");
    assert_eq!(env.get("HOME"), Some("/home/user"));
    assert_eq!(env.get("MISSING"), None);
}

#[test]
fn test_env_expand() {
    let mut env = Environment::new();
    env.set("USER", "alice");
    env.set("HOME", "/home/alice");
    assert_eq!(env.expand("Hello $USER!"), "Hello alice!");
    assert_eq!(env.expand("${HOME}/bin"), "/home/alice/bin");
}

#[test]
fn test_env_expand_missing() {
    let env = Environment::new();
    assert_eq!(env.expand("$MISSING"), "");
}

#[test]
fn test_env_remove() {
    let mut env = Environment::new();
    env.set("KEY", "val");
    assert!(env.remove("KEY"));
    assert!(!env.remove("KEY"));
}

// ===== Topic 4: Glob Matching =====

#[test]
fn test_glob_star() {
    assert!(glob_match("*.rs", "main.rs"));
    assert!(!glob_match("*.rs", "main.txt"));
    assert!(glob_match("*", "anything"));
}

#[test]
fn test_glob_question() {
    assert!(glob_match("file?.txt", "file1.txt"));
    assert!(!glob_match("file?.txt", "file12.txt"));
}

#[test]
fn test_glob_filter() {
    let files = vec!["main.rs", "lib.rs", "README.md", "test.rs"];
    assert_eq!(glob_filter(&files, "*.rs"), vec!["main.rs", "lib.rs", "test.rs"]);
}

// ===== Topic 5: Redirect Parsing =====

#[test]
fn test_redirect_stdout() {
    let cmd = CommandWithRedirects::parse("echo hello > output.txt").unwrap();
    assert_eq!(cmd.command.program, "echo");
    assert!(cmd.has_redirects());
    assert_eq!(cmd.redirects[0], Redirect::StdoutWrite("output.txt".into()));
}

#[test]
fn test_redirect_append() {
    let cmd = CommandWithRedirects::parse("echo hello >> log.txt").unwrap();
    assert_eq!(cmd.redirects[0], Redirect::StdoutAppend("log.txt".into()));
}

#[test]
fn test_redirect_stdin() {
    let cmd = CommandWithRedirects::parse("sort < input.txt").unwrap();
    assert_eq!(cmd.redirects[0], Redirect::StdinRead("input.txt".into()));
}

// ===== Topic 6: Shell Builtins =====

#[test]
fn test_builtin_echo() {
    let mut shell = ShellState::new();
    assert_eq!(shell.execute_builtin("echo hello world"), Some("hello world".into()));
}

#[test]
fn test_builtin_pwd_cd() {
    let mut shell = ShellState::new();
    assert!(shell.execute_builtin("pwd").unwrap().contains("/home"));
    shell.execute_builtin("cd /tmp");
    assert_eq!(shell.execute_builtin("pwd"), Some("/tmp".into()));
}

#[test]
fn test_builtin_export() {
    let mut shell = ShellState::new();
    shell.execute_builtin("export KEY=value");
    assert_eq!(shell.env.get("KEY"), Some("value"));
}

#[test]
fn test_builtin_history() {
    let mut shell = ShellState::new();
    shell.execute_builtin("echo a");
    shell.execute_builtin("echo b");
    let hist = shell.execute_builtin("history").unwrap();
    assert!(hist.contains("echo a"));
    assert!(hist.contains("echo b"));
    assert_eq!(shell.history_count(), 3);
}

#[test]
fn test_not_builtin() {
    let mut shell = ShellState::new();
    assert_eq!(shell.execute_builtin("git status"), None);
}
