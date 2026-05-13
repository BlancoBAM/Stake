#!/usr/bin/env bash
set -euo pipefail

APP=Stake
BIN=stake
ARCH="${ARCH:-x86_64}"
APPDIR="AppDir"

cargo build --release

rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/256x256/apps" "$APPDIR/usr/share/icons/hicolor/scalable/apps"
cp target/release/$BIN "$APPDIR/usr/bin/"
cp assets/stake.desktop "$APPDIR/usr/share/applications/"

if [ -f assets/stake.png ]; then
  cp assets/stake.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/stake.png"
fi

<<<<<<< ours
=======
if [ -f assets/stake.svg ]; then
  cp assets/stake.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/stake.svg"
fi

>>>>>>> theirs
if ! command -v linuxdeploy >/dev/null 2>&1; then
  echo "Please install linuxdeploy first: https://github.com/linuxdeploy/linuxdeploy"
  exit 1
fi

linuxdeploy --appdir "$APPDIR" --desktop-file assets/stake.desktop --output appimage

echo "Done. AppImage created in project root."
<<<<<<< ours

if [ -f assets/stake.svg ]; then
  cp assets/stake.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/stake.svg"
fi
=======
>>>>>>> theirs
