//! Dependency Tracking
//!
//! Tracks reactive dependencies between signals and their consumers.

use std::collections::{HashMap, HashSet};

/// Dependency graph for reactive updates
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    /// All signals in the component
    pub signals: HashSet<String>,
    /// Dependencies: consumer -> set of signals it depends on
    pub dependencies: HashMap<String, HashSet<String>>,
    /// Reverse mapping: signal -> set of consumers
    pub dependents: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a signal
    pub fn add_signal(&mut self, name: String) {
        self.signals.insert(name);
    }

    /// Check if a name is a signal
    pub fn is_signal(&self, name: &str) -> bool {
        self.signals.contains(name)
    }

    /// Add a dependency: consumer depends on signal
    pub fn add_dependency(&mut self, consumer: String, signal: String) {
        // consumer -> signals
        self.dependencies
            .entry(consumer.clone())
            .or_default()
            .insert(signal.clone());

        // signal -> consumers (reverse)
        self.dependents
            .entry(signal)
            .or_default()
            .insert(consumer);
    }

    /// Get all signals that a consumer depends on
    pub fn get_dependencies(&self, consumer: &str) -> HashSet<String> {
        self.dependencies
            .get(consumer)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all consumers that depend on a signal
    pub fn get_dependents(&self, signal: &str) -> HashSet<String> {
        self.dependents
            .get(signal)
            .cloned()
            .unwrap_or_default()
    }

    /// Get signals that have no dependents (unused)
    pub fn unused_signals(&self) -> Vec<String> {
        self.signals
            .iter()
            .filter(|s| !self.dependents.contains_key(*s))
            .cloned()
            .collect()
    }

    /// Topological sort of signals for update order
    pub fn update_order(&self) -> Vec<String> {
        // Simple case: just return signals in arbitrary order
        // Full implementation would do proper topological sort
        self.signals.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_signal("count".to_string());
        graph.add_signal("doubled".to_string());

        graph.add_dependency("doubled".to_string(), "count".to_string());
        graph.add_dependency("display".to_string(), "count".to_string());
        graph.add_dependency("display".to_string(), "doubled".to_string());

        assert!(graph.is_signal("count"));
        assert!(graph.is_signal("doubled"));
        assert!(!graph.is_signal("display"));

        let count_deps = graph.get_dependents("count");
        assert!(count_deps.contains("doubled"));
        assert!(count_deps.contains("display"));

        let display_deps = graph.get_dependencies("display");
        assert!(display_deps.contains("count"));
        assert!(display_deps.contains("doubled"));
    }
}
