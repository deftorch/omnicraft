# 📐 OmniCraft - Software Design Document (SDD)

**Version:** 3.0  
**Date:** January 06, 2026  
**Status:** Final Design Specification  
**Classification:** Technical Architecture & Design

---

## Document Control

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-02 | Architecture Team | Initial runtime-based design |
| 2.0 | 2026-01-04 | Architecture Team | Compiler-first redesign |
| 3.0 | 2026-01-06 | Architecture Team | Enhanced DSL, modularity, and developer experience |

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [System Overview](#2-system-overview)
3. [Architectural Design](#3-architectural-design)
4. [Design Patterns](#4-design-patterns)
5. [SOLID Principles Implementation](#5-solid-principles-implementation)
6. [Component Design](#6-component-design)
7. [UML Diagrams](#7-uml-diagrams)
8. [Database Design](#8-database-design)
9. [Interface Design (API)](#9-interface-design-api)
10. [UI/UX Design](#10-uiux-design)
11. [Security Design](#11-security-design)
12. [Scalability & Performance](#12-scalability--performance)
13. [Testing Strategy](#13-testing-strategy)
14. [Deployment Architecture](#14-deployment-architecture)
15. [Developer Experience (DX)](#15-developer-experience-dx)
16. [Risk Analysis](#16-risk-analysis)

---

## 1. Executive Summary

### 1.1 Project Vision

**OmniCraft** adalah universal visual content creation platform yang menggabungkan:
- **Compiler-first architecture** (Svelte-inspired)
- **Fine-grained reactivity** (SolidJS-inspired)
- **ECS core engine** (Bevy-inspired)
- **Multi-level DSL** (Progressive disclosure)

**Core Innovation:** Compile-time optimizations + Runtime performance + Developer experience excellence

### 1.2 Key Objectives

| Objective | Target | Status |
|-----------|--------|--------|
| **Bundle Size** | < 50 KB (gzipped) | Target: 45 KB |
| **Initial Load** | < 150 ms | Target: 80 ms |
| **Memory (1k entities)** | < 5 MB | Target: 2.5 MB |
| **Update Time (1k entities)** | < 1 ms | Target: 0.15 ms |
| **Compilation Time** | < 250 ms | Target: 220 ms (first), 35 ms (incremental) |
| **Developer Onboarding** | < 1 hour | Target: 30 minutes |

### 1.3 Success Criteria

**Technical:**
- ✅ All performance targets met
- ✅ 90%+ test coverage
- ✅ Zero critical bugs

**Business:**
- ✅ 1,000 active users (6 months)
- ✅ 50+ community components
- ✅ $1M ARR (Year 1)

---

## 2. System Overview

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        USER LAYER                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Browser    │  │   VSCode     │  │     CLI      │      │
│  │   (Web App)  │  │  (Extension) │  │    (Tool)    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
┌─────────▼──────────────────▼──────────────────▼─────────────┐
│                   DEVELOPMENT LAYER                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              OmniCraft Compiler                     │    │
│  │  ┌───────────┐  ┌──────────┐  ┌──────────────┐    │    │
│  │  │   Lexer   │→ │  Parser  │→ │   Analyzer   │    │    │
│  │  └───────────┘  └──────────┘  └──────┬───────┘    │    │
│  │                                       │            │    │
│  │  ┌───────────┐  ┌──────────┐  ┌──────▼───────┐    │    │
│  │  │Code Gen   │← │Optimizer │← │Type Checker  │    │    │
│  │  └─────┬─────┘  └──────────┘  └──────────────┘    │    │
│  └────────┼─────────────────────────────────────────┘    │
└───────────┼──────────────────────────────────────────────┘
            │
┌───────────▼──────────────────────────────────────────────┐
│                   BUILD LAYER                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │  Rust Code   │→ │ Rust Compiler│→ │  WASM Binary │   │
│  └──────────────┘  └──────────────┘  └──────┬───────┘   │
│                                              │           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────▼───────┐   │
│  │   JS Glue    │  │  Source Maps │  │ Type Defs    │   │
│  └──────────────┘  └──────────────┘  └──────────────┘   │
└──────────────────────────────────────────────────────────┘
            │
┌───────────▼──────────────────────────────────────────────┐
│                   RUNTIME LAYER                           │
│  ┌─────────────────────────────────────────────────┐     │
│  │           OmniCraft Runtime (WASM)              │     │
│  │  ┌────────────────┐  ┌────────────────┐        │     │
│  │  │ Reactive System│  │   ECS Engine   │        │     │
│  │  │  - Signals     │  │  - Entities    │        │     │
│  │  │  - Effects     │  │  - Components  │        │     │
│  │  │  - Computed    │  │  - Systems     │        │     │
│  │  └────────┬───────┘  └────────┬───────┘        │     │
│  │           │                   │                 │     │
│  │  ┌────────▼──────────────────▼────────┐        │     │
│  │  │      Rendering Pipeline            │        │     │
│  │  │  - Transform  - Layout  - Render   │        │     │
│  │  └────────┬───────────────────────────┘        │     │
│  └───────────┼──────────────────────────────────┘     │
└──────────────┼───────────────────────────────────────┘
               │
┌──────────────▼───────────────────────────────────────────┐
│                   OUTPUT LAYER                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │  Canvas  │  │   WebGL  │  │   SVG    │  │   Video  │ │
│  │ (2D Ctx) │  │(3D Accel)│  │ (Vector) │  │  (FFmpeg)│ │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘ │
└──────────────────────────────────────────────────────────┘
```

### 2.2 System Context Diagram

```
┌──────────────────────────────────────────────────────────┐
│                   External Systems                        │
│                                                           │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐           │
│  │   NPM     │  │  GitHub   │  │   CDN     │           │
│  │ Registry  │  │    Repo   │  │(Delivery) │           │
│  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘           │
└────────┼──────────────┼──────────────┼──────────────────┘
         │              │              │
         ↓              ↓              ↓
┌─────────────────────────────────────────────────────────┐
│                  OmniCraft Platform                      │
│                                                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │              Core Components                     │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │  │
│  │  │Compiler  │  │ Runtime  │  │   CLI    │      │  │
│  │  └──────────┘  └──────────┘  └──────────┘      │  │
│  └──────────────────────────────────────────────────┘  │
│                                                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │           Developer Tools                        │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │  │
│  │  │   LSP    │  │ DevTools │  │ VS Code  │      │  │
│  │  └──────────┘  └──────────┘  └──────────┘      │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
         ↓              ↓              ↓
┌────────┼──────────────┼──────────────┼──────────────────┐
│        │              │              │                   │
│  ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐          │
│  │   Users   │  │Developers │  │ Community │          │
│  │(End Users)│  │ (Creators)│  │(Contrib.) │          │
│  └───────────┘  └───────────┘  └───────────┘          │
│                                                          │
│                  Target Audience                         │
└─────────────────────────────────────────────────────────┘
```

### 2.3 Technology Stack

#### **Frontend/Development:**
```yaml
Language: TypeScript 5.3
Framework: Next.js 14
UI Library: Shadcn/ui + Tailwind CSS
State: Zustand
Build: Turbopack
Package Manager: pnpm
```

#### **Compiler (Rust):**
```yaml
Language: Rust 1.75+
Parser: Custom (nom-based)
Type System: Custom inference engine
Optimization: LLVM-based
Testing: cargo test
```

#### **Runtime (Rust → WASM):**
```yaml
Language: Rust 1.75+
ECS: Custom (Bevy-inspired)
Reactivity: Custom (SolidJS-inspired)
Target: wasm32-unknown-unknown
Bindings: wasm-bindgen
```

#### **Backend (Optional):**
```yaml
Platform: Supabase
Database: PostgreSQL
Auth: Supabase Auth
Storage: Supabase Storage
Real-time: Supabase Realtime
```

#### **DevOps:**
```yaml
Hosting: Vercel (Edge Functions)
CDN: Cloudflare
CI/CD: GitHub Actions
Monitoring: Sentry + PostHog
Registry: NPM + crates.io
```

---

## 3. Architectural Design

### 3.1 Architectural Pattern: Layered Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   PRESENTATION LAYER                     │
│  - Web UI (Next.js)                                     │
│  - VSCode Extension                                     │
│  - CLI Interface                                        │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                   APPLICATION LAYER                      │
│  - Compiler Pipeline                                    │
│  - Build Orchestration                                  │
│  - Project Management                                   │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                   BUSINESS LOGIC LAYER                   │
│  - Parsing & Analysis                                   │
│  - Type Checking & Inference                            │
│  - Code Generation                                      │
│  - Optimization                                         │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                   RUNTIME LAYER                          │
│  - Reactive System                                      │
│  - ECS Engine                                           │
│  - Rendering Pipeline                                   │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                   DATA/INFRASTRUCTURE LAYER              │
│  - File System                                          │
│  - Cache Management                                     │
│  - Package Registry                                     │
└─────────────────────────────────────────────────────────┘
```

### 3.2 Microservices Architecture (Future)

```
┌─────────────────────────────────────────────────────────┐
│                   API Gateway (Edge)                     │
│                   (Load Balancing + Routing)             │
└────────┬──────────────────┬──────────────────┬──────────┘
         │                  │                  │
    ┌────▼────┐      ┌─────▼─────┐     ┌─────▼─────┐
    │Compiler │      │  Runtime  │     │  Storage  │
    │ Service │      │  Service  │     │  Service  │
    └─────────┘      └───────────┘     └───────────┘
         │                  │                  │
    ┌────▼────┐      ┌─────▼─────┐     ┌─────▼─────┐
    │  Cache  │      │ Analytics │     │   Auth    │
    │ (Redis) │      │(PostHog)  │     │(Supabase) │
    └─────────┘      └───────────┘     └───────────┘
```

### 3.3 Event-Driven Architecture (Compilation Pipeline)

```
┌─────────────────────────────────────────────────────────┐
│                   Event Bus                              │
└────┬─────────┬─────────┬─────────┬─────────┬───────────┘
     │         │         │         │         │
┌────▼───┐ ┌──▼────┐ ┌──▼────┐ ┌──▼────┐ ┌──▼────┐
│ Parse  │→│Analyze│→│Optimize│→│CodeGen│→│ Build │
│ Event  │ │ Event │ │ Event  │ │ Event │ │ Event │
└────┬───┘ └───┬───┘ └───┬───┘ └───┬───┘ └───┬───┘
     │         │         │         │         │
     ↓         ↓         ↓         ↓         ↓
┌────────────────────────────────────────────────────┐
│           Event Store (Compilation Log)            │
└────────────────────────────────────────────────────┘
```

---

## 4. Design Patterns

### 4.1 Creational Patterns

#### **Factory Pattern** (Component Creation)

```rust
// factory/component_factory.rs

pub trait ComponentFactory {
    fn create(&self, config: ComponentConfig) -> Box<dyn Component>;
}

pub struct ButtonFactory;

impl ComponentFactory for ButtonFactory {
    fn create(&self, config: ComponentConfig) -> Box<dyn Component> {
        Box::new(Button {
            label: config.get("label").unwrap_or("Button"),
            variant: config.get("variant").unwrap_or("primary"),
            // ...
        })
    }
}

pub struct ComponentRegistry {
    factories: HashMap<String, Box<dyn ComponentFactory>>,
}

impl ComponentRegistry {
    pub fn register(&mut self, name: &str, factory: Box<dyn ComponentFactory>) {
        self.factories.insert(name.to_string(), factory);
    }
    
    pub fn create(&self, name: &str, config: ComponentConfig) -> Box<dyn Component> {
        self.factories
            .get(name)
            .expect("Component not registered")
            .create(config)
    }
}

// Usage
let mut registry = ComponentRegistry::new();
registry.register("Button", Box::new(ButtonFactory));
registry.register("Card", Box::new(CardFactory));

let button = registry.create("Button", config);
```

#### **Builder Pattern** (Complex Object Construction)

```rust
// builder/component_builder.rs

pub struct ComponentBuilder {
    name: String,
    props: HashMap<String, Value>,
    children: Vec<Box<dyn Component>>,
    styles: Option<Style>,
    events: HashMap<String, EventHandler>,
}

impl ComponentBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            props: HashMap::new(),
            children: Vec::new(),
            styles: None,
            events: HashMap::new(),
        }
    }
    
    pub fn prop(mut self, key: &str, value: Value) -> Self {
        self.props.insert(key.to_string(), value);
        self
    }
    
    pub fn child(mut self, component: Box<dyn Component>) -> Self {
        self.children.push(component);
        self
    }
    
    pub fn style(mut self, style: Style) -> Self {
        self.styles = Some(style);
        self
    }
    
    pub fn on(mut self, event: &str, handler: EventHandler) -> Self {
        self.events.insert(event.to_string(), handler);
        self
    }
    
    pub fn build(self) -> Box<dyn Component> {
        Box::new(GenericComponent {
            name: self.name,
            props: self.props,
            children: self.children,
            styles: self.styles,
            events: self.events,
        })
    }
}

// Usage
let component = ComponentBuilder::new("Card")
    .prop("width", Value::Number(300.0))
    .prop("height", Value::Number(200.0))
    .style(Style::default())
    .child(Button::new("Click me"))
    .on("click", |_| println!("Clicked!"))
    .build();
```

#### **Singleton Pattern** (Global Registry)

```rust
// singleton/compiler_context.rs

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct CompilerContext {
    config: CompilerConfig,
    cache: HashMap<String, CachedModule>,
    diagnostics: Vec<Diagnostic>,
}

static COMPILER_CONTEXT: Lazy<Mutex<CompilerContext>> = Lazy::new(|| {
    Mutex::new(CompilerContext::new())
});

impl CompilerContext {
    fn new() -> Self {
        Self {
            config: CompilerConfig::default(),
            cache: HashMap::new(),
            diagnostics: Vec::new(),
        }
    }
    
    pub fn instance() -> &'static Mutex<CompilerContext> {
        &COMPILER_CONTEXT
    }
    
    pub fn get_config(&self) -> &CompilerConfig {
        &self.config
    }
    
    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

// Usage
let ctx = CompilerContext::instance().lock().unwrap();
ctx.add_diagnostic(Diagnostic::error("Type mismatch"));
```

### 4.2 Structural Patterns

#### **Adapter Pattern** (Multiple Renderers)

```rust
// adapter/renderer_adapter.rs

pub trait Renderer {
    fn render(&self, component: &Component) -> Result<()>;
}

// Canvas 2D Adapter
pub struct Canvas2DAdapter {
    context: CanvasRenderingContext2d,
}

impl Renderer for Canvas2DAdapter {
    fn render(&self, component: &Component) -> Result<()> {
        match component.shape {
            Shape::Circle { radius, .. } => {
                self.context.begin_path();
                self.context.arc(x, y, radius, 0.0, 2.0 * PI)?;
                self.context.fill();
            }
            // ...
        }
        Ok(())
    }
}

// WebGL Adapter
pub struct WebGLAdapter {
    gl: WebGlRenderingContext,
}

impl Renderer for WebGLAdapter {
    fn render(&self, component: &Component) -> Result<()> {
        // WebGL-specific rendering
        Ok(())
    }
}

// SVG Adapter
pub struct SVGAdapter {
    document: Document,
}

impl Renderer for SVGAdapter {
    fn render(&self, component: &Component) -> Result<()> {
        // SVG DOM manipulation
        Ok(())
    }
}

// Usage
let renderer: Box<dyn Renderer> = match config.output_format {
    OutputFormat::Canvas => Box::new(Canvas2DAdapter::new()),
    OutputFormat::WebGL => Box::new(WebGLAdapter::new()),
    OutputFormat::SVG => Box::new(SVGAdapter::new()),
};

renderer.render(&component)?;
```

#### **Composite Pattern** (Component Tree)

```rust
// composite/component_tree.rs

pub trait Component {
    fn render(&self, ctx: &RenderContext) -> Result<()>;
    fn get_children(&self) -> &[Box<dyn Component>];
    fn add_child(&mut self, child: Box<dyn Component>);
}

// Leaf component
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Component for Circle {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.draw_circle(self.x, self.y, self.radius)
    }
    
    fn get_children(&self) -> &[Box<dyn Component>] {
        &[] // Leaf has no children
    }
    
    fn add_child(&mut self, _: Box<dyn Component>) {
        panic!("Cannot add child to leaf component");
    }
}

// Composite component
pub struct Group {
    children: Vec<Box<dyn Component>>,
    transform: Transform,
}

impl Component for Group {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.save();
        ctx.apply_transform(&self.transform);
        
        for child in &self.children {
            child.render(ctx)?;
        }
        
        ctx.restore();
        Ok(())
    }
    
    fn get_children(&self) -> &[Box<dyn Component>] {
        &self.children
    }
    
    fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

// Usage
let mut group = Group::new();
group.add_child(Box::new(Circle { x: 100.0, y: 100.0, radius: 50.0 }));
group.add_child(Box::new(Circle { x: 200.0, y: 200.0, radius: 30.0 }));

group.render(&ctx)?;
```

#### **Decorator Pattern** (Component Enhancement)

```rust
// decorator/component_decorator.rs

pub trait Component {
    fn render(&self, ctx: &RenderContext) -> Result<()>;
}

// Base component
pub struct BasicButton {
    label: String,
}

impl Component for BasicButton {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.draw_text(&self.label)
    }
}

// Decorator
pub struct ShadowDecorator {
    component: Box<dyn Component>,
    shadow_color: Color,
    shadow_blur: f32,
}

impl Component for ShadowDecorator {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.set_shadow(self.shadow_color, self.shadow_blur);
        self.component.render(ctx)?;
        ctx.clear_shadow();
        Ok(())
    }
}

// Another decorator
pub struct AnimationDecorator {
    component: Box<dyn Component>,
    animation: Animation,
}

impl Component for AnimationDecorator {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.apply_animation(&self.animation);
        self.component.render(ctx)?;
        Ok(())
    }
}

// Usage (Stack decorators!)
let button = BasicButton::new("Click me");
let button_with_shadow = ShadowDecorator::new(Box::new(button), Color::BLACK, 4.0);
let animated_button = AnimationDecorator::new(Box::new(button_with_shadow), fade_in_animation);

animated_button.render(&ctx)?;
```

### 4.3 Behavioral Patterns

#### **Observer Pattern** (Reactive System)

```rust
// observer/reactive.rs

pub trait Observer {
    fn update(&mut self);
}

pub struct Signal<T> {
    value: RefCell<T>,
    observers: RefCell<Vec<Weak<RefCell<dyn Observer>>>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(initial: T) -> Rc<Self> {
        Rc::new(Self {
            value: RefCell::new(initial),
            observers: RefCell::new(Vec::new()),
        })
    }
    
    pub fn subscribe(&self, observer: Weak<RefCell<dyn Observer>>) {
        self.observers.borrow_mut().push(observer);
    }
    
    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        self.notify();
    }
    
    fn notify(&self) {
        let observers = self.observers.borrow();
        for observer in observers.iter() {
            if let Some(obs) = observer.upgrade() {
                obs.borrow_mut().update();
            }
        }
    }
}

// Concrete observer
pub struct TextComponent {
    signal: Rc<Signal<String>>,
    rendered_text: String,
}

impl Observer for TextComponent {
    fn update(&mut self) {
        self.rendered_text = self.signal.get();
        self.render();
    }
}

// Usage
let count = Signal::new(0);
let text = Rc::new(RefCell::new(TextComponent::new(count.clone())));

count.subscribe(Rc::downgrade(&text));
count.set(42); // Automatically triggers text.update()
```

#### **Command Pattern** (Undo/Redo)

```rust
// command/editor_commands.rs

pub trait Command {
    fn execute(&mut self) -> Result<()>;
    fn undo(&mut self) -> Result<()>;
}

pub struct CreateComponentCommand {
    world: Rc<RefCell<World>>,
    component: Option<Component>,
    entity: Option<Entity>,
}

impl Command for CreateComponentCommand {
    fn execute(&mut self) -> Result<()> {
        let entity = self.world.borrow_mut().create_entity();
        self.world.borrow_mut().add_component(entity, self.component.take().unwrap());
        self.entity = Some(entity);
        Ok(())
    }
    
    fn undo(&mut self) -> Result<()> {
        if let Some(entity) = self.entity {
            self.world.borrow_mut().despawn(entity);
        }
        Ok(())
    }
}

pub struct CommandHistory {
    history: Vec<Box<dyn Command>>,
    current: usize,
}

impl CommandHistory {
    pub fn execute(&mut self, mut command: Box<dyn Command>) -> Result<()> {
        command.execute()?;
        
        // Clear redo history
        self.history.truncate(self.current);
        
        self.history.push(command);
        self.current += 1;
        
        Ok(())
    }
    
    pub fn undo(&mut self) -> Result<()> {
        if self.current > 0 {
            self.current -= 1;
            self.history[self.current].undo()?;
        }
        Ok(())
    }
    
    pub fn redo(&mut self) -> Result<()> {
        if self.current < self.history.len() {
            self.history[self.current].execute()?;
            self.current += 1;
        }
        Ok(())
    }
}

// Usage
let mut history = CommandHistory::new();

history.execute(Box::new(CreateComponentCommand::new(world, button)))?;
history.execute(Box::new(DeleteComponentCommand::new(world, entity)))?;

history.undo()?; // Undoes delete
history.redo()?; // Redoes delete
```

#### **Strategy Pattern** (Optimization Strategies)

```rust
// strategy/optimization.rs

pub trait OptimizationStrategy {
    fn optimize(&self, ast: &mut AST) -> Result<()>;
}

pub struct DeadCodeElimination;

impl OptimizationStrategy for DeadCodeElimination {
    fn optimize(&self, ast: &mut AST) -> Result<()> {
        // Remove unused variables, unreachable code, etc.
        Ok(())
    }
}

pub struct ConstantFolding;

impl OptimizationStrategy for ConstantFolding {
    fn optimize(&self, ast: &mut AST) -> Result<()> {
        // Evaluate constant expressions at compile time
        Ok(())
    }
}

pub struct InlineExpansion;

impl OptimizationStrategy for InlineExpansion {
    fn optimize(&self, ast: &mut AST) -> Result<()> {
        // Inline small functions
        Ok(())
    }
}

pub struct Optimizer {
    strategies: Vec<Box<dyn OptimizationStrategy>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            strategies: vec![
                Box::new(DeadCodeElimination),
                Box::new(ConstantFolding),
                Box::new(InlineExpansion),
            ],
        }
    }
    
    pub fn add_strategy(&mut self, strategy: Box<dyn OptimizationStrategy>) {
        self.strategies.push(strategy);
    }
    
    pub fn optimize(&self, ast: &mut AST, level: OptimizationLevel) -> Result<()> {
        match level {
            OptimizationLevel::None => Ok(()),
            OptimizationLevel::Basic => {
                self.strategies[0].optimize(ast)?; // Dead code only
                Ok(())
            }
            OptimizationLevel::Standard => {
                for strategy in &self.strategies[0..2] {
                    strategy.optimize(ast)?;
                }
                Ok(())
            }
            OptimizationLevel::Aggressive => {
                for strategy in &self.strategies {
                    strategy.optimize(ast)?;
                }
                Ok(())
            }
        }
    }
}

// Usage
let mut optimizer = Optimizer::new();
optimizer.optimize(&mut ast, OptimizationLevel::Aggressive)?;
```

#### **Visitor Pattern** (AST Traversal)

```rust
// visitor/ast_visitor.rs

pub trait ASTVisitor {
    fn visit_module(&mut self, module: &Module) -> Result<()>;
    fn visit_component(&mut self, component: &Component) -> Result<()>;
    fn visit_element(&mut self, element: &Element) -> Result<()>;
    fn visit_expression(&mut self, expr: &Expression) -> Result<()>;
}

// Concrete visitor: Type checker
pub struct TypeChecker {
    type_env: HashMap<String, Type>,
    errors: Vec<TypeError>,
}

impl ASTVisitor for TypeChecker {
    fn visit_module(&mut self, module: &Module) -> Result<()> {
        for component in &module.components {
            self.visit_component(component)?;
        }
        Ok(())
    }
    
    fn visit_component(&mut self, component: &Component) -> Result<()> {
        // Check props types
        for prop in &component.props {
            self.type_env.insert(prop.name.clone(), prop.ty.clone());
        }
        
        // Check template
        for element in &component.template {
            self.visit_element(element)?;
        }
        
        Ok(())
    }
    
    fn visit_element(&mut self, element: &Element) -> Result<()> {
        // Type check attributes
        for attr in &element.attributes {
            self.visit_expression(&attr.value)?;
        }
        Ok(())
    }
    
    fn visit_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Binary { left, op, right } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                
                if !self.are_compatible(left_type, right_type, op) {
                    self.errors.push(TypeError::IncompatibleTypes {
                        left: left_type,
                        right: right_type,
                        op: *op,
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// Concrete visitor: Code generator
pub struct CodeGenerator {
    output: String,
    indent: usize,
}

impl ASTVisitor for CodeGenerator {
    fn visit_module(&mut self, module: &Module) -> Result<()> {
        self.writeln("// Auto-generated code");
        for component in &module.components {
            self.visit_component(component)?;
        }
        Ok(())
    }
    
    fn visit_component(&mut self, component: &Component) -> Result<()> {
        self.writeln(&format!("pub struct {} {{", component.name));
        self.indent();
        // Generate fields...
        self.dedent();
        self.writeln("}");
        Ok(())
    }
    
    fn visit_element(&mut self, element: &Element) -> Result<()> {
        // Generate rendering code
        Ok(())
    }
    
    fn visit_expression(&mut self, expr: &Expression) -> Result<()> {
        // Generate expression code
        Ok(())
    }
}

// Usage
let mut type_checker = TypeChecker::new();
type_checker.visit_module(&ast)?;

let mut code_gen = CodeGenerator::new();
code_gen.visit_module(&ast)?;
let generated_code = code_gen.output;
```

#### **Chain of Responsibility** (Middleware Pipeline)

```rust
// middleware/compilation_pipeline.rs

pub trait CompilationMiddleware {
    fn process(&self, context: &mut CompilationContext) -> Result<()>;
    fn next(&self) -> Option<&dyn CompilationMiddleware>;
}

pub struct ValidationMiddleware {
    next: Option<Box<dyn CompilationMiddleware>>,
}

impl CompilationMiddleware for ValidationMiddleware {
    fn process(&self, context: &mut CompilationContext) -> Result<()> {
        // Validate syntax
        if !self.validate(context) {
            return Err(anyhow!("Validation failed"));
        }
        
        // Pass to next middleware
        if let Some(next) = &self.next {
            next.process(context)?;
        }
        
        Ok(())
    }
    
    fn next(&self) -> Option<&dyn CompilationMiddleware> {
        self.next.as_ref().map(|n| n.as_ref())
    }
}

pub struct OptimizationMiddleware {
    next: Option<Box<dyn CompilationMiddleware>>,
}

impl CompilationMiddleware for OptimizationMiddleware {
    fn process(&self, context: &mut CompilationContext) -> Result<()> {
        // Optimize AST
        self.optimize(context)?;
        
        // Pass to next
        if let Some(next) = &self.next {
            next.process(context)?;
        }
        
        Ok(())
    }
    
    fn next(&self) -> Option<&dyn CompilationMiddleware> {
        self.next.as_ref().map(|n| n.as_ref())
    }
}

// Build pipeline
let pipeline = ValidationMiddleware {
    next: Some(Box::new(OptimizationMiddleware {
        next: Some(Box::new(CodeGenMiddleware {
            next: None,
        })),
    })),
};

pipeline.process(&mut context)?;
```

---

## 5. SOLID Principles Implementation

### 5.1 Single Responsibility Principle (SRP)

**Before (Violates SRP):**
```rust
// BAD: One class doing too many things
pub struct Component {
    name: String,
    props: Props,
    
    // Rendering responsibility
    pub fn render(&self, ctx: &RenderContext) -> Result<()> { ... }
    
    // State management responsibility
    pub fn update_state(&mut self, new_state: State) { ... }
    
    // Serialization responsibility
    pub fn to_json(&self) -> String { ... }
    
    // Validation responsibility
    pub fn validate(&self) -> Result<()> { ... }
}
```

**After (Follows SRP):**
```rust
// GOOD: Each class has one responsibility

// Rendering
pub struct ComponentRenderer {
    context: RenderContext,
}

impl ComponentRenderer {
    pub fn render(&self, component: &Component) -> Result<()> {
        // Only responsible for rendering
    }
}

// State management
pub struct ComponentState {
    data: HashMap<String, Value>,
}

impl ComponentState {
    pub fn update(&mut self, key: &str, value: Value) {
        // Only responsible for state
    }
}

// Serialization
pub struct ComponentSerializer;

impl ComponentSerializer {
    pub fn to_json(&self, component: &Component) -> String {
        // Only responsible for serialization
    }
}

// Validation
pub struct ComponentValidator;

impl ComponentValidator {
    pub fn validate(&self, component: &Component) -> Result<()> {
        // Only responsible for validation
    }
}

// Component now focuses on data
pub struct Component {
    name: String,
    props: Props,
}
```

### 5.2 Open/Closed Principle (OCP)

**Before (Violates OCP):**
```rust
// BAD: Must modify class to add new shapes
pub struct Renderer {
    pub fn render(&self, shape: &Shape) -> Result<()> {
        match shape.kind {
            ShapeKind::Circle => self.render_circle(shape),
            ShapeKind::Rectangle => self.render_rectangle(shape),
            // Must add new match arm for every new shape! ❌
        }
    }
}
```

**After (Follows OCP):**
```rust
// GOOD: Open for extension, closed for modification

pub trait Renderable {
    fn render(&self, ctx: &RenderContext) -> Result<()>;
}

pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Renderable for Circle {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.draw_circle(self.x, self.y, self.radius)
    }
}

pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Renderable for Rectangle {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.draw_rect(self.x, self.y, self.width, self.height)
    }
}

// Adding new shape doesn't require modifying existing code ✅
pub struct Triangle {
    points: [Point; 3],
}

impl Renderable for Triangle {
    fn render(&self, ctx: &RenderContext) -> Result<()> {
        ctx.draw_polygon(&self.points)
    }
}

// Generic renderer
pub struct Renderer;

impl Renderer {
    pub fn render(&self, renderable: &dyn Renderable) -> Result<()> {
        renderable.render(&self.context)
    }
}
```

### 5.3 Liskov Substitution Principle (LSP)

**Before (Violates LSP):**
```rust
// BAD: Square violates LSP

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }
    
    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
}

pub struct Square {
    side: f32,
}

impl Square {
    pub fn set_width(&mut self, width: f32) {
        self.side = width; // Violates expectation!
    }
    
    pub fn set_height(&mut self, height: f32) {
        self.side = height; // Setting height changes width!
    }
}

// This breaks with Square:
fn test_rectangle(rect: &mut dyn Shape) {
    rect.set_width(5.0);
    rect.set_height(10.0);
    assert_eq!(rect.area(), 50.0); // Fails for Square!
}
```

**After (Follows LSP):**
```rust
// GOOD: Proper abstraction

pub trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}

pub struct Square {
    side: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
    
    fn perimeter(&self) -> f32 {
        4.0 * self.side
    }
}

// Works correctly for all shapes ✅
fn calculate_total_area(shapes: &[&dyn Shape]) -> f32 {
    shapes.iter().map(|s| s.area()).sum()
}
```

### 5.4 Interface Segregation Principle (ISP)

**Before (Violates ISP):**
```rust
// BAD: Fat interface

pub trait Component {
    fn render(&self) -> Result<()>;
    fn update(&mut self, delta: f32);
    fn handle_input(&mut self, event: InputEvent);
    fn serialize(&self) -> String;
    fn deserialize(&mut self, data: &str);
    fn animate(&mut self, animation: Animation);
    fn on_mount(&mut self);
    fn on_unmount(&mut self);
    // Many more methods...
}

// Simple component must implement ALL methods ❌
pub struct SimpleText {
    content: String,
}

impl Component for SimpleText {
    fn render(&self) -> Result<()> {
        // OK
    }
    
    fn update(&mut self, delta: f32) {
        // Don't need this!
    }
    
    fn handle_input(&mut self, event: InputEvent) {
        // Don't need this!
    }
    
    fn animate(&mut self, animation: Animation) {
        // Don't need this!
    }
    
    // ... forced to implement 10+ unnecessary methods
}
```

**After (Follows ISP):**
```rust
// GOOD: Segregated interfaces

pub trait Renderable {
    fn render(&self) -> Result<()>;
}

pub trait Updatable {
    fn update(&mut self, delta: f32);
}

pub trait Interactive {
    fn handle_input(&mut self, event: InputEvent);
}

pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(&mut self, data: &str);
}

pub trait Animatable {
    fn animate(&mut self, animation: Animation);
}

pub trait Lifecycle {
    fn on_mount(&mut self);
    fn on_unmount(&mut self);
}

// Simple component only implements what it needs ✅
pub struct SimpleText {
    content: String,
}

impl Renderable for SimpleText {
    fn render(&self) -> Result<()> {
        // Only this!
    }
}

// Complex component implements multiple interfaces
pub struct Button {
    label: String,
    state: ButtonState,
}

impl Renderable for Button {
    fn render(&self) -> Result<()> { ... }
}

impl Interactive for Button {
    fn handle_input(&mut self, event: InputEvent) { ... }
}

impl Animatable for Button {
    fn animate(&mut self, animation: Animation) { ... }
}
```

### 5.5 Dependency Inversion Principle (DIP)

**Before (Violates DIP):**
```rust
// BAD: High-level depends on low-level

pub struct FileCache {
    path: PathBuf,
}

impl FileCache {
    pub fn get(&self, key: &str) -> Option<String> {
        // Read from file
    }
    
    pub fn set(&mut self, key: &str, value: String) {
        // Write to file
    }
}

pub struct Compiler {
    cache: FileCache, // Depends on concrete implementation ❌
}

impl Compiler {
    pub fn compile(&mut self, source: &str) -> Result<String> {
        if let Some(cached) = self.cache.get(source) {
            return Ok(cached);
        }
        
        let result = self.do_compile(source)?;
        self.cache.set(source, result.clone());
        Ok(result)
    }
}
```

**After (Follows DIP):**
```rust
// GOOD: Both depend on abstraction

pub trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
}

pub struct FileCache {
    path: PathBuf,
}

impl Cache for FileCache {
    fn get(&self, key: &str) -> Option<String> {
        // Read from file
    }
    
    fn set(&mut self, key: &str, value: String) {
        // Write to file
    }
}

pub struct MemoryCache {
    data: HashMap<String, String>,
}

impl Cache for MemoryCache {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
    
    fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }
}

pub struct Compiler {
    cache: Box<dyn Cache>, // Depends on abstraction ✅
}

impl Compiler {
    pub fn new(cache: Box<dyn Cache>) -> Self {
        Self { cache }
    }
    
    pub fn compile(&mut self, source: &str) -> Result<String> {
        if let Some(cached) = self.cache.get(source) {
            return Ok(cached);
        }
        
        let result = self.do_compile(source)?;
        self.cache.set(source, result.clone());
        Ok(result)
    }
}

// Easy to switch implementations
let compiler1 = Compiler::new(Box::new(FileCache::new()));
let compiler2 = Compiler::new(Box::new(MemoryCache::new()));
```

---

## 6. Component Design

### 6.1 Compiler Components

```rust
// compiler/src/lib.rs

pub struct OmniCraftCompiler {
    lexer: Lexer,
    parser: Parser,
    analyzer: SemanticAnalyzer,
    type_checker: TypeChecker,
    optimizer: Optimizer,
    code_generator: CodeGenerator,
    config: CompilerConfig,
}

impl OmniCraftCompiler {
    pub fn new(config: CompilerConfig) -> Self {
        Self {
            lexer: Lexer::new(),
            parser: Parser::new(),
            analyzer: SemanticAnalyzer::new(),
            type_checker: TypeChecker::new(),
            optimizer: Optimizer::new(config.optimization_level),
            code_generator: CodeGenerator::new(config.target),
            config,
        }
    }
    
    pub fn compile(&mut self, source: &str) -> Result<CompilationOutput> {
        // 1. Tokenize
        let tokens = self.lexer.tokenize(source)?;
        
        // 2. Parse
        let ast = self.parser.parse(&tokens)?;
        
        // 3. Semantic analysis
        let analyzed = self.analyzer.analyze(&ast)?;
        
        // 4. Type checking
        self.type_checker.check(&analyzed)?;
        
        // 5. Optimize
        let optimized = self.optimizer.optimize(analyzed)?;
        
        // 6. Generate code
        let output = self.code_generator.generate(&optimized)?;
        
        Ok(output)
    }
    
    pub fn compile_incremental(
        &mut self,
        source: &str,
        cache: &mut CompilationCache,
    ) -> Result<CompilationOutput> {
        // Check cache
        let hash = self.compute_hash(source);
        if let Some(cached) = cache.get(&hash) {
            return Ok(cached.clone());
        }
        
        // Compile
        let output = self.compile(source)?;
        
        // Cache result
        cache.insert(hash, output.clone());
        
        Ok(output)
    }
}

pub struct CompilationOutput {
    pub rust_code: String,
    pub wasm_binary: Vec<u8>,
    pub type_definitions: String,
    pub source_map: Option<SourceMap>,
    pub diagnostics: Vec<Diagnostic>,
}
```

### 6.2 Runtime Components

```rust
// runtime/src/lib.rs

pub struct OmniCraftRuntime {
    world: World,
    reactive_system: ReactiveSystem,
    render_pipeline: RenderPipeline,
    event_system: EventSystem,
}

impl OmniCraftRuntime {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self> {
        let context = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        
        Ok(Self {
            world: World::new(),
            reactive_system: ReactiveSystem::new(),
            render_pipeline: RenderPipeline::new(context),
            event_system: EventSystem::new(),
        })
    }
    
    pub fn create_entity(&mut self) -> Entity {
        self.world.spawn()
    }
    
    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) {
        self.world.insert(entity, component);
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Update reactive system
        self.reactive_system.flush_effects();
        
        // Update ECS systems
        self.world.run_systems(delta_time);
        
        // Render
        self.render_pipeline.render(&self.world);
    }
    
    pub fn handle_event(&mut self, event: DomEvent) {
        self.event_system.dispatch(event, &mut self.world);
    }
}

// World management
pub struct World {
    entities: Vec<Entity>,
    components: ComponentStorage,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn spawn(&mut self) -> Entity {
        let entity = Entity(self.entities.len());
        self.entities.push(entity);
        entity
    }
    
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) {
        self.components.insert(entity, component);
    }
    
    pub fn query<Q: Query>(&self) -> QueryIter<Q> {
        QueryIter::new(&self.components)
    }
    
    pub fn run_systems(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system.run(self, delta_time);
        }
    }
}
```

### 6.3 Reactive System Components

```rust
// runtime/src/reactive/mod.rs

pub struct ReactiveSystem {
    signals: Vec<SignalData>,
    effects: Vec<EffectData>,
    batch_queue: Vec<EffectId>,
    is_batching: bool,
}

impl ReactiveSystem {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
            effects: Vec::new(),
            batch_queue: Vec::new(),
            is_batching: false,
        }
    }
    
    pub fn create_signal<T>(&mut self, initial: T) -> SignalId
    where
        T: 'static + Clone,
    {
        let id = SignalId(self.signals.len());
        self.signals.push(SignalData {
            value: Box::new(initial),
            subscribers: Vec::new(),
        });
        id
    }
    
    pub fn create_effect(&mut self, callback: impl Fn() + 'static) -> EffectId {
        let id = EffectId(self.effects.len());
        self.effects.push(EffectData {
            callback: Box::new(callback),
            dependencies: Vec::new(),
        });
        id
    }
    
    pub fn set_signal<T>(&mut self, id: SignalId, value: T)
    where
        T: 'static + Clone,
    {
        if let Some(signal) = self.signals.get_mut(id.0) {
            signal.value = Box::new(value);
            
            // Queue effects
            for effect_id in &signal.subscribers {
                self.batch_queue.push(*effect_id);
            }
            
            // Flush if not batching
            if !self.is_batching {
                self.flush_effects();
            }
        }
    }
    
    pub fn flush_effects(&mut self) {
        let effects: Vec<_> = self.batch_queue.drain(..).collect();
        
        for effect_id in effects {
            if let Some(effect) = self.effects.get(effect_id.0) {
                (effect.callback)();
            }
        }
    }
    
    pub fn batch(&mut self, f: impl FnOnce()) {
        self.is_batching = true;
        f();
        self.is_batching = false;
        self.flush_effects();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SignalId(usize);

#[derive(Copy, Clone, Debug)]
pub struct EffectId(usize);

struct SignalData {
    value: Box<dyn Any>,
    subscribers: Vec<EffectId>,
}

struct EffectData {
    callback: Box<dyn Fn()>,
    dependencies: Vec<SignalId>,
}
```

---

## 7. UML Diagrams

### 7.1 Class Diagram (Core Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│                   <<abstract>>                               │
│                   Component                                  │
├─────────────────────────────────────────────────────────────┤
│ # name: String                                               │
│ # props: HashMap<String, Value>                              │
│ # children: Vec<Box<dyn Component>>                          │
├─────────────────────────────────────────────────────────────┤
│ + render(ctx: &RenderContext): Result<()>                    │
│ + update(delta: f32): void                                   │
│ + add_child(child: Box<dyn Component>): void                 │
└────────────────┬────────────────────────────────────────────┘
                 │
      ┌──────────┼──────────┐
      │          │          │
      ▼          ▼          ▼
┌─────────┐ ┌────────┐ ┌─────────┐
│ Circle  │ │  Rect  │ │  Text   │
├─────────┤ ├────────┤ ├─────────┤
│ x: f32  │ │ x: f32 │ │ x: f32  │
│ y: f32  │ │ y: f32 │ │ y: f32  │
│ radius  │ │ width  │ │ content │
└─────────┘ └────────┘ └─────────┘
```

### 7.2 Sequence Diagram (Compilation Flow)

```
User     CLI      Compiler   Parser   Analyzer   Optimizer   CodeGen   Output
 │        │          │         │         │           │          │         │
 │─compile─>         │         │         │           │          │         │
 │        │──read──> │         │         │           │          │         │
 │        │<─source──│         │         │           │          │         │
 │        │──────────┼──parse─>│         │           │          │         │
 │        │<──────────────AST───│         │           │          │         │
 │        │──────────┼─────────┼analyze─>│           │          │         │
 │        │<──────────────────────graph───│           │          │         │
 │        │──────────┼─────────┼────────┼optimize──>│          │         │
 │        │<────────────────────────────────opt-AST──│          │         │
 │        │──────────┼─────────┼────────┼───────────┼generate─>│         │
 │        │<──────────────────────────────────────────rust-code│         │
 │        │──────────┼─────────┼────────┼───────────┼─────────┼compile─>│
 │        │<─────────────────────────────────────────────────────wasm────│
 │<success│          │         │         │           │          │         │
```

### 7.3 Use Case Diagram

```
┌────────────────────────────────────────────────────────────┐
│                    OmniCraft System                         │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                                                       │ │
│  │  (Create Component)────────(Compile)────────(Export) │ │
│  │       │                      │                │       │ │
│  │       │                      │                │       │ │
│  │       └──────┐               │         ┌──────┘       │ │
│  │              │               │         │              │ │
│  │       (Edit Component)       │    (Deploy)            │ │
│  │              │               │         │              │ │
│  │              │               │         │              │ │
│  │       (Debug)────────────────┘         │              │ │
│  │                                        │              │ │
│  └────────────────────────────────────────┼──────────────┘ │
│                                           │                │
└───────────────────────────────────────────┼────────────────┘
                                            │
                    ┌───────────────────────┼────────────────┐
                    │                       │                │
               ┌────▼────┐           ┌─────▼─────┐    ┌────▼────┐
               │Developer│           │  Designer │    │Marketing│
               └─────────┘           └───────────┘    └─────────┘
```

### 7.4 Activity Diagram (Development Workflow)

```
    [Start]
       │
       ▼
  (Write .omni file)
       │
       ▼
  {Auto-save?}─────No────>(Manual save)
       │                       │
      Yes                      │
       │◄──────────────────────┘
       ▼
  (Compiler triggered)
       │
       ▼
  (Parse source)
       │
       ▼
  {Valid syntax?}──No──>(Show errors)──>(Fix errors)
       │                                      │
      Yes                                     │
       │◄─────────────────────────────────────┘
       ▼
  (Type checking)
       │
       ▼
  {Type errors?}───Yes──>(Show errors)──>(Fix errors)
       │                                      │
       No                                     │
       │◄─────────────────────────────────────┘
       ▼
  (Optimize AST)
       │
       ▼
  (Generate Rust code)
       │
       ▼
  (Compile to WASM)
       │
       ▼
  {Compilation success?}──No──>(Show errors)
       │                              │
      Yes                             │
       │◄───────────────────────────────┘
       ▼
  (Hot reload)
       │
       ▼
  (Update preview)
       │
       ▼
    [Ready]
```

### 7.5 State Diagram (Component Lifecycle)

```
         ┌─────────────┐
         │  CREATED    │
         └──────┬──────┘
                │ mount()
                ▼
         ┌─────────────┐
         │  MOUNTING   │
         └──────┬──────┘
                │ onMount()
                ▼
         ┌─────────────┐
    ┌───>│  MOUNTED    │◄───┐
    │    └──────┬──────┘    │
    │    └──────┬──────┘    │
    │           │           │
    │           │ update()  │
    │           ▼           │
    │    ┌─────────────┐    │
    │    │  UPDATING   │────┘
    │    └──────┬──────┘
    │           │ unmount()
    │           ▼
    │    ┌─────────────┐
    │    │ UNMOUNTING  │
    │    └──────┬──────┘
    │           │ onUnmount()
    │           ▼
    │    ┌─────────────┐
    │    │ UNMOUNTED   │
    │    └──────┬──────┘
    │           │ destroy()
    │           ▼
    │    ┌─────────────┐
    └────│ DESTROYED   │
         └─────────────┘
```

### 7.6 Component Diagram (System Modules)

```
┌───────────────────────────────────────────────────────────┐
│                   OmniCraft Platform                       │
│                                                            │
│  ┌──────────────────────────────────────────────────┐    │
│  │             Frontend Layer                       │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │    │
│  │  │  Web UI  │  │  VSCode  │  │   CLI    │      │    │
│  │  │ (Next.js)│  │   Ext    │  │  (Rust)  │      │    │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘      │    │
│  └───────┼─────────────┼─────────────┼─────────────┘    │
│          │             │             │                   │
│  ┌───────▼─────────────▼─────────────▼─────────────┐    │
│  │            Compiler Layer                        │    │
│  │  ┌──────┐  ┌────────┐  ┌─────────┐  ┌────────┐ │    │
│  │  │Lexer │─>│ Parser │─>│Analyzer │─>│Codegen │ │    │
│  │  └──────┘  └────────┘  └─────────┘  └────────┘ │    │
│  └──────────────────────────┬───────────────────────┘    │
│                             │                            │
│  ┌──────────────────────────▼───────────────────────┐    │
│  │            Runtime Layer (WASM)                  │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │    │
│  │  │Reactive  │  │   ECS    │  │ Renderer │      │    │
│  │  │ System   │  │  Engine  │  │ Pipeline │      │    │
│  │  └──────────┘  └──────────┘  └──────────┘      │    │
│  └──────────────────────────────────────────────────┘    │
│                                                            │
│  ┌──────────────────────────────────────────────────┐    │
│  │         Infrastructure Layer                      │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │    │
│  │  │ Storage  │  │  Cache   │  │   CDN    │      │    │
│  │  │(Supabase)│  │ (Redis)  │  │(Cloudflare│      │    │
│  │  └──────────┘  └──────────┘  └──────────┘      │    │
│  └──────────────────────────────────────────────────┘    │
└───────────────────────────────────────────────────────────┘
```

### 7.7 Deployment Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         Internet                             │
└────────────┬───────────────────────────┬────────────────────┘
             │                           │
┌────────────▼─────────┐    ┌───────────▼──────────┐
│  Cloudflare CDN      │    │  Vercel Edge Network │
│  ┌────────────────┐  │    │  ┌────────────────┐  │
│  │  Static Assets │  │    │  │  Next.js App   │  │
│  │  - WASM files  │  │    │  │  - SSR/SSG     │  │
│  │  - JS bundles  │  │    │  │  - API Routes  │  │
│  │  - CSS         │  │    │  └────────────────┘  │
│  └────────────────┘  │    └───────────┬──────────┘
└──────────────────────┘                │
                                        │
                           ┌────────────▼──────────┐
                           │   Supabase Cloud      │
                           │  ┌─────────────────┐  │
                           │  │   PostgreSQL    │  │
                           │  │   (Database)    │  │
                           │  └─────────────────┘  │
                           │  ┌─────────────────┐  │
                           │  │  Storage Bucket │  │
                           │  │  (User assets)  │  │
                           │  └─────────────────┘  │
                           │  ┌─────────────────┐  │
                           │  │  Auth Service   │  │
                           │  └─────────────────┘  │
                           └───────────────────────┘
```

---

## 8. Database Design

### 8.1 Entity Relationship Diagram (ERD)

```
┌─────────────────┐         ┌─────────────────┐
│      Users      │         │    Projects     │
├─────────────────┤         ├─────────────────┤
│ PK id           │1       *│ PK id           │
│    email        │─────────│ FK user_id      │
│    password     │         │    name         │
│    created_at   │         │    description  │
│    updated_at   │         │    created_at   │
└─────────────────┘         │    updated_at   │
                            └────────┬────────┘
                                     │
                                     │1
                                     │
                                     │*
                            ┌────────▼────────┐
                            │   Components    │
                            ├─────────────────┤
                            │ PK id           │
                            │ FK project_id   │
                            │    name         │
                            │    type         │
                            │    source_code  │
                            │    compiled     │
                            │    version      │
                            │    created_at   │
                            │    updated_at   │
                            └────────┬────────┘
                                     │
                         ┌───────────┼───────────┐
                         │1          │1          │1
                         │           │           │
                         │*          │*          │*
              ┌──────────▼─┐  ┌──────▼──┐  ┌────▼─────┐
              │   Assets   │  │ Versions│  │  Exports │
              ├────────────┤  ├─────────┤  ├──────────┤
              │ PK id      │  │ PK id   │  │ PK id    │
              │ FK comp_id │  │ FK c_id │  │ FK c_id  │
              │    type    │  │  number │  │   format │
              │    url     │  │  source │  │   url    │
              │    size    │  │  date   │  │   size   │
              └────────────┘  └─────────┘  └──────────┘
```

### 8.2 Database Schema (PostgreSQL)

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    avatar_url TEXT,
    subscription_tier VARCHAR(50) DEFAULT 'free',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Projects table
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    settings JSONB DEFAULT '{}',
    is_public BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT projects_name_user_unique UNIQUE (user_id, name)
);

-- Components table
CREATE TABLE components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL, -- 'module', 'component', 'template'
    source_code TEXT NOT NULL,
    compiled_code TEXT,
    metadata JSONB DEFAULT '{}',
    version INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT components_name_project_unique UNIQUE (project_id, name)
);

-- Component versions (for history)
CREATE TABLE component_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component_id UUID NOT NULL REFERENCES components(id) ON DELETE CASCADE,
    version_number INTEGER NOT NULL,
    source_code TEXT NOT NULL,
    compiled_code TEXT,
    commit_message TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT component_versions_unique UNIQUE (component_id, version_number)
);

-- Assets table
CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL, -- 'image', 'video', 'audio', 'font'
    url TEXT NOT NULL,
    size_bytes BIGINT,
    mime_type VARCHAR(100),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT assets_name_project_unique UNIQUE (project_id, name)
);

-- Exports table
CREATE TABLE exports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component_id UUID NOT NULL REFERENCES components(id) ON DELETE CASCADE,
    format VARCHAR(50) NOT NULL, -- 'video', 'gif', 'code', 'svg'
    url TEXT NOT NULL,
    size_bytes BIGINT,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Collaboration (team members)
CREATE TABLE project_collaborators (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'viewer', -- 'owner', 'editor', 'viewer'
    invited_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT project_collaborators_unique UNIQUE (project_id, user_id)
);

-- Component templates (marketplace)
CREATE TABLE component_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    tags TEXT[],
    source_code TEXT NOT NULL,
    preview_url TEXT,
    downloads INTEGER DEFAULT 0,
    rating DECIMAL(3,2),
    is_featured BOOLEAN DEFAULT false,
    is_premium BOOLEAN DEFAULT false,
    price DECIMAL(10,2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Usage analytics
CREATE TABLE analytics_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_projects_user_id ON projects(user_id);
CREATE INDEX idx_components_project_id ON components(project_id);
CREATE INDEX idx_assets_project_id ON assets(project_id);
CREATE INDEX idx_exports_component_id ON exports(component_id);
CREATE INDEX idx_component_versions_component_id ON component_versions(component_id);
CREATE INDEX idx_collaborators_project_id ON project_collaborators(project_id);
CREATE INDEX idx_collaborators_user_id ON project_collaborators(user_id);
CREATE INDEX idx_templates_category ON component_templates(category);
CREATE INDEX idx_templates_featured ON component_templates(is_featured);
CREATE INDEX idx_analytics_user_id ON analytics_events(user_id);
CREATE INDEX idx_analytics_event_type ON analytics_events(event_type);
CREATE INDEX idx_analytics_created_at ON analytics_events(created_at);

-- Full-text search
CREATE INDEX idx_components_name_search ON components USING GIN(to_tsvector('english', name));
CREATE INDEX idx_templates_search ON component_templates USING GIN(
    to_tsvector('english', name || ' ' || COALESCE(description, ''))
);
```

### 8.3 Database Triggers & Functions

```sql
-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_components_updated_at
    BEFORE UPDATE ON components
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Auto-create component version on update
CREATE OR REPLACE FUNCTION create_component_version()
RETURNS TRIGGER AS $$
BEGIN
    -- Only create version if source_code changed
    IF OLD.source_code IS DISTINCT FROM NEW.source_code THEN
        INSERT INTO component_versions (
            component_id,
            version_number,
            source_code,
            compiled_code,
            created_at
        ) VALUES (
            NEW.id,
            NEW.version,
            OLD.source_code,
            OLD.compiled_code,
            OLD.updated_at
        );
        
        -- Increment version
        NEW.version = NEW.version + 1;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER auto_version_component
    BEFORE UPDATE ON components
    FOR EACH ROW
    EXECUTE FUNCTION create_component_version();

-- Increment template downloads
CREATE OR REPLACE FUNCTION increment_template_downloads(template_id UUID)
RETURNS void AS $$
BEGIN
    UPDATE component_templates
    SET downloads = downloads + 1
    WHERE id = template_id;
END;
$$ LANGUAGE plpgsql;
```

### 8.4 Row Level Security (RLS) Policies

```sql
-- Enable RLS
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE components ENABLE ROW LEVEL SECURITY;
ALTER TABLE assets ENABLE ROW LEVEL SECURITY;

-- Users can only see their own projects or projects they collaborate on
CREATE POLICY projects_user_policy ON projects
    FOR ALL
    USING (
        user_id = auth.uid() OR
        EXISTS (
            SELECT 1 FROM project_collaborators
            WHERE project_id = projects.id
            AND user_id = auth.uid()
        )
    );

-- Users can only modify projects they own or have editor role
CREATE POLICY projects_modify_policy ON projects
    FOR UPDATE
    USING (
        user_id = auth.uid() OR
        EXISTS (
            SELECT 1 FROM project_collaborators
            WHERE project_id = projects.id
            AND user_id = auth.uid()
            AND role IN ('owner', 'editor')
        )
    );

-- Components inherit project permissions
CREATE POLICY components_policy ON components
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM projects
            WHERE id = components.project_id
            AND (
                user_id = auth.uid() OR
                EXISTS (
                    SELECT 1 FROM project_collaborators
                    WHERE project_id = projects.id
                    AND user_id = auth.uid()
                )
            )
        )
    );

-- Assets inherit project permissions
CREATE POLICY assets_policy ON assets
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM projects
            WHERE id = assets.project_id
            AND (
                user_id = auth.uid() OR
                EXISTS (
                    SELECT 1 FROM project_collaborators
                    WHERE project_id = projects.id
                    AND user_id = auth.uid()
                )
            )
        )
    );
```

---

## 9. Interface Design (API)

### 9.1 REST API Endpoints

```typescript
// api/routes.ts

/**
 * Authentication
 */
POST   /api/auth/register      // Register new user
POST   /api/auth/login         // Login
POST   /api/auth/logout        // Logout
POST   /api/auth/refresh       // Refresh token
GET    /api/auth/me            // Get current user

/**
 * Projects
 */
GET    /api/projects           // List user's projects
POST   /api/projects           // Create project
GET    /api/projects/:id       // Get project details
PUT    /api/projects/:id       // Update project
DELETE /api/projects/:id       // Delete project
GET    /api/projects/:id/export // Export entire project

/**
 * Components
 */
GET    /api/projects/:projectId/components          // List components
POST   /api/projects/:projectId/components          // Create component
GET    /api/components/:id                          // Get component
PUT    /api/components/:id                          // Update component
DELETE /api/components/:id                          // Delete component
POST   /api/components/:id/compile                  // Compile component
GET    /api/components/:id/versions                 // Get version history
POST   /api/components/:id/versions/:version/restore // Restore version

/**
 * Assets
 */
GET    /api/projects/:projectId/assets    // List assets
POST   /api/projects/:projectId/assets    // Upload asset
GET    /api/assets/:id                    // Get asset
DELETE /api/assets/:id                    // Delete asset
POST   /api/assets/upload                 // Direct upload (S3 presigned)

/**
 * Exports
 */
POST   /api/components/:id/export         // Export component
GET    /api/exports/:id                   // Get export status
GET    /api/exports/:id/download          // Download export

/**
 * Templates (Marketplace)
 */
GET    /api/templates                     // Browse templates
GET    /api/templates/:id                 // Get template details
POST   /api/templates                     // Publish template
POST   /api/templates/:id/install         // Install template
GET    /api/templates/search              // Search templates
GET    /api/templates/categories          // Get categories

/**
 * Collaboration
 */
GET    /api/projects/:id/collaborators    // List collaborators
POST   /api/projects/:id/collaborators    // Invite collaborator
PUT    /api/projects/:id/collaborators/:userId // Update role
DELETE /api/projects/:id/collaborators/:userId // Remove collaborator

/**
 * Compilation Service
 */
POST   /api/compile                       // Compile .omni source
POST   /api/compile/validate              // Validate syntax
POST   /api/compile/types                 // Generate types
```

### 9.2 API Request/Response Examples

#### **Compile Component**

```typescript
// POST /api/components/:id/compile

// Request
{
  "optimizationLevel": "aggressive",
  "target": "wasm",
  "sourceMaps": true
}

// Response (Success)
{
  "success": true,
  "data": {
    "compilationId": "abc123",
    "status": "completed",
    "output": {
      "wasmUrl": "https://cdn.omnicraft.dev/wasm/abc123.wasm",
      "jsGlueUrl": "https://cdn.omnicraft.dev/js/abc123.js",
      "typesUrl": "https://cdn.omnicraft.dev/types/abc123.d.ts",
      "sourceMapUrl": "https://cdn.omnicraft.dev/maps/abc123.map"
    },
    "stats": {
      "bundleSize": 45120,
      "compilationTime": 218,
      "optimizationsApplied": [
        "dead_code_elimination",
        "constant_folding",
        "inline_expansion"
      ]
    },
    "diagnostics": []
  }
}

// Response (Error)
{
  "success": false,
  "error": {
    "code": "COMPILATION_ERROR",
    "message": "Type mismatch in component",
    "diagnostics": [
      {
        "severity": "error",
        "message": "Expected string, found number",
        "location": {
          "file": "Button.omni",
          "line": 15,
          "column": 20,
          "span": 5
        },
        "suggestions": [
          {
            "message": "Convert to string: String(value)",
            "fix": {
              "range": [15, 20, 15, 25],
              "replacement": "String(value)"
            }
          }
        ]
      }
    ]
  }
}
```

#### **Export Component**

```typescript
// POST /api/components/:id/export

// Request
{
  "format": "video",
  "settings": {
    "resolution": "1080p",
    "fps": 60,
    "codec": "h264",
    "quality": "high"
  }
}

// Response
{
  "success": true,
  "data": {
    "exportId": "export-xyz789",
    "status": "processing",
    "estimatedTime": 45,
    "statusUrl": "/api/exports/export-xyz789"
  }
}

// GET /api/exports/:id (Poll for status)
{
  "success": true,
  "data": {
    "exportId": "export-xyz789",
    "status": "completed",
    "downloadUrl": "https://cdn.omnicraft.dev/exports/export-xyz789.mp4",
    "fileSize": 2457600,
    "duration": 5.2,
    "expiresAt": "2026-01-07T12:00:00Z"
  }
}
```

### 9.3 WebSocket API (Real-time Features)

```typescript
// websocket/events.ts

/**
 * Real-time collaboration events
 */

// Client → Server
interface ClientEvents {
  // Join project room
  'project:join': {
    projectId: string;
    userId: string;
  };
  
  // Component edit
  'component:edit': {
    componentId: string;
    changes: Delta; // Operational transform
    cursor: { line: number; column: number };
  };
  
  // Component lock (prevent concurrent edits)
  'component:lock': {
    componentId: string;
    userId: string;
  };
  
  // Component unlock
  'component:unlock': {
    componentId: string;
    userId: string;
  };
  
  // Cursor position
  'cursor:move': {
    componentId: string;
    position: { line: number; column: number };
  };
}

// Server → Client
interface ServerEvents {
  // User joined
  'user:joined': {
    userId: string;
    userName: string;
    avatar: string;
  };
  
  // User left
  'user:left': {
    userId: string;
  };
  
  // Component updated (by another user)
  'component:updated': {
    componentId: string;
    changes: Delta;
    userId: string;
  };
  
  // Component locked
  'component:locked': {
    componentId: string;
    userId: string;
    userName: string;
  };
  
  // Component unlocked
  'component:unlocked': {
    componentId: string;
    userId: string;
  };
  
  // Cursor moved (another user)
  'cursor:moved': {
    userId: string;
    componentId: string;
    position: { line: number; column: number };
  };
  
  // Compilation completed
  'compilation:completed': {
    componentId: string;
    success: boolean;
    output?: CompilationOutput;
    errors?: Diagnostic[];
  };
}
```

### 9.4 GraphQL Schema (Alternative API)

```graphql
# schema.graphql

type User {
  id: ID!
  email: String!
  name: String
  avatarUrl: String
  subscriptionTier: SubscriptionTier!
  projects: [Project!]!
  createdAt: DateTime!
}

enum SubscriptionTier {
  FREE
  PRO
  TEAM
  ENTERPRISE
}

type Project {
  id: ID!
  name: String!
  description: String
  owner: User!
  collaborators: [Collaborator!]!
  components: [Component!]!
  assets: [Asset!]!
  isPublic: Boolean!
  createdAt: DateTime!
  updatedAt: DateTime!
}

type Collaborator {
  user: User!
  role: CollaboratorRole!
  joinedAt: DateTime!
}

enum CollaboratorRole {
  OWNER
  EDITOR
  VIEWER
}

type Component {
  id: ID!
  project: Project!
  name: String!
  type: ComponentType!
  sourceCode: String!
  compiledCode: String
  metadata: JSON
  version: Int!
  versions: [ComponentVersion!]!
  exports: [Export!]!
  isActive: Boolean!
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum ComponentType {
  MODULE
  COMPONENT
  TEMPLATE
}

type ComponentVersion {
  id: ID!
  component: Component!
  versionNumber: Int!
  sourceCode: String!
  compiledCode: String
  commitMessage: String
  createdBy: User
  createdAt: DateTime!
}

type Asset {
  id: ID!
  project: Project!
  name: String!
  type: AssetType!
  url: String!
  sizeBytes: Int!
  mimeType: String
  metadata: JSON
  createdAt: DateTime!
}

enum AssetType {
  IMAGE
  VIDEO
  AUDIO
  FONT
}

type Export {
  id: ID!
  component: Component!
  format: ExportFormat!
  url: String!
  sizeBytes: Int!
  settings: JSON
  createdAt: DateTime!
}

enum ExportFormat {
  VIDEO
  GIF
  CODE
  SVG
  LOTTIE
}

type CompilationResult {
  success: Boolean!
  output: CompilationOutput
  diagnostics: [Diagnostic!]!
  stats: CompilationStats
}

type CompilationOutput {
  wasmUrl: String!
  jsGlueUrl: String!
  typesUrl: String
  sourceMapUrl: String
}

type Diagnostic {
  severity: DiagnosticSeverity!
  message: String!
  location: SourceLocation!
  suggestions: [Suggestion!]!
}

enum DiagnosticSeverity {
  ERROR
  WARNING
  INFO
}

type SourceLocation {
  file: String!
  line: Int!
  column: Int!
  span: Int!
}

type Suggestion {
  message: String!
  fix: CodeFix
}

type CodeFix {
  range: SourceRange!
  replacement: String!
}

type SourceRange {
  startLine: Int!
  startColumn: Int!
  endLine: Int!
  endColumn: Int!
}

type CompilationStats {
  bundleSize: Int!
  compilationTime: Int!
  optimizationsApplied: [String!]!
}

# Queries
type Query {
  me: User
  project(id: ID!): Project
  projects: [Project!]!
  component(id: ID!): Component
  template(id: ID!): ComponentTemplate
  templates(
    category: String
    featured: Boolean
    search: String
    limit: Int
    offset: Int
  ): TemplateConnection!
}

type TemplateConnection {
  nodes: [ComponentTemplate!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}

type ComponentTemplate {
  id: ID!
  author: User!
  name: String!
  description: String
  category: String
  tags: [String!]!
  sourceCode: String!
  previewUrl: String
  downloads: Int!
  rating: Float
  isFeatured: Boolean!
  isPremium: Boolean!
  price: Float
  createdAt: DateTime!
}

# Mutations
type Mutation {
  # Projects
  createProject(input: CreateProjectInput!): Project!
  updateProject(id: ID!, input: UpdateProjectInput!): Project!
  deleteProject(id: ID!): Boolean!
  
  # Components
  createComponent(input: CreateComponentInput!): Component!
  updateComponent(id: ID!, input: UpdateComponentInput!): Component!
  deleteComponent(id: ID!): Boolean!
  compileComponent(id: ID!, input: CompileInput!): CompilationResult!
  restoreComponentVersion(id: ID!, version: Int!): Component!
  
  # Assets
  uploadAsset(input: UploadAssetInput!): Asset!
  deleteAsset(id: ID!): Boolean!
  
  # Exports
  exportComponent(id: ID!, input: ExportInput!): Export!
  
  # Collaboration
  inviteCollaborator(projectId: ID!, email: String!, role: CollaboratorRole!): Collaborator!
  updateCollaboratorRole(projectId: ID!, userId: ID!, role: CollaboratorRole!): Collaborator!
  removeCollaborator(projectId: ID!, userId: ID!): Boolean!
  
  # Templates
  publishTemplate(input: PublishTemplateInput!): ComponentTemplate!
  installTemplate(id: ID!, projectId: ID!): Component!
}

# Input types
input CreateProjectInput {
  name: String!
  description: String
  isPublic: Boolean
}

input UpdateProjectInput {
  name: String
  description: String
  isPublic: Boolean
  settings: JSON
}

input CreateComponentInput {
  projectId: ID!
  name: String!
  type: ComponentType!
  sourceCode: String!
}

input UpdateComponentInput {
  name: String
  sourceCode: String
  metadata: JSON
}

input CompileInput {
  optimizationLevel: OptimizationLevel!
  target: CompilationTarget!
  sourceMaps: Boolean
}

enum OptimizationLevel {
  NONE
  BASIC
  STANDARD
  AGGRESSIVE
}

enum CompilationTarget {
  WASM
  JS
  BOTH
}

input UploadAssetInput {
  projectId: ID!
  name: String!
  type: AssetType!
  file: Upload!
}

input ExportInput {
  format: ExportFormat!
  settings: JSON
}

input PublishTemplateInput {
  name: String!
  description: String
  category: String
  tags: [String!]!
  sourceCode: String!
  previewUrl: String
  isPremium: Boolean
  price: Float
}

# Subscriptions (Real-time)
type Subscription {
  componentUpdated(componentId: ID!): Component!
  projectCollaboratorJoined(projectId: ID!): Collaborator!
  compilationCompleted(componentId: ID!): CompilationResult!
}

# Custom scalars
scalar DateTime
scalar JSON
scalar Upload
```

---

## 10. UI/UX Design

### 10.1 Design System

#### **Color Palette**

```css
/* colors.css */

:root {
  /* Primary colors */
  --primary-50: #e0f7ff;
  --primary-100: #b3e9ff;
  --primary-200: #80dbff;
  --primary-300: #4dcdff;
  --primary-400: #26c2ff;
  --primary-500: #00b8ff; /* Main brand color */
  --primary-600: #00a8e8;
  --primary-700: #0094cc;
  --primary-800: #0081b0;
  --primary-900: #006085;
  
  /* Secondary colors */
  --secondary-50: #fff4e6;
  --secondary-100: #ffe4c1;
  --secondary-200: #ffd299;
  --secondary-300: #ffc070;
  --secondary-400: #ffb252;
  --secondary-500: #ffa433;
  --secondary-600: #ff962e;
  --secondary-700: #ff8526;
  --secondary-800: #ff751f;
  --secondary-900: #ff5711;
  
  /* Neutral colors */
  --gray-50: #fafafa;
  --gray-100: #f5f5f5;
  --gray-200: #e5e5e5;
  --gray-300: #d4d4d4;
  --gray-400: #a3a3a3;
  --gray-500: #737373;
  --gray-600: #525252;
  --gray-700: #404040;
  --gray-800: #262626;
  --gray-900: #171717;
  
  /* Semantic colors */
  --success: #22c55e;
  --warning: #f59e0b;
  --error: #ef4444;
  --info: #3b82f6;
  
  /* Surface colors (dark mode) */
  --surface-0: #0a0a0a;
  --surface-1: #141414;
  --surface-2: #1e1e1e;
  --surface-3: #282828;
  --surface-4: #323232;
}
```

#### **Typography**

```css
/* typography.css */

:root {
  /* Font families */
  --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', monospace;
  
  /* Font sizes */
  --text-xs: 0.75rem;    /* 12px */
  --text-sm: 0.875rem;   /* 14px */
  --text-base: 1rem;     /* 16px */
  --text-lg: 1.125rem;   /* 18px */
  --text-xl: 1.25rem;    /* 20px */
  --text-2xl: 1.5rem;    /* 24px */
  --text-3xl: 1.875rem;  /* 30px */
  --text-4xl: 2.25rem;   /* 36px */
  --text-5xl: 3rem;      /* 48px */
  
  /* Font weights */
  --font-normal: 400;
  --font-medium: 500;
  --font-semibold: 600;
  --font-bold: 700;
  
  /* Line heights */
  --leading-tight: 1.25;
  --leading-normal: 1.5;
  --leading-relaxed: 1.75;
  
  /* Letter spacing */
  --tracking-tight: -0.025em;
  --tracking-normal: 0;
  --tracking-wide: 0.025em;
}

/* Typography classes */
.heading-1 {
  font-size: var(--text-5xl);
  font-weight: var(--font-bold);
  line-height: var(--leading-tight);
  letter-spacing: var(--tracking-tight);
}

.heading-2 {
  font-size: var(--text-4xl);
  font-weight: var(--font-bold);
  line-height: var(--leading-tight);
}

.heading-3 {
  font-size: var(--text-3xl);
  font-weight: var(--font-semibold);
  line-height: var(--leading-tight);
}

.body-large {
  font-size: var(--text-lg);
  line-height: var(--leading-normal);
}

.body {
  font-size: var(--text-base);
  line-height: var(--leading-normal);
}

.body-small {
  font-size: var(--text-sm);
  line-height: var(--leading-normal);
}

.code {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  line-height: var(--leading-relaxed);
}
```

#### **Spacing Scale**

```css
/* spacing.css */

:root {
  --space-0: 0;
  --space-1: 0.25rem;   /* 4px */
  --space-2: 0.5rem;    /* 8px */
  --space-3: 0.75rem;   /* 12px */
  --space-4: 1rem;      /* 16px */
  --space-5: 1.25rem;   /* 20px */
  --space-6: 1.5rem;    /* 24px */
  --space-8: 2rem;      /* 32px */
  --space-10: 2.5rem;   /* 40px */
  --space-12: 3rem;     /* 48px */
  --space-16: 4rem;     /* 64px */
  --space-20: 5rem;     /* 80px */
  --space-24: 6rem;     /* 96px */
}
```

#### **Component Library**

```typescript
// components/Button.tsx

import { cn } from '@/lib/utils';

interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
  loading?: boolean;
}

export function Button({
  variant = 'primary',
  size = 'md',
  children,
  onClick,
  disabled,
  loading,
}: ButtonProps) {
  return (
    <button
      className={cn(
        'inline-flex items-center justify-center rounded-lg font-medium transition-colors',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2',
        'disabled:opacity-50 disabled:cursor-not-allowed',
        {
          'bg-primary-500 text-white hover:bg-primary-600': variant === 'primary',
          'bg-gray-200 text-gray-900 hover:bg-gray-300': variant === 'secondary',
          'bg-transparent hover:bg-gray-100': variant === 'ghost',
          'bg-error text-white hover:bg-red-600': variant === 'danger',
          'px-3 py-1.5 text-sm': size === 'sm',
          'px-4 py-2 text-base': size === 'md',
          'px-6 py-3 text-lg': size === 'lg',
        }
      )}
      onClick={onClick}
      disabled={disabled || loading}
    >
      {loading && <Spinner className="mr-2" />}
      {children}
    </button>
  );
}
```

### 10.2 User Interface Layout

#### **Main Application Layout**

```
┌─────────────────────────────────────────────────────────────┐
│ Top Bar (Fixed)                                             │
│ ┌─────────┐  ┌──────────────────┐  ┌─────────────────┐   │
│ │  Logo   │  │  Project: MyApp  │  │ User Menu ▼     │   │
│ └─────────┘  └──────────────────┘  └─────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌───────────┬─────────────────────────────┬────────────┐ │
│  │           │                             │            │ │
│  │  Left     │      Main Canvas Area       │   Right    │ │
│  │  Sidebar  │                             │   Sidebar  │ │
│  │           │  ┌───────────────────────┐  │            │ │
│  │ ┌───────┐ │  │                       │  │ ┌────────┐ │ │
│  │ │Layers │ │  │      Canvas           │  │ │Props   │ │ │
│  │ └───────┘ │  │      (800x600)        │  │ └────────┘ │ │
│  │           │  │                       │  │            │ │
│  │ ┌───────┐ │  └───────────────────────┘  │ ┌────────┐ │ │
│  │ │Assets │ │                             │ │Styles  │ │ │
│  │ └───────┘ │  ┌───────────────────────┐  │ └────────┘ │ │
│  │           │  │      Timeline         │  │            │ │
│  │ ┌───────┐ │  │  ┌──┬──┬──┬──┬──┐    │  │ ┌────────┐ │ │
│  │ │Comps  │ │  │  │  │  │  │  │  │    │  │ │Events  │ │ │
│  │ └───────┘ │  │  └──┴──┴──┴──┴──┘    │  │ └────────┘ │ │
│  │           │  └───────────────────────┘  │            │ │
│  └───────────┴─────────────────────────────┴────────────┘ │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### **Code Editor View**

```
┌─────────────────────────────────────────────────────────────┐
│ File: Button.omni                              [Save] [×]   │
├─────────────────────────────────────────────────────────────┤
│  1  │ <component name="Button">                             │
│  2  │   <props>                                             │
│  3  │     <prop name="label" type="string" />              │
│  4  │   </props>                                            │
│  5  │                                                       │
│  6  │   <script>                                            │
│  7  │     const isHovered = signal(false);                 │
│  8  │   </script>                                           │
│  9  │                                                       │
│ 10  │   <template>                                          │
│ 11  │     <rect                                             │
│ 12  │       width={100}                                     │
│ 13  │       height={40}                                     │
│ 14  │       fill={isHovered() ? '#66bb6a' : '#4caf50'}    │
│ 15  │       @mouseenter={() => isHovered.set(true)}        │
│ 16  │     />                                                │
│ 17  │   </template>                                         │
│ 18  │ </component>                                          │
│     │                                                       │
├─────────────────────────────────────────────────────────────┤
│ ✓ No errors  │  2 warnings  │  Line 15, Col 8             │
└─────────────────────────────────────────────────────────────┘
```

### 10.3 User Flows

#### **Flow 1: Create New Component**

```
[Dashboard]
    │
    ├─> Click "New Component"
    │
    ▼
[Component Template Selection]
    │
    ├─> Choose "Blank"
    ├─> Choose "Button"
    ├─> Choose "Card"
    └─> Choose "From Template"
    │
    ▼
[Editor Opens]
    │
    ├─> Edit code
    ├─> Preview updates live
    ├─> Adjust properties
    │
    ▼
[Save Component]
    │
    ├─> Auto-compile
    ├─> Show compilation result
    │
    ▼
[Component Ready]
    │
    ├─> Export as video
    ├─> Export as code
    └─> Share with team
```

#### **Flow 2: Collaborate on Project**

```
[Receive Invitation Email]
    │
    ├─> Click "Accept Invitation"
    │
    ▼
[Join Project]
    │
    ├─> See project overview
    ├─> See team members
    │
    ▼
[Open Component]
    │
    ├─> See live cursors (other users)
    ├─> Locked components (editing by others)
    │
    ▼
[Edit Component]
    │
    ├─> Changes synced in real-time
    ├─> Chat with team (sidebar)
    ├─> Comment on code
    │
    ▼
[Save & Deploy]
```

### 10.4 Responsive Design

#### **Breakpoints**

```css
/* breakpoints.css */

:root {
  --breakpoint-sm: 640px;   /* Mobile */
  --breakpoint-md: 768px;   /* Tablet */
  --breakpoint-lg: 1024px;  /* Desktop */
  --breakpoint-xl: 1280px;  /* Large Desktop */
  --breakpoint-2xl: 1536px; /* Extra Large */
}

/* Mobile First Approach */

/* Base styles (mobile) */
.container {
  padding: var(--space-4);
}

/* Tablet and up */
@media (min-width: 768px) {
  .container {
    padding: var(--space-6);
  }
  
  .sidebar {
    display: block;
  }
}

/* Desktop and up */
@media (min-width: 1024px) {
  .container {
    padding: var(--space-8);
  }
  
  .editor {
    grid-template-columns: 240px 1fr 300px;
  }
}
```

#### **Mobile Layout Adaptations**

```
Mobile (< 768px):
┌─────────────────┐
│   Top Bar       │
├─────────────────┤
│                 │
│   Canvas        │
│   (Full Width)  │
│                 │
├─────────────────┤
│ [Layers] [Props]│ ← Bottom Sheet (Collapsible)
└─────────────────┘

Tablet (768px - 1024px):
┌──────────────────────────┐
│      Top Bar             │
├────────┬─────────────────┤
│        │                 │
│ Sidebar│     Canvas      │
│        │                 │
├────────┴─────────────────┤
│       Timeline           │
└──────────────────────────┘

Desktop (> 1024px):
Full layout (3 columns)
```

### 10.5 Accessibility (a11y)

```typescript
// Accessibility features

// 1. Keyboard Navigation
const KeyboardShortcuts = {
  'Ctrl+S': 'Save',
  'Ctrl+Z': 'Undo',
  'Ctrl+Shift+Z': 'Redo',
  'Ctrl+C': 'Copy',
  'Ctrl+V': 'Paste',
  'Del': 'Delete',
  'F5': 'Preview',
  'Ctrl+E': 'Export',
};

// 2. ARIA Labels
<button 
  aria-label="Create new component"
  aria-describedby="tooltip-new-component"
>
  <PlusIcon />
</button>

// 3. Focus Management
<div 
  tabIndex={0}
  onKeyDown={handleKeyDown}
  role="toolbar"
  aria-label="Editor toolbar"
>
  {/* Toolbar items */}
</div>

// 4. Screen Reader Support
<div role="status" aria-live="polite" aria-atomic="true">
  {compilationStatus === 'success' 
    ? 'Compilation completed successfully' 
    : 'Compilation failed with errors'}
</div>

// 5. Color Contrast
// All text meets WCAG AA standard (4.5:1 ratio)
// Important UI elements meet AAA standard (7:1 ratio)

// 6. Motion Preferences
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## 11. Security Design

### 11.1 Authentication & Authorization

#### **Authentication Flow**

```typescript
// auth/flow.ts

/**
 * JWT-based authentication with refresh tokens
 */

// 1. Login
POST /api/auth/login
{
  email: "user@example.com",
  password: "hashed_password"
}

// Response
{
  accessToken: "eyJhbGc...", // Short-lived (15 min)
  refreshToken: "dGhpcyB...", // Long-lived (7 days)
  user: {
    id: "uuid",
    email: "user@example.com",
    name: "John Doe"
  }
}

// 2. Access Protected Resource
GET /api/projects
Headers: {
  Authorization: "Bearer eyJhbGc..."
}

// 3. Token Expiry → Refresh
POST /api/auth/refresh
{
  refreshToken: "dGhpcyB..."
}

// Response
{
  accessToken: "new_token...",
  refreshToken: "new_refresh..."
}

// 4. Logout (Invalidate tokens)
POST /api/auth/logout
{
  refreshToken: "dGhpcyB..."
}
```

#### **Authorization Levels**

```typescript
// auth/permissions.ts

enum Permission {
  // Projects
  PROJECT_CREATE = 'project:create',
  PROJECT_READ = 'project:read',
  PROJECT_UPDATE = 'project:update',
  PROJECT_DELETE = 'project:delete',
  PROJECT_SHARE = 'project:share',
  
  // Components
  COMPONENT_CREATE = 'component:create',
  COMPONENT_READ = 'component:read',
  COMPONENT_UPDATE = 'component:update',
  COMPONENT_DELETE = 'component:delete',
  
  // Compilation
  COMPILE_RUN = 'compile:run',
  COMPILE_EXPORT = 'compile:export',
  
  // Templates
  TEMPLATE_PUBLISH = 'template:publish',
  TEMPLATE_INSTALL = 'template:install',
}

const RolePermissions = {
  FREE: [
    Permission.PROJECT_CREATE, // Limited to 3
    Permission.PROJECT_READ,
    Permission.COMPONENT_CREATE,
    Permission.COMPONENT_READ,
    Permission.COMPILE_RUN, // 720p only
  ],
  
  PRO: [
    Permission.PROJECT_CREATE, // Unlimited
    Permission.PROJECT_READ,
    Permission.PROJECT_UPDATE,
    Permission.PROJECT_DELETE,
    Permission.COMPONENT_CREATE,
    Permission.COMPONENT_READ,
    Permission.COMPONENT_UPDATE,
    Permission.COMPONENT_DELETE,
    Permission.COMPILE_RUN, // 4K
    Permission.COMPILE_EXPORT,
  ],
  
  TEAM: [
    ...RolePermissions.PRO,
    Permission.PROJECT_SHARE,
    Permission.TEMPLATE_PUBLISH,
  ],
  
  ENTERPRISE: [
    ...RolePermissions.TEAM,
    // All permissions
  ],
};

// Check permission
function hasPermission(user: User, permission: Permission): boolean {
  const rolePermissions = RolePermissions[user.subscriptionTier];
  return rolePermissions.includes(permission);
}
```

### 11.2 Input Validation & Sanitization

```typescript
// validation/schemas.ts

import { z } from 'zod';

// Component schema
export const ComponentSchema = z.object({
  name: z.string()
    .min(1, 'Name is required')
    .max(100, 'Name too long')
    .regex(/^[a-zA-Z0-9_-]+$/, 'Invalid characters'),
  
  type: z.enum(['module', 'component', 'template']),
  
  sourceCode: z.string()
    .min(1, 'Source code is required')
    .max(100000, 'Source code too large'), // 100 KB limit
  
  metadata: z.object({}).passthrough().optional(),
});

// Project schema
export const ProjectSchema = z.object({
  name: z.string()
    .min(1, 'Name is required')
    .max(100, 'Name too long'),
  
  description: z.string()
    .max(500, 'Description too long')
    .optional(),
  
  isPublic: z.boolean().default(false),
});

// Asset upload schema
export const AssetUploadSchema = z.object({
  name: z.string().min(1).max(255),
  type: z.enum(['image', 'video', 'audio', 'font']),
  file: z.instanceof(File)
    .refine((file) => file.size <= 10 * 1024 * 1024, {
      message: 'File size must be less than 10MB',
    })
    .refine((file) => {
      const allowedTypes = [
        'image/jpeg',
        'image/png',
        'image/gif',
        'image/webp',
        'video/mp4',
      ];
      return allowedTypes.includes(file.type);
    }, {
      message: 'Invalid file type',
    }),
});

// Sanitize HTML (prevent XSS)
import DOMPurify from 'isomorphic-dompurify';

export function sanitizeHTML(html: string): string {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a'],
    ALLOWED_ATTR: ['href'],
  });
}

// Sanitize file paths (prevent path traversal)
export function sanitizePath(path: string): string {
  return path.replace(/\.\./g, '').replace(/\//g, '_');
}
```

### 11.3 API Rate Limiting

```typescript
// middleware/rate-limit.ts

import { Ratelimit } from '@upstash/ratelimit';
import { Redis } from '@upstash/redis';

const redis = new Redis({
  url: process.env.REDIS_URL,
  token: process.env.REDIS_TOKEN,
});

// Different limits for different tiers
const rateLimiters = {
  free: new Ratelimit({
    redis,
    limiter: Ratelimit.slidingWindow(10, '1 m'), // 10 requests per minute
  }),
  
  pro: new Ratelimit({
    redis,
    limiter: Ratelimit.slidingWindow(100, '1 m'), // 100 requests per minute
  }),
  
  team: new Ratelimit({
    redis,
    limiter: Ratelimit.slidingWindow(500, '1 m'), // 500 requests per minute
  }),
  
  enterprise: new Ratelimit({
    redis,
    limiter: Ratelimit.slidingWindow(10000, '1 m'), // 10k requests per minute
  }),
};

export async function rateLimitMiddleware(
  req: Request,
  user: User
): Promise<Response | null> {
  const limiter = rateLimiters[user.subscriptionTier];
  const { success, limit, remaining, reset } = await limiter.limit(user.id);
  
  if (!success) {
    return new Response('Rate limit exceeded', {
      status: 429,
      headers: {
        'X-RateLimit-Limit': limit.toString(),
        'X-RateLimit-Remaining': remaining.toString(),
        'X-RateLimit-Reset': reset.toString(),
      },
    });
  }
  
  return null; // Continue
}
```

### 11.4 Content Security Policy (CSP)

```typescript
// security/csp.ts

export const CSP_HEADERS = {
  'Content-Security-Policy': [
    "default-src 'self'",
    "script-src 'self' 'unsafe-eval' 'unsafe-inline' https://cdn.omnicraft.dev",
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' data: https:",
    "font-src 'self' data:",
    "connect-src 'self' https://api.omnicraft.dev wss://realtime.omnicraft.dev",
    "media-src 'self' https://cdn.omnicraft.dev",
    "object-src 'none'",
    "frame-src 'none'",
    "base-uri 'self'",
    "form-action 'self'",
    "frame-ancestors 'none'",
    "upgrade-insecure-requests",
  ].join('; '),
  
  'X-Content-Type-Options': 'nosniff',
  'X-Frame-Options': 'DENY',
  'X-XSS-Protection': '1; mode=block',
  'Referrer-Policy': 'strict-origin-when-cross-origin',
  'Permissions-Policy': 'geolocation=(), microphone=(), camera=()',
};
```

### 11.5 Secure Code Generation

```rust
// compiler/src/security.rs

/// Sanitize generated code to prevent code injection
pub struct CodeSanitizer;

impl CodeSanitizer {
    pub fn sanitize_rust_code(&self, code: &str) -> Result<String> {
        // 1. Check for dangerous patterns
        let dangerous_patterns = [
            r"std::process::Command",
            r"std::fs::remove",
            r"unsafe\s*\{",
            r"libc::",
            r"#\[no_mangle\]",
        ];
        
        for pattern in &dangerous_patterns {
            if Regex::new(pattern)?.is_match(code) {
                return Err(anyhow!("Dangerous code pattern detected: {}", pattern));
            }
        }
        
        // 2. Limit imports
        let allowed_imports = [
            "omnicraft_runtime",
            "wasm_bindgen",
            "web_sys",
            "serde",
        ];
        
        // Parse imports
        for line in code.lines() {
            if line.trim_start().starts_with("use ") {
                let import = line.trim_start()
                    .strip_prefix("use ")
                    .unwrap()
                    .split("::")
                    .next()
                    .unwrap();
                
                if !allowed_imports.contains(&import) {
                    return Err(anyhow!("Disallowed import: {}", import));
                }
            }
        }
        
        // 3. Sandbox WASM execution
        // (WASM already sandboxed by design, but add extra checks)
        
        Ok(code.to_string())
    }
}
```

### 11.6 Secrets Management

```typescript
// config/secrets.ts

/**
 * Environment variables (Never commit to git!)
 */

interface Secrets {
  // Database
  DATABASE_URL: string;
  DATABASE_PASSWORD: string;
  
  // Authentication
  JWT_SECRET: string;
  JWT_REFRESH_SECRET: string;
  
  // Storage
  S3_ACCESS_KEY: string;
  S3_SECRET_KEY: string;
  S3_BUCKET: string;
  
  // External APIs
  STRIPE_SECRET_KEY: string;
  SENDGRID_API_KEY: string;
  
  // Encryption
  ENCRYPTION_KEY: string; // For encrypting sensitive data
}

// Load from environment
export const secrets: Secrets = {
  DATABASE_URL: process.env.DATABASE_URL!,
  DATABASE_PASSWORD: process.env.DATABASE_PASSWORD!,
  JWT_SECRET: process.env.JWT_SECRET!,
  JWT_REFRESH_SECRET: process.env.JWT_REFRESH_SECRET!,
  S3_ACCESS_KEY: process.env.S3_ACCESS_KEY!,
  S3_SECRET_KEY: process.env.S3_SECRET_KEY!,
  S3_BUCKET: process.env.S3_BUCKET!,
  STRIPE_SECRET_KEY: process.env.STRIPE_SECRET_KEY!,
  SENDGRID_API_KEY: process.env.SENDGRID_API_KEY!,
  ENCRYPTION_KEY: process.env.ENCRYPTION_KEY!,
};

// Validate all secrets are present
Object.entries(secrets).forEach(([key, value]) => {
  if (!value) {
    throw new Error(`Missing required secret: ${key}`);
  }
});

// Encrypt sensitive data before storing
import crypto from 'crypto';

export function encrypt(text: string): string {
  const cipher = crypto.createCipher('aes-256-cbc', secrets.ENCRYPTION_KEY);
  let encrypted = cipher.update(text, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

export function decrypt(encrypted: string): string {
  const decipher = crypto.createDecipher('aes-256-cbc', secrets.ENCRYPTION_KEY);
  let decrypted = decipher.update(encrypted, 'hex', 'utf8');
  decrypted += decipher.final('utf8');
  return decrypted;
}
```

---

## 12. Scalability & Performance

### 12.1 Horizontal Scaling Strategy

```
┌──────────────────────────────────────────────────────────┐
┌──────────────────────────────────────────────────────────┐
│                    Load Balancer                          │
│                  (Cloudflare + Vercel)                    │
└────────────┬──────────────────┬──────────────────────────┘
             │                  │
    ┌────────▼────────┐  ┌──────▼────────┐
    │  Edge Region 1  │  │ Edge Region 2 │  ... (Global)
    │   (US-East)     │  │   (EU-West)   │
    └────────┬────────┘  └──────┬────────┘
             │                  │
    ┌────────▼──────────────────▼────────┐
    │        Application Servers          │
    │  (Auto-scaling, Serverless)         │
    │  ┌──────┐ ┌──────┐ ┌──────┐        │
    │  │ API  │ │ API  │ │ API  │  ...   │
    │  │ Node │ │ Node │ │ Node │        │
    │  └──┬───┘ └──┬───┘ └──┬───┘        │
    └─────┼────────┼────────┼─────────────┘
          │        │        │
    ┌─────▼────────▼────────▼─────────────┐
    │      Database Cluster                │
    │  ┌─────────┐  ┌─────────┐          │
    │  │ Primary │──│ Replica │  ...      │
    │  │  (RW)   │  │  (RO)   │          │
    │  └─────────┘  └─────────┘          │
    └──────────────────────────────────────┘
```

### 12.2 Caching Strategy

```typescript
// cache/strategy.ts

/**
 * Multi-layer caching
 */

// Layer 1: Browser Cache (Service Worker)
// - Static assets (WASM, JS, CSS)
// - Component previews
// - User preferences

// Layer 2: CDN Cache (Cloudflare)
// - Compiled WASM modules
// - Static assets
// - API responses (public data)

// Layer 3: Redis Cache (Backend)
// - User sessions
// - Compilation results
// - Database queries

interface CacheConfig {
  browser: {
    maxAge: number;
    strategy: 'cache-first' | 'network-first';
  };
  cdn: {
    maxAge: number;
    swr: number; // Stale-while-revalidate
  };
  redis: {
    ttl: number; // Time to live
  };
}

const cacheConfig: Record<string, CacheConfig> = {
  // Compiled WASM modules
  'wasm-modules': {
    browser: { maxAge: 86400, strategy: 'cache-first' }, // 1 day
    cdn: { maxAge: 2592000, swr: 86400 }, // 30 days, SWR 1 day
    redis: { ttl: 604800 }, // 7 days
  },
  
  // Component source code
  'component-source': {
    browser: { maxAge: 0, strategy: 'network-first' }, // Always fresh
    cdn: { maxAge: 0, swr: 0 },
    redis: { ttl: 3600 }, // 1 hour
  },
  
  // User projects list
  'user-projects': {
    browser: { maxAge: 300, strategy: 'network-first' }, // 5 min
    cdn: { maxAge: 0, swr: 0 },
    redis: { ttl: 600 }, // 10 min
  },
  
  // Templates (marketplace)
  'templates': {
    browser: { maxAge: 3600, strategy: 'cache-first' }, // 1 hour
    cdn: { maxAge: 86400, swr: 3600 }, // 1 day, SWR 1 hour
    redis: { ttl: 7200 }, // 2 hours
  },
};

// Redis cache implementation
import { Redis } from '@upstash/redis';

const redis = new Redis({
  url: process.env.REDIS_URL!,
  token: process.env.REDIS_TOKEN!,
});

export class CacheService {
  async get<T>(key: string): Promise<T | null> {
    const value = await redis.get(key);
    return value as T | null;
  }
  
  async set<T>(key: string, value: T, ttl?: number): Promise<void> {
    if (ttl) {
      await redis.setex(key, ttl, JSON.stringify(value));
    } else {
      await redis.set(key, JSON.stringify(value));
    }
  }
  
  async invalidate(pattern: string): Promise<void> {
    const keys = await redis.keys(pattern);
    if (keys.length > 0) {
      await redis.del(...keys);
    }
  }
  
  // Cache-aside pattern
  async getOrSet<T>(
    key: string,
    fetcher: () => Promise<T>,
    ttl: number
  ): Promise<T> {
    // Try cache first
    const cached = await this.get<T>(key);
    if (cached) return cached;
    
    // Fetch and cache
    const value = await fetcher();
    await this.set(key, value, ttl);
    
    return value;
  }
}

export const cache = new CacheService();
```

### 12.3 Database Optimization

```sql
-- Indexing Strategy

-- Composite indexes for common queries
CREATE INDEX idx_components_project_active 
  ON components(project_id, is_active) 
  WHERE is_active = true;

CREATE INDEX idx_exports_component_format 
  ON exports(component_id, format);

-- Partial indexes (PostgreSQL)
CREATE INDEX idx_projects_public 
  ON projects(id) 
  WHERE is_public = true;

-- Expression indexes
CREATE INDEX idx_components_name_lower 
  ON components(LOWER(name));

-- Covering indexes (include frequently accessed columns)
CREATE INDEX idx_projects_user_cover 
  ON projects(user_id) 
  INCLUDE (name, created_at, updated_at);

-- Query optimization
-- 1. Use EXPLAIN ANALYZE to check query plans
EXPLAIN ANALYZE
SELECT c.* 
FROM components c
JOIN projects p ON c.project_id = p.id
WHERE p.user_id = 'user-123'
  AND c.is_active = true
ORDER BY c.updated_at DESC
LIMIT 10;

-- 2. Materialized views for expensive queries
CREATE MATERIALIZED VIEW popular_templates AS
SELECT 
  t.*,
  COUNT(DISTINCT i.user_id) as install_count,
  AVG(r.rating) as avg_rating
FROM component_templates t
LEFT JOIN template_installs i ON t.id = i.template_id
LEFT JOIN template_ratings r ON t.id = r.template_id
GROUP BY t.id
ORDER BY install_count DESC, avg_rating DESC;

-- Refresh materialized view (scheduled job)
REFRESH MATERIALIZED VIEW CONCURRENTLY popular_templates;

-- 3. Partitioning large tables
CREATE TABLE analytics_events (
  id UUID PRIMARY KEY,
  user_id UUID,
  event_type VARCHAR(100),
  event_data JSONB,
  created_at TIMESTAMP
) PARTITION BY RANGE (created_at);

-- Monthly partitions
CREATE TABLE analytics_events_2026_01 
  PARTITION OF analytics_events
  FOR VALUES FROM ('2026-01-01') TO ('2026-02-01');

CREATE TABLE analytics_events_2026_02 
  PARTITION OF analytics_events
  FOR VALUES FROM ('2026-02-01') TO ('2026-03-01');

-- Auto-create partitions (function)
CREATE OR REPLACE FUNCTION create_monthly_partition()
RETURNS void AS $$
DECLARE
  start_date DATE;
  end_date DATE;
  partition_name TEXT;
BEGIN
  start_date := DATE_TRUNC('month', CURRENT_DATE + INTERVAL '1 month');
  end_date := start_date + INTERVAL '1 month';
  partition_name := 'analytics_events_' || TO_CHAR(start_date, 'YYYY_MM');
  
  EXECUTE format(
    'CREATE TABLE IF NOT EXISTS %I PARTITION OF analytics_events
     FOR VALUES FROM (%L) TO (%L)',
    partition_name, start_date, end_date
  );
END;
$$ LANGUAGE plpgsql;

-- Connection pooling
-- Use PgBouncer or Supabase connection pooler
-- Max connections: 100
-- Pool size per instance: 10
-- Total instances: 10
-- Reserve: 100 - (10 * 10) = 0 (scale up if needed)
```

### 12.4 Compilation Service Optimization

```rust
// compiler/src/optimization.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Compilation cache to avoid recompiling unchanged files
pub struct CompilationCache {
    cache: Arc<RwLock<HashMap<String, CachedCompilation>>>,
    max_size: usize,
}

#[derive(Clone)]
pub struct CachedCompilation {
    pub source_hash: String,
    pub output: CompilationOutput,
    pub timestamp: i64,
}

impl CompilationCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }
    
    pub async fn get(&self, source_hash: &str) -> Option<CompilationOutput> {
        let cache = self.cache.read().await;
        cache.get(source_hash).map(|c| c.output.clone())
    }
    
    pub async fn set(&self, source_hash: String, output: CompilationOutput) {
        let mut cache = self.cache.write().await;
        
        // Evict oldest if cache full
        if cache.len() >= self.max_size {
            if let Some(oldest_key) = self.find_oldest(&cache) {
                cache.remove(&oldest_key);
            }
        }
        
        cache.insert(source_hash.clone(), CachedCompilation {
            source_hash,
            output,
            timestamp: chrono::Utc::now().timestamp(),
        });
    }
    
    fn find_oldest(&self, cache: &HashMap<String, CachedCompilation>) -> Option<String> {
        cache.iter()
            .min_by_key(|(_, v)| v.timestamp)
            .map(|(k, _)| k.clone())
    }
}

/// Parallel compilation for multiple files
pub async fn compile_batch(
    sources: Vec<String>,
    compiler: &Compiler,
) -> Vec<Result<CompilationOutput>> {
    use tokio::task;
    
    let tasks: Vec<_> = sources.into_iter()
        .map(|source| {
            let compiler = compiler.clone();
            task::spawn(async move {
                compiler.compile(&source).await
            })
        })
        .collect();
    
    let results = futures::future::join_all(tasks).await;
    
    results.into_iter()
        .map(|r| r.unwrap())
        .collect()
}

/// Compilation queue with priority
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct CompilationTask {
    pub priority: u8, // 0 = highest, 255 = lowest
    pub source: String,
    pub timestamp: i64,
}

impl Ord for CompilationTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower priority value = higher priority
        other.priority.cmp(&self.priority)
            .then_with(|| self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialOrd for CompilationTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::collections::BinaryHeap;

pub struct CompilationQueue {
    queue: Arc<RwLock<BinaryHeap<CompilationTask>>>,
}

impl CompilationQueue {
    pub async fn push(&self, task: CompilationTask) {
        let mut queue = self.queue.write().await;
        queue.push(task);
    }
    
    pub async fn pop(&self) -> Option<CompilationTask> {
        let mut queue = self.queue.write().await;
        queue.pop()
    }
}
```

### 12.5 Asset Delivery Optimization

```typescript
// cdn/asset-delivery.ts

/**
 * Progressive image loading
 */
export function generateImageSrcSet(url: string): string {
  const sizes = [320, 640, 768, 1024, 1280, 1920];
  
  return sizes
    .map(width => {
      const optimizedUrl = `${url}?w=${width}&q=80&f=webp`;
      return `${optimizedUrl} ${width}w`;
    })
    .join(', ');
}

/**
 * Lazy loading images
 */
export function lazyLoadImage(img: HTMLImageElement) {
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const img = entry.target as HTMLImageElement;
        img.src = img.dataset.src!;
        observer.unobserve(img);
      }
    });
  });
  
  observer.observe(img);
}

/**
 * Video streaming with adaptive bitrate
 */
export async function generateVideoManifest(videoId: string) {
  // Generate HLS manifest with multiple quality levels
  return {
    type: 'application/x-mpegURL',
    url: `https://cdn.omnicraft.dev/videos/${videoId}/manifest.m3u8`,
    qualities: [
      { resolution: '360p', bitrate: 500000 },
      { resolution: '480p', bitrate: 1000000 },
      { resolution: '720p', bitrate: 2500000 },
      { resolution: '1080p', bitrate: 5000000 },
    ],
  };
}

/**
 * WASM streaming compilation
 */
export async function loadWasmModule(url: string) {
  // Use streaming instantiation for faster startup
  const response = await fetch(url);
  const { instance } = await WebAssembly.instantiateStreaming(response);
  return instance;
}

/**
 * Code splitting for large bundles
 */
// webpack.config.js
module.exports = {
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        // Vendor code
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          priority: 10,
        },
        
        // Common code
        common: {
          minChunks: 2,
          priority: 5,
          reuseExistingChunk: true,
        },
        
        // Runtime code separately
        runtime: {
          test: /[\\/]runtime[\\/]/,
          name: 'runtime',
          priority: 15,
        },
      },
    },
  },
};
```

### 12.6 Performance Monitoring

```typescript
// monitoring/performance.ts

import { PostHog } from 'posthog-node';

const posthog = new PostHog(process.env.POSTHOG_API_KEY!, {
  host: 'https://app.posthog.com',
});

/**
 * Track performance metrics
 */
export class PerformanceMonitor {
  // Web Vitals
  trackWebVitals() {
    if (typeof window === 'undefined') return;
    
    import('web-vitals').then(({ getCLS, getFID, getFCP, getLCP, getTTFB }) => {
      getCLS(this.sendToAnalytics);
      getFID(this.sendToAnalytics);
      getFCP(this.sendToAnalytics);
      getLCP(this.sendToAnalytics);
      getTTFB(this.sendToAnalytics);
    });
  }
  
  private sendToAnalytics(metric: any) {
    posthog.capture({
      distinctId: 'anonymous',
      event: 'web_vital',
      properties: {
        name: metric.name,
        value: metric.value,
        rating: metric.rating,
      },
    });
  }
  
  // Custom metrics
  trackCompilationTime(duration: number, success: boolean) {
    posthog.capture({
      distinctId: 'system',
      event: 'compilation_completed',
      properties: {
        duration,
        success,
      },
    });
  }
  
  trackRenderTime(componentId: string, duration: number) {
    posthog.capture({
      distinctId: 'system',
      event: 'render_completed',
      properties: {
        componentId,
        duration,
      },
    });
  }
  
  // Error tracking
  trackError(error: Error, context?: Record<string, any>) {
    console.error(error);
    
    posthog.capture({
      distinctId: 'system',
      event: 'error',
      properties: {
        message: error.message,
        stack: error.stack,
        ...context,
      },
    });
    
    // Also send to Sentry
    if (typeof window !== 'undefined' && window.Sentry) {
      window.Sentry.captureException(error, { extra: context });
    }
  }
}

export const performanceMonitor = new PerformanceMonitor();

/**
 * Backend performance monitoring
 */
export function measureDuration(fn: () => Promise<any>) {
  return async function measured(...args: any[]) {
    const start = performance.now();
    
    try {
      const result = await fn.apply(this, args);
      const duration = performance.now() - start;
      
      performanceMonitor.trackCompilationTime(duration, true);
      
      return result;
    } catch (error) {
      const duration = performance.now() - start;
      performanceMonitor.trackCompilationTime(duration, false);
      throw error;
    }
  };
}

/**
 * Real-time dashboard metrics
 */
export async function getSystemMetrics() {
  return {
    // Application metrics
    activeUsers: await countActiveUsers(),
    compilationsPerMinute: await getCompilationRate(),
    averageCompilationTime: await getAverageCompilationTime(),
    errorRate: await getErrorRate(),
    
    // Infrastructure metrics
    cpuUsage: process.cpuUsage(),
    memoryUsage: process.memoryUsage(),
    
    // Database metrics
    databaseConnections: await getDatabaseConnections(),
    slowQueries: await getSlowQueries(),
    
    // Cache metrics
    cacheHitRate: await getCacheHitRate(),
  };
}
```

---

## 13. Testing Strategy

### 13.1 Testing Pyramid

```
         ┌─────────────┐
         │     E2E     │  5%  (Slow, Expensive)
         │   Tests     │
         └─────────────┘
        ┌───────────────┐
        │  Integration  │  15% (Medium Speed)
        │    Tests      │
        └───────────────┘
       ┌─────────────────┐
       │   Unit Tests    │  80% (Fast, Cheap)
       │                 │
       └─────────────────┘
```

### 13.2 Unit Tests

```rust
// compiler/tests/parser_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_component() {
        let source = r#"
            <component name="Button">
              <props>
                <prop name="label" type="string" />
              </props>
              
              <template>
                <rect width={100} height={40} />
              </template>
            </component>
        "#;
        
        let mut parser = Parser::new(source, "Button.omni".to_string());
        let component = parser.parse().unwrap();
        
        assert_eq!(component.name, "Button");
        assert_eq!(component.props.len(), 1);
        assert_eq!(component.template.children.len(), 1);
    }
    
    #[test]
    fn test_type_checking_error() {
        let source = r#"
            <script>
              const count = signal(0);
              const name = signal("Alice");
              const result = count() + name(); // Type error!
            </script>
        "#;
        
        let mut parser = Parser::new(source, "Test.omni".to_string());
        let component = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&component);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Type mismatch"));
    }
    
    #[test]
    fn test_optimization_dead_code_elimination() {
        let source = r#"
            <script>
              const used = signal(1);
              const unused = signal(2); // Should be removed
              
              function usedFunc() { return used(); }
              function unusedFunc() { return unused(); } // Should be removed
            </script>
        "#;
        
        let mut parser = Parser::new(source, "Test.omni".to_string());
        let mut component = parser.parse().unwrap();
        
        let mut optimizer = Optimizer::new();
        optimizer.optimize(&mut component).unwrap();
        
        // Check that unused code was removed
        let script = component.script.as_ref().unwrap();
        assert_eq!(
            script.statements.iter().filter(|s| matches!(s, Statement::VariableDeclaration { name, .. } if name == "unused")).count(),
            0
        );
    }
}
```

### 13.3 Integration Tests

```typescript
// tests/integration/compilation.test.ts

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { OmniCraftCompiler } from '@/compiler';
import { TestServer } from './helpers/test-server';

