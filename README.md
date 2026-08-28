# mterm

```
  ███╗   ███╗████████╗███████╗██████╗ ███╗   ███╗
  ████╗ ████║╚══██╔══╝██╔════╝██╔══██╗████╗ ████║
  ██╔████╔██║   ██║   █████╗  ██████╔╝██╔████╔██║
  ██║╚██╔╝██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║
  ██║ ╚═╝ ██║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║
  ╚═╝     ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝
   ── executable markdown · readme that runs ──
```

**markdown viewer that can run commands. one binary, one step.**

`mterm` turns any `.md` file into a runnable document. No server, no config.

```markdown
> [!BUTTON] Check Status

```sh :run
git status
```
```

Renders as markdown with a `Run ▶` button. Nothing executes until you click it.

### install — one file, one click (macOS / Linux / Windows)

**Best — polyglot installer (no Rust needed):**

```bash
# macOS / Linux
sh install.md              # detects OS/arch -> ~/.local/bin/mterm
sh install.md --dry-run    # preview

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -File install.ps1.md
powershell -ExecutionPolicy Bypass -File install.ps1.md --dry-run
```

The installer *is* an `mterm` file — open `install.md` / `install.ps1.md` in `mterm` and click `[Install mterm now]`, or run with `sh`/`powershell`. No Rust needed.

**Alternatives:**
```bash
cargo install --git https://github.com/<org>/mterm
# or download binary directly from Releases:
# mterm-linux-x64, mterm-linux-arm64, mterm-macos-x64, mterm-macos-arm64, mterm-windows-x64.exe
```

### usage

```bash
mterm app ./runbook.md    # interactive viewer (terminal ui, auto fallback)
mterm run ./runbook.md --block check   # run single block to stdout
mterm check ./runbook.md  # validate frontmatter + runnable blocks
```

Plain `cat`/`nvim` still shows readable markdown. Output and state are cached in `~/.cache/mterm/` and never written back to the file.

### file format

* Runnable block: ````sh :run` (any language tag)
* Button binding: `> [!BUTTON] Label` directly above a runnable block
* Checkbox: `- [ ] task` (ephemeral, git-clean)
* Permissions (frontmatter):
```yaml
---
mterm:
  permissions:
    shell: ["git status", "./deploy.sh *"]
    allow_unspecified: prompt
---
```

### status

v0.1 — local, file-based, offline. Web and Tauri renderers planned, terminal fallback works today.

### license

MIT
