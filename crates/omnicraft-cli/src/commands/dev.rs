//! Dev Server Command
//!
//! Starts a development server with hot reload.

use crate::hmr::{FileWatcher, HmrEvent, HmrMessage, inject_hmr_script};
use anyhow::Result;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::{StreamExt, SplitSink}};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    collections::HashMap,
};
use tokio::sync::broadcast;
use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use tracing::{info, error, warn};

/// Shared state for the dev server
struct AppState {
    tx: broadcast::Sender<HmrMessage>,
    public_dir: PathBuf,
    port: u16,
}

/// Run the dev server
pub async fn run(dir: PathBuf, port: u16, open: bool) -> Result<()> {
    info!("Starting development server at http://localhost:{}", port);
    info!("Project directory: {:?}", dir);

    let public_dir = dir.join("public");
    let dist_dir = dir.join("dist");

    // Create dist directory if it doesn't exist
    if !dist_dir.exists() {
        tokio::fs::create_dir_all(&dist_dir).await?;
    }

    // Setup HMR broadcast channel
    let (tx, _rx) = broadcast::channel(100);
    let state = Arc::new(AppState {
        tx: tx.clone(),
        public_dir: public_dir.clone(),
        port,
    });

    // Setup file watcher
    let mut watcher = FileWatcher::new()?;
    watcher.watch(&dir)?;

    // Initial build
    if let Err(e) = build_project(&dir).await {
        error!("Initial build failed: {}", e);
    } else {
        info!("Initial build successful");
    }

    // Start file watcher loop
    let tx_clone = tx.clone();
    let dir_clone = dir.clone(); // Clone dir for the task
    tokio::spawn(async move {
        loop {
            // We use a short sleep to prevent tight loop if poll returns immediately None
            if let Some(event) = watcher.poll() {
                match event {
                    HmrEvent::Modified(path) | HmrEvent::Created(path) | HmrEvent::Deleted(path) => {
                        info!("File changed: {:?}", path);
                        match build_project(&dir_clone).await {
                            Ok(_) => {
                                info!("Build successful");
                                let _ = tx_clone.send(HmrMessage::Reload);
                            }
                            Err(e) => {
                                error!("Build failed: {}", e);
                                let _ = tx_clone.send(HmrMessage::Error { message: e.to_string() });
                            }
                        }
                    }
                    HmrEvent::Batch(paths) => {
                        info!("Files changed: {} files", paths.len());
                         match build_project(&dir_clone).await {
                            Ok(_) => {
                                info!("Build successful");
                                let _ = tx_clone.send(HmrMessage::Reload);
                            }
                            Err(e) => {
                                error!("Build failed: {}", e);
                                let _ = tx_clone.send(HmrMessage::Error { message: e.to_string() });
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    // Setup Router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/hmr", get(ws_handler))
        .fallback_service(
            ServeDir::new(&public_dir)
                .fallback(ServeDir::new(&dist_dir))
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Platform-specific browser opening
    if open {
        info!("Opening browser...");
        #[cfg(target_os = "linux")]
        let _ = std::process::Command::new("xdg-open")
            .arg(format!("http://localhost:{}", port))
            .spawn();
    }

    // Run server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let index_path = state.public_dir.join("index.html");
    match tokio::fs::read_to_string(index_path).await {
        Ok(html) => {
            let injected = inject_hmr_script(&html, state.port);
            Html(injected).into_response()
        }
        Err(_) => {
            // If index.html not found, fallthrough or 404
            // But since we want to handle the specific route, maybe return 404 or try fallbacks?
            // For now, simple 404
            (axum::http::StatusCode::NOT_FOUND, "index.html not found").into_response()
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Spawn a task to forward broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Keep the connection alive until client disconnects
    while let Some(result) = receiver.next().await {
        if result.is_err() {
            break;
        }
    }

    send_task.abort();
}

async fn build_project(dir: &std::path::Path) -> Result<()> {
    // 1. Compile Omni to Rust
    let src_dir = dir.join("src");
    let dist_dir = dir.join("dist");
    crate::commands::compile::compile_directory(&src_dir, &dist_dir, "rust").await?;

    // 2. Build WASM
    info!("Building WASM...");
    let output = tokio::process::Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown"])
        .current_dir(dir)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("WASM build failed: {}", stderr));
    }

    // 3. Generate Bindings
    let package_name = dir.file_name().unwrap().to_str().unwrap();
    let wasm_file_name = package_name.replace("-", "_");
    let wasm_path = dir.join("target/wasm32-unknown-unknown/debug").join(format!("{}.wasm", wasm_file_name));
    let pkg_dir = dir.join("public/pkg");

    crate::commands::build::build_wasm(&wasm_path, &pkg_dir)?;

    Ok(())
}

