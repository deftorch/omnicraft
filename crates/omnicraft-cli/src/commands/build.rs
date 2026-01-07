//! Build Command
//!
//! Builds the project for production.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::info;
use omnicraft_compiler::CompilationTarget;

/// Run the build command
pub async fn run(
    dir: PathBuf,
    output: PathBuf,
    minify: bool,
    sourcemap: bool,
) -> Result<()> {
    info!("Building project for production...");
    info!("Source: {:?}", dir);
    info!("Output: {:?}", output);
    info!("Minify: {}, Sourcemap: {}", minify, sourcemap);

    // Ensure output directory exists
    tokio::fs::create_dir_all(&output)
        .await
        .context("Failed to create output directory")?;

    // 1. Find all .omni files
    let mut files = Vec::new();
    find_omni_files(&dir, &mut files).await?;

    info!("Found {} .omni files", files.len());

    // 2. Compile all files
    for file in &files {
        let source = tokio::fs::read_to_string(file).await?;
        let file_name = file.file_name().and_then(|n| n.to_str()).unwrap_or("Component");

        match omnicraft_compiler::compile(&source, file_name, CompilationTarget::Rust) {
            Ok(rust_code) => {
                let output_name = file.file_stem().and_then(|n| n.to_str()).unwrap_or("output");
                let output_path = output.join(format!("{}.rs", output_name));

                tokio::fs::write(&output_path, rust_code).await?;
                info!("✓ {}", file_name);
            }
            Err(e) => {
                eprintln!("✗ {} - {}", file_name, e);
            }
        }
    }

    // 3. Generate Cargo.toml for the compiled code
    let cargo_toml = generate_build_cargo_toml();
    tokio::fs::write(output.join("Cargo.toml"), cargo_toml).await?;

    // 4. Build to WASM
    info!("Building WASM...");
    // In a full implementation, we would run:
    // cargo build --target wasm32-unknown-unknown --release
    // wasm-bindgen ...
    // wasm-opt ...

    info!("Build complete!");

    Ok(())
}

async fn find_omni_files(dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries = tokio::fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() && path.extension().map(|e| e == "omni").unwrap_or(false) {
            files.push(path);
        } else if path.is_dir() {
            Box::pin(find_omni_files(&path, files)).await?;
        }
    }

    Ok(())
}

fn generate_build_cargo_toml() -> String {
    r#"[package]
name = "omnicraft-app"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
omnicraft-runtime = { path = "../crates/omnicraft-runtime" }
wasm-bindgen = "0.2"

[profile.release]
opt-level = "z"
lto = true
"#
    .to_string()
    .to_string()
}

/// Run wasm-bindgen on a given wasm file
pub fn build_wasm(wasm_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    info!("Generating WASM bindings for {:?}", wasm_path);
    
    let mut bindgen = wasm_bindgen_cli_support::Bindgen::new();
    bindgen
        .input_path(wasm_path)
        .web(true)?
        .generate(out_dir)?;

    Ok(())
}
