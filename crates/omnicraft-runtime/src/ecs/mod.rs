//! ECS Components and World Extensions
//!
//! Core components and systems for the OmniCraft ECS architecture.

use bevy_ecs::prelude::*;

pub mod world_ext;

pub use world_ext::*;

/// Transform component for position, rotation, and scale
#[derive(Component, Debug, Clone, Default)]
pub struct Transform {
    pub position: glam::Vec2,
    pub rotation: f32,
    pub scale: glam::Vec2,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            rotation: 0.0,
            scale: glam::Vec2::ONE,
        }
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            position: glam::Vec2::new(x, y),
            rotation: 0.0,
            scale: glam::Vec2::ONE,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = glam::Vec2::new(x, y);
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, sx: f32, sy: f32) -> Self {
        self.scale = glam::Vec2::new(sx, sy);
        self
    }
}

/// Shape component defining visual geometry
#[derive(Component, Debug, Clone)]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Ellipse { rx: f32, ry: f32 },
    Line { x2: f32, y2: f32 },
    Path { data: String },
    Polygon { points: Vec<glam::Vec2> },
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Circle { radius: 10.0 }
    }
}

/// Style component for visual appearance
#[derive(Component, Debug, Clone, Default)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub opacity: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            fill: Some(Color::WHITE),
            stroke: None,
            stroke_width: 1.0,
            opacity: 1.0,
        }
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some(color);
        self.stroke_width = width;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

/// Color representation
#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// Parse a color from a hex string like "#00d4ff" or "#00d4ff80"
    pub fn parse(s: &str) -> Self {
        let s = s.trim_start_matches('#');
        
        if s.len() == 6 {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0) as f32 / 255.0;
            Self::rgb(r, g, b)
        } else if s.len() == 8 {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0) as f32 / 255.0;
            let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255) as f32 / 255.0;
            Self::rgba(r, g, b, a)
        } else if s.len() == 3 {
            let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap_or(0) as f32 / 255.0;
            Self::rgb(r, g, b)
        } else {
            Self::BLACK
        }
    }

    /// Convert to CSS color string
    pub fn to_css(&self) -> String {
        if self.a >= 1.0 {
            format!(
                "rgb({}, {}, {})",
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8
            )
        } else {
            format!(
                "rgba({}, {}, {}, {})",
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8,
                self.a
            )
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        if self.a >= 1.0 {
            format!(
                "#{:02x}{:02x}{:02x}",
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8
            )
        } else {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8,
                (self.a * 255.0) as u8
            )
        }
    }
}

/// Text content component
#[derive(Component, Debug, Clone, Default)]
pub struct TextContent {
    pub text: String,
    pub font_size: f32,
    pub font_family: String,
    pub font_weight: FontWeight,
    pub text_align: TextAlign,
}

impl TextContent {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 16.0,
            font_family: "sans-serif".to_string(),
            font_weight: FontWeight::Normal,
            text_align: TextAlign::Left,
        }
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = family.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
    Light,
    Custom(u16),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Image component
#[derive(Component, Debug, Clone, Default)]
pub struct Image {
    pub src: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

/// Visibility component
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Visibility {
    pub visible: bool,
}

impl Visibility {
    pub fn visible() -> Self {
        Self { visible: true }
    }

    pub fn hidden() -> Self {
        Self { visible: false }
    }
}

/// Parent component for hierarchy
#[derive(Component, Debug, Clone, Copy)]
pub struct Parent(pub Entity);

/// Children component for hierarchy
#[derive(Component, Debug, Clone, Default)]
pub struct Children(pub Vec<Entity>);

/// Name component for debugging
#[derive(Component, Debug, Clone, Default)]
pub struct Name(pub String);

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// Z-index for rendering order
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ZIndex(pub i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_parse() {
        let color = Color::parse("#00d4ff");
        assert!((color.r - 0.0).abs() < 0.01);
        assert!((color.g - 0.83).abs() < 0.01);
        assert!((color.b - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_to_css() {
        let color = Color::rgb(1.0, 0.0, 0.5);
        assert!(color.to_css().starts_with("rgb("));
    }

    #[test]
    fn test_transform() {
        let transform = Transform::from_xy(100.0, 200.0)
            .with_rotation(std::f32::consts::PI)
            .with_scale(2.0, 2.0);

        assert_eq!(transform.position, glam::Vec2::new(100.0, 200.0));
        assert!((transform.rotation - std::f32::consts::PI).abs() < 0.001);
        assert_eq!(transform.scale, glam::Vec2::new(2.0, 2.0));
    }
}
