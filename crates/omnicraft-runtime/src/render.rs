//! Rendering System
//!
//! Renders ECS entities to HTML Canvas using Lyon for tessellation.

use crate::ecs::{Color, Shape, Style, TextContent, Transform, Visibility};
use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

/// Canvas configuration resource
#[derive(Resource, Debug, Clone)]
pub struct CanvasConfig {
    pub width: f32,
    pub height: f32,
    pub background: Color,
    pub pixel_ratio: f32,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            background: Color::WHITE,
            pixel_ratio: 1.0,
        }
    }
}

/// Render commands to send to JavaScript
#[derive(Debug, Clone)]
pub enum RenderCommand {
    Clear { color: Color },
    BeginPath,
    ClosePath,
    MoveTo { x: f32, y: f32 },
    LineTo { x: f32, y: f32 },
    Arc { x: f32, y: f32, radius: f32, start: f32, end: f32 },
    Rect { x: f32, y: f32, width: f32, height: f32 },
    Fill { color: Color },
    Stroke { color: Color, width: f32 },
    SetFillStyle { color: Color },
    SetStrokeStyle { color: Color, width: f32 },
    FillText { text: String, x: f32, y: f32 },
    SetFont { font: String },
    Save,
    Restore,
    Translate { x: f32, y: f32 },
    Rotate { angle: f32 },
    Scale { x: f32, y: f32 },
}

/// Render queue for batching commands
#[derive(Resource, Debug, Clone, Default)]
pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn push(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }
}

/// Renderer that produces canvas drawing commands
pub struct Renderer;

impl Renderer {
    /// Render all visible entities to the render queue
    pub fn render(world: &mut World, queue: &mut RenderQueue) {
        web_sys::console::log_1(&"Renderer::render called".into());
        queue.clear();

        // Clear background
        if let Some(config) = world.get_resource::<CanvasConfig>() {
            web_sys::console::log_1(&format!("CanvasConfig found: width={}, height={}, bg={:?}", config.width, config.height, config.background).into());
            queue.push(RenderCommand::Clear { color: config.background });
        } else {
            web_sys::console::log_1(&"CanvasConfig NOT found".into());
        }

        // Query all renderable entities
        let mut query = world.query::<(
            &Transform,
            Option<&Shape>,
            Option<&Style>,
            Option<&TextContent>,
            Option<&Visibility>,
        )>();

        let mut entity_count = 0;
        for (transform, shape, style, text, visibility) in query.iter(world) {
            entity_count += 1;

            // Skip invisible entities
            if let Some(vis) = visibility {
                if !vis.visible {
                    continue;
                }
            }

            let style = style.cloned().unwrap_or_default();

            // Save transform state
            queue.push(RenderCommand::Save);

            // Apply transform
            queue.push(RenderCommand::Translate {
                x: transform.position.x,
                y: transform.position.y,
            });

            if transform.rotation != 0.0 {
                queue.push(RenderCommand::Rotate { angle: transform.rotation });
            }

            if transform.scale != glam::Vec2::ONE {
                queue.push(RenderCommand::Scale {
                    x: transform.scale.x,
                    y: transform.scale.y,
                });
            }

            // Render shape
            if let Some(shape) = shape {
                Self::render_shape(shape, &style, queue);
            }

            // Render text
            if let Some(text) = text {
                Self::render_text(text, &style, queue);
            }

            // Restore transform state
            queue.push(RenderCommand::Restore);
        }
        web_sys::console::log_1(&format!("Entities processed: {}", entity_count).into());
        web_sys::console::log_1(&format!("Queue commands pushed: {}", queue.commands.len()).into());
    }

