//! Abstract Syntax Tree (AST) for `.omni` components
//!
//! Represents the parsed structure of an OmniCraft component file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root AST node for an `.omni` component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub script: Option<Script>,
    pub template: Template,
    pub style: Option<Style>,
    pub metadata: ComponentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComponentMetadata {
    pub file_path: String,
    pub hash: String,
    pub exports: Vec<String>,
}

// ============================================================================
// Script Section
// ============================================================================

/// Script section containing JavaScript-like code
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Script {
    pub statements: Vec<Statement>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        kind: VarKind,
        name: String,
        init: Option<Expression>,
        reactive: ReactiveKind,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Statement>,
        is_async: bool,
    },
    Expression(Expression),
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    For {
        init: Box<Statement>,
        condition: Expression,
        update: Expression,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum VarKind {
    Const,
    Let,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ReactiveKind {
    None,
    Signal,
    Memo,
    Effect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub ty: Option<Type>,
    pub default: Option<Expression>,
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    Member {
        object: Box<Expression>,
        property: String,
        computed: bool,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Arrow {
        params: Vec<Parameter>,
        body: ArrowBody,
    },
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Template {
        parts: Vec<TemplatePart>,
    },
    Array(Vec<Expression>),
    Object(Vec<(String, Expression)>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArrowBody {
    Expression(Box<Expression>),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplatePart {
    String(String),
    Expression(Expression),
}

// ============================================================================
// Template Section
// ============================================================================

/// Template section containing visual elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    pub canvas: CanvasNode,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CanvasNode {
    pub width: Option<Expression>,
    pub height: Option<Expression>,
    pub background: Option<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Node {
    Element {
        tag: ElementTag,
        attributes: Vec<Attribute>,
        children: Vec<Node>,
        directives: Vec<Directive>,
        key: Option<Expression>,
    },
    Text {
        content: Expression,
    },
    Expression {
        expr: Expression,
    },
    IfBlock {
        condition: Expression,
        then_branch: Vec<Node>,
        else_branch: Option<Vec<Node>>,
    },
    EachBlock {
        expression: Expression,
        binding: String,
        index: Option<String>,
        body: Vec<Node>,
        key: Option<Expression>,
    },
    Slot {
        name: Option<String>,
        props: Vec<Attribute>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementTag {
    Circle,
    Rectangle,
    Ellipse,
    Line,
    Path,
    Polygon,
    Text,
    Image,
    Video,
    Group,
    Component(String),
}

impl ElementTag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "circle" => Some(ElementTag::Circle),
            "rectangle" | "rect" => Some(ElementTag::Rectangle),
            "ellipse" => Some(ElementTag::Ellipse),
            "line" => Some(ElementTag::Line),
            "path" => Some(ElementTag::Path),
            "polygon" => Some(ElementTag::Polygon),
            "text" => Some(ElementTag::Text),
            "image" | "img" => Some(ElementTag::Image),
            "video" => Some(ElementTag::Video),
            "group" | "g" => Some(ElementTag::Group),
            _ => {
                // Check if it's a custom component (starts with uppercase)
                if s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    Some(ElementTag::Component(s.to_string()))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeValue {
    Static(Literal),
    Dynamic(Expression),
    Spread(Expression),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Directive {
    pub name: DirectiveName,
    pub arg: Option<String>,
    pub value: Expression,
    pub modifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DirectiveName {
    On,   // @click, on:click
    Bind, // :value, bind:value
    Ref,  // ref={element}
    Use,  // use:action
}

// ============================================================================
// Style Section
// ============================================================================

/// Style section containing scoped CSS
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Style {
    pub rules: Vec<CssRule>,
    pub scoped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssRule {
    pub selector: String,
    pub declarations: Vec<CssDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssDeclaration {
    pub property: String,
    pub value: String,
}

// ============================================================================
// Type System
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Void,
    Any,
    Array(Box<Type>),
    Object(HashMap<String, Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Signal(Box<Type>),
    Union(Vec<Type>),
    Custom(String),
}

// ============================================================================
// Imports/Exports
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub source: String,
    pub specifiers: Vec<ImportSpecifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSpecifier {
    Named {
        name: String,
        alias: Option<String>,
    },
    Default(String),
    Namespace(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub value: Option<Expression>,
}
