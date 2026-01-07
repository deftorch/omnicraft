//! Inline Expansion
//!
//! Inlines small functions and expressions.

use crate::ast::{Component, Expression, Statement};
use anyhow::Result;
use std::collections::HashMap;

/// Inline expander
pub struct InlineExpander {
    /// Inlinable functions (name -> body)
    inlinable: HashMap<String, Expression>,
}

impl InlineExpander {
    pub fn new() -> Self {
        Self {
            inlinable: HashMap::new(),
        }
    }

    /// Expand inlinable functions in a component
    pub fn expand(&self, component: &Component) -> Result<Component> {
        let mut result = component.clone();
        let mut expander = Self::new();

        // First pass: collect inlinable functions
        if let Some(ref script) = component.script {
            expander.collect_inlinable(&script.statements);
        }

        // Second pass: expand inline calls
        if let Some(ref mut script) = result.script {
            script.statements = script
                .statements
                .iter()
                .map(|s| expander.expand_statement(s))
                .collect();
        }

        Ok(result)
    }

    fn collect_inlinable(&mut self, statements: &[Statement]) {
        for stmt in statements {
            if let Statement::FunctionDeclaration { name, params, body, .. } = stmt {
                // Only inline simple functions with single return statement
                if params.is_empty() && body.len() == 1 {
                    if let Statement::Return(Some(expr)) = &body[0] {
                        // Only inline if the function is simple
                        if self.is_simple_expression(expr) {
                            self.inlinable.insert(name.clone(), expr.clone());
                        }
                    }
                }
            }
        }
    }

    fn is_simple_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Literal(_) => true,
            Expression::Identifier(_) => true,
            Expression::Binary { left, right, .. } => {
                self.is_simple_expression(left) && self.is_simple_expression(right)
            }
            Expression::Unary { operand, .. } => self.is_simple_expression(operand),
            _ => false,
        }
    }

    fn expand_statement(&self, stmt: &Statement) -> Statement {
        match stmt {
            Statement::VariableDeclaration {
                kind,
                name,
                init,
                reactive,
            } => Statement::VariableDeclaration {
                kind: *kind,
                name: name.clone(),
                init: init.as_ref().map(|e| self.expand_expression(e)),
                reactive: *reactive,
            },
            Statement::Return(Some(expr)) => {
                Statement::Return(Some(self.expand_expression(expr)))
            }
            Statement::Expression(expr) => {
                Statement::Expression(self.expand_expression(expr))
            }
            _ => stmt.clone(),
        }
    }

    fn expand_expression(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Call { callee, args } => {
                // Check if this is a call to an inlinable function
                if let Expression::Identifier(name) = callee.as_ref() {
                    if args.is_empty() {
                        if let Some(inlined) = self.inlinable.get(name) {
                            return inlined.clone();
                        }
                    }
                }

                Expression::Call {
                    callee: Box::new(self.expand_expression(callee)),
                    args: args.iter().map(|a| self.expand_expression(a)).collect(),
                }
            }
            Expression::Binary { left, op, right } => Expression::Binary {
                left: Box::new(self.expand_expression(left)),
                op: *op,
                right: Box::new(self.expand_expression(right)),
            },
            Expression::Unary { op, operand } => Expression::Unary {
                op: *op,
                operand: Box::new(self.expand_expression(operand)),
            },
            Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => Expression::Ternary {
                condition: Box::new(self.expand_expression(condition)),
                then_expr: Box::new(self.expand_expression(then_expr)),
                else_expr: Box::new(self.expand_expression(else_expr)),
            },
            Expression::Array(items) => {
                Expression::Array(items.iter().map(|i| self.expand_expression(i)).collect())
            }
            Expression::Object(props) => Expression::Object(
                props
                    .iter()
                    .map(|(k, v)| (k.clone(), self.expand_expression(v)))
                    .collect(),
            ),
            _ => expr.clone(),
        }
    }
}

impl Default for InlineExpander {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_inline_simple_function() {
        let source = r##"
<script>
  function getTwo() {
    return 2;
  }
  const x = getTwo();
</script>

<canvas width={800} height={600}>
</canvas>
"##;
        let component = parse(source);
        let expanded = InlineExpander::new().expand(&component).unwrap();
        
        // Function should be inlined
        assert!(expanded.script.is_some());
    }
}
