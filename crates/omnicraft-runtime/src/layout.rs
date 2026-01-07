//! Layout System
//!
//! Provides Flexbox and Grid layout support using Taffy.
//! Automatically positions visual elements based on layout rules.

use bevy_ecs::prelude::*;
use glam::Vec2;
use std::collections::HashMap;

/// Re-export taffy for advanced users
pub use taffy;

/// Layout component for entities that participate in layout
#[derive(Component, Debug, Clone, Default)]
pub struct LayoutNode {
    /// Unique identifier for layout tree connection
    pub id: u32,
}

/// Simple layout style that can be converted to Taffy style
#[derive(Debug, Clone)]
pub struct SimpleLayoutStyle {
    /// Display mode: "flex" or "none"
    pub display: String,
    /// Flex direction: "row" or "column"
    pub flex_direction: String,
    /// Justify content: "start", "end", "center", "space-between", "space-around"
    pub justify_content: String,
    /// Align items: "start", "end", "center", "stretch"
    pub align_items: String,
    /// Flex grow factor
    pub flex_grow: f32,
    /// Flex shrink factor
    pub flex_shrink: f32,
    /// Width (None = auto)
    pub width: Option<f32>,
    /// Height (None = auto)
    pub height: Option<f32>,
    /// Padding (all sides)
    pub padding: f32,
    /// Margin (all sides) 
    pub margin: f32,
    /// Gap between children
    pub gap: f32,
}

impl Default for SimpleLayoutStyle {
    fn default() -> Self {
        Self {
            display: "flex".to_string(),
            flex_direction: "row".to_string(),
            justify_content: "start".to_string(),
            align_items: "stretch".to_string(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            width: None,
            height: None,
            padding: 0.0,
            margin: 0.0,
            gap: 0.0,
        }
    }
}

impl SimpleLayoutStyle {
    pub fn flex_row() -> Self {
        Self {
            flex_direction: "row".to_string(),
            ..Default::default()
        }
    }

    pub fn flex_column() -> Self {
        Self {
            flex_direction: "column".to_string(),
            ..Default::default()
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_flex_grow(mut self, grow: f32) -> Self {
        self.flex_grow = grow;
        self
    }

    pub fn justify_center(mut self) -> Self {
        self.justify_content = "center".to_string();
        self
    }

    pub fn align_center(mut self) -> Self {
        self.align_items = "center".to_string();
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Convert to Taffy style
    pub fn to_taffy_style(&self) -> taffy::Style {
        use taffy::prelude::*;

        let display = match self.display.as_str() {
            "none" => Display::None,
            _ => Display::Flex,
        };

        let flex_direction = match self.flex_direction.as_str() {
            "column" => FlexDirection::Column,
            _ => FlexDirection::Row,
        };

        let justify_content = match self.justify_content.as_str() {
            "end" => JustifyContent::FlexEnd,
            "center" => JustifyContent::Center,
            "space-between" => JustifyContent::SpaceBetween,
            "space-around" => JustifyContent::SpaceAround,
            _ => JustifyContent::FlexStart,
        };

        let align_items = match self.align_items.as_str() {
            "end" => AlignItems::FlexEnd,
            "center" => AlignItems::Center,
            "stretch" => AlignItems::Stretch,
            _ => AlignItems::FlexStart,
        };

        let width = self.width.map(|w| Dimension::length(w)).unwrap_or(Dimension::auto());
        let height = self.height.map(|h| Dimension::length(h)).unwrap_or(Dimension::auto());

        taffy::Style {
            display,
            flex_direction,
            justify_content: Some(justify_content),
            align_items: Some(align_items),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            size: taffy::Size { width, height },
            padding: taffy::Rect {
                left: LengthPercentage::length(self.padding),
                right: LengthPercentage::length(self.padding),
                top: LengthPercentage::length(self.padding),
                bottom: LengthPercentage::length(self.padding),
            },
            margin: taffy::Rect {
                left: LengthPercentageAuto::length(self.margin),
                right: LengthPercentageAuto::length(self.margin),
                top: LengthPercentageAuto::length(self.margin),
                bottom: LengthPercentageAuto::length(self.margin),
            },
            gap: taffy::Size {
                width: LengthPercentage::length(self.gap),
                height: LengthPercentage::length(self.gap),
            },
            ..Default::default()
        }
    }
}

/// Computed layout result
#[derive(Component, Debug, Clone, Default)]
pub struct ComputedLayout {
    /// Position relative to parent
    pub position: Vec2,
    /// Size after layout
    pub size: Vec2,
}

/// Layout manager that wraps TaffyTree
pub struct LayoutManager {
    taffy: taffy::TaffyTree,
    id_to_node: HashMap<u32, taffy::NodeId>,
    node_to_id: HashMap<taffy::NodeId, u32>,
    next_id: u32,
    root: Option<taffy::NodeId>,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            taffy: taffy::TaffyTree::new(),
            id_to_node: HashMap::new(),
            node_to_id: HashMap::new(),
            next_id: 1,
            root: None,
        }
    }

    /// Create a root node and return its ID
    pub fn create_root(&mut self, style: &SimpleLayoutStyle) -> u32 {
        let node = self.taffy.new_leaf(style.to_taffy_style()).unwrap();
        self.root = Some(node);
        
        let id = self.next_id;
        self.next_id += 1;
        self.id_to_node.insert(id, node);
        self.node_to_id.insert(node, id);
        id
    }

    /// Add a child node and return its ID
    pub fn add_node(&mut self, style: &SimpleLayoutStyle, parent_id: Option<u32>) -> u32 {
        let node = self.taffy.new_leaf(style.to_taffy_style()).unwrap();
        
        if let Some(pid) = parent_id {
            if let Some(&parent_node) = self.id_to_node.get(&pid) {
                let _ = self.taffy.add_child(parent_node, node);
            }
        }
        
        let id = self.next_id;
        self.next_id += 1;
        self.id_to_node.insert(id, node);
        self.node_to_id.insert(node, id);
        id
    }

    /// Compute layout for a given size
    pub fn compute(&mut self, width: f32, height: f32) {
        if let Some(root) = self.root {
            let available = taffy::Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            };
            let _ = self.taffy.compute_layout(root, available);
        }
    }

    /// Get computed layout for a node ID
    pub fn get_layout(&self, id: u32) -> Option<ComputedLayout> {
        self.id_to_node
            .get(&id)
            .and_then(|node| self.taffy.layout(*node).ok())
            .map(|layout| ComputedLayout {
                position: Vec2::new(layout.location.x, layout.location.y),
                size: Vec2::new(layout.size.width, layout.size.height),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_manager() {
        let mut manager = LayoutManager::new();
        
        let root_style = SimpleLayoutStyle::flex_column().with_size(800.0, 600.0);
        let root = manager.create_root(&root_style);
        
        let child_style = SimpleLayoutStyle::flex_row().with_height(100.0).with_flex_grow(1.0);
        let child = manager.add_node(&child_style, Some(root));
        
        manager.compute(800.0, 600.0);
        
        let layout = manager.get_layout(child);
        assert!(layout.is_some());
    }

    #[test]
    fn test_simple_layout_style() {
        let style = SimpleLayoutStyle::flex_row()
            .with_size(100.0, 50.0)
            .justify_center()
            .align_center()
            .with_padding(10.0);
        
        assert_eq!(style.flex_direction, "row");
        assert_eq!(style.justify_content, "center");
        assert_eq!(style.align_items, "center");
        assert_eq!(style.width, Some(100.0));
        assert_eq!(style.height, Some(50.0));
    }
}
