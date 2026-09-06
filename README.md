# LCJ

> A lightweight local judge for competitive programming.

## Features

* [X] Compile submitted solutions
* [X] Run solutions against test cases
* [X] Compare output with expected output
* 🚧 Support multiple programming languages (currently only C, C++, Rust)
* [X] Detect available compilers
* 🚧 Cross-platform support (have not yay tested on Windows)
* 🚧 Test case management (currently have no `freopen()` support)

## Supported Platforms

* Linux
* Windows

## Supported Languages

| Language | Compiler          | Status |
| -------- | ----------------- | ------ |
| C++      | `g++` / `clang++` | 🚧 - currently have no `freopen()` support     |
| C        | `gcc` / `clang`   | 🚧 - currently have no `freopen()` support    |
| Rust     | `rustc`           | 🚧 - implemented with `rustc`, only support `stdin`, `stdout`    |

## Installation

### From source

Requirements:

* [Rust](https://www.rust-lang.org/)
* A supported compiler (`gcc`, `clang`, or `rustc`)

```bash
git clone https://github.com/msdotrh/lcj.git
cd lcj

cargo build --release
```

The compiled binary will be located at:

```text
target/release/lcj
```
### Using package managers ❌ (currently aren't supported)

## Usage

```bash
lcj [OPTIONS] [COMMAND]
```

> Usage documentation will be added as the CLI stabilizes.

## Configuration

Configuration options will be documented here.

## Project Structure

```text
lcj/
├── .github/
│   └── workflows/
├── src/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

## Development

Build the project:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

Build an optimized release:

```bash
cargo build --release
```

## Contributing

Contributions, bug reports, and suggestions are welcome.

1. Fork the repository
2. Create a branch
3. Make your changes
4. Open a pull request

## License

This project is licensed under the **GNU General Public License v3.0**.

See [`LICENSE`](LICENSE) for more information.
