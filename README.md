# Stake

Stake is a Rust Linux desktop app that provides a themed GUI for running [`pake`](https://github.com/tw93/pake):

- Website URL input
- App Title input
- **Create** button that runs `pake <url> --name <title>`

## Important: how to use this with your local `pake` clone

You **do not** need to `git apply` these files into the upstream `pake` repo.

Instead, keep `Stake` as its **own repository/project**, then install `pake` on the system and let Stake call it as an external command.

If you already tried applying patches into `pake` and got conflicts (`U .gitignore`, `U README.md`), discard that attempt in the pake repo:

```bash
git merge --abort || true
git reset --hard
git clean -fd
```

Then build Stake separately.

## Build

```bash
cargo run
```

## Packaging for Linux distro integration

### Build `.deb`

```bash
./scripts/build-deb.sh
```

Output: `target/debian/*.deb`

### Build `.AppImage`

1. Install `linuxdeploy` (from its official releases).
2. A default vector icon is already included at `assets/stake.svg` (and optional PNG override at `assets/stake.png`).
3. Run:

```bash
./scripts/build-appimage.sh
```

## Desktop launcher

Desktop entry template is at:

- `assets/stake.desktop`
- `assets/stake.svg`

It is included in `.deb` packaging metadata and used by the AppImage script.
