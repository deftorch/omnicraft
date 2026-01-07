//! Server Capabilities
//!
//! Defines what features the LSP server supports.

use tower_lsp::lsp_types::*;

/// Get the server capabilities
pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        // Sync full document on change
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(true),
                })),
                ..Default::default()
            },
        )),
        
        // Completion
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![
                "<".to_string(),
                "{".to_string(),
                ".".to_string(),
                ":".to_string(),
                "@".to_string(),
            ]),
            resolve_provider: Some(false),
            ..Default::default()
        }),
        
        // Hover
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        
        // Document formatting (future)
        // document_formatting_provider: Some(OneOf::Left(true)),
        
        // Go to definition (future)
        // definition_provider: Some(OneOf::Left(true)),
        
        ..Default::default()
    }
}
