# OmniCraft IDE Support (LSP)

The Language Server Protocol (LSP) implementation for OmniCraft support in editors like VS Code.

## ✨ Features

- **Diagnostics**: Real-time syntax error reporting.
- **Syntax Highlighting**: Semantic token support.
- **Go to Definition** (Planned): Jump to component definitions.
- **Autocomplete** (Planned): Suggestions for props and components.

## 🔌 Integration

This binary communicates over standard IO (stdin/stdout) using the LSP JSON-RPC protocol.

### VS Code Configuration

To use this with a generic LSP client:

```json
{
    "omnicraft": {
        "command": "omnicraft-lsp",
        "filetypes": ["omni"]
    }
}
```
