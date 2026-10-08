# file-flier (ff)

A terminal disk space analyzer written in Rust.

`ff` scans a directory in parallel, calculates the size of everything inside it and opens an interactive TUI where you can walk through the tree and find out which folders and files take the most space.

It started as a Rust learning project and solves a very practical problem: not enough disk space.

<!-- Add a screenshot or a short GIF (asciinema / vhs) here. -->
<!-- ![screenshot](docs/screenshot.png) -->

## Features

- Parallel recursive scan built on [rayon](https://github.com/rayon-rs/rayon)
- Interactive TUI built with [ratatui](https://github.com/ratatui/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm)
- Navigate into any directory and go back, vim-style keys supported
- Open a file with the default system application
- Sorting by size or by name
- Loading screen and scan time shown in the header
- Skips virtual system directories (`/proc`, `/sys`, `/dev`, `/run`)

## Installation

You need a recent stable [Rust toolchain](https://rustup.rs) (the project uses edition 2024).

```sh
git clone https://github.com/vovob0t/file-flier
cd file-flier
cargo build --release
```

The binary will be at `target/release/file_flier`.

## Usage

```sh
file_flier --path=/home/user --sort=size
# or, from the source tree
cargo run --release -- -p=/ --sort=size
```

Arguments are passed as `name=value`, the `=` is required.

| Option         | Short | Description                                      | Default |
| -------------- | ----- | ------------------------------------------------ | ------- |
| `--path`       | `-p`  | Directory to analyze (`~` is expanded to home)   | `./`    |
| `--sort`       | `-s`  | Sorting mode, see below                          | `natural` |
| `--help`       | `-h`  | Print help                                       |         |

### Sorting modes

| Mode           | Behavior                                                          |
| -------------- | ----------------------------------------------------------------- |
| `size`         | Largest first                                                     |
| `alphabetical` | A to Z                                                            |
| `natural`      | Order returned by the filesystem (default, also used for unknown values) |
| `modification` | Accepted, but not implemented yet (behaves like `natural`)        |

### Keybindings

| Key                        | Action                                              |
| -------------------------- | --------------------------------------------------- |
| `↑` / `k`, `↓` / `j`       | Move selection up / down                            |
| `Enter` / `l`              | Enter the selected directory (on a file: open it with the default application) |
| `Backspace` / `h`, `Ctrl+o`| Go back to the parent directory                     |
| `g` / `G`                  | Jump to the first / last entry                      |
| `←` / `Esc`                | Clear selection                                     |
| `q`                        | Quit                                                |

## How it works

The scan builds an in-memory tree of `FileNode`s, where every directory knows the total size of its contents. After the scan, navigation in the TUI only walks this tree and never touches the disk again.

- The directory traversal is recursive. Entries of each directory are processed in parallel with rayon (`par_bridge`), so subdirectories are scanned concurrently on the rayon thread pool.
- Sizes of a directory's entries are accumulated into an `AtomicU64`, so no lock is needed while summing.
- The tree is stored as `Arc<RwLock<FileNode>>` (using the `parking_lot` lock) so nodes can be shared between the navigation history and the rendering code.
- The TUI keeps a stack of visited nodes, which makes going back a single `pop`.

The first version of the scanner was written with `std::thread`, `Arc<Mutex<...>>` and `mpsc` channels. It was later replaced with rayon, which removed most of the manual synchronization.

### Limitations

- Unix only: the code uses `std::os::unix`. Windows is not supported yet.
- Sizes are apparent file sizes (not blocks actually used on disk), shown in decimal units (1 KB = 1000 B).
- Symlinks are not followed.
- Directories that cannot be read are counted as empty.

## Roadmap

- [x] Loading screen
- [x] Parallel file scanning
- [ ] Sorting by modification time
- [ ] Windows support
- [ ] Search in the current directory (`/`)
- [ ] Mark files and delete them from the TUI
- [ ] Directory space visualization (pie charts and similar)
- [ ] Animated tree initialization on startup

## Tech stack

Rust 2024, [ratatui](https://github.com/ratatui/ratatui), [crossterm](https://github.com/crossterm-rs/crossterm), [rayon](https://github.com/rayon-rs/rayon), [parking_lot](https://github.com/Amanieu/parking_lot), [color-eyre](https://github.com/eyre-rs/eyre), [open](https://github.com/Byron/open-rs)

## License

<!-- Choose a license (MIT is the usual default for small tools) and add a LICENSE file. -->
