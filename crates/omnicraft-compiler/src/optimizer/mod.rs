//! Optimizer for OmniCraft
//!
//! Optimizes the analyzed AST for better runtime performance.
//!
//! ## Optimizations
//! - Dead code elimination (DCE)
//! - Constant folding
//! - Inline expansion
//! - Static evaluation

pub mod const_fold;
pub mod dce;
pub mod inline;

use crate::analyzer::AnalyzedComponent;
use crate::ast::{Component, Expression, Literal, Node, Statement};
use anyhow::Result;

pub use const_fold::ConstantFolder;
pub use dce::DeadCodeEliminator;
pub use inline::InlineExpander;

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptLevel {
    /// No optimization
    None,
    /// Basic optimizations (default)
    #[default]
    Basic,
    /// Aggressive optimizations
    Aggressive,
}

/// Optimizer configuration
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    pub level: OptLevel,
    pub dead_code_elimination: bool,
    pub constant_folding: bool,
    pub inline_expansion: bool,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            level: OptLevel::Basic,
            dead_code_elimination: true,
            constant_folding: true,
            inline_expansion: true,
        }
    }
}

/// Optimizer that applies various optimizations
pub struct Optimizer {
    config: OptimizerConfig,
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            config: OptimizerConfig::default(),
        }
    }

    pub fn with_config(config: OptimizerConfig) -> Self {
        Self { config }
    }

    /// Optimize an analyzed component
    pub fn optimize(&self, analyzed: &AnalyzedComponent) -> Result<Component> {
        let mut component = analyzed.component.clone();

        if self.config.level == OptLevel::None {
            return Ok(component);
        }

        // 1. Constant folding
        if self.config.constant_folding {
            component = ConstantFolder::new().fold(&component)?;
        }

        // 2. Dead code elimination
        if self.config.dead_code_elimination {
            component = DeadCodeEliminator::new(&analyzed.dependencies).eliminate(&component)?;
        }

        // 3. Inline expansion
        if self.config.inline_expansion && self.config.level == OptLevel::Aggressive {
            component = InlineExpander::new().expand(&component)?;
        }

        Ok(component)
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimize an analyzed component with default settings
pub fn optimize(analyzed: &AnalyzedComponent) -> Result<Component> {
    Optimizer::new().optimize(analyzed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::analyze;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn compile_and_analyze(source: &str) -> AnalyzedComponent {
        let tokens = Lexer::new(source).tokenize().unwrap();
        let component = Parser::new(tokens, "test.omni").parse().unwrap();
        analyze(&component).unwrap()
    }

    #[test]
    fn test_optimizer_basic() {
        let source = r##"
<canvas width={800} height={600}>
  <circle x={400} y={300} radius={50} fill="#00d4ff" />
</canvas>
"##;
        let analyzed = compile_and_analyze(source);
        let optimized = optimize(&analyzed).unwrap();
        assert_eq!(optimized.name, analyzed.component.name);
    }

    #[test]
    fn test_optimizer_with_config() {
        let source = r##"
<script>
  const x = 1 + 2;
</script>

<canvas width={800} height={600}>
  <text x={100} y={100} content={x} />
</canvas>
"##;
        let analyzed = compile_and_analyze(source);
        
        let config = OptimizerConfig {
            level: OptLevel::Aggressive,
            ..Default::default()
        };
        
        let optimizer = Optimizer::with_config(config);
        let optimized = optimizer.optimize(&analyzed).unwrap();
        assert!(!optimized.name.is_empty());
    }
}
