#!/bin/sh
# mterm installer — Linux (double-click: Run in Terminal)
# Usage: double-click (Run) or: sh install.sh [--dry-run]
set -e
REPO="masonleetompkins/mterm"
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
echo "   ── installer · Linux/macOS · double-click to install ──"
echo ""

OS="$(uname -s 2>/dev/null || echo unknown)"
ARCH="$(uname -m 2>/dev/null || echo unknown)"
echo "[detect] OS=$OS ARCH=$ARCH"

TARGET=""
case "$OS" in
  Linux) case "$ARCH" in x86_64|amd64) TARGET="mterm-linux-x64";; aarch64|arm64) TARGET="mterm-linux-arm64";; *) echo "unsupported arch: $ARCH" >&2; exit 1;; esac;;
  Darwin) case "$ARCH" in x86_64) TARGET="mterm-macos-x64";; arm64|aarch64) TARGET="mterm-macos-arm64";; *) echo "unsupported arch: $ARCH" >&2; exit 1;; esac;;
  *) echo "unsupported OS: $OS" >&2; exit 1;;
esac

echo "[target] $TARGET"

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
case ":$PATH:" in *":$INSTALL_DIR:"*) ;; *) echo "[hint] add to PATH: export PATH=\"\$HOME/.local/bin:\$PATH\"  (add to ~/.bashrc)";; esac
echo ""
echo "try: mterm app examples/runbook.md"
# keep terminal open when double-clicked
if [ -t 0 ]; then
  printf "Press Enter to close..."; read dummy
fi