describe('Compilation Integration', () => {
  let server: TestServer;
  let compiler: OmniCraftCompiler;
  
  beforeAll(async () => {
    server = await TestServer.start();
    compiler = new OmniCraftCompiler({
      optimizationLevel: 'aggressive',
      target: 'wasm',
    });
  });
  
  afterAll(async () => {
    await server.stop();
  });
  
  it('should compile component end-to-end', async () => {
    const source = `
      <component name="Counter">
        <script>
          const count = signal(0);
        </script>
        
        <template>
          <text content={count()} />
        </template>
      </component>
    `;
    
    const result = await compiler.compile(source);
    
    expect(result.success).toBe(true);
    expect(result.output.wasmUrl).toBeDefined();
    expect(result.output.typesUrl).toBeDefined();
    expect(result.stats.bundleSize).toBeLessThan(100000); // < 100KB
  });
  
  it('should handle compilation errors gracefully', async () => {
    const source = `
      <component name="Invalid">
        <script>
          const x = unknown_function(); // Error!
        </script>
      </component>
    `;
    
    const result = await compiler.compile(source);
    
    expect(result.success).toBe(false);
    expect(result.diagnostics.length).toBeGreaterThan(0);
    expect(result.diagnostics[0].severity).toBe('error');
  });
  
  it('should use cache for unchanged files', async () => {
    const source = `<component name="Test"><template><rect /></template></component>`;
    
    // First compilation
    const start1 = Date.now();
    await compiler.compile(source);
    const duration1 = Date.now() - start1;
    
    // Second compilation (should use cache)
    const start2 = Date.now();
    await compiler.compile(source);
    const duration2 = Date.now() - start2;
    
    // Cached compilation should be much faster
    expect(duration2).toBeLessThan(duration1 / 2);
  });
});
```

### 13.4 End-to-End Tests

```typescript
// tests/e2e/create-component.spec.ts

