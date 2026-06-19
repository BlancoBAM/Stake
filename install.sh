#!/usr/bin/env bash
# Stake — Install Script
# Builds from source and installs system-wide.
set -euo pipefail

BOLD='\033[1m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; RED='\033[0;31m'; NC='\033[0m'
info()  { echo -e "${GREEN}[✓]${NC} $*"; }
warn()  { echo -e "${YELLOW}[!]${NC} $*"; }
error() { echo -e "${RED}[✗]${NC} $*"; exit 1; }
step()  { echo -e "\n${BOLD}${CYAN}▶ $*${NC}"; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

step "Installing build dependencies..."
sudo apt-get install -y pkg-config libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev build-essential 2>/dev/null || \
    warn "Some apt deps may be missing — install manually if build fails"

# Stake also needs pake-cli (npm)
if ! command -v pake &>/dev/null; then
    warn "pake-cli not found — Stake requires it for app forging"
    if command -v npm &>/dev/null; then
        step "Installing pake-cli globally..."
        sudo npm install -g pake-cli
        info "pake-cli installed"
    else
        warn "npm not found. Install Node.js first: https://nodejs.org"
        warn "Then run: sudo npm install -g pake-cli"
    fi
fi

if ! command -v cargo &>/dev/null; then
    error "Rust/Cargo not found. Install from https://rustup.rs"
fi
info "Rust $(rustc --version | cut -d' ' -f2) found"

step "Building Stake from source..."
cd "$SCRIPT_DIR"
cargo build --release
if [[ ! -f "target/release/stake" ]]; then
    error "Build failed — binary not found at target/release/stake"
fi
info "Build complete"

step "Installing binary to /usr/local/bin/..."
sudo install -m 0755 target/release/stake /usr/local/bin/stake
info "Binary installed: /usr/local/bin/stake"

step "Installing icon and desktop entry..."
ICON_SRC="$SCRIPT_DIR/assets/stake-256.png"
if [[ ! -f "$ICON_SRC" ]]; then
    ICON_SRC=$(find "$SCRIPT_DIR/assets" -name "*.png" | head -1)
fi
if [[ -n "$ICON_SRC" && -f "$ICON_SRC" ]]; then
    sudo mkdir -p /usr/share/pixmaps
    sudo cp "$ICON_SRC" /usr/share/pixmaps/stake.png
    info "Icon installed"
fi

sudo tee /usr/share/applications/stake.desktop > /dev/null << 'DESKTOP'
[Desktop Entry]
Type=Application
Name=Stake
GenericName=Web App Forger
Comment=Turn any website into a native desktop app
Exec=stake
Icon=stake
Categories=Network;Utility;
Terminal=false
StartupNotify=true
DESKTOP

if command -v update-desktop-database &>/dev/null; then
    sudo update-desktop-database /usr/share/applications 2>/dev/null || true
fi
if command -v gtk-update-icon-cache &>/dev/null; then
    sudo gtk-update-icon-cache /usr/share/icons/hicolor 2>/dev/null || true
fi

echo ""
echo -e "${GREEN}${BOLD}✅ Stake installed!${NC}"
echo "  Run: stake"
echo "  Or find 'Stake' in your application menu."
