![Crates.io Total Downloads](https://img.shields.io/crates/d/tarts)
![GitHub License](https://img.shields.io/github/license/oiwn/tui-screen-savers-rs)
[![codecov](https://codecov.io/gh/oiwn/tui-screen-savers-rs/graph/badge.svg?token=C7G4AX1ASV)](https://codecov.io/gh/oiwn/tui-screen-savers-rs)

# 🦀 TARTS: Terminal Arts 🎨

> **BLAZINGLY FAST** terminal screensavers written in Rust!

`tarts` (shortcut from **T**erminal **Arts**) is a collection of **MEMORY SAFE** terminal-based screen savers that bring visual delight to your command line. Built with **ZERO-COST ABSTRACTIONS**, these screen savers run efficiently while providing stunning visual effects.

![digital rain](https://i.imgur.com/OPKC7Rb.png)

## ✨ Features

- 🌧️ **Matrix Rain**: Experience the famous "Matrix" digital rain effect right in your terminal
- 🧫 **Conway's Game of Life**: Watch the classic cellular automaton evolve before your eyes
- 🧩 **Maze Generation**: Get lost in procedurally generated mazes
- 🐦 **Boids**: Witness the emergent flocking behavior of these simulated birds
- 🧊 **3D Cube**: Renders a rotating 3D cube using terminal graphics with braille patterns for higher resolution.
- 🦀 **Crab**: Animated crabs walking across your screen, interacting with each other and the environment.

## 🚀 Installation

Install directly using cargo:

```bash
cargo install tarts
```

## 🛠️ Usage

To use the screen savers, run the executable with the desired screen saver's name as an argument:

```bash
tarts matrix  # The classic digital rain effect
tarts life    # Conway's Game of Life
tarts maze    # Watch a maze generate itself
tarts boids   # Bird-like flocking simulation
tarts cube    # 3d rotating cube using braille patterns
tarts crab    # Ferris the crab with collisions
```

Press `q` or `Esc` to exit.

## ⚙️ Configuration

The screen savers can be configured via command line arguments:

```bash
# Test an effect for a specific number of frames
tarts --check --effect matrix --frames 100
```

## 🧪 Development

This project uses standard Rust tooling:

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Benchmark performance
cargo bench
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, and suggest features.

## 📜 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

---

<div align="center">
  <sub>Built with ❤️ and <strong>FEARLESS CONCURRENCY</strong></sub>
</div>


### More?

- Args parser to run with configuration (yaml or something)
- add rotating donut? https://www.a1k0n.net/2011/07/20/donut-math.html
- add pipes? https://asciinema.org/a/427066
- add cellular automation like https://www.reddit.com/r/neovim/comments/z70mg3/cellularautomatonnvim_my_first_plugin/
