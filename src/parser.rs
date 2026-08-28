use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MtermFrontmatter {
    pub mterm: Option<MtermConfig>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MtermConfig {
    pub version: Option<u8>,
    pub permissions: Option<Permissions>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Permissions {
    pub shell: Option<Vec<String>>,
    #[serde(default)]
    pub allow_unspecified: Option<String>, // prompt | deny
}

#[derive(Debug, Clone)]
pub struct RunnableBlock {
    pub id: String,
    pub lang: String,
    pub code: String,
    pub line_start: usize,
    pub button_label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedDoc {
    pub raw: String,
    pub frontmatter: MtermConfig,
    pub blocks: Vec<RunnableBlock>,
}

pub fn parse_file(path: &Path) -> Result<ParsedDoc> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    parse_str(&raw)
}

pub fn parse_str(raw: &str) -> Result<ParsedDoc> {
    let (frontmatter, body) = extract_frontmatter(raw)?;

    let blocks = extract_blocks(&body)?;
    Ok(ParsedDoc {
        raw: raw.to_string(),
        frontmatter,
        blocks,
    })
}

fn extract_frontmatter(raw: &str) -> Result<(MtermConfig, String)> {
    // Support polyglot files: frontmatter may be after a shebang/shell preamble.
    // Look for first "---\n" block within first 4KB.
    let search_window = if raw.len() > 4096 { &raw[..4096] } else { raw };
    // find first frontmatter start: line starting with "---"
    let fm_start = search_window.find("\n---\n").or_else(|| {
        if raw.starts_with("---\n") { Some(0) } else { None }
    });
    if let Some(start) = fm_start {
        let fm_offset = if start == 0 { 0 } else { start + 1 }; // skip \n
        // fm content starts after "---\n"
        let after_start = fm_offset + 4;
        if let Some(end_rel) = raw[after_start..].find("\n---") {
            let fm_str = &raw[after_start..after_start + end_rel];
            // only treat as frontmatter if it parses and contains mterm:
            if fm_str.contains("mterm:") {
                if let Ok(fm) = serde_yaml::from_str::<MtermFrontmatter>(fm_str) {
                    if fm.mterm.is_some() {
                        let body_start = after_start + end_rel + 4;
                        let body = raw[body_start..].trim_start_matches(|c| c == '\n' || c == '\r').to_string();
                        return Ok((fm.mterm.unwrap_or_default(), body));
                    }
                }
            }
        }
    }
    // fallback: classic start-of-file check
    if raw.starts_with("---\n") || raw.starts_with("---\r\n") {
        if let Some(end) = raw[4..].find("\n---") {
            let fm_str = &raw[4..4 + end];
            let fm: MtermFrontmatter = serde_yaml::from_str(fm_str).unwrap_or_default();
            let body = raw[4 + end + 4..].to_string();
            let body = body.trim_start_matches('\n').trim_start_matches("\r\n").to_string();
            return Ok((fm.mterm.unwrap_or_default(), body));
        }
    }
    Ok((MtermConfig::default(), raw.to_string()))
}

fn extract_blocks(body: &str) -> Result<Vec<RunnableBlock>> {
    // Matches ```lang :run [id=xxx] \n code \n ```
    // lang is like sh, bash, py, zsh etc ; (?s) so . matches newline for code
    let re = Regex::new(r"(?m)^```([a-zA-Z0-9_-]*)\s*:run[ \t]*([^\n]*)\n(?P<code>(?s:.*?))\n```").unwrap();
    let button_re = Regex::new(r"(?m)^>\s*\[!(BUTTON|RUN)\]\s*(?P<label>.*)$").unwrap();

    let mut blocks = Vec::new();
    for (idx, cap) in re.captures_iter(body).enumerate() {
        let lang = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let lang = if lang.is_empty() { "sh".to_string() } else { lang };
        let meta = cap.get(2).map(|m| m.as_str()).unwrap_or("").trim().to_string();
        let code = cap.get(3).unwrap().as_str().to_string();

        let full_match = cap.get(0).unwrap();
        let start = full_match.start();
        let line_start = body[..start].matches('\n').count() + 1;

        // look back ~5 lines for button
        let before = &body[..start];
        let before_lines: Vec<&str> = before.lines().rev().take(5).collect();
        let mut button_label = None;
        for line in before_lines {
            if line.trim().is_empty() {
                continue;
            }
            if let Some(bcap) = button_re.captures(line) {
                let label = bcap.name("label").map(|m| m.as_str().trim().to_string()).unwrap_or_default();
                button_label = Some(if label.is_empty() { "Run".to_string() } else { label });
                break;
            } else {
                // if we hit non-button, non-empty line, stop looking
                if !line.trim().starts_with('>') {
                    break;
                }
            }
        }

        // parse id=xxx from meta
        let id = if let Some(id_cap) = Regex::new(r"id=([a-zA-Z0-9_-]+)").unwrap().captures(&meta) {
            id_cap.get(1).unwrap().as_str().to_string()
        } else {
            format!("block-{}", idx + 1)
        };

        blocks.push(RunnableBlock {
            id,
            lang,
            code,
            line_start,
            button_label,
        });
    }
    Ok(blocks)
}

pub fn is_allowed(cmd: &str, perms: &Permissions) -> bool {
    let Some(patterns) = &perms.shell else {
        return false;
    };
    let cmd = cmd.trim();
    for pat in patterns {
        if glob_match(pat, cmd) {
            return true;
        }
    }
    false
}

fn glob_match(pattern: &str, text: &str) -> bool {
    // very small glob: supports * only
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == text;
    }
    // split by *
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if i == 0 && !text.starts_with(*part) {
            return false;
        }
        if i == parts.len() - 1 && !part.is_empty() && !text.ends_with(*part) {
            return false;
        }
        if let Some(found) = text[pos..].find(*part) {
            pos += found + part.len();
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_runbook() {
        let s = r#"
> [!BUTTON] Check
```sh :run id=check
git status
```
```py :run
print("hi")
```
"#;
        let doc = parse_str(s).unwrap();
        assert_eq!(doc.blocks.len(), 2);
        assert_eq!(doc.blocks[0].id, "check");
        assert_eq!(doc.blocks[0].button_label, Some("Check".to_string()));
        assert_eq!(doc.blocks[1].lang, "py");
    }
}
