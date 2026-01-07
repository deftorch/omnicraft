//! Parser for `.omni` files
//!
//! Recursive descent parser that converts tokens into AST.

use crate::ast::*;
use crate::lexer::{Token, TokenKind};
use thiserror::Error;
use tracing::{instrument, trace};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found} at position {pos}")]
    UnexpectedToken {
        expected: String,
        found: String,
        pos: usize,
    },

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Invalid element tag: {0}")]
    InvalidElementTag(String),

    #[error("Missing closing tag for <{0}>")]
    MissingClosingTag(String),

    #[error("Invalid attribute syntax")]
    InvalidAttribute,

    #[error("Invalid expression")]
    InvalidExpression,
}

type ParseResult<T> = Result<T, ParseError>;

/// Parser for `.omni` files
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    file_name: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file_name: &str) -> Self {
        Self {
            tokens,
            pos: 0,
            file_name: file_name.to_string(),
        }
    }

    /// Parse the entire component
    #[instrument(skip(self), fields(file = %self.file_name))]
    pub fn parse(&mut self) -> ParseResult<Component> {
        trace!("Starting parse");
        let name = self.infer_component_name();

        let script = self.parse_script_section()?;
        let (canvas, children) = self.parse_canvas_section()?;
        let style = self.parse_style_section()?;
        trace!("Parse complete");

        Ok(Component {
            name,
            script,
            template: Template { canvas, children },
            style,
            metadata: ComponentMetadata {
                file_path: self.file_name.clone(),
                hash: String::new(),
                exports: Vec::new(),
            },
        })
    }

    fn infer_component_name(&self) -> String {
        self.file_name
            .split('/')
            .last()
            .unwrap_or("Component")
            .trim_end_matches(".omni")
            .to_string()
    }

    // ========================================================================
    // Script Section
    // ========================================================================

    #[instrument(skip(self))]
    fn parse_script_section(&mut self) -> ParseResult<Option<Script>> {
        if !self.check_sequence(&[TokenKind::LessThan, TokenKind::Script]) {
            return Ok(None);
        }

        self.consume(TokenKind::LessThan)?;
        self.consume(TokenKind::Script)?;
        self.consume(TokenKind::GreaterThan)?;

        let mut statements = Vec::new();
        while !self.check_sequence(&[TokenKind::ClosingTag, TokenKind::Script]) {
            if self.is_at_end() {
                return Err(ParseError::MissingClosingTag("script".to_string()));
            }
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenKind::ClosingTag)?;
        self.consume(TokenKind::Script)?;
        self.consume(TokenKind::GreaterThan)?;

        Ok(Some(Script {
            statements,
            imports: Vec::new(),
            exports: Vec::new(),
        }))
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.peek_kind() {
            Some(TokenKind::Const) | Some(TokenKind::Let) => self.parse_variable_declaration(),
            Some(TokenKind::Function) => self.parse_function_declaration(),
            Some(TokenKind::If) => self.parse_if_statement(),
            Some(TokenKind::Return) => self.parse_return_statement(),
            _ => {
                let expr = self.parse_expression()?;
                self.consume_if(TokenKind::Semicolon);
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> ParseResult<Statement> {
        let kind = if self.consume_if(TokenKind::Const) {
            VarKind::Const
        } else {
            self.consume(TokenKind::Let)?;
            VarKind::Let
        };

        let name = self.consume_identifier()?;
        self.consume(TokenKind::Equals)?;

        let init = self.parse_expression()?;

        // Check for reactive types
        let reactive = self.detect_reactive_kind(&init);

        self.consume_if(TokenKind::Semicolon);

        Ok(Statement::VariableDeclaration {
            kind,
            name,
            init: Some(init),
            reactive,
        })
    }

    fn detect_reactive_kind(&self, expr: &Expression) -> ReactiveKind {
        if let Expression::Call { callee, .. } = expr {
            if let Expression::Identifier(name) = callee.as_ref() {
                return match name.as_str() {
                    "signal" => ReactiveKind::Signal,
                    "memo" => ReactiveKind::Memo,
                    "effect" => ReactiveKind::Effect,
                    _ => ReactiveKind::None,
                };
            }
        }
        ReactiveKind::None
    }

    fn parse_function_declaration(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Function)?;

        let name = self.consume_identifier()?;

        self.consume(TokenKind::LeftParen)?;
        let params = self.parse_parameter_list()?;
        self.consume(TokenKind::RightParen)?;

        self.consume(TokenKind::LeftBrace)?;
        let body = self.parse_statement_block()?;
        self.consume(TokenKind::RightBrace)?;

        Ok(Statement::FunctionDeclaration {
            name,
            params,
            body,
            is_async: false,
        })
    }

    fn parse_parameter_list(&mut self) -> ParseResult<Vec<Parameter>> {
        let mut params = Vec::new();

        while !self.check(TokenKind::RightParen) {
            let name = self.consume_identifier()?;
            params.push(Parameter {
                name,
                ty: None,
                default: None,
            });

            if !self.consume_if(TokenKind::Comma) {
                break;
            }
        }

        Ok(params)
    }

    fn parse_statement_block(&mut self) -> ParseResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_if_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::If)?;
        self.consume(TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(TokenKind::RightParen)?;

        self.consume(TokenKind::LeftBrace)?;
        let then_branch = self.parse_statement_block()?;
        self.consume(TokenKind::RightBrace)?;

        let else_branch = if self.consume_if(TokenKind::Else) {
            self.consume(TokenKind::LeftBrace)?;
            let branch = self.parse_statement_block()?;
            self.consume(TokenKind::RightBrace)?;
            Some(branch)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Return)?;

        let value = if !self.check(TokenKind::Semicolon) && !self.check(TokenKind::RightBrace) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume_if(TokenKind::Semicolon);

        Ok(Statement::Return(value))
    }

    // ========================================================================
    // Expressions
    // ========================================================================

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> ParseResult<Expression> {
        let condition = self.parse_or()?;

        if self.consume_if(TokenKind::Question) {
            let then_expr = self.parse_expression()?;
            self.consume(TokenKind::Colon)?;
            let else_expr = self.parse_expression()?;

            return Ok(Expression::Ternary {
                condition: Box::new(condition),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            });
        }

        Ok(condition)
    }

    fn parse_or(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_and()?;

        while self.consume_if(TokenKind::Or) {
            let right = self.parse_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_equality()?;

        while self.consume_if(TokenKind::And) {
            let right = self.parse_equality()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_comparison()?;

        loop {
            let op = if self.consume_if(TokenKind::DoubleEquals) {
                BinaryOp::Eq
            } else if self.consume_if(TokenKind::NotEquals) {
                BinaryOp::Ne
            } else {
                break;
            };

            let right = self.parse_comparison()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_additive()?;

        loop {
            let op = if self.consume_if(TokenKind::LessThan) {
                BinaryOp::Lt
            } else if self.consume_if(TokenKind::GreaterThan) {
                BinaryOp::Gt
            } else if self.consume_if(TokenKind::LessEquals) {
                BinaryOp::Le
            } else if self.consume_if(TokenKind::GreaterEquals) {
                BinaryOp::Ge
            } else {
                break;
            };

            let right = self.parse_additive()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = if self.consume_if(TokenKind::Plus) {
                BinaryOp::Add
            } else if self.consume_if(TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.parse_multiplicative()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_unary()?;

        loop {
            let op = if self.consume_if(TokenKind::Star) {
                BinaryOp::Mul
            } else if self.consume_if(TokenKind::Slash) {
                BinaryOp::Div
            } else if self.consume_if(TokenKind::Percent) {
                BinaryOp::Mod
            } else {
                break;
            };

            let right = self.parse_unary()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> ParseResult<Expression> {
        if self.consume_if(TokenKind::Not) {
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            });
        }

        if self.consume_if(TokenKind::Minus) {
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            });
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.consume_if(TokenKind::LeftParen) {
                let args = self.parse_argument_list()?;
                self.consume(TokenKind::RightParen)?;
                expr = Expression::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.consume_if(TokenKind::Dot) {
                let property = self.consume_identifier()?;
                expr = Expression::Member {
                    object: Box::new(expr),
                    property,
                    computed: false,
                };
            } else if self.consume_if(TokenKind::LeftBracket) {
                let index = self.parse_expression()?;
                self.consume(TokenKind::RightBracket)?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_argument_list(&mut self) -> ParseResult<Vec<Expression>> {
        let mut args = Vec::new();

        while !self.check(TokenKind::RightParen) {
            args.push(self.parse_expression()?);
            if !self.consume_if(TokenKind::Comma) {
                break;
            }
        }

        Ok(args)
    }

    fn parse_primary(&mut self) -> ParseResult<Expression> {
        // Number
        if let Some(TokenKind::Number(n)) = self.peek_kind() {
            let n = n;
            self.advance();
            return Ok(Expression::Literal(Literal::Number(n)));
        }

        // String
        if let Some(TokenKind::StringLiteral(s)) = self.peek_kind() {
            let s = s.clone();
            self.advance();
            return Ok(Expression::Literal(Literal::String(s)));
        }
        if let Some(TokenKind::StringLiteralSingle(s)) = self.peek_kind() {
            let s = s.clone();
            self.advance();
            return Ok(Expression::Literal(Literal::String(s)));
        }

        // Boolean
        if self.consume_if(TokenKind::True) {
            return Ok(Expression::Literal(Literal::Boolean(true)));
        }
        if self.consume_if(TokenKind::False) {
            return Ok(Expression::Literal(Literal::Boolean(false)));
        }

        // Null
        if self.consume_if(TokenKind::Null) {
            return Ok(Expression::Literal(Literal::Null));
        }

        // Identifier or arrow function
        if let Some(TokenKind::Identifier(name)) = self.peek_kind() {
            let name = name.clone();
            self.advance();
            return Ok(Expression::Identifier(name));
        }

        // Keywords as identifiers (signal, memo, etc.)
        if self.check(TokenKind::Signal)
            || self.check(TokenKind::Memo)
            || self.check(TokenKind::Effect)
        {
            let name = self.peek().map(|t| t.text.clone()).unwrap_or_default();
            self.advance();
            return Ok(Expression::Identifier(name));
        }

        // Array
        if self.consume_if(TokenKind::LeftBracket) {
            let mut elements = Vec::new();
            while !self.check(TokenKind::RightBracket) {
                elements.push(self.parse_expression()?);
                if !self.consume_if(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(TokenKind::RightBracket)?;
            return Ok(Expression::Array(elements));
        }

        // Parenthesized expression or arrow function
        if self.consume_if(TokenKind::LeftParen) {
            // Check if it's an arrow function
            if self.check(TokenKind::RightParen) {
                self.consume(TokenKind::RightParen)?;
                self.consume(TokenKind::Arrow)?;
                let body = self.parse_arrow_body()?;
                return Ok(Expression::Arrow {
                    params: Vec::new(),
                    body,
                });
            }

            let expr = self.parse_expression()?;

            // Check if it's an arrow function with params
            if self.consume_if(TokenKind::RightParen) {
                if self.consume_if(TokenKind::Arrow) {
                    // It's an arrow function
                    let params = self.expr_to_params(expr)?;
                    let body = self.parse_arrow_body()?;
                    return Ok(Expression::Arrow { params, body });
                }
                return Ok(expr);
            }

            // Multiple params for arrow
            if self.consume_if(TokenKind::Comma) {
                let mut params = vec![self.expr_to_param(expr)?];
                while !self.check(TokenKind::RightParen) {
                    let param_expr = self.parse_expression()?;
                    params.push(self.expr_to_param(param_expr)?);
                    if !self.consume_if(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(TokenKind::RightParen)?;
                self.consume(TokenKind::Arrow)?;
                let body = self.parse_arrow_body()?;
                return Ok(Expression::Arrow { params, body });
            }

            self.consume(TokenKind::RightParen)?;
            return Ok(expr);
        }

        Err(ParseError::InvalidExpression)
    }

    fn parse_arrow_body(&mut self) -> ParseResult<ArrowBody> {
        if self.consume_if(TokenKind::LeftBrace) {
            let statements = self.parse_statement_block()?;
            self.consume(TokenKind::RightBrace)?;
            Ok(ArrowBody::Block(statements))
        } else {
            let expr = self.parse_expression()?;
            Ok(ArrowBody::Expression(Box::new(expr)))
        }
    }

    fn expr_to_params(&self, expr: Expression) -> ParseResult<Vec<Parameter>> {
        Ok(vec![self.expr_to_param(expr)?])
    }

    fn expr_to_param(&self, expr: Expression) -> ParseResult<Parameter> {
        if let Expression::Identifier(name) = expr {
            Ok(Parameter {
                name,
                ty: None,
                default: None,
            })
        } else {
            Err(ParseError::InvalidExpression)
        }
    }

    // ========================================================================
    // Canvas Section
    // ========================================================================

    #[instrument(skip(self))]
    fn parse_canvas_section(&mut self) -> ParseResult<(CanvasNode, Vec<Node>)> {
        if !self.check_sequence(&[TokenKind::LessThan, TokenKind::Canvas]) {
            // No canvas, create default
            return Ok((CanvasNode::default(), Vec::new()));
        }

        self.consume(TokenKind::LessThan)?;
        self.consume(TokenKind::Canvas)?;

        let mut canvas = CanvasNode::default();

        // Parse canvas attributes
        while !self.check(TokenKind::GreaterThan) && !self.check(TokenKind::SelfClosing) {
            let (name, value) = self.parse_attribute_pair()?;
            match name.as_str() {
                "width" => canvas.width = Some(self.attr_value_to_expr(value)?),
                "height" => canvas.height = Some(self.attr_value_to_expr(value)?),
                "background" => canvas.background = Some(self.attr_value_to_expr(value)?),
                _ => {}
            }
        }

        if self.consume_if(TokenKind::SelfClosing) {
            return Ok((canvas, Vec::new()));
        }

        self.consume(TokenKind::GreaterThan)?;

        // Parse children
        let mut children = Vec::new();
        while !self.check_sequence(&[TokenKind::ClosingTag, TokenKind::Canvas]) {
            if self.is_at_end() {
                return Err(ParseError::MissingClosingTag("canvas".to_string()));
            }
            children.push(self.parse_node()?);
        }

        self.consume(TokenKind::ClosingTag)?;
        self.consume(TokenKind::Canvas)?;
        self.consume(TokenKind::GreaterThan)?;

        Ok((canvas, children))
    }

    fn parse_node(&mut self) -> ParseResult<Node> {
        self.consume(TokenKind::LessThan)?;

        // Get tag name
        let tag_name = self.consume_element_tag()?;
        let tag = ElementTag::from_str(&tag_name)
            .ok_or_else(|| ParseError::InvalidElementTag(tag_name.clone()))?;

        // Parse attributes
        let mut attributes = Vec::new();
        let mut directives = Vec::new();

        while !self.check(TokenKind::GreaterThan) && !self.check(TokenKind::SelfClosing) {
            let (name, value) = self.parse_attribute_pair()?;

            // Check for directives
            if name.starts_with('@') || name.starts_with("on:") {
                let event_name = name
                    .strip_prefix('@')
                    .or_else(|| name.strip_prefix("on:"))
                    .unwrap_or(&name);
                directives.push(Directive {
                    name: DirectiveName::On,
                    arg: Some(event_name.to_string()),
                    value: self.attr_value_to_expr(value)?,
                    modifiers: Vec::new(),
                });
            } else if name.starts_with(':') || name.starts_with("bind:") {
                let prop_name = name
                    .strip_prefix(':')
                    .or_else(|| name.strip_prefix("bind:"))
                    .unwrap_or(&name);
                directives.push(Directive {
                    name: DirectiveName::Bind,
                    arg: Some(prop_name.to_string()),
                    value: self.attr_value_to_expr(value)?,
                    modifiers: Vec::new(),
                });
            } else {
                attributes.push(Attribute {
                    name,
                    value: self.to_attribute_value(value)?,
                });
            }
        }

        // Self-closing or with children
        if self.consume_if(TokenKind::SelfClosing) {
            return Ok(Node::Element {
                tag,
                attributes,
                children: Vec::new(),
                directives,
                key: None,
            });
        }

        self.consume(TokenKind::GreaterThan)?;

        // Parse children
        let mut children = Vec::new();
        while !self.is_closing_tag(&tag_name) {
            if self.is_at_end() {
                return Err(ParseError::MissingClosingTag(tag_name));
            }

            // Check for text or expression
            if self.check(TokenKind::LeftBrace) {
                self.consume(TokenKind::LeftBrace)?;
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RightBrace)?;
                children.push(Node::Expression { expr });
            } else if self.check(TokenKind::LessThan) {
                if self.check_sequence(&[TokenKind::LessThan, TokenKind::ClosingTag]) {
                    break;
                }
                children.push(self.parse_node()?);
            } else {
                // Skip whitespace/unknown
                self.advance();
            }
        }

        // Consume closing tag
        self.consume(TokenKind::ClosingTag)?;
        self.consume_element_tag()?; // consume tag name
        self.consume(TokenKind::GreaterThan)?;

        Ok(Node::Element {
            tag,
            attributes,
            children,
            directives,
            key: None,
        })
    }

    fn parse_attribute_pair(&mut self) -> ParseResult<(String, AttrValueRaw)> {
        let name = self.consume_any_identifier()?;

        if !self.consume_if(TokenKind::Equals) {
            // Boolean attribute
            return Ok((name, AttrValueRaw::Boolean(true)));
        }

        // Check for expression
        if self.consume_if(TokenKind::LeftBrace) {
            let expr = self.parse_expression()?;
            self.consume(TokenKind::RightBrace)?;
            return Ok((name, AttrValueRaw::Expression(expr)));
        }

        // String literal
        if let Some(TokenKind::StringLiteral(s)) = self.peek_kind() {
            let s = s.clone();
            self.advance();
            return Ok((name, AttrValueRaw::String(s)));
        }
        if let Some(TokenKind::StringLiteralSingle(s)) = self.peek_kind() {
            let s = s.clone();
            self.advance();
            return Ok((name, AttrValueRaw::String(s)));
        }

        Err(ParseError::InvalidAttribute)
    }

    fn attr_value_to_expr(&self, value: AttrValueRaw) -> ParseResult<Expression> {
        match value {
            AttrValueRaw::String(s) => Ok(Expression::Literal(Literal::String(s))),
            AttrValueRaw::Expression(e) => Ok(e),
            AttrValueRaw::Boolean(b) => Ok(Expression::Literal(Literal::Boolean(b))),
        }
    }

    fn to_attribute_value(&self, value: AttrValueRaw) -> ParseResult<AttributeValue> {
        match value {
            AttrValueRaw::String(s) => Ok(AttributeValue::Static(Literal::String(s))),
            AttrValueRaw::Expression(e) => Ok(AttributeValue::Dynamic(e)),
            AttrValueRaw::Boolean(b) => Ok(AttributeValue::Boolean(b)),
        }
    }

    fn is_closing_tag(&self, tag_name: &str) -> bool {
        if !self.check_sequence(&[TokenKind::ClosingTag]) {
            return false;
        }

        // Look ahead to check tag name
        if let Some(token) = self.tokens.get(self.pos + 1) {
            let matches = match &token.kind {
                TokenKind::Circle => tag_name == "circle",
                TokenKind::Rectangle | TokenKind::Rect => {
                    tag_name == "rectangle" || tag_name == "rect"
                }
                TokenKind::Ellipse => tag_name == "ellipse",
                TokenKind::Line => tag_name == "line",
                TokenKind::Path => tag_name == "path",
                TokenKind::Polygon => tag_name == "polygon",
                TokenKind::Text => tag_name == "text",
                TokenKind::Image => tag_name == "image",
                TokenKind::Group => tag_name == "group",
                TokenKind::Identifier(s) => s == tag_name,
                _ => false,
            };
            return matches;
        }

        false
    }

    // ========================================================================
    // Style Section
    // ========================================================================

    fn parse_style_section(&mut self) -> ParseResult<Option<Style>> {
        if !self.check_sequence(&[TokenKind::LessThan, TokenKind::Style]) {
            return Ok(None);
        }

        self.consume(TokenKind::LessThan)?;
        self.consume(TokenKind::Style)?;
        self.consume(TokenKind::GreaterThan)?;

        // For now, skip style content
        while !self.check_sequence(&[TokenKind::ClosingTag, TokenKind::Style]) {
            if self.is_at_end() {
                return Err(ParseError::MissingClosingTag("style".to_string()));
            }
            self.advance();
        }

        self.consume(TokenKind::ClosingTag)?;
        self.consume(TokenKind::Style)?;
        self.consume(TokenKind::GreaterThan)?;

        Ok(Some(Style {
            rules: Vec::new(),
            scoped: true,
        }))
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|t| t.kind.clone())
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    fn check_sequence(&self, kinds: &[TokenKind]) -> bool {
        for (i, kind) in kinds.iter().enumerate() {
            if let Some(token) = self.tokens.get(self.pos + i) {
                if std::mem::discriminant(&token.kind) != std::mem::discriminant(kind) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn consume(&mut self, kind: TokenKind) -> ParseResult<Token> {
        if self.check(kind.clone()) {
            Ok(self.advance().unwrap().clone())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", kind),
                found: self.peek().map(|t| format!("{:?}", t.kind)).unwrap_or("EOF".to_string()),
                pos: self.peek().map(|t| t.span.start).unwrap_or(0),
            })
        }
    }

    fn consume_if(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume_identifier(&mut self) -> ParseResult<String> {
        if let Some(TokenKind::Identifier(s)) = self.peek_kind() {
            self.advance();
            Ok(s)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self.peek().map(|t| format!("{:?}", t.kind)).unwrap_or("EOF".to_string()),
                pos: self.peek().map(|t| t.span.start).unwrap_or(0),
            })
        }
    }

    fn consume_any_identifier(&mut self) -> ParseResult<String> {
        // Including keywords that can be used as attribute names
        if let Some(token) = self.peek() {
            let result = match &token.kind {
                TokenKind::Identifier(s) => Some(s.clone()),
                TokenKind::Circle => Some("circle".to_string()),
                TokenKind::Rectangle => Some("rectangle".to_string()),
                TokenKind::Rect => Some("rect".to_string()),
                TokenKind::Ellipse => Some("ellipse".to_string()),
                TokenKind::Line => Some("line".to_string()),
                TokenKind::Path => Some("path".to_string()),
                TokenKind::Polygon => Some("polygon".to_string()),
                TokenKind::Text => Some("text".to_string()),
                TokenKind::Image => Some("image".to_string()),
                TokenKind::Group => Some("group".to_string()),
                _ => None,
            };

            if let Some(s) = result {
                self.advance();
                return Ok(s);
            }
        }

        Err(ParseError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: self.peek().map(|t| format!("{:?}", t.kind)).unwrap_or("EOF".to_string()),
            pos: self.peek().map(|t| t.span.start).unwrap_or(0),
        })
    }

    fn consume_element_tag(&mut self) -> ParseResult<String> {
        if let Some(token) = self.peek() {
            let result = match &token.kind {
                TokenKind::Circle => Some("circle".to_string()),
                TokenKind::Rectangle => Some("rectangle".to_string()),
                TokenKind::Rect => Some("rect".to_string()),
                TokenKind::Ellipse => Some("ellipse".to_string()),
                TokenKind::Line => Some("line".to_string()),
                TokenKind::Path => Some("path".to_string()),
                TokenKind::Polygon => Some("polygon".to_string()),
                TokenKind::Text => Some("text".to_string()),
                TokenKind::Image => Some("image".to_string()),
                TokenKind::Group => Some("group".to_string()),
                TokenKind::Identifier(s) => Some(s.clone()),
                _ => None,
            };

            if let Some(s) = result {
                self.advance();
                return Ok(s);
            }
        }

        Err(ParseError::UnexpectedToken {
            expected: "element tag".to_string(),
            found: self.peek().map(|t| format!("{:?}", t.kind)).unwrap_or("EOF".to_string()),
            pos: self.peek().map(|t| t.span.start).unwrap_or(0),
        })
    }
}

#[derive(Debug)]
enum AttrValueRaw {
    String(String),
    Expression(Expression),
    Boolean(bool),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> ParseResult<Component> {
        let tokens = Lexer::new(source).tokenize().unwrap();
        Parser::new(tokens, "test.omni").parse()
    }

    #[test]
    fn test_parse_simple_canvas() {
        let source = r##"
<canvas width={800} height={600}>
  <circle x={400} y={300} radius={50} fill="#00d4ff" />
</canvas>
"##;

        let result = parse(source);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let component = result.unwrap();
        assert_eq!(component.template.children.len(), 1);
    }

    #[test]
    fn test_parse_script_section() {
        let source = r#"
<script>
  const count = signal(0);
  const doubled = memo(() => count() * 2);
</script>

<canvas width={800} height={600}>
</canvas>
"#;

        let result = parse(source);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let component = result.unwrap();
        assert!(component.script.is_some());
        assert_eq!(component.script.unwrap().statements.len(), 2);
    }
}