import { test, expect } from '@playwright/test';

test.describe('Create Component Flow', () => {
  test('user can create and compile a new component', async ({ page }) => {
    // 1. Login
    await page.goto('http://localhost:3000/login');
    await page.fill('[name="email"]', 'test@example.com');
    await page.fill('[name="password"]', 'password123');
    await page.click('button[type="submit"]');
    
    // 2. Navigate to dashboard
    await expect(page).toHaveURL('/dashboard');
    
    // 3. Create new project
    await page.click('text=New Project');
    await page.fill('[name="projectName"]', 'Test Project');
    await page.click('button:has-text("Create")');
    
    // 4. Create new component
    await page.click('text=New Component');
    await page.fill('[name="componentName"]', 'Button');
    await page.click('button:has-text("Create")');
    
    // 5. Edit component in code editor
    await expect(page.locator('.monaco-editor')).toBeVisible();
    
    await page.keyboard.type(`
      <component name="Button">
        <props>
          <prop name="label" type="string" />
        </props>
        
        <template>
          <rect width={100} height={40} fill="#4caf50" />
          <text content={props.label} />
        </template>
      </component>
    `);
    
    // 6. Save and compile
    await page.keyboard.press('Control+S');
    
    // 7. Wait for compilation
    await expect(page.locator('text=Compilation successful')).toBeVisible({ timeout: 10000 });
    
    // 8. Check preview
    const canvas = page.locator('canvas');
    await expect(canvas).toBeVisible();
    
    // 9. Export component
    await page.click('text=Export');
    await page.click('text=Export as Code');
    
    // 10. Verify download
    const [download] = await Promise.all([
      page.waitForEvent('download'),
      page.click('button:has-text("Download")'),
    ]);
    
    expect(download.suggestedFilename()).toBe('Button.zip');
  });
  
  test('shows error on invalid syntax', async ({ page }) => {
    await page.goto('http://localhost:3000/editor');
    
    // Type invalid code
    await page.keyboard.type('<component name="Invalid">');
    await page.keyboard.press('Control+S');
    
    // Should show error
    await expect(page.locator('.error-message')).toContainText('Missing closing tag');
    await expect(page.locator('.error-location')).toContainText('Line 1');
  });
});
```

### 13.5 Performance Tests

```typescript
// tests/performance/compilation-bench.ts

