//! OmniCraft Language Server
//!
//! Provides IDE support for `.omni` files through the Language Server Protocol.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod capabilities;
mod diagnostics;
mod completion;
mod hover;

pub use capabilities::server_capabilities;
pub use diagnostics::DiagnosticsProvider;
pub use completion::CompletionProvider;
pub use hover::HoverProvider;

/// OmniCraft Language Server backend
pub struct OmniCraftLsp {
    client: Client,
    diagnostics: DiagnosticsProvider,
    completion: CompletionProvider,
    hover: HoverProvider,
}

impl OmniCraftLsp {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            diagnostics: DiagnosticsProvider::new(),
            completion: CompletionProvider::new(),
            hover: HoverProvider::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for OmniCraftLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(ServerInfo {
                name: "omnicraft-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "OmniCraft LSP initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        let diagnostics = self.diagnostics.validate(&text);
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.first() {
            let diagnostics = self.diagnostics.validate(&change.text);
            self.client.publish_diagnostics(uri, diagnostics, None).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(text) = params.text {
            let diagnostics = self.diagnostics.validate(&text);
            self.client.publish_diagnostics(uri, diagnostics, None).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(
            self.completion.get_completions(&params),
        )))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        Ok(self.hover.get_hover(&params))
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(OmniCraftLsp::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
