# OmniCraft Runtime

The browser-side runtime engine for OmniCraft applications. It combines a high-performance Entity Component System (ECS) with fine-grained reactivity.

## ⚡ Features

- **Fine-Grained Reactivity**: Signal-based reactivity inspired by SolidJS. Updates are pinpointed to specific DOM nodes.
- **ECS Architecture**: Bevy-inspired ECS for managing state, logic, and rendering.
- **WASM Interop**: Seamless bindings to `web-sys` and `js-sys` for DOM manipulation.
- **Virtual DOM-less**: Direct DOM updates for maximum performance.

## 🧩 Key Components

- `Signal<T>`: Reactive primitive for state management.
- `Runtime`: The central orchestrator (ECS World + Scheduler).
- `Component`: Traits for UI components.
- `render()`: Entry point for mounting the application.

## 🛠️ Usage

This crate is primarily used by the code generated from the `omnicraft-compiler`.

```rust
use omnicraft_runtime::{prelude::*, render};

#[component]
fn App(cx: Scope) -> impl IntoView {
    let count = create_signal(cx, 0);

    view! { cx,
        button(on:click=move |_| count.update(|n| *n += 1)) {
            "Count: " {move || count.get()}
        }
    }
}
```