import { suite, add, cycle, complete } from 'benny';
import { OmniCraftCompiler } from '@/compiler';

export default suite(
  'Compilation Performance',
  
  add('Compile small component (< 100 lines)', async () => {
    const compiler = new OmniCraftCompiler();
    const source = generateSmallComponent();
    await compiler.compile(source);
  }),
  
  add('Compile medium component (100-500 lines)', async () => {
    const compiler = new OmniCraftCompiler();
    const source = generateMediumComponent();
    await compiler.compile(source);
  }),
  
  add('Compile large component (> 500 lines)', async () => {
    const compiler = new OmniCraftCompiler();
    const source = generateLargeComponent();
    await compiler.compile(source);
  }),
  
  cycle(),
  complete(),
);

// Target benchmarks:
// Small component: < 100ms
// Medium component: < 300ms
// Large component: < 1000ms
```

### 13.6 Visual Regression Tests

```typescript
// tests/visual/screenshots.test.ts

import { test, expect } from '@playwright/test';

test.describe('Visual Regression', () => {
  test('button component renders correctly', async ({ page }) => {
    await page.goto('http://localhost:3000/preview/button');
    
    // Wait for render
    await page.waitForTimeout(1000);
    
    // Take screenshot
    await expect(page).toHaveScreenshot('button-default.png', {
      maxDiffPixels: 100, // Allow 100 pixels difference
    });
  });
  
  test('button hover state', async ({ page }) => {
    await page.goto('http://localhost:3000/preview/button');
    
    const button = page.locator('canvas');
    await button.hover();
    await page.waitForTimeout(500);
    
    await expect(page).toHaveScreenshot('button-hover.png');
  });
  
  test('responsive layout on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 }); // iPhone SE
    await page.goto('http://localhost:3000/editor');
    
    await expect(page).toHaveScreenshot('mobile-editor.png');
  });
});
```

---

## 14. Deployment Architecture

### 14.1 Deployment Environments

```yaml
# Environment configuration

