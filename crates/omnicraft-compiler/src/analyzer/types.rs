//! Type Inference
//!
//! Infers types from expressions and tracks type information.

use std::collections::HashMap;

/// Inferred type
#[derive(Debug, Clone, PartialEq, Default)]
pub enum InferredType {
    #[default]
    Unknown,
    Number,
    String,
    Boolean,
    Null,
    Array,
    Object,
    Function,
    Signal(Box<InferredType>),
    Memo,
    Effect,
}

impl InferredType {
    pub fn is_reactive(&self) -> bool {
        matches!(self, InferredType::Signal(_) | InferredType::Memo | InferredType::Effect)
    }

    pub fn inner_type(&self) -> &InferredType {
        match self {
            InferredType::Signal(inner) => inner,
            _ => self,
        }
    }
}

impl std::fmt::Display for InferredType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferredType::Unknown => write!(f, "unknown"),
            InferredType::Number => write!(f, "number"),
            InferredType::String => write!(f, "string"),
            InferredType::Boolean => write!(f, "boolean"),
            InferredType::Null => write!(f, "null"),
            InferredType::Array => write!(f, "array"),
            InferredType::Object => write!(f, "object"),
            InferredType::Function => write!(f, "function"),
            InferredType::Signal(inner) => write!(f, "Signal<{}>", inner),
            InferredType::Memo => write!(f, "Memo"),
            InferredType::Effect => write!(f, "Effect"),
        }
    }
}

/// Type context for storing inferred types
#[derive(Debug, Clone, Default)]
pub struct TypeContext {
    types: HashMap<String, InferredType>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, name: String, ty: InferredType) {
        self.types.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&InferredType> {
        self.types.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Get all reactive types
    pub fn reactive_types(&self) -> Vec<(&String, &InferredType)> {
        self.types.iter().filter(|(_, ty)| ty.is_reactive()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inferred_type_display() {
        assert_eq!(InferredType::Number.to_string(), "number");
        assert_eq!(InferredType::Signal(Box::new(InferredType::Number)).to_string(), "Signal<number>");
    }

    #[test]
    fn test_type_context() {
        let mut ctx = TypeContext::new();
        ctx.set("count".to_string(), InferredType::Signal(Box::new(InferredType::Number)));
        ctx.set("name".to_string(), InferredType::String);

        assert!(ctx.has("count"));
        assert!(ctx.get("count").unwrap().is_reactive());
        assert!(!ctx.get("name").unwrap().is_reactive());
    }
}
