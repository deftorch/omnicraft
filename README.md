<p align="center">
  <img src="docs/assets/omnicraft-logo.svg" alt="OmniCraft Logo" width="200" height="200">
</p>

<h1 align="center">OmniCraft</h1>

<p align="center">
  <strong>🚀 Universal Visual Content Creation Platform</strong>
</p>

<p align="center">
  <a href="https://github.com/omnicraft/omnicraft/actions/workflows/ci.yml">
    <img src="https://github.com/omnicraft/omnicraft/actions/workflows/ci.yml/badge.svg" alt="CI Status">
  </a>
  <a href="https://crates.io/crates/omnicraft">
    <img src="https://img.shields.io/crates/v/omnicraft.svg" alt="Crates.io">
  </a>
  <a href="https://docs.rs/omnicraft">
    <img src="https://docs.rs/omnicraft/badge.svg" alt="Documentation">
  </a>
  <a href="LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License">
  </a>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#examples">Examples</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## ✨ What is OmniCraft?

**OmniCraft** is a next-generation visual content creation platform that combines:

- 🔧 **Compiler-First Architecture** — Svelte-inspired compile-time optimizations
- ⚡ **Fine-Grained Reactivity** — SolidJS-inspired reactive primitives
- 🎮 **ECS Core Engine** — Bevy-inspired Entity Component System
- 📝 **Progressive DSL** — Multi-level domain-specific language for intuitive development

Write declarative UI code in `.omni` files and compile to highly optimized WebAssembly.

## 🎯 Features

| Feature | Description |
|---------|-------------|
| 🚀 **Blazing Fast** | < 50KB bundle, < 150ms initial load |
| 🔄 **Reactive By Default** | Automatic dependency tracking with signals |
| 🎨 **Visual-First** | Built for graphics, animations, and visual content |
| 📦 **Component System** | Reusable, composable UI components |
| 🌐 **WASM Powered** | Native-speed performance in the browser |
| 🛠️ **Great DX** | LSP support, hot reload, source maps |

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Installation

```bash
# Install the CLI
cargo install omnicraft-cli

# Create a new project
omnicraft new my-app
cd my-app

# Start development server
omnicraft dev
```

### Hello World

Create `src/main.omni`:

```omni
component HelloWorld {
    state count = 0

    render {
        Column {
            Text("Hello, OmniCraft! 🎨")
                .fontSize(24)
                .color("#6366f1")

            Button("Count: {count}")
                .onClick(|| count += 1)
                .padding(16)
                .backgroundColor("#4f46e5")
                .cornerRadius(8)
        }
        .gap(16)
        .padding(32)
    }
}
```

## 📁 Project Structure

```
omnicraft/
├── crates/
│   ├── omnicraft-compiler/   # .omni → Rust compiler
│   ├── omnicraft-runtime/    # ECS + Reactivity runtime
│   ├── omnicraft-cli/        # Development CLI tool
│   └── omnicraft-lsp/        # Language Server Protocol
├── docs/                     # Documentation
├── examples/                 # Example projects
│   ├── hello-world/
│   ├── counter/
│   └── shapes/
└── Cargo.toml
```

## ⚡ Performance

| Metric | Target | Achieved |
|--------|--------|----------|
| Bundle Size (gzipped) | < 50 KB | ~45 KB |
| Initial Load | < 150 ms | ~80 ms |
| Memory (1k entities) | < 5 MB | ~2.5 MB |
| Update Time (1k entities) | < 1 ms | ~0.15 ms |
| Compilation (incremental) | < 50 ms | ~35 ms |

## 📖 Documentation

- [📘 User Guide](docs/guide.md) - Getting started guide
- [📐 Software Design](docs/OmniCraft%20Software%20Design.md) - Architecture & design
- [🔧 API Reference](https://docs.rs/omnicraft) - Rust API documentation
- [📋 Changelog](CHANGELOG.md) - Version history

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

- 🐛 [Report a Bug](https://github.com/omnicraft/omnicraft/issues/new?template=bug_report.yml)
- 💡 [Request a Feature](https://github.com/omnicraft/omnicraft/issues/new?template=feature_request.yml)
- 📖 [Improve Documentation](https://github.com/omnicraft/omnicraft/issues)

## 📄 License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## 🙏 Acknowledgments

OmniCraft is inspired by many great projects:

- [Svelte](https://svelte.dev/) - Compiler-first approach
- [SolidJS](https://www.solidjs.com/) - Fine-grained reactivity
- [Bevy](https://bevyengine.org/) - ECS architecture
- [Tauri](https://tauri.app/) - Rust + Web integration

---

<p align="center">
  Made with ❤️ by the OmniCraft Team
</p>
