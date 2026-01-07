//! Check Command
//!
//! Check files for errors without generating output.

use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

/// Run the check command
pub async fn run(files: Vec<PathBuf>) -> Result<()> {
    info!("Checking {} file(s)...", files.len());

    let mut errors = 0;
    let mut warnings = 0;

    for file in &files {
        if !file.exists() {
            eprintln!("✗ File not found: {:?}", file);
            errors += 1;
            continue;
        }

        let source = tokio::fs::read_to_string(file).await?;
        let file_name = file.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");

        // Tokenize
        let tokens = match omnicraft_compiler::Lexer::new(&source).tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("✗ {} - Lexer error: {}", file_name, e);
                errors += 1;
                continue;
            }
        };

        // Parse
        match omnicraft_compiler::Parser::new(tokens, file_name).parse() {
            Ok(_component) => {
                println!("✓ {} - OK", file_name);
            }
            Err(e) => {
                eprintln!("✗ {} - Parse error: {}", file_name, e);
                errors += 1;
            }
        }
    }

    println!();
    if errors == 0 && warnings == 0 {
        println!("All checks passed! ✓");
    } else {
        println!(
            "Check complete: {} error(s), {} warning(s)",
            errors, warnings
        );
    }

    if errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}
