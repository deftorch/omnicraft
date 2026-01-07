//! Dead Code Elimination (DCE)
//!
//! Removes unused variables, functions, and expressions.

use crate::analyzer::DependencyGraph;
use crate::ast::{Component, Node, Script, Statement};
use anyhow::Result;

/// Dead code eliminator
pub struct DeadCodeEliminator<'a> {
    dependencies: &'a DependencyGraph,
}

impl<'a> DeadCodeEliminator<'a> {
    pub fn new(dependencies: &'a DependencyGraph) -> Self {
        Self { dependencies }
    }

    /// Eliminate dead code from a component
    pub fn eliminate(&self, component: &Component) -> Result<Component> {
        let mut result = component.clone();

        if let Some(ref mut script) = result.script {
            script.statements = self.eliminate_statements(&script.statements);
        }

        result.template.children = self.eliminate_nodes(&result.template.children);

        Ok(result)
    }

    fn eliminate_statements(&self, statements: &[Statement]) -> Vec<Statement> {
        statements
            .iter()
            .filter(|stmt| !self.is_dead_statement(stmt))
            .cloned()
            .collect()
    }

    fn is_dead_statement(&self, stmt: &Statement) -> bool {
        match stmt {
            Statement::VariableDeclaration { name, reactive, .. } => {
                // Keep all reactive variables
                if *reactive != crate::ast::ReactiveKind::None {
                    return false;
                }

                // Check if variable is used
                // For now, keep all variables (conservative approach)
                // Full DCE would check if the variable is referenced anywhere
                false
            }
            Statement::FunctionDeclaration { name, .. } => {
                // Keep functions that are used as event handlers
                // For now, keep all functions
                false
            }
            _ => false,
        }
    }

    fn eliminate_nodes(&self, nodes: &[Node]) -> Vec<Node> {
        nodes
            .iter()
            .filter_map(|node| self.eliminate_node(node))
            .collect()
    }

    fn eliminate_node(&self, node: &Node) -> Option<Node> {
        match node {
            Node::Element {
                tag,
                attributes,
                children,
                directives,
                key,
            } => {
                let children = self.eliminate_nodes(children);
                Some(Node::Element {
                    tag: tag.clone(),
                    attributes: attributes.clone(),
                    children,
                    directives: directives.clone(),
                    key: key.clone(),
                })
            }
            Node::IfBlock {
                condition,
                then_branch,
                else_branch,
            } => {
                // TODO: Eliminate static false conditions
                let then_branch = self.eliminate_nodes(then_branch);
                let else_branch = else_branch.as_ref().map(|b| self.eliminate_nodes(b));
                Some(Node::IfBlock {
                    condition: condition.clone(),
                    then_branch,
                    else_branch,
                })
            }
            Node::EachBlock {
                expression,
                binding,
                index,
                body,
                key,
            } => {
                let body = self.eliminate_nodes(body);
                Some(Node::EachBlock {
                    expression: expression.clone(),
                    binding: binding.clone(),
                    index: index.clone(),
                    body,
                    key: key.clone(),
                })
            }
            _ => Some(node.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{analyze, DependencyGraph};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_dce_preserves_reactive() {
        let deps = DependencyGraph::new();
        let dce = DeadCodeEliminator::new(&deps);

        let source = r##"
<script>
  const count = signal(0);
</script>

<canvas width={800} height={600}>
  <text x={100} y={100} content={count()} />
</canvas>
"##;
        let tokens = Lexer::new(source).tokenize().unwrap();
        let component = Parser::new(tokens, "test.omni").parse().unwrap();

        let result = dce.eliminate(&component).unwrap();
        assert!(result.script.is_some());
        assert!(!result.script.unwrap().statements.is_empty());
    }
}
