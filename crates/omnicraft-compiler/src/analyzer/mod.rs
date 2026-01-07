//! Semantic Analyzer for OmniCraft
//!
//! Analyzes the AST to:
//! - Track reactive dependencies
//! - Build scope tree
//! - Infer types

pub mod dependency;
pub mod scope;
pub mod types;

use crate::ast::{Component, Expression, Node, ReactiveKind, Statement};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use tracing::{instrument, debug, trace};

pub use dependency::DependencyGraph;
pub use scope::{Scope, ScopeKind, Symbol};
pub use types::{InferredType, TypeContext};

/// Analyzed component with semantic information
#[derive(Debug, Clone)]
pub struct AnalyzedComponent {
    /// Original component
    pub component: Component,
    /// Scope tree
    pub root_scope: Scope,
    /// Dependency graph for reactive updates
    pub dependencies: DependencyGraph,
    /// Type information
    pub types: TypeContext,
}

/// Analyzer for semantic analysis
pub struct Analyzer {
    scope_stack: Vec<Scope>,
    dependencies: DependencyGraph,
    types: TypeContext,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![Scope::new(ScopeKind::Global)],
            dependencies: DependencyGraph::new(),
            types: TypeContext::new(),
        }
    }

    /// Analyze a component
    #[instrument(skip(self), fields(component = %component.name))]
    pub fn analyze(&mut self, component: &Component) -> Result<AnalyzedComponent> {
        debug!("Starting analysis");
        // 1. Analyze script section (variables, functions)
        if let Some(ref script) = component.script {
            for stmt in &script.statements {
                self.analyze_statement(stmt)?;
            }
        }

        // 2. Analyze template section (element bindings)
        self.analyze_template(&component.template)?;

        Ok(AnalyzedComponent {
            component: component.clone(),
            root_scope: self.scope_stack.first().cloned().unwrap_or_default(),
            dependencies: self.dependencies.clone(),
            types: self.types.clone(),
        })
    }

    #[instrument(skip(self))]
    fn analyze_statement(&mut self, stmt: &Statement) -> Result<()> {
        trace!("Analyzing statement");
        match stmt {
            Statement::VariableDeclaration {
                name,
                init,
                reactive,
                ..
            } => {
                // Infer type from initializer
                let inferred_type = if let Some(expr) = init {
                    self.infer_expression_type(expr)
                } else {
                    InferredType::Unknown
                };

                // Register in scope
                let symbol = Symbol {
                    name: name.clone(),
                    ty: inferred_type.clone(),
                    reactive: *reactive,
                    mutable: matches!(stmt, Statement::VariableDeclaration { kind, .. } if *kind == crate::ast::VarKind::Let),
                };
                self.current_scope_mut().add_symbol(symbol);

                // Track reactive signal
                if *reactive != ReactiveKind::None {
                    self.dependencies.add_signal(name.clone());
                }

                // Analyze initializer for dependencies
                if let Some(expr) = init {
                    self.analyze_expression(expr, Some(name))?;
                }

                self.types.set(name.clone(), inferred_type);
            }

            Statement::FunctionDeclaration { name, body, .. } => {
                // Enter function scope
                self.push_scope(ScopeKind::Function);

                for stmt in body {
                    self.analyze_statement(stmt)?;
                }

                // Exit function scope
                self.pop_scope();

                // Register function in parent scope
                let symbol = Symbol {
                    name: name.clone(),
                    ty: InferredType::Function,
                    reactive: ReactiveKind::None,
                    mutable: false,
                };
                self.current_scope_mut().add_symbol(symbol);
            }

            Statement::If { condition, then_branch, else_branch } => {
                self.analyze_expression(condition, None)?;
                
                self.push_scope(ScopeKind::Block);
                for stmt in then_branch {
                    self.analyze_statement(stmt)?;
                }
                self.pop_scope();

                if let Some(else_stmts) = else_branch {
                    self.push_scope(ScopeKind::Block);
                    for stmt in else_stmts {
                        self.analyze_statement(stmt)?;
                    }
                    self.pop_scope();
                }
            }

            Statement::Return(Some(expr)) => {
                self.analyze_expression(expr, None)?;
            }

            Statement::Expression(expr) => {
                self.analyze_expression(expr, None)?;
            }

            _ => {}
        }

        Ok(())
    }

    #[instrument(skip(self))]
    fn analyze_expression(&mut self, expr: &Expression, context: Option<&str>) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                // Check if this is a signal access
                if self.dependencies.is_signal(name) {
                    if let Some(ctx) = context {
                        self.dependencies.add_dependency(ctx.to_string(), name.clone());
                    }
                }
            }

            Expression::Call { callee, args } => {
                // Check for signal() calls
                if let Expression::Identifier(name) = callee.as_ref() {
                    if name == "signal" || name == "memo" || name == "effect" {
                        // Reactive primitive call
                        for arg in args {
                            self.analyze_expression(arg, context)?;
                        }
                        return Ok(());
                    }
                }

                // Check for signal.get() or signal() accessor
                self.analyze_expression(callee, context)?;
                for arg in args {
                    self.analyze_expression(arg, context)?;
                }
            }

            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left, context)?;
                self.analyze_expression(right, context)?;
            }

            Expression::Member { object, .. } => {
                self.analyze_expression(object, context)?;
            }

            Expression::Arrow { body, .. } => {
                self.push_scope(ScopeKind::Function);
                match body {
                    crate::ast::ArrowBody::Expression(expr) => {
                        self.analyze_expression(expr, context)?;
                    }
                    crate::ast::ArrowBody::Block(stmts) => {
                        for stmt in stmts {
                            self.analyze_statement(&stmt)?;
                        }
                    }
                }
                self.pop_scope();
            }

            Expression::Template { parts } => {
                for part in parts {
                    if let crate::ast::TemplatePart::Expression(expr) = part {
                        self.analyze_expression(expr, context)?;
                    }
                }
            }

            Expression::Ternary { condition, then_expr, else_expr } => {
                self.analyze_expression(condition, context)?;
                self.analyze_expression(then_expr, context)?;
                self.analyze_expression(else_expr, context)?;
            }

            Expression::Array(items) => {
                for item in items {
                    self.analyze_expression(item, context)?;
                }
            }

            Expression::Object(props) => {
                for (_, value) in props {
                    self.analyze_expression(value, context)?;
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn analyze_template(&mut self, template: &crate::ast::Template) -> Result<()> {
        for node in &template.children {
            self.analyze_node(node)?;
        }
        Ok(())
    }

    #[instrument(skip(self))]
    fn analyze_node(&mut self, node: &Node) -> Result<()> {
        match node {
            Node::Element { attributes, children, directives, .. } => {
                // Analyze dynamic attributes
                for attr in attributes {
                    if let crate::ast::AttributeValue::Dynamic(expr) = &attr.value {
                        self.analyze_expression(expr, Some(&attr.name))?;
                    }
                }

                // Analyze directives
                for directive in directives {
                    self.analyze_expression(&directive.value, None)?;
                }

                // Recurse into children
                for child in children {
                    self.analyze_node(child)?;
                }
            }

            Node::Text { content } => {
                self.analyze_expression(content, None)?;
            }

            Node::Expression { expr } => {
                self.analyze_expression(expr, None)?;
            }

            Node::IfBlock { condition, then_branch, else_branch } => {
                self.analyze_expression(condition, None)?;

                for child in then_branch {
                    self.analyze_node(child)?;
                }

                if let Some(else_nodes) = else_branch {
                    for child in else_nodes {
                        self.analyze_node(child)?;
                    }
                }
            }

            Node::EachBlock { expression, body, .. } => {
                self.analyze_expression(expression, None)?;

                for child in body {
                    self.analyze_node(child)?;
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn infer_expression_type(&self, expr: &Expression) -> InferredType {
        match expr {
            Expression::Literal(lit) => match lit {
                crate::ast::Literal::String(_) => InferredType::String,
                crate::ast::Literal::Number(_) => InferredType::Number,
                crate::ast::Literal::Boolean(_) => InferredType::Boolean,
                crate::ast::Literal::Null => InferredType::Null,
            },
            Expression::Call { callee, args } => {
                if let Expression::Identifier(name) = callee.as_ref() {
                    if name == "signal" {
                        if let Some(first_arg) = args.first() {
                            return InferredType::Signal(Box::new(self.infer_expression_type(first_arg)));
                        }
                    }
                    if name == "memo" {
                        return InferredType::Memo;
                    }
                }
                InferredType::Unknown
            }
            Expression::Arrow { .. } => InferredType::Function,
            Expression::Array(_) => InferredType::Array,
            Expression::Object(_) => InferredType::Object,
            _ => InferredType::Unknown,
        }
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scope_stack.last_mut().expect("scope stack is empty")
    }

    fn push_scope(&mut self, kind: ScopeKind) {
        self.scope_stack.push(Scope::new(kind));
    }

    fn pop_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Analyze a component
pub fn analyze(component: &Component) -> Result<AnalyzedComponent> {
    Analyzer::new().analyze(component)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse(source: &str) -> Component {
        let tokens = Lexer::new(source).tokenize().unwrap();
        Parser::new(tokens, "test.omni").parse().unwrap()
    }

    #[test]
    fn test_analyze_simple() {
        let source = r##"
<canvas width={800} height={600}>
  <circle x={400} y={300} radius={50} fill="#00d4ff" />
</canvas>
"##;
        let component = parse(source);
        let analyzed = analyze(&component).unwrap();
        assert!(analyzed.dependencies.signals.is_empty());
    }

    #[test]
    fn test_analyze_with_signal() {
        let source = r##"
<script>
  const count = signal(0);
</script>

<canvas width={800} height={600}>
  <text x={400} y={300} content={count()} fill="#ffffff" />
</canvas>
"##;
        let component = parse(source);
        let analyzed = analyze(&component).unwrap();
        assert!(analyzed.dependencies.is_signal("count"));
    }
}
