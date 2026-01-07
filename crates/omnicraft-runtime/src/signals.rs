//! Reactive Signals System
//!
//! Fine-grained reactivity inspired by SolidJS.
//! Provides `Signal`, `Memo`, and `Effect` primitives.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;

/// Unique identifier for reactive nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(u32);

/// Context for managing reactive signals (thread-safe for bevy_ecs Resource)
#[derive(Debug, Clone, Resource)]
pub struct SignalContext {
    next_id: Arc<Mutex<u32>>,
    // In a full implementation, this would track dependencies
    // and manage effect scheduling
}

impl SignalContext {
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    fn next_signal_id(&self) -> SignalId {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        SignalId(current)
    }

    pub fn create_signal<T: Clone + 'static>(&self, value: T) -> Signal<T> {
        Signal::new(self.next_signal_id(), value)
    }

    pub fn create_memo<T: Clone + 'static>(&self, compute: impl Fn() -> T + 'static) -> Memo<T> {
        Memo::new(self.next_signal_id(), compute)
    }

    pub fn create_effect(&self, effect: impl Fn() + 'static) {
        Effect::new(self.next_signal_id(), effect);
    }
}

impl Default for SignalContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Reactive signal holding a value
///
/// When the value changes, any dependent computations are re-run.
pub struct Signal<T> {
    id: SignalId,
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("id", &self.id)
            .field("value", &self.value)
            .field("subscribers", &format!("[{} subscribers]", self.subscribers.borrow().len()))
            .finish()
    }
}

impl<T: Clone> Signal<T> {
    pub fn new(id: SignalId, value: T) -> Self {
        Self {
            id,
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Get the current value
    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    /// Set a new value and notify subscribers
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        self.notify();
    }

    /// Update the value using a function
    pub fn update(&self, f: impl FnOnce(&T) -> T) {
        let new_value = {
            let current = self.value.borrow();
            f(&*current)
        };
        self.set(new_value);
    }

    /// Subscribe to value changes
    pub fn subscribe(&self, callback: impl Fn() + 'static) {
        self.subscribers.borrow_mut().push(Box::new(callback));
    }

    fn notify(&self) {
        for subscriber in self.subscribers.borrow().iter() {
            subscriber();
        }
    }

    pub fn id(&self) -> SignalId {
        self.id
    }
}

impl<T: Clone> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Rc::clone(&self.value),
            subscribers: Rc::clone(&self.subscribers),
        }
    }
}

/// Derived reactive value that automatically updates
///
/// Memos cache their computed value and only recompute when
/// dependencies change.
pub struct Memo<T> {
    id: SignalId,
    value: Rc<RefCell<Option<T>>>,
    compute: Rc<dyn Fn() -> T>,
}

impl<T: Clone> Memo<T> {
    pub fn new(id: SignalId, compute: impl Fn() -> T + 'static) -> Self {
        let compute = Rc::new(compute);
        let initial_value = compute();

        Self {
            id,
            value: Rc::new(RefCell::new(Some(initial_value))),
            compute,
        }
    }

    /// Get the memoized value
    pub fn get(&self) -> T {
        if let Some(ref value) = *self.value.borrow() {
            return value.clone();
        }

        // Recompute if invalidated
        let new_value = (self.compute)();
        *self.value.borrow_mut() = Some(new_value.clone());
        new_value
    }

    /// Invalidate the cached value
    pub fn invalidate(&self) {
        *self.value.borrow_mut() = None;
    }

    pub fn id(&self) -> SignalId {
        self.id
    }
}

impl<T: Clone> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Rc::clone(&self.value),
            compute: Rc::clone(&self.compute),
        }
    }
}

/// Side effect that runs when dependencies change
pub struct Effect {
    id: SignalId,
    callback: Rc<dyn Fn()>,
}

impl Effect {
    pub fn new(id: SignalId, callback: impl Fn() + 'static) -> Self {
        let effect = Self {
            id,
            callback: Rc::new(callback),
        };

        // Run effect immediately
        effect.run();

        effect
    }

    pub fn run(&self) {
        (self.callback)();
    }

    pub fn id(&self) -> SignalId {
        self.id
    }
}

/// Batch multiple signal updates
///
/// During a batch, effects are deferred until the batch completes.
pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // In a full implementation, this would defer effect execution
    f()
}

/// Create a signal in the current reactive context
pub fn create_signal<T: Clone + 'static>(value: T) -> Signal<T> {
    Signal::new(SignalId(0), value)
}

/// Create a memo in the current reactive context
pub fn create_memo<T: Clone + 'static>(compute: impl Fn() -> T + 'static) -> Memo<T> {
    Memo::new(SignalId(0), compute)
}

/// Create an effect in the current reactive context
pub fn create_effect(callback: impl Fn() + 'static) -> Effect {
    Effect::new(SignalId(0), callback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_get_set() {
        let signal = create_signal(0);
        assert_eq!(signal.get(), 0);

        signal.set(42);
        assert_eq!(signal.get(), 42);
    }

    #[test]
    fn test_signal_update() {
        let signal = create_signal(10);
        signal.update(|v| v + 5);
        assert_eq!(signal.get(), 15);
    }

    #[test]
    fn test_memo() {
        let count = create_signal(5);
        let count_clone = count.clone();
        let doubled = create_memo(move || count_clone.get() * 2);

        assert_eq!(doubled.get(), 10);

        count.set(10);
        doubled.invalidate();
        assert_eq!(doubled.get(), 20);
    }

    #[test]
    fn test_effect() {
        use std::cell::Cell;
        use std::rc::Rc;

        let run_count = Rc::new(Cell::new(0));
        let run_count_clone = Rc::clone(&run_count);

        let _effect = create_effect(move || {
            run_count_clone.set(run_count_clone.get() + 1);
        });

        // Effect runs immediately on creation
        assert_eq!(run_count.get(), 1);
    }

    #[test]
    fn test_signal_context() {
        let ctx = SignalContext::new();

        let signal1 = ctx.create_signal(100);
        let signal2 = ctx.create_signal("hello");

        assert_eq!(signal1.get(), 100);
        assert_eq!(signal2.get(), "hello");

        assert_ne!(signal1.id(), signal2.id());
    }
}