development:
  name: Development
  url: http://localhost:3000
  database: postgres://localhost/omnicraft_dev
  redis: redis://localhost:6379
  features:
    - hot_reload
    - debug_mode
    - mock_services

staging:
  name: Staging
  url: https://staging.omnicraft.dev
  database: postgres://staging-db.supabase.co/postgres
  redis: redis://staging-redis.upstash.io
  features:
    - performance_monitoring
    - error_tracking
    - beta_features

production:
  name: Production
  url: https://omnicraft.dev
  database: postgres://prod-db.supabase.co/postgres
  redis: redis://prod-redis.upstash.io
  features:
    - performance_monitoring
    - error_tracking
    - analytics
    - rate_limiting
```

### 14.2 CI/CD Pipeline

```yaml
# .github/workflows/ci-cd.yml

name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'pnpm'
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Install dependencies
        run: pnpm install
        
      - name: Run linter
        run: pnpm lint
        
      - name: Run type check
        run: pnpm type-check
        
      - name: Run unit tests
        run: pnpm test:unit
        
      - name: Run integration tests
        run: pnpm test:integration
        
      - name: Build compiler
        run: cd compiler && cargo build --release
        
      - name: Run Rust tests
        run: cd compiler && cargo test
        
      - name: Code coverage
        run: pnpm test:coverage
        
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info

  build:
    needs: test
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'pnpm'
          
      - name: Install dependencies
        run: pnpm install
        
      - name: Build application
        run: pnpm build
        env:
          NODE_ENV: production
          
      - name: Build WASM runtime
        run: |
          cd runtime
          cargo build --release --target wasm32-unknown-unknown
          wasm-pack build --target web
          
      - name: Optimize WASM
        run: |
          wasm-opt -Oz runtime/pkg/*.wasm -o runtime/pkg/optimized.wasm
          
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: build-artifacts
          path: |
            dist/
            runtime/pkg/

  e2e:
    needs: build
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        
      - name: Install dependencies
        run: pnpm install
        
      - name: Download artifacts
        uses: actions/download-artifact@v3
        with:
          name: build-artifacts
          
      - name: Install Playwright
        run: pnpm exec playwright install --with-deps
        
      - name: Run E2E tests
        run: pnpm test:e2e
        
      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: playwright-results
          path: test-results/

  deploy-staging:
    needs: [build, e2e]
    if: github.ref == 'refs/heads/develop'
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Download artifacts
        uses: actions/download-artifact@v3
        with:
          name: build-artifacts
          
      - name: Deploy to Vercel (Staging)
        uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          vercel-args: '--prod'
          working-directory: ./
          
      - name: Run smoke tests
        run: pnpm test:smoke
        env:
          BASE_URL: https://staging.omnicraft.dev
          
      - name: Notify Slack
        uses: 8398a7/action-slack@v3
        with:
          status: ${{ job.status }}
          text: 'Staging deployment completed'
          webhook_url: ${{ secrets.SLACK_WEBHOOK }}

  deploy-production:
    needs: [build, e2e]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    environment: production
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Download artifacts
        uses: actions/download-artifact@v3
        with:
          name: build-artifacts
          
      - name: Deploy to Vercel (Production)
        uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          vercel-args: '--prod'
          alias-domains: |
            omnicraft.dev
            www.omnicraft.dev
            
      - name: Upload to CDN
        run: |
          aws s3 sync ./runtime/pkg s3://cdn.omnicraft.dev/wasm/ \
            --cache-control "public, max-age=31536000, immutable"
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          
      - name: Invalidate CloudFront cache
        run: |
          aws cloudfront create-invalidation \
            --distribution-id ${{ secrets.CLOUDFRONT_DISTRIBUTION_ID }} \
            --paths "/*"
            
      - name: Run smoke tests
        run: pnpm test:smoke
        env:
          BASE_URL: https://omnicraft.dev
          
      - name: Create Sentry release
        uses: getsentry/action-release@v1
        env:
          SENTRY_AUTH_TOKEN: ${{ secrets.SENTRY_AUTH_TOKEN }}
          SENTRY_ORG: omnicraft
          SENTRY_PROJECT: web
        with:
          environment: production
          
      - name: Notify team
        uses: 8398a7/action-slack@v3
        with:
          status: ${{ job.status }}
          text: '🚀 Production deployment completed!'
          webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

### 14.3 Infrastructure as Code

```typescript
// infrastructure/main.ts (using Pulumi)

import * as pulumi from '@pulumi/pulumi';
import * as aws from '@pulumi/aws';
import * as cloudflare from '@pulumi/cloudflare';

// S3 bucket for CDN
const cdnBucket = new aws.s3.Bucket('cdn-bucket', {
  bucket: 'cdn.omnicraft.dev',
  acl: 'public-read',
  website: {
    indexDocument: 'index.html',
  },
  corsRules: [{
    allowedHeaders: ['*'],
    allowedMethods: ['GET', 'HEAD'],
    allowedOrigins: ['*'],
    maxAgeSeconds: 3000,
  }],
});

// CloudFront distribution
const cdn = new aws.cloudfront.Distribution('cdn', {
  enabled: true,
  aliases: ['cdn.omnicraft.dev'],
  
  origins: [{
    originId: 's3',
    domainName: cdnBucket.bucketRegionalDomainName,
    s3OriginConfig: {
      originAccessIdentity: 'origin-access-identity/cloudfront/...',
    },
  }],
  
  defaultCacheBehavior: {
    targetOriginId: 's3',
    viewerProtocolPolicy: 'redirect-to-https',
    allowedMethods: ['GET', 'HEAD', 'OPTIONS'],
    cachedMethods: ['GET', 'HEAD'],
    
    forwardedValues: {
      queryString: true,
      cookies: { forward: 'none' },
    },
    
    compress: true,
    defaultTtl: 86400, // 1 day
    maxTtl: 31536000, // 1 year
  },
  
  viewerCertificate: {
    acmCertificateArn: 'arn:aws:acm:us-east-1:...',
    sslSupportMethod: 'sni-only',
  },
  
  restrictions: {
    geoRestriction: {
      restrictionType: 'none',
    },
  },
});

// Cloudflare DNS
const zone = cloudflare.getZone({
  name: 'omnicraft.dev',
});

const mainRecord = new cloudflare.Record('main', {
  zoneId: zone.then(z => z.id),
  name: '@',
  type: 'CNAME',
  value: 'cname.vercel-dns.com',
  proxied: true,
});

const cdnRecord = new cloudflare.Record('cdn', {
  zoneId: zone.then(z => z.id),
  name: 'cdn',
  type: 'CNAME',
  value: cdn.domainName,
  proxied: false, // Direct to CloudFront
});

// Export outputs
export const cdnUrl = pulumi.interpolate`https://${cdn.domainName}`;
export const cdnBucketName = cdnBucket.id;
```

### 14.4 Docker Configuration

```dockerfile
# Dockerfile (Multi-stage build)

# Stage 1: Build Rust compiler
FROM rust:1.75-alpine AS rust-builder

WORKDIR /app/compiler

RUN apk add --no-cache musl-dev

COPY compiler/Cargo.toml compiler/Cargo.lock ./
COPY compiler/src ./src

RUN cargo build --release

# Stage 2: Build WASM runtime
FROM rust:1.75-alpine AS wasm-builder

WORKDIR /app/runtime

RUN apk add --no-cache musl-dev wasm-pack

COPY runtime/Cargo.toml runtime/Cargo.lock ./
COPY runtime/src ./src

RUN wasm-pack build --target web --release

# Stage 3: Build Node.js app
FROM node:18-alpine AS node-builder

WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# Copy package files
COPY package.json pnpm-lock.yaml ./
COPY packages/*/package.json ./packages/

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source
COPY . .

