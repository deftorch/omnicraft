//! Hover Provider
//!
//! Provides hover information for `.omni` files.

use tower_lsp::lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

/// Provider for hover information
pub struct HoverProvider;

impl HoverProvider {
    pub fn new() -> Self {
        Self
    }

    /// Get hover information for the given parameters
    pub fn get_hover(&self, _params: &HoverParams) -> Option<Hover> {
        // TODO: Implement proper hover based on AST analysis
        // For now, return None (no hover info)
        None
    }

    /// Get documentation for a keyword
    #[allow(dead_code)]
    fn get_keyword_docs(&self, keyword: &str) -> Option<Hover> {
        let docs = match keyword {
            "signal" => Some((
                "signal(initialValue)",
                "Creates a reactive signal. A signal holds a value that can be read and updated.\n\n```js\nconst count = signal(0);\ncount()      // read: 0\ncount.set(5) // write\ncount()      // read: 5\n```"
            )),
            "memo" => Some((
                "memo(() => computation)",
                "Creates a memoized computation. Memos cache their result and only recompute when dependencies change.\n\n```js\nconst doubled = memo(() => count() * 2);\n```"
            )),
            "effect" => Some((
                "effect(() => { ... })",
                "Creates a side effect that runs when dependencies change.\n\n```js\neffect(() => {\n  console.log(count());\n});\n```"
            )),
            "canvas" => Some((
                "<canvas width={} height={}>",
                "The canvas section defines the visual output area. All visual elements must be inside a canvas.\n\n```html\n<canvas width={800} height={600}>\n  <circle x={400} y={300} radius={50} />\n</canvas>\n```"
            )),
            "circle" => Some((
                "<circle x={} y={} radius={} />",
                "A circle shape element.\n\n**Attributes:**\n- `x`, `y`: Center position\n- `radius`: Circle radius\n- `fill`: Fill color\n- `stroke`: Stroke color"
            )),
            "rectangle" | "rect" => Some((
                "<rectangle x={} y={} width={} height={} />",
                "A rectangle shape element.\n\n**Attributes:**\n- `x`, `y`: Top-left position\n- `width`, `height`: Dimensions\n- `fill`: Fill color\n- `stroke`: Stroke color"
            )),
            "text" => Some((
                "<text x={} y={} content=\"\" />",
                "A text element.\n\n**Attributes:**\n- `x`, `y`: Position\n- `content`: Text string\n- `fill`: Text color\n- `fontSize`: Font size"
            )),
            "group" => Some((
                "<group>...</group>",
                "A container for grouping elements together. Transforms applied to the group affect all children."
            )),
            _ => None,
        };

        docs.map(|(title, body)| Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("### {}\n\n{}", title, body),
            }),
            range: None,
        })
    }
}

impl Default for HoverProvider {
    fn default() -> Self {
        Self::new()
    }
}
