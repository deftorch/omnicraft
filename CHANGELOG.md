# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with workspace setup
- `omnicraft-compiler` crate with lexer, parser, and code generator
- `omnicraft-runtime` crate with ECS and reactive system
- `omnicraft-cli` crate with development server
- `omnicraft-lsp` crate for IDE support
- Basic `.omni` file syntax support
- WASM compilation target

### Changed
- Nothing yet

### Deprecated
- Nothing yet

### Removed
- Nothing yet

### Fixed
- Nothing yet

### Security
- Nothing yet

## [0.1.0] - 2026-01-08

### Added
- 🎉 Initial release
- Core compiler with `.omni` → Rust → WASM pipeline
- ECS-based runtime engine
- Fine-grained reactivity with signals
- Basic component system (Text, Button, Column, Row)
- CLI with `new`, `build`, and `dev` commands
- Hot Module Replacement (HMR) for development
- Language Server Protocol (LSP) basics

[Unreleased]: https://github.com/omnicraft/omnicraft/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/omnicraft/omnicraft/releases/tag/v0.1.0
