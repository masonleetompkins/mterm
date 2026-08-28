#!/bin/sh
# : <<'MTERM_MD'
# mterm installer — polyglot markdown + shell
# This file is BOTH a markdown document and an executable installer.
# - Open in mterm: renders as a doc with a [Run] button
# - Run as shell: `sh examples/install.md` or double-click (after chmod +x) installs mterm
# : <<'MTERM_MD'

# --- shell installer starts here ---
set -e
REPO="masonleetompkins/mterm"  # <-- change to your github org/repo
BIN_NAME="mterm"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

DRY_RUN=0
if [ "$1" = "--dry-run" ] || [ "$1" = "-n" ]; then DRY_RUN=1; fi

echo "  ███╗   ███╗████████╗███████╗██████╗ ███╗   ███╗"
echo "  ████╗ ████║╚══██╔══╝██╔════╝██╔══██╗████╗ ████║"
echo "  ██╔████╔██║   ██║   █████╗  ██████╔╝██╔████╔██║"
echo "  ██║╚██╔╝██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║"
echo "  ██║ ╚═╝ ██║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║"
echo "  ╚═╝     ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝"
echo "   ── installer · checks system and installs mterm ──"
echo ""

# detect OS/arch
OS="$(uname -s 2>/dev/null || echo unknown)"
ARCH="$(uname -m 2>/dev/null || echo unknown)"
echo "[detect] OS=$OS ARCH=$ARCH"

TARGET=""
case "$OS" in
  Linux) case "$ARCH" in x86_64|amd64) TARGET="mterm-linux-x64";; aarch64|arm64) TARGET="mterm-linux-arm64";; *) echo "unsupported arch: $ARCH" >&2; exit 1;; esac;;
  Darwin) case "$ARCH" in x86_64) TARGET="mterm-macos-x64";; arm64|aarch64) TARGET="mterm-macos-arm64";; *) echo "unsupported arch: $ARCH" >&2; exit 1;; esac;;
  *) echo "unsupported OS: $OS (try cargo install)" >&2; exit 1;;
esac

echo "[target] $TARGET"

# check existing
if command -v mterm >/dev/null 2>&1; then
  echo "[found] $(command -v mterm) -> $(mterm --version 2>/dev/null || echo unknown)"
  printf "reinstall? [y/N] "; read ans; case "$ans" in y|Y) ;; *) echo "abort"; exit 0;; esac
fi

URL="https://github.com/$REPO/releases/latest/download/$TARGET"
echo "[download] $URL"
echo "[install]  -> $INSTALL_DIR/$BIN_NAME"

if [ "$DRY_RUN" = "1" ]; then
  echo "[dry-run] would curl -fsSL $URL -o $INSTALL_DIR/$BIN_NAME && chmod +x"
  exit 0
fi

mkdir -p "$INSTALL_DIR"
if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$URL" -o "$INSTALL_DIR/$BIN_NAME"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$INSTALL_DIR/$BIN_NAME" "$URL"
else
  echo "need curl or wget" >&2; exit 1
fi
chmod +x "$INSTALL_DIR/$BIN_NAME"
echo "[done] installed to $INSTALL_DIR/$BIN_NAME"

# PATH hint
case ":$PATH:" in *":$INSTALL_DIR:"*) ;; *) echo "[hint] add to PATH: export PATH=\"\$HOME/.local/bin:\$PATH\"  (add to ~/.bashrc / ~/.zshrc)";; esac

# register .md handler hint (optional, no root needed)
if command -v xdg-mime >/dev/null 2>&1 && [ -f "$HOME/.local/share/applications/mterm.desktop" ]; then
  echo "[mime] mterm.desktop already exists"
else
  echo "[mime] to make double-click on *.mterm.md open with mterm, run: mterm --register  (coming soon)"
fi

echo ""
echo "try: mterm app examples/runbook.md"
exit 0

# MTERM_MD
# --- markdown starts here (rendered in mterm) ---
---
mterm:
  permissions:
    shell: ["sh examples/install.md *", "curl *", "wget *"]
---

# Install mterm

> **Don't have mterm yet?** You won't see buttons. Open a terminal and run `sh install.md` (or `sh examples/install.md` if you cloned the repo). After install, reopen this file with `mterm app install.md` to get clickable buttons.

**If you have mterm — click to run:**

> [!BUTTON] Install mterm now

```sh :run id=install
sh examples/install.md
```

> [!BUTTON] Dry run (check what it would do)

```sh :run id=dry
sh examples/install.md --dry-run
```

**If you don't have mterm — run in terminal:**

```bash
cd ~/Downloads   # or repo root if cloned
sh install.md
```

**What it does:**
1. Detects your system (`Linux x64/arm64`, `macOS x64/arm64`)
2. Downloads the right binary from GitHub Releases
3. Installs to `~/.local/bin/mterm` (no sudo)
4. Leaves your `runbook.md` files double-clickable

No `cargo`, no `rust`, no `curl | sh` from a random URL.

**Manual fallback:**
```sh
curl -fsSL https://github.com/masonleetompkins/mterm/releases/latest/download/mterm-linux-x64 -o ~/.local/bin/mterm && chmod +x ~/.local/bin/mterm
```

> [!NOTE]
> This is the same file you just opened. Markdown + shell polyglot — dogfooding `mterm`.