# Copy compiled artifacts from previous stages
COPY --from=rust-builder /app/compiler/target/release/omnicraft-compiler ./compiler/bin/
COPY --from=wasm-builder /app/runtime/pkg ./runtime/pkg/

# Build application
RUN pnpm build

# Stage 4: Production image
FROM node:18-alpine

WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# Copy built artifacts
COPY --from=node-builder /app/dist ./dist
COPY --from=node-builder /app/node_modules ./node_modules
COPY --from=node-builder /app/package.json ./

# Copy binaries
COPY --from=rust-builder /app/compiler/target/release/omnicraft-compiler /usr/local/bin/
COPY --from=wasm-builder /app/runtime/pkg ./runtime/pkg/

# Create non-root user
RUN addgroup -g 1001 -S nodejs && \
    adduser -S nextjs -u 1001

USER nextjs

EXPOSE 3000

ENV NODE_ENV=production
ENV PORT=3000

CMD ["pnpm", "start"]
```

```yaml
# docker-compose.yml (Development)

version: '3.9'

services:
  web:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - '3000:3000'
    volumes:
      - .:/app
      - /app/node_modules
      - /app/.next
    environment:
      - NODE_ENV=development
      - DATABASE_URL=postgresql://postgres:password@db:5432/omnicraft
      - REDIS_URL=redis://redis:6379
    depends_on:
      - db
      - redis
    command: pnpm dev

  db:
    image: postgres:15-alpine
    ports:
      - '5432:5432'
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=omnicraft
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - '6379:6379'
    volumes:
      - redis_data:/data

  compiler:
    build:
      context: ./compiler
      dockerfile: Dockerfile
    volumes:
      - ./compiler:/app
    command: cargo watch -x run

