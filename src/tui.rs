use crate::parser::{ParsedDoc, RunnableBlock};
use crate::runtime::{PermissionDecision, check_permission, execute_block};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{io, time::Duration};

struct BlockState {
    block: RunnableBlock,
    last_result: Option<String>,
    last_exit: Option<i32>,
    running: bool,
}

pub fn run_tui(doc: ParsedDoc, file_path: &str) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut blocks: Vec<BlockState> = doc
        .blocks
        .into_iter()
        .map(|b| BlockState {
            block: b,
            last_result: None,
            last_exit: None,
            running: false,
        })
        .collect();

    let mut selected = 0usize;
    let mut log = String::from("j/k navigate • Enter run • q quit");
    let mut prompt: Option<(String, usize)> = None; // (cmd, idx)

    loop {
        terminal.draw(|f| draw(f, &blocks, selected, &log, file_path, prompt.as_ref()))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(k) = event::read()? {
                if let Some((cmd, idx)) = prompt.clone() {
                    match k.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            let code = blocks[idx].block.code.clone();
                            let lang = blocks[idx].block.lang.clone();
                            blocks[idx].running = true;
                            terminal.draw(|f| draw(f, &blocks, selected, &log, file_path, None))?;
                            let res = execute_block(&lang, &code);
                            blocks[idx].running = false;
                            match res {
                                Ok(r) => {
                                    blocks[idx].last_exit = Some(r.exit_code);
                                    let out = if r.stderr.is_empty() {
                                        r.stdout
                                    } else {
                                        format!("{}\n[stderr]\n{}", r.stdout, r.stderr)
                                    };
                                    blocks[idx].last_result = Some(out);
                                    log = format!("ran '{}' exit {}", cmd, r.exit_code);
                                }
                                Err(e) => {
                                    blocks[idx].last_result = Some(format!("error: {e}"));
                                    log = format!("error: {e}");
                                }
                            }
                            prompt = None;
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            log = "denied".to_string();
                            prompt = None;
                        }
                        _ => {}
                    }
                    continue;
                }

                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        if !blocks.is_empty() {
                            selected = (selected + 1) % blocks.len();
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if !blocks.is_empty() {
                            selected = (selected + blocks.len() - 1) % blocks.len();
                        }
                    }
                    KeyCode::Enter => {
                        if blocks.is_empty() {
                            continue;
                        }
                        let perms = doc.frontmatter.permissions.clone().unwrap_or_default();
                        let code = blocks[selected].block.code.clone();
                        let lang = blocks[selected].block.lang.clone();
                        match check_permission(&code, &perms) {
                            crate::runtime::PermissionDecision::Allowed => {
                                blocks[selected].running = true;
                                terminal.draw(|f| draw(f, &blocks, selected, &log, file_path, None))?;
                                let res = execute_block(&lang, &code);
                                blocks[selected].running = false;
                                match res {
                                    Ok(r) => {
                                        blocks[selected].last_exit = Some(r.exit_code);
                                        let out = if r.stderr.is_empty() { r.stdout.clone() } else { format!("{}\n[stderr]\n{}", r.stdout, r.stderr) };
                                        blocks[selected].last_result = Some(out);
                                        log = format!("exit {} • ran {}", r.exit_code, blocks[selected].block.id);
                                    }
                                    Err(e) => {
                                        blocks[selected].last_result = Some(format!("error: {e}"));
                                        log = format!("error: {e}");
                                    }
                                }
                            }
                            PermissionDecision::Prompt(cmd) => {
                                prompt = Some((cmd, selected));
                            }
                            PermissionDecision::Denied => {
                                log = "blocked by permissions (allow_unspecified: deny)".to_string();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw(
    f: &mut Frame,
    blocks: &[BlockState],
    selected: usize,
    log: &str,
    file_path: &str,
    prompt: Option<&(String, usize)>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
        .split(f.area());

    // header
    let banner = Paragraph::new(Line::from(vec![
        Span::styled(" mterm ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!("  {}  •  {} runnable blocks", file_path, blocks.len())),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" executable markdown "));
    f.render_widget(banner, chunks[0]);

    if blocks.is_empty() {
        let p = Paragraph::new("No runnable blocks found.\nAdd ```sh :run  to make a block runnable.\n\nExample:\n> [!BUTTON] Run\n```sh :run\necho hi\n```")
            .block(Block::default().borders(Borders::ALL).title(" blocks "))
            .wrap(Wrap { trim: true });
        f.render_widget(p, chunks[1]);
    } else {
        let sub = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
            .split(chunks[1]);

        // list
        let items: Vec<ListItem> = blocks
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let label = b.block.button_label.clone().unwrap_or_else(|| "Run".to_string());
                let status = if b.running {
                    " ● running"
                } else if let Some(code) = b.last_exit {
                    if code == 0 { " ✓" } else { " ✗" }
                } else {
                    ""
                };
                let prefix = if i == selected { "▶ " } else { "  " };
                let line = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::Cyan)),
                    Span::styled(format!("{} ", b.block.id), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(format!("[{}] ", b.block.lang), Style::default().fg(Color::DarkGray)),
                    Span::raw(label),
                    Span::styled(status, Style::default().fg(if b.last_exit == Some(0) { Color::Green } else { Color::Red })),
                ]);
                let subline = Line::from(vec![Span::styled(
                    format!("  {}", b.block.code.lines().next().unwrap_or("").trim()),
                    Style::default().fg(Color::DarkGray),
                )]);
                ListItem::new(vec![line, subline])
            })
            .collect();

        let mut state = ListState::default();
        state.select(Some(selected));
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" blocks (j/k • Enter) "))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));
        f.render_stateful_widget(list, sub[0], &mut state);

        // detail pane
        let detail_rect = sub[1];
        if let Some(b) = blocks.get(selected) {
            let out = b.last_result.clone().unwrap_or_else(|| "(not run yet — press Enter)".to_string());
            let code = format!("```{} :run id={}\n{}\n```", b.block.lang, b.block.id, b.block.code);
            let body = format!("{code}\n\n── output ──\n{out}");
            let p = Paragraph::new(body)
                .block(Block::default().borders(Borders::ALL).title(format!(" {} ", b.block.id)))
                .wrap(Wrap { trim: false });
            f.render_widget(p, detail_rect);
        }
    }

    // footer/log + prompt overlay
    if let Some((cmd, _)) = prompt {
        let msg = format!(" Allow '{}' to run?  [y]es / [n]o ", cmd);
        let p = Paragraph::new(msg)
            .style(Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title(" permission "));
        f.render_widget(p, chunks[2]);
    } else {
        let p = Paragraph::new(log.to_string())
            .block(Block::default().borders(Borders::ALL).title(" log "))
            .wrap(Wrap { trim: true });
        f.render_widget(p, chunks[2]);
    }
}