    fn render_shape(shape: &Shape, style: &Style, queue: &mut RenderQueue) {
        match shape {
            Shape::Circle { radius } => {
                queue.push(RenderCommand::BeginPath);
                queue.push(RenderCommand::Arc {
                    x: 0.0,
                    y: 0.0,
                    radius: *radius,
                    start: 0.0,
                    end: std::f32::consts::TAU,
                });
                queue.push(RenderCommand::ClosePath);

                if let Some(fill) = style.fill {
                    queue.push(RenderCommand::Fill { color: fill });
                }
                if let Some(stroke) = style.stroke {
                    queue.push(RenderCommand::Stroke {
                        color: stroke,
                        width: style.stroke_width,
                    });
                }
            }

            Shape::Rectangle { width, height } => {
                queue.push(RenderCommand::BeginPath);
                queue.push(RenderCommand::Rect {
                    x: -width / 2.0,
                    y: -height / 2.0,
                    width: *width,
                    height: *height,
                });
                queue.push(RenderCommand::ClosePath);

                if let Some(fill) = style.fill {
                    queue.push(RenderCommand::Fill { color: fill });
                }
                if let Some(stroke) = style.stroke {
                    queue.push(RenderCommand::Stroke {
                        color: stroke,
                        width: style.stroke_width,
                    });
                }
            }

            Shape::Ellipse { rx, ry } => {
                // Approximate ellipse with arc and scale
                queue.push(RenderCommand::Save);
                queue.push(RenderCommand::Scale { x: *rx, y: *ry });
                queue.push(RenderCommand::BeginPath);
                queue.push(RenderCommand::Arc {
                    x: 0.0,
                    y: 0.0,
                    radius: 1.0,
                    start: 0.0,
                    end: std::f32::consts::TAU,
                });
                queue.push(RenderCommand::ClosePath);
                queue.push(RenderCommand::Restore);

                if let Some(fill) = style.fill {
                    queue.push(RenderCommand::Fill { color: fill });
                }
                if let Some(stroke) = style.stroke {
                    queue.push(RenderCommand::Stroke {
                        color: stroke,
                        width: style.stroke_width,
                    });
                }
            }

            Shape::Line { x2, y2 } => {
                queue.push(RenderCommand::BeginPath);
                queue.push(RenderCommand::MoveTo { x: 0.0, y: 0.0 });
                queue.push(RenderCommand::LineTo { x: *x2, y: *y2 });

                if let Some(stroke) = style.stroke {
                    queue.push(RenderCommand::Stroke {
                        color: stroke,
                        width: style.stroke_width,
                    });
                }
            }

            Shape::Polygon { points } => {
                if points.is_empty() {
                    return;
                }

                queue.push(RenderCommand::BeginPath);
                queue.push(RenderCommand::MoveTo {
                    x: points[0].x,
                    y: points[0].y,
                });

                for point in points.iter().skip(1) {
                    queue.push(RenderCommand::LineTo { x: point.x, y: point.y });
                }

                queue.push(RenderCommand::ClosePath);

                if let Some(fill) = style.fill {
                    queue.push(RenderCommand::Fill { color: fill });
                }
                if let Some(stroke) = style.stroke {
                    queue.push(RenderCommand::Stroke {
                        color: stroke,
                        width: style.stroke_width,
                    });
                }
            }

            Shape::Path { data: _ } => {
                // TODO: Parse SVG path data
            }
        }
    }

    fn render_text(text: &TextContent, style: &Style, queue: &mut RenderQueue) {
        let font = format!("{}px {}", text.font_size, text.font_family);
        queue.push(RenderCommand::SetFont { font });

        if let Some(fill) = style.fill {
            queue.push(RenderCommand::SetFillStyle { color: fill });
            queue.push(RenderCommand::FillText {
                text: text.text.clone(),
                x: 0.0,
                y: 0.0,
            });
        }
    }
}

/// WASM bindings for canvas rendering
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::JsCast;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

    pub struct CanvasRenderer {
        ctx: CanvasRenderingContext2d,
    }

    impl CanvasRenderer {
        pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document
                .get_element_by_id(canvas_id)
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()?;

            let ctx = canvas
                .get_context("2d")?
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()?;

            Ok(Self { ctx })
        }

        pub fn execute(&self, commands: &[RenderCommand]) {
            for command in commands {
                self.execute_command(command);
            }
        }

        fn execute_command(&self, command: &RenderCommand) {
            match command {
                RenderCommand::Clear { color } => {
                    self.ctx.set_fill_style(&JsValue::from_str(&color.to_css()));
                    self.ctx.fill_rect(0.0, 0.0, 10000.0, 10000.0);
                }
                RenderCommand::BeginPath => {
                    self.ctx.begin_path();
                }
                RenderCommand::ClosePath => {
                    self.ctx.close_path();
                }
                RenderCommand::MoveTo { x, y } => {
                    self.ctx.move_to(*x as f64, *y as f64);
                }
                RenderCommand::LineTo { x, y } => {
                    self.ctx.line_to(*x as f64, *y as f64);
                }
                RenderCommand::Arc { x, y, radius, start, end } => {
                    let _ = self.ctx.arc(
                        *x as f64,
                        *y as f64,
                        *radius as f64,
                        *start as f64,
                        *end as f64,
                    );
                }
                RenderCommand::Rect { x, y, width, height } => {
                    self.ctx.rect(*x as f64, *y as f64, *width as f64, *height as f64);
                }
                RenderCommand::Fill { color } => {
                    self.ctx.set_fill_style(&JsValue::from_str(&color.to_css()));
                    self.ctx.fill();
                }
                RenderCommand::Stroke { color, width } => {
                    self.ctx.set_stroke_style(&JsValue::from_str(&color.to_css()));
                    self.ctx.set_line_width(*width as f64);
                    self.ctx.stroke();
                }
                RenderCommand::SetFillStyle { color } => {
                    self.ctx.set_fill_style(&JsValue::from_str(&color.to_css()));
                }
                RenderCommand::SetStrokeStyle { color, width } => {
                    self.ctx.set_stroke_style(&JsValue::from_str(&color.to_css()));
                    self.ctx.set_line_width(*width as f64);
                }
                RenderCommand::FillText { text, x, y } => {
                    let _ = self.ctx.fill_text(text, *x as f64, *y as f64);
                }
                RenderCommand::SetFont { font } => {
                    self.ctx.set_font(font);
                }
                RenderCommand::Save => {
                    self.ctx.save();
                }
                RenderCommand::Restore => {
                    self.ctx.restore();
                }
                RenderCommand::Translate { x, y } => {
                    let _ = self.ctx.translate(*x as f64, *y as f64);
                }
                RenderCommand::Rotate { angle } => {
                    let _ = self.ctx.rotate(*angle as f64);
                }
                RenderCommand::Scale { x, y } => {
                    let _ = self.ctx.scale(*x as f64, *y as f64);
                }
            }
        }
    }
}
