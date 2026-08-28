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

**You don't have `mterm` yet — so you run the installer with your system shell, not with `mterm`.** After install, `mterm` can open `*.md` files itself.

**1. Download the installer for your OS from [Releases](https://github.com/masonleetompkins/mterm/releases/latest):**

| Your OS | Click to download | File |
| :--- | :--- | :--- |
| **Linux** | [`install.md`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.md) | shell script disguised as markdown |
| **macOS** | [`install.md`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.md) | same file, auto-detects Intel/Apple Silicon |
| **Windows** | [`install.ps1.md`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.ps1.md) | PowerShell script disguised as markdown |

> If you open the downloaded file in a text editor or GitHub preview you'll just see markdown — that's expected. It's *also* an executable. Don't double-click it yet.

**2. Open a terminal/shell and run it:**

```bash
# Linux — any distro (terminal)
cd ~/Downloads              # or wherever your browser saved it
sh install.md               # installs to ~/.local/bin/mterm
sh install.md --dry-run     # preview without installing

# macOS — Intel or Apple Silicon (Terminal.app)
cd ~/Downloads
sh install.md
sh install.md --dry-run

# Windows — PowerShell (no admin, Start Menu -> PowerShell)
cd $HOME\Downloads
powershell -ExecutionPolicy Bypass -File install.ps1.md
powershell -ExecutionPolicy Bypass -File install.ps1.md --dry-run
```

**One-liner alternative (no download step):**
```bash
# Linux/macOS
curl -fsSL https://github.com/masonleetompkins/mterm/releases/latest/download/install.md | sh
# Windows
Invoke-WebRequest https://github.com/masonleetompkins/mterm/releases/latest/download/install.ps1.md -OutFile install.ps1.md; powershell -ExecutionPolicy Bypass -File install.ps1.md
```

**3. Add to PATH if prompted, then verify:**
```bash
# Linux/macOS
export PATH="$HOME/.local/bin:$PATH"   # add to ~/.bashrc or ~/.zshrc
mterm --version
mterm app examples/runbook.md   # now you can double-click .md files

# Windows PowerShell (current session)
$env:PATH += ";$env:USERPROFILE\.local\bin"
# Windows PowerShell (permanent)
setx PATH "$env:PATH;$env:USERPROFILE\.local\bin"
mterm --version
```

> After `mterm` is installed, the *same* `install.md` / `install.ps1.md` files become clickable markdown: `mterm app install.md` shows a `[Install mterm now]` button that runs the shell script for you.

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
