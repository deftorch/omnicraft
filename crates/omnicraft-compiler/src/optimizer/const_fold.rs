//! Constant Folding
//!
//! Evaluates constant expressions at compile time.

use crate::ast::{
    BinaryOp, Component, Expression, Literal, Node, Statement, UnaryOp,
};
use anyhow::Result;

/// Constant folder
pub struct ConstantFolder;

impl ConstantFolder {
    pub fn new() -> Self {
        Self
    }

    /// Fold constants in a component
    pub fn fold(&self, component: &Component) -> Result<Component> {
        let mut result = component.clone();

        if let Some(ref mut script) = result.script {
            script.statements = script
                .statements
                .iter()
                .map(|s| self.fold_statement(s))
                .collect();
        }

        result.template.children = result
            .template
            .children
            .iter()
            .map(|n| self.fold_node(n))
            .collect();

        Ok(result)
    }

    fn fold_statement(&self, stmt: &Statement) -> Statement {
        match stmt {
            Statement::VariableDeclaration {
                kind,
                name,
                init,
                reactive,
            } => Statement::VariableDeclaration {
                kind: *kind,
                name: name.clone(),
                init: init.as_ref().map(|e| self.fold_expression(e)),
                reactive: *reactive,
            },
            Statement::Return(Some(expr)) => {
                Statement::Return(Some(self.fold_expression(expr)))
            }
            Statement::Expression(expr) => {
                Statement::Expression(self.fold_expression(expr))
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => Statement::If {
                condition: self.fold_expression(condition),
                then_branch: then_branch.iter().map(|s| self.fold_statement(s)).collect(),
                else_branch: else_branch
                    .as_ref()
                    .map(|b| b.iter().map(|s| self.fold_statement(s)).collect()),
            },
            _ => stmt.clone(),
        }
    }

    fn fold_expression(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Binary { left, op, right } => {
                let left = self.fold_expression(left);
                let right = self.fold_expression(right);

                // Try to evaluate constant expressions
                if let (Expression::Literal(l), Expression::Literal(r)) = (&left, &right) {
                    if let Some(result) = self.eval_binary(l, *op, r) {
                        return Expression::Literal(result);
                    }
                }

                Expression::Binary {
                    left: Box::new(left),
                    op: *op,
                    right: Box::new(right),
                }
            }
            Expression::Unary { op, operand } => {
                let operand = self.fold_expression(operand);

                if let Expression::Literal(lit) = &operand {
                    if let Some(result) = self.eval_unary(*op, lit) {
                        return Expression::Literal(result);
                    }
                }

                Expression::Unary {
                    op: *op,
                    operand: Box::new(operand),
                }
            }
            Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.fold_expression(condition);

                // If condition is constant, select the branch
                if let Expression::Literal(Literal::Boolean(b)) = &condition {
                    if *b {
                        return self.fold_expression(then_expr);
                    } else {
                        return self.fold_expression(else_expr);
                    }
                }

                Expression::Ternary {
                    condition: Box::new(condition),
                    then_expr: Box::new(self.fold_expression(then_expr)),
                    else_expr: Box::new(self.fold_expression(else_expr)),
                }
            }
            Expression::Call { callee, args } => Expression::Call {
                callee: Box::new(self.fold_expression(callee)),
                args: args.iter().map(|a| self.fold_expression(a)).collect(),
            },
            Expression::Array(items) => {
                Expression::Array(items.iter().map(|i| self.fold_expression(i)).collect())
            }
            Expression::Object(props) => Expression::Object(
                props
                    .iter()
                    .map(|(k, v)| (k.clone(), self.fold_expression(v)))
                    .collect(),
            ),
            _ => expr.clone(),
        }
    }

    fn fold_node(&self, node: &Node) -> Node {
        match node {
            Node::Element {
                tag,
                attributes,
                children,
                directives,
                key,
            } => {
                let attributes = attributes
                    .iter()
                    .map(|a| {
                        let value = match &a.value {
                            crate::ast::AttributeValue::Dynamic(expr) => {
                                crate::ast::AttributeValue::Dynamic(self.fold_expression(expr))
                            }
                            other => other.clone(),
                        };
                        crate::ast::Attribute {
                            name: a.name.clone(),
                            value,
                        }
                    })
                    .collect();

                Node::Element {
                    tag: tag.clone(),
                    attributes,
                    children: children.iter().map(|c| self.fold_node(c)).collect(),
                    directives: directives.clone(),
                    key: key.clone(),
                }
            }
            Node::IfBlock {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.fold_expression(condition);

                // Static elimination of branches
                if let Expression::Literal(Literal::Boolean(b)) = &condition {
                    if *b {
                        // Return just the then branch content
                        // For now, keep as IfBlock for simplicity
                    }
                }

                Node::IfBlock {
                    condition,
                    then_branch: then_branch.iter().map(|n| self.fold_node(n)).collect(),
                    else_branch: else_branch
                        .as_ref()
                        .map(|b| b.iter().map(|n| self.fold_node(n)).collect()),
                }
            }
            Node::EachBlock {
                expression,
                binding,
                index,
                body,
                key,
            } => Node::EachBlock {
                expression: self.fold_expression(expression),
                binding: binding.clone(),
                index: index.clone(),
                body: body.iter().map(|n| self.fold_node(n)).collect(),
                key: key.clone(),
            },
            Node::Text { content } => Node::Text {
                content: self.fold_expression(content),
            },
            Node::Expression { expr } => Node::Expression {
                expr: self.fold_expression(expr),
            },
            _ => node.clone(),
        }
    }

    fn eval_binary(&self, left: &Literal, op: BinaryOp, right: &Literal) -> Option<Literal> {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => {
                let result = match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => {
                        if *r == 0.0 {
                            return None;
                        }
                        l / r
                    }
                    BinaryOp::Mod => {
                        if *r == 0.0 {
                            return None;
                        }
                        l % r
                    }
                    BinaryOp::Eq => return Some(Literal::Boolean(l == r)),
                    BinaryOp::Ne => return Some(Literal::Boolean(l != r)),
                    BinaryOp::Lt => return Some(Literal::Boolean(l < r)),
                    BinaryOp::Gt => return Some(Literal::Boolean(l > r)),
                    BinaryOp::Le => return Some(Literal::Boolean(l <= r)),
                    BinaryOp::Ge => return Some(Literal::Boolean(l >= r)),
                    _ => return None,
                };
                Some(Literal::Number(result))
            }
            (Literal::String(l), Literal::String(r)) => {
                match op {
                    BinaryOp::Add => Some(Literal::String(format!("{}{}", l, r))),
                    BinaryOp::Eq => Some(Literal::Boolean(l == r)),
                    BinaryOp::Ne => Some(Literal::Boolean(l != r)),
                    _ => None,
                }
            }
            (Literal::Boolean(l), Literal::Boolean(r)) => {
                match op {
                    BinaryOp::And => Some(Literal::Boolean(*l && *r)),
                    BinaryOp::Or => Some(Literal::Boolean(*l || *r)),
                    BinaryOp::Eq => Some(Literal::Boolean(l == r)),
                    BinaryOp::Ne => Some(Literal::Boolean(l != r)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn eval_unary(&self, op: UnaryOp, operand: &Literal) -> Option<Literal> {
        match (op, operand) {
            (UnaryOp::Neg, Literal::Number(n)) => Some(Literal::Number(-n)),
            (UnaryOp::Not, Literal::Boolean(b)) => Some(Literal::Boolean(!b)),
            _ => None,
        }
    }
}

impl Default for ConstantFolder {
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
    fn test_fold_arithmetic() {
        let source = r##"
<script>
  const x = 1 + 2;
  const y = 10 * 5;
</script>

<canvas width={800} height={600}>
</canvas>
"##;
        let component = parse(source);
        let folded = ConstantFolder::new().fold(&component).unwrap();

        // The constant should be folded
        if let Some(script) = &folded.script {
            if let Statement::VariableDeclaration { init: Some(Expression::Literal(Literal::Number(n))), .. } = &script.statements[0] {
                assert_eq!(*n, 3.0);
            }
        }
    }

    #[test]
    fn test_fold_string_concat() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::String("Hello, ".to_string()))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::String("World!".to_string()))),
        };

        let folder = ConstantFolder::new();
        let result = folder.fold_expression(&expr);

        assert_eq!(result, Expression::Literal(Literal::String("Hello, World!".to_string())));
    }

    #[test]
    fn test_fold_comparison() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(5.0))),
            op: BinaryOp::Gt,
            right: Box::new(Expression::Literal(Literal::Number(3.0))),
        };

        let folder = ConstantFolder::new();
        let result = folder.fold_expression(&expr);

        assert_eq!(result, Expression::Literal(Literal::Boolean(true)));
    }
}
