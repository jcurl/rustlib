# RUST Utilities <!-- omit in toc -->

This repository contains small utilities, written in Rust.

## Development Environment

The development environment used is:

- Visual Studio Code (Linux)
- Extensions that are relevant for this project
  - rust-analyzer
  - Markdown All in One
  - Rewrap
  - Spell Right
  - Trailing Spaces

## General Rust Guidelines

These are generic instructions by Rust. Ensure you're in the project base
repository, where the `Cargo.toml` file is found.

- To build

  ```sh
  cargo build --release
  ```
- To test

  ```sh
  cargo test
  ```

- To check test case coverage. Ensure you have `llvm-cov` installed.

  ```sh
  cargo llvm-cov
  ```

## Libraries

The libraries provided:

* readelf - simple library to read the contents of an ELF file

