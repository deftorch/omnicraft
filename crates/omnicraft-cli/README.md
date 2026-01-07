# OmniCraft CLI

The official command-line interface for developing OmniCraft applications.

## 🚀 Commands

- `omnicraft new <project-name>`: Scaffolds a new OmniCraft project.
- `omnicraft dev`: Starts the development server with HMR (Hot Module Replacement).
- `omnicraft build`: Builds the project for production (WASM + JS bundling).

## 🛠️ Installation

```bash
cargo install omnicraft-cli
```

## 🔧 Internal Architecture

The CLI handles the entire build orchestration:
1.  Watches `.omni` files for changes.
2.  Invokes `omnicraft-compiler` to generate Rust code.
3.  Calls `cargo build --target wasm-bindgen` to compile WASM.
4.  Runs `wasm-bindgen` CLI tool to generate Glue JS.
5.  Serves the application via a local HTTP server.
