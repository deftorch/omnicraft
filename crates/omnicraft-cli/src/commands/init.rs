//! Init Command
//!
//! Initialize a new OmniCraft project.

use anyhow::{Context, Result};
use tracing::info;

/// Run the init command
pub async fn run(name: String, template: String) -> Result<()> {
    info!("Creating new OmniCraft project: {}", name);
    info!("Using template: {}", template);

    // Create project directory
    tokio::fs::create_dir_all(&name)
        .await
        .context("Failed to create project directory")?;

    // Create subdirectories
    tokio::fs::create_dir_all(format!("{}/src", name)).await?;
    tokio::fs::create_dir_all(format!("{}/public", name)).await?;

    // Create main component
    let app_omni = match template.as_str() {
        "basic" => TEMPLATE_BASIC,
        "counter" => TEMPLATE_COUNTER,
        _ => TEMPLATE_BASIC,
    };
    tokio::fs::write(format!("{}/src/App.omni", name), app_omni).await?;

    // Create index.html
    let index_html = generate_index_html(&name);
    tokio::fs::write(format!("{}/public/index.html", name), index_html).await?;

    // Create omnicraft.config.json
    let config = generate_config(&name);
    tokio::fs::write(format!("{}/omnicraft.config.json", name), config).await?;

    // Create README
    let readme = generate_readme(&name);
    tokio::fs::write(format!("{}/README.md", name), readme).await?;

    info!("✓ Project created successfully!");
    info!("");
    info!("Next steps:");
    info!("  cd {}", name);
    info!("  omnicraft dev");
    info!("");

    Ok(())
}

const TEMPLATE_BASIC: &str = r##"<canvas width={800} height={600} background="#1a1a2e">
  <circle x={400} y={300} radius={50} fill="#00d4ff" />
  <text x={400} y={400} content="Hello, OmniCraft!" fill="#ffffff" />
</canvas>
"##;

const TEMPLATE_COUNTER: &str = r##"<script>
  const count = signal(0);
  
  function increment() {
    count.set(count.get() + 1);
  }
  
  function decrement() {
    count.set(count.get() - 1);
  }
</script>

<canvas width={800} height={600} background="#1a1a2e">
  <text x={400} y={250} content={`Count: ${count()}`} fill="#ffffff" />
  
  <rectangle x={300} y={350} width={80} height={40} fill="#00d4ff" @click={decrement} />
  <text x={300} y={355} content="-" fill="#ffffff" />
  
  <rectangle x={500} y={350} width={80} height={40} fill="#00d4ff" @click={increment} />
  <text x={500} y={355} content="+" fill="#ffffff" />
</canvas>
"##;

fn generate_index_html(name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{}</title>
  <style>
    * {{
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }}
    body {{
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
      background: #0f0f1a;
    }}
    #canvas {{
      border-radius: 8px;
      box-shadow: 0 4px 20px rgba(0, 212, 255, 0.2);
    }}
  </style>
</head>
<body>
  <canvas id="canvas" width="800" height="600"></canvas>
  <script type="module">
    import init from './pkg/app.js';
    init();
  </script>
</body>
</html>
"#,
        name
    )
}

fn generate_config(name: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "entry": "src/App.omni",
  "output": "dist",
  "dev": {{
    "port": 3000,
    "open": true
  }},
  "build": {{
    "minify": true,
    "sourcemap": true,
    "target": "wasm"
  }}
}}
"#,
        name
    )
}

fn generate_readme(name: &str) -> String {
    format!(
        r#"# {}

An OmniCraft project.

## Development

```bash
omnicraft dev
```

## Build

```bash
omnicraft build
```

## Project Structure

```
{}/
├── src/
│   └── App.omni      # Main component
├── public/
│   └── index.html    # HTML template
├── omnicraft.config.json
└── README.md
```
"#,
        name, name
    )
}
