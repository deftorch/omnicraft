//! Diagnostics Provider
//!
//! Validates `.omni` source files and produces diagnostics.

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/// Provider for source code diagnostics
pub struct DiagnosticsProvider;

impl DiagnosticsProvider {
    pub fn new() -> Self {
        Self
    }

    /// Validate source code and return diagnostics
    pub fn validate(&self, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Try lexing
        match omnicraft_compiler::Lexer::new(source).tokenize() {
            Ok(tokens) => {
                // Try parsing
                match omnicraft_compiler::Parser::new(tokens, "document.omni").parse() {
                    Ok(_component) => {
                        // Parsing succeeded - no diagnostics
                    }
                    Err(parse_err) => {
                        // Add parse error diagnostic
                        let message = format!("{}", parse_err);
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: 1 },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(tower_lsp::lsp_types::NumberOrString::String("parse-error".to_string())),
                            source: Some("omnicraft".to_string()),
                            message,
                            ..Default::default()
                        });
                    }
                }
            }
            Err(lex_err) => {
                // Add lex error diagnostic
                let message = format!("{}", lex_err);
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 1 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(tower_lsp::lsp_types::NumberOrString::String("lex-error".to_string())),
                    source: Some("omnicraft".to_string()),
                    message,
                    ..Default::default()
                });
            }
        }

        diagnostics
    }
}

impl Default for DiagnosticsProvider {
    fn default() -> Self {
        Self::new()
    }
}
