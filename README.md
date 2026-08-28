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

**One binary. Any `.md` file becomes an app you can click to run.**

No server, no config, no new language. If you can write a README, you can build a tool.

---

### Normal markdown vs mterm

| Normal markdown (`README.md`) | mterm (`runbook.md`) |
| :--- | :--- |
| Shows a command: <br>`````sh<br>git status<br>`````<br>You copy-paste it into a terminal | Shows a **button**:<br>`````markdown<br>> [!BUTTON] Check status<br>```sh :run<br>git status<br>```<br>`````<br>Click `Run ▶` and the output appears right in the document |
| Static text. Steps get out of date, people skip them | Same text, but each step can actually run. The doc *is* the tool |
| Opens anywhere (GitHub, VS Code) | Also opens anywhere — without `mterm` it looks like normal markdown. With `mterm` the blocks become clickable |

**In short:** your existing markdown keeps working. Add `:run` to a code block and `> [!BUTTON]` above it — that's the only new syntax.

---

### Where did this come from?

Most teams already have a `README.md` with setup steps, a `runbook.md` for on-call, or a Notion page with commands to copy-paste. Writing a real TUI or internal web app for those steps takes hours. This started as:

> "What if a terminal UI was as easy to write as markdown, auto-themed to your system, and any `.md` file could have a `Run` button?"

`mterm` is the smallest answer: a markdown viewer that can execute allowlisted shell blocks locally, nothing more. The file stays plain text and git-friendly.

---

### When to use it

**Great for:**
* **Runbooks** — `incident.md` where each step has a `[Run]` button (`check logs`, `restart service`) instead of copy-paste
* **Onboarding / READMEs** — `setup.md` where `Check Go installed`, `Run migrations` are verifiable buttons
* **Dev dashboards** — `deploy.md` or `status.md` for side projects / homelabs (`git status`, `tail logs`, `deploy`)
* **Homelab / personal tooling** — you live in the terminal and want a quick UI without building one

**Not for:**
* Pixel-perfect apps, spreadsheets, or anything needing complex layout (use a real TUI/web app)
* Non-technical workflows that need no-code integrations (this runs local shell commands, not Zapier)

---

### How it works

1. You write markdown as usual, add `:run` to any code fence you want runnable
2. `mterm` renders it with a `Run ▶` button — nothing runs until you click
3. Output streams back into the doc, cached in `~/.cache/mterm/`, never written into the `.md` file (so `git diff` stays clean)

Example:

````markdown
> [!BUTTON] Check Git Status

```sh :run id=check
git status
```
````

Plain `cat`, `nvim`, or GitHub preview still shows readable markdown.

---

### Install — double-click

**1. Download from [Releases](https://github.com/masonleetompkins/mterm/releases/latest):**

| Your OS | File | Double-click |
| :--- | :--- | :--- |
| **Linux** | [`install.sh`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.sh) | File manager → right-click `install.sh` → `Run` / `Run in Terminal` |
| **macOS** | [`install.command`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.command) | Double-click `install.command` → Terminal opens and installs |
| **Windows** | [`install.bat`](https://github.com/masonleetompkins/mterm/releases/latest/download/install.bat) | Double-click `install.bat` → `cmd` window installs |

Installs to `~/.local/bin/mterm` (Linux/macOS) or `%USERPROFILE%\.local\bin\mterm.exe` (Windows). No admin.

> First run may show: macOS `“unidentified developer”` → System Settings → Privacy → Open Anyway. Windows `SmartScreen` → More info → Run anyway. (Binaries unsigned in v0.1.)

**Prefer terminal?**

```bash
# Linux / macOS
sh install.sh --dry-run   # preview
sh install.sh             # or: sh install.command

# Windows
install.bat --dry-run
```

**Verify:**

```bash
mterm --version
mterm app examples/runbook.md   # opens viewer, j/k navigate, Enter runs, q quits
# if "command not found", add to PATH:
# Linux/macOS: export PATH="$HOME/.local/bin:$PATH"  (add to ~/.bashrc)
# Windows: setx PATH "%PATH%;%USERPROFILE%\.local\bin"
```

**Alternatives (no installer):**
```bash
cargo install --git https://github.com/masonleetompkins/mterm
# or download binary directly: mterm-linux-x64, mterm-macos-x64, mterm-windows-x64.exe, etc.
```

---

### Usage

```bash
mterm app ./runbook.md                  # interactive viewer (terminal UI)
mterm run ./runbook.md --block check    # run one block to stdout (for CI/ssh)
mterm run ./runbook.md --yes            # run all blocks, auto-allow
mterm check ./runbook.md                # validate blocks + permissions
mterm banner                            # show banner
```

---

### File format

Only 2 additions to normal markdown:

* **Runnable block:** ````sh :run`` (any language: `sh`, `bash`, `py`, `ps1`)
  * Optional `id=name` for `mterm run --block name`
* **Button:** `> [!BUTTON] Label` on the line directly above a runnable block
* **Permissions (optional frontmatter):**
```yaml
---
mterm:
  permissions:
    shell: ["git status", "./deploy.sh *"]  # glob allowlist
    allow_unspecified: prompt               # prompt | deny
---
```
Without frontmatter, every run prompts once for approval.

---

### Status

v0.1 — local, file-based, offline. No server. Text stays plain markdown; execution is opt-in. Web/Tauri renderers planned, terminal fallback works today.

### License

MIT
