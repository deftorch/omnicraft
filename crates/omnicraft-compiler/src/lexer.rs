//! Lexer for `.omni` files
//!
//! Uses `logos` for fast tokenization of the OmniCraft component syntax.

use logos::Logos;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character at position {0}")]
    UnexpectedChar(usize),

    #[error("Unterminated string starting at position {0}")]
    UnterminatedString(usize),

    #[error("Invalid number format at position {0}")]
    InvalidNumber(usize),
}

/// Token types for `.omni` files
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
pub enum TokenKind {
    // Tags
    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("</")]
    ClosingTag,

    #[token("/>")]
    SelfClosing,

    // Braces for expressions
    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    // Operators
    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("==")]
    DoubleEquals,

    #[token("!=")]
    NotEquals,

    #[token("<=")]
    LessEquals,

    #[token(">=")]
    GreaterEquals,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    #[token("?")]
    Question,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token(".")]
    Dot,

    #[token("=>")]
    Arrow,

    #[token("`")]
    Backtick,

    #[token("${")]
    TemplateExprStart,

    // Keywords
    #[token("const")]
    Const,

    #[token("let")]
    Let,

    #[token("function")]
    Function,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("return")]
    Return,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    #[token("signal")]
    Signal,

    #[token("effect")]
    Effect,

    #[token("memo")]
    Memo,

    // Tag names (special)
    #[token("script")]
    Script,

    #[token("canvas")]
    Canvas,

    #[token("style")]
    Style,

    // Element tags
    #[token("circle")]
    Circle,

    #[token("rectangle")]
    Rectangle,

    #[token("rect")]
    Rect,

    #[token("ellipse")]
    Ellipse,

    #[token("line")]
    Line,

    #[token("path")]
    Path,

    #[token("polygon")]
    Polygon,

    #[token("text")]
    Text,

    #[token("image")]
    Image,

    #[token("group")]
    Group,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Literals
    #[regex(r"-?[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    Number(f64),

    #[regex(r#""[^"]*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLiteral(String),

    #[regex(r#"'[^']*'"#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLiteralSingle(String),

    // Comments
    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
    LineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,

    #[regex(r"<!--([^-]|-[^-]|--[^>])*-->", logos::skip)]
    HtmlComment,
}

/// A token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String,
}

/// Source span
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// Lexer for `.omni` files
pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Tokenize the source into a list of tokens
    pub fn tokenize(&self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut lexer = TokenKind::lexer(self.source);

        while let Some(result) = lexer.next() {
            match result {
                Ok(kind) => {
                    let span = lexer.span();
                    tokens.push(Token {
                        kind,
                        span: Span::new(span.start, span.end),
                        text: lexer.slice().to_string(),
                    });
                }
                Err(_) => {
                    return Err(LexerError::UnexpectedChar(lexer.span().start));
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "<circle x={100} />";
        let tokens = Lexer::new(source).tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::LessThan);
        assert_eq!(tokens[1].kind, TokenKind::Circle);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Equals);
        assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[5].kind, TokenKind::Number(100.0));
        assert_eq!(tokens[6].kind, TokenKind::RightBrace);
        assert_eq!(tokens[7].kind, TokenKind::SelfClosing);
    }

    #[test]
    fn test_string_literals() {
        let source = r##"fill="#00d4ff""##;
        let tokens = Lexer::new(source).tokenize().unwrap();

        assert_eq!(
            tokens[2].kind,
            TokenKind::StringLiteral("#00d4ff".to_string())
        );
    }

    #[test]
    fn test_script_section() {
        let source = "<script> const count = signal(0); </script>";
        let tokens = Lexer::new(source).tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::LessThan);
        assert_eq!(tokens[1].kind, TokenKind::Script);
        assert_eq!(tokens[2].kind, TokenKind::GreaterThan);
        assert_eq!(tokens[3].kind, TokenKind::Const);
    }
}