volumes:
  postgres_data:
  redis_data:
```

### 14.5 Kubernetes Deployment

```yaml
# k8s/deployment.yaml

apiVersion: apps/v1
kind: Deployment
metadata:
  name: omnicraft-web
  namespace: production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: omnicraft-web
  template:
    metadata:
      labels:
        app: omnicraft-web
    spec:
      containers:
      - name: web
        image: omnicraft/web:latest
        ports:
        - containerPort: 3000
        env:
        - name: NODE_ENV
          value: production
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: omnicraft-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: omnicraft-secrets
              key: redis-url
        resources:
          requests:
            memory: '512Mi'
            cpu: '500m'
          limits:
            memory: '1Gi'
            cpu: '1000m'
        livenessProbe:
          httpGet:
            path: /api/health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5

---
apiVersion: v1
kind: Service
metadata:
  name: omnicraft-web
  namespace: production
spec:
  selector:
    app: omnicraft-web
  ports:
  - protocol: TCP
    port: 80
    targetPort: 3000
  type: LoadBalancer

---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: omnicraft-web-hpa
  namespace: production
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: omnicraft-web
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

---

## 15. Developer Experience (DX)

### 15.1 Getting Started Guide

```markdown
# Quick Start Guide

## Installation

### Option 1: NPM (Recommended)
```bash
npm install -g omnicraft
omnicraft --version
```

### Option 2: Cargo
```bash
cargo install omnicraft-cli
omnicraft --version
```

## Create Your First Project

```bash
# Create new project
omnicraft new my-app

