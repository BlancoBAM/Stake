#!/usr/bin/env bash
set -euo pipefail

# Run this from repo root (the directory containing Cargo.toml)
if [[ ! -f "Cargo.toml" || ! -f "src/main.rs" ]]; then
  echo "Error: run this script from the Stake project root." >&2
  exit 1
fi

echo "Applying Stake compatibility + icon integration fixes..."

# 1) Fix egui 0.29 compile issues in src/main.rs
sed -i \
  -e 's/use eframe::egui::{self, Color32, CornerRadius, RichText, Stroke, Vec2};/use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};/' \
  -e 's/egui::Frame::new()/egui::Frame::none()/g' \
  -e 's/\.corner_radius(CornerRadius::same(20))/\.rounding(Rounding::same(20.0))/g' \
  -e 's/egui::Margin::same(26)/egui::Margin::same(26.0)/g' \
  src/main.rs

# 2) Ensure scalable icon is included in Cargo .deb metadata
if ! grep -q 'assets/stake.svg' Cargo.toml; then
  awk '
    /\[package.metadata.deb\]/ { in_deb=1 }
    { print }
    in_deb && /\["assets\/stake.desktop", "usr\/share\/applications\/stake.desktop", "644"\],/ {
      print "  [\"assets/stake.svg\", \"usr/share/icons/hicolor/scalable/apps/stake.svg\", \"644\"],"
      in_deb=0
    }
  ' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml
fi

# 3) Add/update default vector icon
mkdir -p assets
cat > assets/stake.svg <<'SVG'
<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="#0b0b12"/>
      <stop offset="100%" stop-color="#15080a"/>
    </linearGradient>
    <filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
      <feGaussianBlur stdDeviation="5" result="c"/>
      <feMerge><feMergeNode in="c"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
  </defs>
  <rect x="10" y="10" width="492" height="492" rx="56" fill="url(#bg)" stroke="#ff3a3a" stroke-width="8"/>
  <text x="60" y="135" font-family="serif" font-size="94" font-weight="700" fill="#ff4b4b" filter="url(#glow)">STAKE</text>
  <rect x="236" y="142" width="40" height="260" rx="16" fill="#7a5335" stroke="#2a1a13" stroke-width="6"/>
  <ellipse cx="256" cy="146" rx="74" ry="30" fill="#846042" stroke="#2a1a13" stroke-width="6"/>
  <path d="M132 290 C170 222, 340 222, 384 298 C398 328, 386 386, 328 402 C246 426, 146 404, 124 350 C116 330,118 308,132 290 Z"
        fill="#8d8395" stroke="#221927" stroke-width="8"/>
  <ellipse cx="256" cy="332" rx="44" ry="26" fill="#2a202d" opacity="0.45"/>
</svg>
SVG

# 4) Update AppImage script to include scalable icon path/copy
if [[ -f scripts/build-appimage.sh ]]; then
  sed -i \
    -e 's|"$APPDIR/usr/share/icons/hicolor/256x256/apps"|"$APPDIR/usr/share/icons/hicolor/256x256/apps" "$APPDIR/usr/share/icons/hicolor/scalable/apps"|' \
    scripts/build-appimage.sh

  if ! grep -q 'scalable/apps/stake.svg' scripts/build-appimage.sh; then
    cat >> scripts/build-appimage.sh <<'APPEND'

if [ -f assets/stake.svg ]; then
  cp assets/stake.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/stake.svg"
fi
APPEND
  fi
fi

# 5) README notes about default icon
if [[ -f README.md ]]; then
  sed -i 's|2\. Optionally place a 256x256 icon at `assets/stake.png`\.|2. A default vector icon is already included at `assets/stake.svg` (and optional PNG override at `assets/stake.png`).|' README.md || true
  if ! grep -q '`assets/stake.svg`' README.md; then
    cat >> README.md <<'README_APPEND'

## Desktop launcher assets

- `assets/stake.desktop`
- `assets/stake.svg`
README_APPEND
  fi
fi

echo "Done. Review with: git diff"
echo "Then run: cargo fmt && cargo build"
