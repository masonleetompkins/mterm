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

### install — double-click

**1. Download from [Releases](https://github.com/masonleetompkins/mterm/releases/latest):**

| Your OS | Download | Double-click |
| :--- | :--- | :--- |
| **Linux** | [`install.sh`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.sh) | Open file manager -> right-click `install.sh` -> `Run` or double-click `install.sh` -> `Run in Terminal` |
| **macOS** | [`install.command`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.command) | Double-click `install.command` -> Terminal opens and installs |
| **Windows** | [`install.bat`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.bat) | Double-click `install.bat` -> `cmd` window opens and installs |

No terminal command needed. The file *is* the installer — it detects OS/arch and downloads the right binary to `~/.local/bin` (Linux/macOS) or `%USERPROFILE%\.local\bin` (Windows). No admin.

> First launch may prompt: macOS `“unidentified developer”` -> System Settings -> Privacy -> Open Anyway. Windows `SmartScreen` -> More info -> Run anyway. (Binaries are unsigned in v0.1).

**Prefer terminal?**

```bash
# Linux / macOS
sh install.sh --dry-run    # preview
sh install.sh              # or: sh install.command
# Windows PowerShell
install.bat --dry-run
```

**Verify after install:**
```bash
mterm --version
mterm app examples/runbook.md
# add to PATH if prompted:
# Linux/macOS: export PATH="$HOME/.local/bin:$PATH"
# Windows: setx PATH "%PATH%;%USERPROFILE%\.local\bin"
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
