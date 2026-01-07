//! OmniCraft CLI
//!
//! Command-line interface for the OmniCraft compiler and dev tools.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod hmr;

#[derive(Parser)]
#[command(name = "omnicraft")]
#[command(author, version, about = "OmniCraft compiler and development tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile .omni files to Rust/WASM
    Compile {
        /// Input file or directory
        #[arg(required = true)]
        input: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Watch for changes
        #[arg(short, long)]
        watch: bool,

        /// Output format (rust, wasm)
        #[arg(short, long, default_value = "rust")]
        format: String,
    },

    /// Start development server
    Dev {
        /// Project directory
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Port number
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Open browser
        #[arg(long)]
        open: bool,
    },

    /// Build for production
    Build {
        /// Project directory
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Enable minification
        #[arg(long)]
        minify: bool,

        /// Enable source maps
        #[arg(long)]
        sourcemap: bool,
    },

    /// Initialize a new project
    Init {
        /// Project name
        name: String,

        /// Template to use
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// Check files for errors without compiling
    Check {
        /// Files to check
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("omnicraft=info".parse().unwrap()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            output,
            watch,
            format,
        } => {
            commands::compile::run(input, output, watch, format).await?;
        }

        Commands::Dev { dir, port, open } => {
            commands::dev::run(dir, port, open).await?;
        }

        Commands::Build {
            dir,
            output,
            minify,
            sourcemap,
        } => {
            commands::build::run(dir, output, minify, sourcemap).await?;
        }

        Commands::Init { name, template } => {
            commands::init::run(name, template).await?;
        }

        Commands::Check { files } => {
            commands::check::run(files).await?;
        }
    }

    Ok(())
}
