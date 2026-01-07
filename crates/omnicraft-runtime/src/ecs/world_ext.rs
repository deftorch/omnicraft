//! World Extensions
//!
//! Convenient methods for working with the ECS world.

use super::*;
use bevy_ecs::prelude::*;

/// Extension trait for World with OmniCraft-specific methods
pub trait WorldExt {
    fn set_canvas_width(&mut self, width: f32);
    fn set_canvas_height(&mut self, height: f32);
    fn spawn_circle(&mut self, x: f32, y: f32, radius: f32) -> Entity;
    fn spawn_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32) -> Entity;
    fn spawn_text(&mut self, x: f32, y: f32, text: &str) -> Entity;
}

impl WorldExt for World {
    fn set_canvas_width(&mut self, width: f32) {
        if let Some(mut config) = self.get_resource_mut::<crate::render::CanvasConfig>() {
            config.width = width;
        }
    }

    fn set_canvas_height(&mut self, height: f32) {
        if let Some(mut config) = self.get_resource_mut::<crate::render::CanvasConfig>() {
            config.height = height;
        }
    }

    fn spawn_circle(&mut self, x: f32, y: f32, radius: f32) -> Entity {
        self.spawn((
            Transform::from_xy(x, y),
            Shape::Circle { radius },
            Style::new(),
            Visibility::visible(),
        ))
        .id()
    }

    fn spawn_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32) -> Entity {
        self.spawn((
            Transform::from_xy(x, y),
            Shape::Rectangle { width, height },
            Style::new(),
            Visibility::visible(),
        ))
        .id()
    }

    fn spawn_text(&mut self, x: f32, y: f32, text: &str) -> Entity {
        self.spawn((
            Transform::from_xy(x, y),
            TextContent::new(text),
            Style::new().with_fill(Color::BLACK),
            Visibility::visible(),
        ))
        .id()
    }
}
