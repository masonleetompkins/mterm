use crate::parser::{Permissions, is_allowed};
use anyhow::Result;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct RunResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn check_permission(code: &str, perms: &Permissions) -> PermissionDecision {
    // code may be multiline; check first non-empty line as command
    let first = code.lines().map(|l| l.trim()).find(|l| !l.is_empty()).unwrap_or("");
    if is_allowed(first, perms) || is_allowed(code.trim(), perms) {
        return PermissionDecision::Allowed;
    }
    match perms.allow_unspecified.as_deref() {
        Some("deny") => PermissionDecision::Denied,
        _ => PermissionDecision::Prompt(first.to_string()),
    }
}

#[derive(Debug, Clone)]
pub enum PermissionDecision {
    Allowed,
    Prompt(String),
    Denied,
}

pub fn execute_block(lang: &str, code: &str) -> Result<RunResult> {
    // Cross-platform dispatch: on Windows, sh/bash/zsh/pwsh map to powershell
    let (prog, arg) = match lang {
        "py" | "python" => {
            #[cfg(windows)]
            { ("python", "-c") }
            #[cfg(not(windows))]
            { ("python3", "-c") }
        },
        "ps1" | "powershell" | "pwsh" => ("powershell", "-Command"),
        "sh" | "bash" | "zsh" | "" => {
            #[cfg(windows)]
            { ("powershell", "-Command") }
            #[cfg(not(windows))]
            { ("sh", "-c") }
        },
        other => (other, "-c"),
    };
    // For powershell on Windows, -Command expects single string; it handles newlines
    #[cfg(windows)]
    let is_powershell = prog == "powershell" || prog == "pwsh";
    #[cfg(not(windows))]
    let is_powershell = false;

    // For shell, run code via prog -c/-Command
    let output = if prog == "python3" || prog == "python" {
        Command::new(prog)
            .arg(arg)
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?
    } else if is_powershell {
        // powershell -Command "<code>"
        Command::new(prog)
            .arg(arg)
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?
    } else {
        Command::new(prog)
            .arg(arg)
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?
    };

    Ok(RunResult {
        exit_code: output.status.code().unwrap_or(0),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}
