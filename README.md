# RISC16

![GitHub License](https://img.shields.io/github/license/julesjung/risc16?style=for-the-badge)
![GitHub Release](https://img.shields.io/github/v/release/julesjung/risc16?style=for-the-badge)

## Installation

### Dependencies

This project uses `customasm` to assemble its files. 
- If you are using Cargo for the installation, you can install it using :
```sh
cargo install customasm
```
- Else, follow the installation instructions from [their GitHub repository](https://github.com/hlorenzi/customasm?tab=readme-ov-file#installation).

### Using Cargo

Make sure that you have Rust installed on your system. If not, download and install it from [the official website](https://www.rust-lang.org/tools/install).
```sh
cargo install --git https://github.com/julesjung/risc16.git risc16
```

### Using pre-built binaries

You can find the pre-built binaries on the [GitHub releases page](https://github.com/julesjung/risc16/releases). Download the one corresponding to your operating system and architecture.

## Usage

### Assembling a file

```sh
risc16 assemble <INPUT> <OUTPUT>
```

### Running an assembly file in the emulator

```sh
risc16 emulate <INPUT>
```

### Running a binary file in the emulator

```sh
risc16 emulate -f bin <INPUT>
```

## License

This project is licensed under the GPLv3 license. See the LICENSE file for more details.