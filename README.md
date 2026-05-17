# <p align="center"><img src="assets/stake-256.png" alt="Stake Logo" width="160"></p>
# <p align="center">S T A K E</p>
<p align="center">
  <strong>Forge lightweight Linux web apps with Pake</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Lilith%20Linux-red?style=for-the-badge&logo=linux" alt="Platform: Lilith Linux">
  <img src="https://img.shields.io/badge/Built%20With-Rust-black?style=for-the-badge&logo=rust" alt="Built With: Rust">
  <img src="https://img.shields.io/badge/Powered%20By-Pake-orange?style=for-the-badge" alt="Powered By: Pake">
</p>

---

## 🩸 Overview

**Stake** is a high-performance, premium-themed Rust/egui desktop application that serves as a custom-designed GUI wrapper around [`pake`](https://github.com/tw93/pake). 

Specifically crafted and optimized to match the gothic and sleek aesthetic of **Lilith Linux**, Stake allows you to instantaneously turn any web page into a native, ultra-lightweight desktop application with a single click.

<p align="center">
  <img src="assets/stake-hero.png" alt="Stake UI Screenshot" width="550" style="border-radius: 12px; box-shadow: 0 8px 24px rgba(0,0,0,0.5);">
</p>

---

## ⚡ Features

- **Instant App Forging**: Enter any website URL and a custom app name to build a lightweight desktop app.
- **Ultra-Lightweight**: Built on top of Rust and Pake, resulting in packages that are up to **40x smaller** than standard Electron apps.
- **Lilith Linux Native**: Designed from the ground up to match the visual language, typography (featuring the Creepster gothic font), and premium dark aesthetic of Lilith Linux.
- **Dual Packaging Options**: Easily generate native `.deb` installers or highly portable `.AppImage` packages.

---

## 🛠️ Installation & Setup

Stake requires a local installation of `pake` on your system. Keep Stake as its own independent repository, install `pake` globally, and Stake will handle the rest!

### Prerequisites

Ensure you have `pake` installed on your system:
```bash
npm install -g pake-cli
```

### Building from Source

To run or build Stake locally:

```bash
# Clone the repository and navigate in
git clone https://github.com/BlancoBAM/Stake.git
cd Stake

# Run the desktop app
cargo run --release
```

---

## 📦 Packaging

Stake includes pre-configured automation scripts to package the application for distribution:

### 1. Build Debian Package (`.deb`)

```bash
./scripts/build-deb.sh
```
*Output: `target/debian/stake_*.deb`*

### 2. Build AppImage (`.AppImage`)

```bash
./scripts/build-appimage.sh
```
*Output: `Stake-*.AppImage`*

---

## 🖤 Credits & Appreciation

- **Pake**: Deep appreciation and gratitude to [tw93/pake](https://github.com/tw93/pake) for creating the phenomenal, lightweight web-app compiler that powers Stake's backend under the hood.
- **Lilith Linux**: Custom-crafted with devotion to match the dark and gothic desktop environments of Lilith Linux.

---
<p align="center">
  <i>Forge your apps. Strike them down. Stake your claim.</i>
</p>
