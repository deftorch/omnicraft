//! OmniCraft Compiler
//!
//! Compiles `.omni` component files into optimized Rust code.
//!
//! ## Pipeline
//! 1. Lexer: Source → Tokens
//! 2. Parser: Tokens → AST
//! 3. Analyzer: AST → Annotated AST (reactive dependencies)
//! 4. Optimizer: Annotated AST → Optimized AST
//! 5. CodeGen: Optimized AST → Rust/TypeScript code

pub mod analyzer;
pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod npm;
pub mod optimizer;
pub mod parser;
pub mod sourcemap;

pub use analyzer::{analyze, AnalyzedComponent, Analyzer};
pub use ast::*;
pub use codegen::CodeGenerator;
pub use lexer::Lexer;
pub use npm::{PackageJson, PackageJsonBuilder};
pub use optimizer::{optimize, Optimizer, OptimizerConfig};
pub use parser::Parser;
pub use sourcemap::{SourceMap, SourceMapGenerator};

use anyhow::Result;

/// Compilation target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationTarget {
    Rust,
    TypeScript,
}

/// Compile an `.omni` file to the specified target
pub fn compile(source: &str, file_name: &str, target: CompilationTarget) -> Result<String> {
    // 1. Tokenize
    let tokens = Lexer::new(source).tokenize()?;

    // 2. Parse
    let component = Parser::new(tokens, file_name).parse()?;

    // 3. Analyze (reactive dependencies, types)
    let analyzed = analyze(&component)?;

    // 4. Optimize
    let optimized = optimize(&analyzed)?;

    // 5. Generate Code
    match target {
        CompilationTarget::Rust => {
            let mut generator = codegen::RustGenerator::new();
            generator.generate(&optimized)
        }
        CompilationTarget::TypeScript => {
            let mut generator = codegen::TypeScriptGenerator::new();
            generator.generate(&optimized)
        }
    }
}

/// Legacy compile function (default to Rust)
pub fn compile_rust(source: &str, file_name: &str) -> Result<String> {
    compile(source, file_name, CompilationTarget::Rust)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_component() {
        let source = r##"
<canvas width={800} height={600}>
  <circle x={400} y={300} radius={50} fill="#00d4ff" />
</canvas>
"##;

        let result = compile_rust(source, "test.omni");
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }
}
