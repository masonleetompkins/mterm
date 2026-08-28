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

### install

**1. Pick your file — download from [Releases](https://github.com/masonleetompkins/mterm/releases/latest):**

| Your OS | Download this file | What it is |
| :--- | :--- | :--- |
| **Linux** | [`install.md`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.md) | shell + markdown polyglot |
| **macOS** | [`install.md`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.md) | same file, detects macOS |
| **Windows** | [`install.ps1.md`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.ps1.md) | PowerShell + markdown polyglot |

> The installer *is* an `mterm` file. Double-click it after install, or open it with `mterm`, and it shows a `[Install mterm now]` button.

**2. Run it from terminal/shell:**

```bash
# Linux — any distro
sh install.md                    # installs to ~/.local/bin/mterm
sh install.md --dry-run          # preview without installing

# macOS — Intel or Apple Silicon
sh install.md
sh install.md --dry-run

# Windows — PowerShell (no admin)
powershell -ExecutionPolicy Bypass -File install.ps1.md
powershell -ExecutionPolicy Bypass -File install.ps1.md --dry-run
```

Add to PATH if prompted:
```bash
# Linux/macOS
export PATH="$HOME/.local/bin:$PATH"   # add to ~/.bashrc or ~/.zshrc

# Windows PowerShell (current session)
$env:PATH += ";$env:USERPROFILE\.local\bin"
# Windows PowerShell (permanent)
setx PATH "$env:PATH;$env:USERPROFILE\.local\bin"
```

**Alternatives (no installer):**
```bash
cargo install --git https://github.com/masonleetompkins/mterm
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
