# Roms file splitter

Some retro gaming devices (e.g. Analogue Pocket, NES Everdrive) have a limit on
how many files they can display per directory. `rom-splitter` solves this by
taking a flat directory of ROM files and organizing them into subdirectories of
a configurable size, sorted alphabetically. Each subdirectory is named by its
position and the range of filenames it contains, e.g. `part-01-A-to-M`.

## Running

```bash
# Split a directory of NES ROMs into subdirectories of 100 files each
rom-splitter -p ~/roms/nes -e nes -m 100

# Split Game Boy Advance ROMs with a smaller limit of 50 per directory
rom-splitter -p ~/roms/gba -e gba -m 50

# Run from within the ROM directory, using the default max of 100 files per directory
cd ~/roms/snes && rom-splitter -e sfc
```

## Installing

### macOS/Linux

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dberg/rom-splitter/releases/download/v0.1.0/rom-splitter-installer.sh | sh
```

### Windows (PowerShell)

```
powershell -ExecutionPolicy Bypass -c "irm https://github.com/dberg/rom-splitter/releases/download/v0.1.0/rom-splitter-installer.ps1 | iex"
```

## Dev

```bash
cargo build
```

Running

```bash
cargo run -- -p PATH_TO_ROMS_DIR -e nes -m 100
```

Releasing a new version

```bash
git tag v0.0.0
git push --tags
```

Listing the tags

```bash
git tag --list | sort -V
```
