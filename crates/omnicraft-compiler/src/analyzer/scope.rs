//! Scope Analysis
//!
//! Tracks variable scopes and symbol tables.

use crate::ast::ReactiveKind;
use std::collections::HashMap;

use super::types::InferredType;

/// Kind of scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScopeKind {
    #[default]
    Global,
    Function,
    Block,
}

/// A symbol in the scope
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: InferredType,
    pub reactive: ReactiveKind,
    pub mutable: bool,
}

/// A scope containing symbols
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub kind: ScopeKind,
    pub symbols: HashMap<String, Symbol>,
    pub children: Vec<Scope>,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Get all reactive symbols
    pub fn reactive_symbols(&self) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|s| s.reactive != ReactiveKind::None)
            .collect()
    }
}
