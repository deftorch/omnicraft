//! OmniCraft Runtime
//!
//! High-performance runtime for OmniCraft applications.
//!
//! ## Core Systems
//! - **Signals**: Fine-grained reactivity system (SolidJS-inspired)
//! - **ECS**: Entity-Component-System architecture (Bevy-inspired)
//! - **Rendering**: 2D rendering pipeline (Lyon + Canvas)
//! - **Tessellation**: GPU-ready path tessellation (Lyon)
//! - **Layout**: Flexbox/Grid layout system (Taffy)

pub mod components;
pub mod ecs;
pub mod layout;
pub mod render;
pub mod signals;
pub mod tessellation;

pub mod prelude {
    //! Prelude module with commonly used exports

    pub use crate::components::*;
    pub use crate::ecs::*;
    pub use crate::layout::*;
    pub use crate::render::*;
    pub use crate::signals::*;
    pub use crate::tessellation::*;
    pub use crate::OmniComponent;
    pub use crate::Context;

    pub use bevy_ecs::prelude::{
        Bundle, Component, Entity, Event, EventReader, EventWriter, Query, Res, ResMut, Resource,
        Schedule, System, World,
    };
    pub use glam::{Vec2, Vec3, Vec4};
}

use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

/// OmniCraft Application
#[wasm_bindgen]
pub struct App {
    world: World,
    schedule: Schedule,
}

#[wasm_bindgen]
impl App {
    /// Create a new OmniCraft application
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Initialize logging for WASM
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        let mut world = World::new();
        let schedule = Schedule::default();

        // Initialize default resources
        world.insert_resource(crate::signals::SignalContext::new());
        world.insert_resource(crate::render::CanvasConfig::default());

        web_sys::console::log_1(&"App::new called".into());
        Self { world, schedule }
    }

    /// Run the application frame
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        web_sys::console::log_1(&"App::tick called".into());
        self.schedule.run(&mut self.world);
    }

    /// Get the current canvas width
    #[wasm_bindgen(getter)]
    pub fn canvas_width(&self) -> f32 {
        self.world
            .get_resource::<crate::render::CanvasConfig>()
            .map(|c| c.width)
            .unwrap_or(800.0)
    }

    /// Get the current canvas height
    #[wasm_bindgen(getter)]
    pub fn canvas_height(&self) -> f32 {
        self.world
            .get_resource::<crate::render::CanvasConfig>()
            .map(|c| c.height)
            .unwrap_or(600.0)
    }
}

// Rust-only API
impl App {
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for OmniCraft components
pub trait OmniComponent {
    fn create(ctx: &mut Context) -> Self;
    fn mount(&self, world: &mut World);
}

/// Context for component creation
pub struct Context {
    signal_ctx: signals::SignalContext,
}

impl Context {
    pub fn new() -> Self {
        Self {
            signal_ctx: signals::SignalContext::new(),
        }
    }

    pub fn create_signal<T: Clone + 'static>(&mut self, value: T) -> signals::Signal<T> {
        self.signal_ctx.create_signal(value)
    }

    pub fn create_memo<T: Clone + 'static>(
        &mut self,
        compute: impl Fn() -> T + 'static,
    ) -> signals::Memo<T> {
        self.signal_ctx.create_memo(compute)
    }

    pub fn create_effect(&mut self, effect: impl Fn() + 'static) {
        self.signal_ctx.create_effect(effect);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
