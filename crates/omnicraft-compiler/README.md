# OmniCraft Compiler

The core compiler infrastructure for OmniCraft. This crate is responsible for transforming `.omni` source code into optimized Rust/WebAssembly.

## 🏗️ Architecture

The compiler follows a standard multi-stage pipeline:

1.  **Lexing** (`lexer.rs`): Tokenizes the source code using `logos`.
2.  **Parsing** (`parser.rs`): Constructs an AST (Abstract Syntax Tree) using a recursive descent parser.
3.  **Analysis** (Planned): Type checking and semantic analysis.
4.  **Code Generation** (`codegen.rs`): Emits Rust code that interfaces with `omnicraft-runtime`.

## 📦 Modules

- `lexer`: Token definitions and lexing logic.
- `parser`: AST definitions and parsing rules.
- `codegen`: Rust code emission.
- `lib`: Public API entry point.

## 🚀 Usage

```rust
use omnicraft_compiler::compile;

let source = r#"
component MyComponent {
    render {
        Text("Hello")
    }
}
"#;

match compile(source) {
    Ok(rust_code) => println!("Generated: {}", rust_code),
    Err(e) => eprintln!("Error: {}", e),
}
```
