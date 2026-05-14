#!/usr/bin/env bash
set -euo pipefail

APP=Stake
BIN=stake
ARCH="${ARCH:-x86_64}"
APPDIR="AppDir"

echo "→ Building release binary..."
cargo build --release

echo "→ Assembling AppDir..."
rm -rf "$APPDIR"
mkdir -p \
  "$APPDIR/usr/bin" \
  "$APPDIR/usr/share/applications" \
  "$APPDIR/usr/share/icons/hicolor/scalable/apps" \
  "$APPDIR/usr/share/icons/hicolor/16x16/apps" \
  "$APPDIR/usr/share/icons/hicolor/32x32/apps" \
  "$APPDIR/usr/share/icons/hicolor/48x48/apps" \
  "$APPDIR/usr/share/icons/hicolor/64x64/apps" \
  "$APPDIR/usr/share/icons/hicolor/96x96/apps" \
  "$APPDIR/usr/share/icons/hicolor/128x128/apps" \
  "$APPDIR/usr/share/icons/hicolor/256x256/apps" \
  "$APPDIR/usr/share/icons/hicolor/512x512/apps" \
  "$APPDIR/usr/share/fonts/truetype/stake"

cp "target/release/$BIN"       "$APPDIR/usr/bin/"
cp "assets/stake.desktop"       "$APPDIR/usr/share/applications/"

# Icons from optimized-icons (symlinked/copied into assets during CI)
for SIZE in 16 32 48 64 96 128 256 512; do
  SRC="assets/stake-${SIZE}.png"
  if [ -f "$SRC" ]; then
    cp "$SRC" "$APPDIR/usr/share/icons/hicolor/${SIZE}x${SIZE}/apps/stake.png"
  fi
done

if [ -f assets/stake.svg ]; then
  cp assets/stake.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/stake.svg"
fi

# Creepster font
if [ -f assets/Creepster-Regular.ttf ]; then
  cp assets/Creepster-Regular.ttf "$APPDIR/usr/share/fonts/truetype/stake/"
fi

# linuxdeploy check
if ! command -v linuxdeploy > /dev/null 2>&1; then
  echo "Downloading linuxdeploy..."
  curl -fsSL -o /usr/local/bin/linuxdeploy \
    "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
  chmod +x /usr/local/bin/linuxdeploy
fi

echo "→ Building AppImage..."
linuxdeploy \
  --appdir "$APPDIR" \
  --desktop-file assets/stake.desktop \
  --icon-file assets/stake-256.png \
  --output appimage

echo "✓ AppImage created in project root."
