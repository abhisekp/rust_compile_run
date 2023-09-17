# Rust Compile and Run

## Description

`Rust Compile and Run` is a command-line utility that simplifies the process of compiling, running, and cleaning up Rust executables. It offers various features like watching for file changes, running in a temporary directory, and optional notifications.

## Installation

To install the package, run:

```bash
cargo install rust_compile_run
```

## Usage

Basic usage:

```bash
rust_compile_run -f your_file.rs
```

Compile and run in a temporary directory:

```bash
rust_compile_run -f your_file.rs -t
```

Watch for file changes and recompile:

```bash
rust_compile_run -f your_file.rs -w
```

Enable notifications:

```bash
rust_compile_run -f your_file.rs -n
```

## Features

- **Compile and Run**: Compiles and runs a given Rust file.
- **Temporary Directory**: Optionally compiles and runs the Rust file in a temporary directory.
- **Watch Mode**: Watches for file changes and recompiles.
- **Notifications**: Optional notifications for compile and run status.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
