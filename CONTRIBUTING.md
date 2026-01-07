# Contributing to OmniCraft

First off, thank you for considering contributing to OmniCraft! 🎉

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Style Guide](#style-guide)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- **Rust** 1.75+ ([rustup.rs](https://rustup.rs/))
- **wasm-pack** ([installation](https://rustwasm.github.io/wasm-pack/installer/))
- **Node.js** 18+ (for tooling)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/omnicraft/omnicraft.git
cd omnicraft

# Build all crates
cargo build

# Run tests
cargo test

# Run the compiler
cargo run -p omnicraft-cli -- help
```

### Project Structure

```
crates/
├── omnicraft-compiler/  # .omni → Rust compiler
├── omnicraft-runtime/   # ECS + Reactivity runtime
├── omnicraft-cli/       # CLI tool
└── omnicraft-lsp/       # Language Server
```

## How to Contribute

### Reporting Bugs

1. **Check existing issues** to avoid duplicates
2. Use the [bug report template](https://github.com/omnicraft/omnicraft/issues/new?template=bug_report.yml)
3. Include:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version)

### Suggesting Features

1. Use the [feature request template](https://github.com/omnicraft/omnicraft/issues/new?template=feature_request.yml)
2. Explain the use case
3. Describe the proposed solution

### Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run lints: `cargo clippy`
6. Format code: `cargo fmt`
7. Commit with a clear message
8. Push and open a Pull Request

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Ensure CI passes** (clippy, tests, fmt)
4. **One approval required** for merge
5. **Squash commits** if requested

### Commit Message Format

```
<type>(<scope>): <short summary>

<body>

<footer>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples:**
```
feat(compiler): add support for inline styles
fix(runtime): resolve memory leak in signal cleanup
docs(readme): update installation instructions
```

## Style Guide

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` with default settings
- Run `cargo clippy` and fix warnings
- Write doc comments for public APIs

### Documentation

- Use clear, concise language
- Include code examples
- Keep README up to date

## Questions?

Feel free to [open a discussion](https://github.com/omnicraft/omnicraft/discussions) or reach out to the maintainers.

---

Thank you for contributing! 🚀
