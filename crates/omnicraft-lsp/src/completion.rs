//! Completion Provider
//!
//! Provides autocomplete suggestions for `.omni` files.

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, InsertTextFormat,
};

/// Provider for code completions
pub struct CompletionProvider;

impl CompletionProvider {
    pub fn new() -> Self {
        Self
    }

    /// Get completions for the given parameters
    pub fn get_completions(&self, _params: &CompletionParams) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Element tags
        completions.extend(self.element_completions());
        
        // Reactive primitives
        completions.extend(self.reactive_completions());
        
        // Sections
        completions.extend(self.section_completions());

        completions
    }

    fn element_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "circle".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<circle x={$1} y={$2} radius={$3} fill=\"$4\" />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Circle shape element".to_string()),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(
                    "Creates a circle at (x, y) with the specified radius and fill color.".to_string()
                )),
                ..Default::default()
            },
            CompletionItem {
                label: "rectangle".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<rectangle x={$1} y={$2} width={$3} height={$4} fill=\"$5\" />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Rectangle shape element".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "ellipse".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<ellipse x={$1} y={$2} rx={$3} ry={$4} fill=\"$5\" />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Ellipse shape element".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "text".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<text x={$1} y={$2} content=\"$3\" fill=\"$4\" />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Text element".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "line".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<line x1={$1} y1={$2} x2={$3} y2={$4} stroke=\"$5\" />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Line element".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "path".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<path d=\"$1\" fill=\"$2\" />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("SVG path element".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "group".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<group>\n  $1\n</group>".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Group container".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "image".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<image x={$1} y={$2} src=\"$3\" width={$4} height={$5} />".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Image element".to_string()),
                ..Default::default()
            },
        ]
    }

    fn reactive_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "signal".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("signal($1)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a reactive signal".to_string()),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(
                    "Creates a reactive signal that can be read with signal() and written with signal.set()".to_string()
                )),
                ..Default::default()
            },
            CompletionItem {
                label: "memo".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("memo(() => $1)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a cached computation".to_string()),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(
                    "Creates a cached reactive computation that only recomputes when dependencies change".to_string()
                )),
                ..Default::default()
            },
            CompletionItem {
                label: "effect".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some("effect(() => {\n  $1\n})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Create a side effect".to_string()),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(
                    "Creates a side effect that runs when its dependencies change".to_string()
                )),
                ..Default::default()
            },
        ]
    }

    fn section_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "script".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<script>\n  $1\n</script>".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Script section".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "canvas".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<canvas width={$1} height={$2}>\n  $3\n</canvas>".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Canvas section".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "style".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text: Some("<style>\n  $1\n</style>".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                detail: Some("Style section".to_string()),
                ..Default::default()
            },
        ]
    }
}

impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}
