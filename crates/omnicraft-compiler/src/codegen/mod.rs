//! Code Generation
//!
//! Handles generation of code for different targets (Rust, TypeScript).

pub mod rust;
pub mod typescript;

pub use rust::RustGenerator;
pub use typescript::TypeScriptGenerator;

// Backward compatibility alias
pub type CodeGenerator = RustGenerator;
