# OmniCraft 2.0 - Recommended Dependencies

**Generated:** January 5, 2026  
**Status:** Latest Stable Versions

---

## 📦 Core Dependencies Summary

| Category | Crate | Version | Purpose |
|----------|-------|---------|---------|
| **ECS** | `bevy_ecs` | `0.17.3` | Entity-Component-System architecture |
| **Rendering** | `lyon` | `1.0.16` | Path tessellation for 2D graphics |
| **Layout** | `taffy` | `0.9.2` | Flexbox/Grid layout engine |
| **Math** | `glam` | `0.30.9` | Fast 3D math (vectors, matrices, SIMD) |
| **WASM** | `wasm-bindgen` | `0.2.106` | Rust ↔ JavaScript bindings |
| **WASM** | `web-sys` | `0.3.83` | Web API bindings |
| **WASM Opt** | `wasm-opt` | `0.116.1` | WASM binary optimization |
| **Serialization** | `serde` | `1.0.228` | Serialization framework |
| **Serialization** | `serde_json` | `1.0.148` | JSON support |
| **Async** | `tokio` | `1.49.0` | Async runtime |
| **CLI** | `clap` | `4.5.54` | Command-line argument parsing |
| **LSP** | `tower-lsp` | `0.20.0` | Language Server Protocol |
| **Lexer** | `logos` | `0.16.0` | Fast lexer generator |
| **File Watch** | `notify` | `8.2.0` | File system watcher (HMR) |
| **Error** | `anyhow` | `1.0.100` | Application error handling |
| **Error** | `thiserror` | `2.0.17` | Library error definitions |

---

## 🗂️ Cargo.toml Configuration

### Workspace Root

```toml
[workspace]
resolver = "2"
members = [
    "crates/omnicraft-compiler",
    "crates/omnicraft-runtime",
    "crates/omnicraft-cli",
    "crates/omnicraft-lsp",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
repository = "https://github.com/your-org/omnicraft"

[workspace.dependencies]
# Core
bevy_ecs = "0.17"
glam = "0.30"
taffy = "0.9"
lyon = "1.0"

# WASM
wasm-bindgen = "0.2"
web-sys = "0.3"
wasm-opt = "0.116"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async & Runtime
tokio = { version = "1.49", features = ["full"] }

# CLI & Tools
clap = { version = "4.5", features = ["derive"] }
notify = "8.2"

# LSP
tower-lsp = "0.20"

# Compiler
logos = "0.16"

# Error Handling
anyhow = "1.0"
thiserror = "2.0"
```

---

### Crate: `omnicraft-compiler`

```toml
[package]
name = "omnicraft-compiler"
version.workspace = true
edition.workspace = true

[dependencies]
# Lexer/Parser
logos = { workspace = true }

# Serialization (AST)
serde = { workspace = true }
serde_json = { workspace = true }

# Error Handling
anyhow = { workspace = true }
thiserror = { workspace = true }

# File Watching (Dev Server)
notify = { workspace = true }
tokio = { workspace = true }
```

---

### Crate: `omnicraft-runtime`

```toml
[package]
name = "omnicraft-runtime"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# ECS Core
bevy_ecs = { workspace = true }

# Math
glam = { workspace = true, features = ["serde"] }

# Layout
taffy = { workspace = true }

# Rendering
lyon = { workspace = true }

# WASM
wasm-bindgen = { workspace = true }
web-sys = { workspace = true, features = [
    "Window",
    "Document",
    "Element",
    "HtmlCanvasElement",
    "CanvasRenderingContext2d",
    "WebGl2RenderingContext",
    "Performance",
    "console",
] }

# Serialization
serde = { workspace = true }

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

---

### Crate: `omnicraft-cli`

```toml
[package]
name = "omnicraft-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "omnicraft"
path = "src/main.rs"

[dependencies]
omnicraft-compiler = { path = "../omnicraft-compiler" }

# CLI
clap = { workspace = true }

# Async
tokio = { workspace = true }

# File Watching
notify = { workspace = true }

# WASM Optimization
wasm-opt = { workspace = true }

# Error Handling
anyhow = { workspace = true }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

---

### Crate: `omnicraft-lsp`

```toml
[package]
name = "omnicraft-lsp"
version.workspace = true
edition.workspace = true

[[bin]]
name = "omnicraft-lsp"
path = "src/main.rs"

[dependencies]
omnicraft-compiler = { path = "../omnicraft-compiler" }

# LSP
tower-lsp = { workspace = true }

# Async
tokio = { workspace = true }

# Error Handling
anyhow = { workspace = true }
```

---

## 📋 Additional Recommended Crates

| Crate | Version | Purpose | When to Use |
|-------|---------|---------|-------------|
| `tracing` | `0.1` | Structured logging | CLI & debugging |
| `tracing-subscriber` | `0.3` | Log output formatting | CLI |
| `wasm-bindgen-test` | `0.3` | WASM unit testing | Runtime tests |
| `criterion` | `0.6` | Benchmarking | Performance testing |
| `insta` | `1.40` | Snapshot testing | Compiler output tests |
| `pretty_assertions` | `1.4` | Better test diffs | All tests |
| `dashmap` | `6.0` | Concurrent HashMap | Cache/parallel ops |
| `parking_lot` | `0.12` | Fast synchronization | Shared state |
| `rayon` | `1.10` | Parallel iteration | Batch processing |
| `miette` | `7.2` | Beautiful error reports | User-facing errors |

---

## 🔧 Recommended Feature Flags

### `glam` Features
```toml
glam = { version = "0.30", features = [
    "serde",           # Serialization support
    "bytemuck",        # Safe transmutes for GPU
    "fast-math",       # Faster but less precise
] }
```

### `web-sys` Essential Features
```toml
web-sys = { version = "0.3", features = [
    "Window",
    "Document", 
    "Element",
    "HtmlCanvasElement",
    "CanvasRenderingContext2d",
    "WebGl2RenderingContext",
    "Performance",
    "console",
    "KeyboardEvent",
    "MouseEvent",
    "WheelEvent",
    "TouchEvent",
    "DomRect",
    "CssStyleDeclaration",
    "ResizeObserver",
] }
```

### `tokio` Features
```toml
tokio = { version = "1.49", features = [
    "rt-multi-thread", # Multi-threaded runtime
    "macros",          # async macros
    "fs",              # File system ops
    "sync",            # Synchronization primitives
    "time",            # Timers
    "io-util",         # I/O utilities
] }
```

---

## 📌 Version Compatibility Notes

| Crate | Note |
|-------|------|
| `bevy_ecs` | v0.18 in RC, stick with 0.17.3 for stability |
| `logos` | v0.16.0 has breaking changes from v0.15 |
| `thiserror` | v2.x is new major version, check migration guide |
| `wasm-opt` | Tracks Binaryen 116, upstream is at 125 |

---

## 🚀 Quick Start Commands

```bash
# Create new project
cargo new omnicraft --lib
cd omnicraft

# Initialize workspace
mkdir -p crates/{omnicraft-compiler,omnicraft-runtime,omnicraft-cli,omnicraft-lsp}

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install tools
cargo install wasm-bindgen-cli
cargo install wasm-opt

# Build for WASM
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM
wasm-opt -O3 -o output.wasm target/wasm32-unknown-unknown/release/omnicraft_runtime.wasm
```
