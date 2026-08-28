mod parser;
mod runtime;
mod tui;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use parser::parse_file;
use runtime::{check_permission, execute_block};
use std::path::PathBuf;

const BANNER: &str = include_str!("../assets/banner.txt");

#[derive(Parser)]
#[command(name = "mterm", version, about = "executable markdown — readme that runs", long_about = BANNER)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// File to open (shorthand for `mterm app <file>`)
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive viewer (terminal UI, auto theme)
    App {
        file: PathBuf,
    },
    /// Run a block in plain terminal mode
    Run {
        file: PathBuf,
        #[arg(long)]
        block: Option<String>,
        #[arg(long, default_value = "false")]
        plain: bool,
        #[arg(long, default_value = "false")]
        yes: bool,
    },
    /// Validate markdown file
    Check {
        file: PathBuf,
    },
    /// Print banner
    Banner,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::App { file }) => app_cmd(file),
        Some(Commands::Run { file, block, plain: _, yes }) => run_cmd(file, block, yes),
        Some(Commands::Check { file }) => check_cmd(file),
        Some(Commands::Banner) => {
            println!("{}", BANNER);
            Ok(())
        }
        None => {
            if let Some(file) = cli.file {
                app_cmd(file)
            } else {
                println!("{}", BANNER);
                println!("Usage: mterm app <file.md>  |  mterm run <file.md> --block <id>  |  mterm check <file.md>");
                Ok(())
            }
        }
    }
}

fn app_cmd(file: PathBuf) -> Result<()> {
    if !file.exists() {
        bail!("file not found: {}", file.display());
    }
    let doc = parse_file(&file)?;
    // If no runnable blocks, still show viewer with message
    tui::run_tui(doc, &file.display().to_string()).map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

fn run_cmd(file: PathBuf, block_id: Option<String>, yes: bool) -> Result<()> {
    let doc = parse_file(&file).with_context(|| format!("parse {}", file.display()))?;
    if doc.blocks.is_empty() {
        bail!("no runnable blocks (add ```sh :run) in {}", file.display());
    }

    let targets: Vec<_> = if let Some(id) = block_id {
        let b = doc.blocks.iter().find(|b| b.id == id).with_context(|| format!("block '{}' not found. available: {}", id, doc.blocks.iter().map(|b| b.id.as_str()).collect::<Vec<_>>().join(", ")))?;
        vec![b.clone()]
    } else {
        doc.blocks.clone()
    };

    let perms = doc.frontmatter.permissions.clone().unwrap_or_default();
    for b in targets {
        match check_permission(&b.code, &perms) {
            runtime::PermissionDecision::Allowed => {},
            runtime::PermissionDecision::Prompt(cmd) if yes => {
                eprintln!("[mterm] auto-allowed '{}' (--yes)", cmd);
            }
            runtime::PermissionDecision::Prompt(cmd) => {
                bail!("blocked '{}' — not in permissions.shell. Add to frontmatter or use --yes to allow. cmd: {}", b.id, cmd);
            }
            runtime::PermissionDecision::Denied => bail!("blocked '{}' by permissions (allow_unspecified: deny)", b.id),
        }
        println!("$ {} [{}:{}]", b.code.lines().next().unwrap_or("").trim(), b.id, b.lang);
        let res = execute_block(&b.lang, &b.code)?;
        if !res.stdout.is_empty() {
            print!("{}", res.stdout);
            if !res.stdout.ends_with('\n') { println!(); }
        }
        if !res.stderr.is_empty() {
            eprint!("{}", res.stderr);
        }
        if res.exit_code != 0 {
            bail!("block '{}' exited with {}", b.id, res.exit_code);
        }
    }
    Ok(())
}

fn check_cmd(file: PathBuf) -> Result<()> {
    let doc = parse_file(&file)?;
    println!("{}: {} runnable block(s)", file.display(), doc.blocks.len());
    for b in &doc.blocks {
        let first = b.code.lines().next().unwrap_or("").trim();
        println!("  - {} [{}] line {} button={:?} :: {}", b.id, b.lang, b.line_start, b.button_label, first);
    }
    if let Some(perms) = doc.frontmatter.permissions {
        println!("permissions: shell={:?} allow_unspecified={:?}", perms.shell, perms.allow_unspecified);
    } else {
        println!("permissions: (none, will prompt)");
    }
    if doc.blocks.is_empty() {
        println!("hint: add ```sh :run to make a block runnable");
    }
    Ok(())
}