# Navigate to project
cd my-app

# Start development server
omnicraft dev
```

## Your First Component

Create `src/Button.omni`:

```omni
<component name="Button">
  <props>
    <prop name="label" type="string" required />
  </props>
  
  <script>
    const isHovered = signal(false);
  </script>
  
  <template>
    <rect 
      width={100} 
      height={40}
      fill={isHovered() ? '#66bb6a' : '#4caf50'}
      @mouseenter={() => isHovered.set(true)}
      @mouseleave={() => isHovered.set(false)}
    />
    <text content={props.label} fill="#ffffff" />
  </template>
</component>
```

Save and see it live in the browser!

## Export Your Component

```bash
# Export as video
omnicraft export Button.omni --format video --output button.mp4

# Export as code
omnicraft export Button.omni --format react --output Button.tsx

# Export as SVG
omnicraft export Button.omni --format svg --output button.svg
```

## Next Steps

- [Read the full documentation](https://docs.omnicraft.dev)
- [Join our Discord](https://discord.gg/omnicraft)
- [Browse examples](https://omnicraft.dev/examples)
```

### 15.2 CLI Tool Design

```typescript
// cli/src/index.ts

import { Command } from 'commander';
import chalk from 'chalk';
import ora from 'ora';
import inquirer from 'inquirer';

const program = new Command();

program
  .name('omnicraft')
  .description('OmniCraft - Visual Content Creation Platform')
  .version('1.0.0');

// Create new project
program
  .command('new <name>')
  .description('Create a new OmniCraft project')
  .option('-t, --template <template>', 'Template to use', 'basic')
  .action(async (name: string, options: any) => {
    const spinner = ora('Creating project...').start();
    
    try {
      await createProject(name, options.template);
      spinner.succeed(chalk.green('Project created successfully!'));
      
      console.log('\nNext steps:');
      console.log(chalk.cyan(`  cd ${name}`));
      console.log(chalk.cyan('  omnicraft dev'));
    } catch (error) {
      spinner.fail(chalk.red('Failed to create project'));
      console.error(error);
    }
  });

// Start dev server
program
  .command('dev')
  .description('Start development server')
  .option('-p, --port <port>', 'Port to listen on', '3000')
  .option('--open', 'Open browser automatically')
  .action(async (options: any) => {
    console.log(chalk.blue('Starting development server...'));
    
    const server = await startDevServer({
      port: parseInt(options.port),
      open: options.open,
    });
    
    console.log(chalk.green(`\n✓ Server running at http://localhost:${options.port}`));
    console.log(chalk.gray('Press Ctrl+C to stop\n'));
  });

// Compile component
program
  .command('compile <file>')
  .description('Compile .omni file')
  .option('-o, --output <dir>', 'Output directory', './dist')
  .option('-O, --opt-level <level>', 'Optimization level (0-3)', '2')
  .option('--watch', 'Watch for changes')
  .action(async (file: string, options: any) => {
    if (options.watch) {
      console.log(chalk.blue(`Watching ${file} for changes...`));
      await watchAndCompile(file, options);
    } else {
      const spinner = ora('Compiling...').start();
      
      try {
        const result = await compile(file, options);
        spinner.succeed(chalk.green('Compilation successful'));
        
        console.log('\nOutput:');
        console.log(chalk.gray(`  WASM: ${result.wasmPath}`));
        console.log(chalk.gray(`  Types: ${result.typesPath}`));
        console.log(chalk.gray(`  Size: ${formatBytes(result.size)}`));
      } catch (error) {
        spinner.fail(chalk.red('Compilation failed'));
        printDiagnostics(error.diagnostics);
      }
    }
  });

// Export component
program
  .command('export <file>')
  .description('Export component to various formats')
  .option('-f, --format <format>', 'Output format (video|code|svg|lottie)', 'video')
  .option('-o, --output <path>', 'Output file path')
  .option('--resolution <res>', 'Video resolution (for video format)', '1080p')
  .option('--fps <fps>', 'Frame rate (for video format)', '60')
  .action(async (file: string, options: any) => {
    const spinner = ora(`Exporting as ${options.format}...`).start();
    
    try {
      const result = await exportComponent(file, options);
      spinner.succeed(chalk.green('Export successful'));
      
      console.log(chalk.gray(`\nSaved to: ${result.outputPath}`));
      console.log(chalk.gray(`Size: ${formatBytes(result.size)}`));
    } catch (error) {
      spinner.fail(chalk.red('Export failed'));
      console.error(error);
    }
  });

// Interactive component creation
program
  .command('create')
  .description('Create a new component (interactive)')
  .action(async () => {
    const answers = await inquirer.prompt([
      {
        type: 'list',
        name: 'type',
        message: 'What type of component?',
        choices: ['Component', 'Template', 'Plugin'],
      },
      {
        type: 'input',
        name: 'name',
        message: 'Component name:',
        validate: (input: string) => {
          if (!input) return 'Name is required';
          if (!/^[A-Z][a-zA-Z0-9]*$/.test(input)) {
            return 'Name must be PascalCase (e.g., MyComponent)';
          }
          return true;
        },
      },
      {
        type: 'confirm',
        name: 'withProps',
        message: 'Add props?',
        default: true,
      },
      {
        type: 'confirm',
        name: 'withState',
        message: 'Add state?',
        default: true,
      },
    ]);
    
    const spinner = ora('Creating component...').start();
    
    try {
      await scaffoldComponent(answers);
      spinner.succeed(chalk.green('Component created!'));
      
      console.log(chalk.gray(`\nCreated: src/components/${answers.name}.omni`));
      console.log(chalk.cyan('\nRun: omnicraft dev'));
    } catch (error) {
      spinner.fail(chalk.red('Failed to create component'));
      console.error(error);
    }
  });

// Utilities
function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}

function printDiagnostics(diagnostics: Diagnostic[]) {
  console.log('\nErrors:');
  
  for (const diagnostic of diagnostics) {
    const color = diagnostic.severity === 'error' ? chalk.red : chalk.yellow;
    
    console.log(color(`\n${diagnostic.severity}: ${diagnostic.message}`));
    console.log(chalk.gray(`  at ${diagnostic.location.file}:${diagnostic.location.line}:${diagnostic.location.column}`));
    
    if (diagnostic.suggestions.length > 0) {
      console.log(chalk.blue('\n  Suggestions:'));
      for (const suggestion of diagnostic.suggestions) {
        console.log(chalk.gray(`    - ${suggestion.message}`));
      }
    }
  }
}

program.parse();
```

### 15.3 VSCode Extension

```typescript
// vscode-extension/src/extension.ts

import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  // Language Server setup
  const serverOptions: ServerOptions = {
    run: {
      command: 'omnicraft-lsp',
      transport: vscode.TransportKind.stdio,
    },
    debug: {
      command: 'omnicraft-lsp',
      transport: vscode.TransportKind.stdio,
      args: ['--debug'],
    },
  };
  
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'omni' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.omni'),
    },
  };
  
  client = new LanguageClient(
    'omnicraft',
    'OmniCraft Language Server',
    serverOptions,
    clientOptions
  );
  
  client.start();
  
  // Commands
  context.subscriptions.push(
    // Compile current file
    vscode.commands.registerCommand('omnicraft.compile', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;
      
      const result = await compileDocument(editor.document);
      
      if (result.success) {
        vscode.window.showInformationMessage('Compilation successful!');
      } else {
        vscode.window.showErrorMessage('Compilation failed');
        showDiagnostics(result.diagnostics);
      }
    }),
    
    // Preview component
    vscode.commands.registerCommand('omnicraft.preview', async () => {
      const panel = vscode.window.createWebviewPanel(
        'omnicraftPreview',
        'OmniCraft Preview',
        vscode.ViewColumn.Beside,
        { enableScripts: true }
      );
      
      panel.webview.html = await generatePreviewHtml();
    }),
    
    // Export component
    vscode.commands.registerCommand('omnicraft.export', async () => {
      const format = await vscode.window.showQuickPick([
        'Video (MP4)',
        'Code (React)',
        'SVG',
        'Lottie JSON',
      ]);
      
      if (format) {
        await exportComponent(format);
      }
    }),
    
    // Create component
    vscode.commands.registerCommand('omnicraft.createComponent', async () => {
      const name = await vscode.window.showInputBox({
        prompt: 'Component name (PascalCase)',
        validateInput: (value) => {
          if (!/^[A-Z][a-zA-Z0-9]*$/.test(value)) {
            return 'Name must be PascalCase';
          }
          return null;
        },
      });
      
      if (name) {
        await createComponentFile(name);
      }
    })
  );
  
  // Status bar
  const statusBar = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );
  statusBar.text = '$(pulse) OmniCraft';
  statusBar.command = 'omnicraft.compile';
  statusBar.show();
  context.subscriptions.push(statusBar);
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
```

---

## 16. Risk Analysis

### 16.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **WASM compilation too slow** | Medium | High | Incremental compilation, dev mode interpreter, caching |
| **Browser compatibility issues** | Low | Medium | Target modern browsers only, feature detection, polyfills |
| **Memory leaks in ECS** | Medium | High | Extensive testing, memory profiling, automated leak detection |
| **Compiler bugs** | High | Critical | Comprehensive test suite, fuzzing, staged rollouts |
| **Performance regressions** | Medium | High | Automated benchmarks, performance budgets, CI checks |

### 16.2 Business Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Low user adoption** | Medium | Critical | Excellent docs, tutorials, free tier, community building |
| **Competition from established tools** | High | High | Focus on unique features (code export, multi-format), better DX |
| **Difficulty monetizing** | Medium | High | Multiple revenue streams, enterprise features, marketplace |
| **High support costs** | Medium | Medium | Good documentation, community support, automation |
| **Security vulnerabilities** | Low | Critical | Security audits, bug bounties, quick patches |

### 16.3 Mitigation Strategies

```markdown
## Risk Mitigation Plan

### 1. Performance Issues
- **Prevention:** Set performance budgets, automated benchmarks
- **Detection:** Monitoring, alerting, user feedback
- **Response:** Quick rollback capability, performance task force

### 2. Security Breaches
- **Prevention:** Security best practices, code reviews, audits
- **Detection:** Intrusion detection, anomaly monitoring
- **Response:** Incident response plan, communication strategy

### 3. Data Loss
- **Prevention:** Regular backups, redundancy
- **Detection:** Integrity checks, monitoring
- **Response:** Backup restoration procedures, RTO < 4 hours

### 4. Service Outages
- **Prevention:** High availability architecture, redundancy
- **Detection:** Health checks, monitoring
- **Response:** Automatic failover, status page updates

### 5. Poor Developer Experience
- **Prevention:** User testing, feedback loops
- **Detection:** Analytics, surveys, support tickets
- **Response:** Rapid iteration, prioritize DX improvements
```

---

## Conclusion

This Software Design Document provides a comprehensive blueprint for building OmniCraft - a next-generation visual content creation platform. Key highlights:

### **Architecture Strengths:**
- ✅ **Compiler-first approach** for maximum performance
- ✅ **Modular design** using SOLID principles
- ✅ **Scalable infrastructure** ready for growth
- ✅ **Security-first** implementation
- ✅ **Developer experience** as priority

### **Success Factors:**
1. **Technical Excellence:** Performance targets are ambitious but achievable
2. **Developer Experience:** Progressive disclosure + excellent tooling
3. **Comprehensive Testing:** 90%+ coverage across all layers
4. **Scalability:** Designed to handle 100K+ users
5. **Security:** Multiple layers of protection

### **Next Steps:**
1. ✅ Review and approve SDD
2. ✅ Begin Phase 1: Core compiler (8 weeks)
3. ✅ Continuous integration with testing
4. ✅ Regular architecture reviews
5. ✅ Community feedback incorporation

**This SDD is a living document and will evolve with the project.**

---

**Document Version:** 3.0  
**Last Updated:** January 06, 2026  
**Status:** ✅ **Approved for Implementation**
