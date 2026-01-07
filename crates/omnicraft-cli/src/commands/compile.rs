//! Compile Command
//!
//! Compiles .omni files to Rust code.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};

/// Run the compile command
pub async fn run(input: PathBuf, output: PathBuf, watch: bool, format: String) -> Result<()> {
    info!("Compiling {:?} to {:?} (format: {})", input, output, format);

    // Ensure output directory exists
    tokio::fs::create_dir_all(&output)
        .await
        .context("Failed to create output directory")?;

    if input.is_file() {
        compile_file(&input, &output, &format).await?;
    } else if input.is_dir() {
        compile_directory(&input, &output, &format).await?;
    } else {
        anyhow::bail!("Input path does not exist: {:?}", input);
    }

    if watch {
        info!("Watching for changes...");
        watch_and_compile(input, output, format).await?;
    }

    Ok(())
}

pub async fn compile_file(input: &PathBuf, output: &PathBuf, format: &str) -> Result<()> {
    let source = tokio::fs::read_to_string(input)
        .await
        .context("Failed to read input file")?;

    let file_name = input
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Component");

    let target = match format {
        "ts" | "typescript" => omnicraft_compiler::CompilationTarget::TypeScript,
        _ => omnicraft_compiler::CompilationTarget::Rust,
    };

    match omnicraft_compiler::compile(&source, file_name, target) {
        Ok(code) => {
            let output_name = input
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("output");

            let output_path = match format {
                "rust" => output.join(format!("{}.rs", output_name)),
                "wasm" => {
                    warn!("WASM output requires additional compilation step");
                    output.join(format!("{}.rs", output_name))
                }
                "ts" | "typescript" => output.join(format!("{}.d.ts", output_name)),
                _ => output.join(format!("{}.rs", output_name)),
            };

            tokio::fs::write(&output_path, code)
                .await
                .context("Failed to write output file")?;

            info!("✓ Compiled {} → {:?}", file_name, output_path);
        }
        Err(e) => {
            eprintln!("✗ Compilation error in {}:", file_name);
            eprintln!("  {}", e);
        }
    }

    Ok(())
}

pub async fn compile_directory(input: &PathBuf, output: &PathBuf, format: &str) -> Result<()> {
    let mut entries = tokio::fs::read_dir(input).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() && path.extension().map(|e| e == "omni").unwrap_or(false) {
            compile_file(&path, output, format).await?;
        } else if path.is_dir() {
            // Recursively compile subdirectories
            let subdir_output = output.join(path.file_name().unwrap_or_default());
            tokio::fs::create_dir_all(&subdir_output).await?;
            Box::pin(compile_directory(&path, &subdir_output, format)).await?;
        }
    }

    Ok(())
}

async fn watch_and_compile(input: PathBuf, output: PathBuf, format: String) -> Result<()> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )?;

    watcher.watch(&input, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                info!("File changed: {:?}", event);

                // Recompile
                if input.is_file() {
                    let _ = compile_file(&input, &output, &format).await;
                } else {
                    let _ = compile_directory(&input, &output, &format).await;
                }
            }
            Err(e) => {
                warn!("Watch error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
