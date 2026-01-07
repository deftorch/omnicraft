//! Hot Module Replacement
//!
//! Provides file watching and live reload capabilities for development.

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;
use tracing::{info, warn, error};

/// HMR event types
#[derive(Debug, Clone)]
pub enum HmrEvent {
    /// A file was modified
    Modified(PathBuf),
    /// A file was created
    Created(PathBuf),
    /// A file was deleted
    Deleted(PathBuf),
    /// Multiple files changed (batch)
    Batch(Vec<PathBuf>),
}

/// File watcher for HMR
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: mpsc::Receiver<Result<notify::Event, notify::Error>>,
    extensions: HashSet<String>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_millis(200)),
        )?;

        let mut extensions = HashSet::new();
        extensions.insert("omni".to_string());
        extensions.insert("css".to_string());
        extensions.insert("js".to_string());
        extensions.insert("ts".to_string());

        Ok(Self {
            watcher,
            receiver: rx,
            extensions,
        })
    }

    /// Watch a directory for changes
    pub fn watch(&mut self, path: &Path) -> Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        info!("Watching for changes in {:?}", path);
        Ok(())
    }

    /// Poll for file change events
    pub fn poll(&self) -> Option<HmrEvent> {
        match self.receiver.try_recv() {
            Ok(Ok(event)) => {
                self.process_event(event)
            }
            Ok(Err(e)) => {
                error!("Watch error: {:?}", e);
                None
            }
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                error!("File watcher disconnected");
                None
            }
        }
    }

    /// Wait for the next file change event (blocking)
    pub fn wait(&self) -> Option<HmrEvent> {
        match self.receiver.recv() {
            Ok(Ok(event)) => self.process_event(event),
            Ok(Err(e)) => {
                error!("Watch error: {:?}", e);
                None
            }
            Err(_) => {
                error!("File watcher disconnected");
                None
            }
        }
    }

    fn process_event(&self, event: notify::Event) -> Option<HmrEvent> {
        use notify::EventKind;

        let paths: Vec<PathBuf> = event
            .paths
            .into_iter()
            .filter(|p| self.should_watch_file(p))
            .collect();

        if paths.is_empty() {
            return None;
        }

        match event.kind {
            EventKind::Create(_) => {
                if paths.len() == 1 {
                    Some(HmrEvent::Created(paths.into_iter().next().unwrap()))
                } else {
                    Some(HmrEvent::Batch(paths))
                }
            }
            EventKind::Modify(_) => {
                if paths.len() == 1 {
                    Some(HmrEvent::Modified(paths.into_iter().next().unwrap()))
                } else {
                    Some(HmrEvent::Batch(paths))
                }
            }
            EventKind::Remove(_) => {
                if paths.len() == 1 {
                    Some(HmrEvent::Deleted(paths.into_iter().next().unwrap()))
                } else {
                    Some(HmrEvent::Batch(paths))
                }
            }
            _ => None,
        }
    }

    fn should_watch_file(&self, path: &Path) -> bool {
        // Skip hidden files and directories
        if path.components().any(|c| {
            c.as_os_str()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        }) {
            return false;
        }

        // Skip node_modules, target, etc.
        if path.components().any(|c| {
            let s = c.as_os_str().to_str().unwrap_or("");
            s == "node_modules" || s == "target" || s == "dist" || s == ".git"
        }) {
            return false;
        }

        // Check extension
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return self.extensions.contains(ext_str);
            }
        }

        false
    }
}

/// HMR client script injected into HTML
pub const HMR_CLIENT_SCRIPT: &str = r#"
<script>
(function() {
    const ws = new WebSocket('ws://localhost:__PORT__/hmr');
    ws.onmessage = function(event) {
        const data = JSON.parse(event.data);
        if (data.type === 'reload') {
            console.log('[HMR] Reloading...');
            window.location.reload();
        } else if (data.type === 'update') {
            console.log('[HMR] Updating module:', data.path);
            // Future: implement partial updates
            window.location.reload();
        }
    };
    ws.onopen = function() {
        console.log('[HMR] Connected');
    };
    ws.onclose = function() {
        console.log('[HMR] Disconnected, attempting reconnect...');
        setTimeout(function() {
            window.location.reload();
        }, 1000);
    };
})();
</script>
"#;

/// Inject HMR client script into HTML
pub fn inject_hmr_script(html: &str, port: u16) -> String {
    let script = HMR_CLIENT_SCRIPT.replace("__PORT__", &port.to_string());
    
    if let Some(pos) = html.find("</body>") {
        let mut result = html.to_string();
        result.insert_str(pos, &script);
        result
    } else if let Some(pos) = html.find("</html>") {
        let mut result = html.to_string();
        result.insert_str(pos, &script);
        result
    } else {
        format!("{}{}", html, script)
    }
}

/// Message types for HMR WebSocket
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum HmrMessage {
    /// Full page reload
    #[serde(rename = "reload")]
    Reload,
    /// Module update (partial)
    #[serde(rename = "update")]
    Update { path: String },
    /// Error message
    #[serde(rename = "error")]
    Error { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_hmr_script() {
        let html = "<html><body><h1>Hello</h1></body></html>";
        let result = inject_hmr_script(html, 3000);
        assert!(result.contains("new WebSocket"));
        assert!(result.contains("3000"));
    }

    #[test]
    fn test_hmr_message_serialize() {
        let msg = HmrMessage::Reload;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("reload"));
    }
}
