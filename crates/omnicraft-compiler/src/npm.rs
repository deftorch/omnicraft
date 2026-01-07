//! NPM Package Generator
//!
//! Generates package.json and related NPM package files for OmniCraft components.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NPM package.json structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports: Option<PackageExports>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies", skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies", skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(rename = "sideEffects", skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageExports {
    #[serde(rename = ".")]
    pub root: ExportPaths,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPaths {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub url: String,
}

/// Builder for package.json
pub struct PackageJsonBuilder {
    package: PackageJson,
}

impl PackageJsonBuilder {
    /// Create a new package.json builder
    pub fn new(name: &str) -> Self {
        Self {
            package: PackageJson {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                description: None,
                main: None,
                module: None,
                types: None,
                exports: None,
                scripts: None,
                dependencies: None,
                dev_dependencies: None,
                peer_dependencies: None,
                keywords: None,
                author: None,
                license: None,
                repository: None,
                files: None,
                side_effects: None,
            },
        }
    }

    /// Create for OmniCraft component
    pub fn omnicraft_component(name: &str) -> Self {
        let mut builder = Self::new(name);
        builder.package.version = "0.1.0".to_string();
        builder.package.main = Some("dist/index.js".to_string());
        builder.package.module = Some("dist/index.mjs".to_string());
        builder.package.types = Some("dist/index.d.ts".to_string());
        builder.package.exports = Some(PackageExports {
            root: ExportPaths {
                import: Some("./dist/index.mjs".to_string()),
                require: Some("./dist/index.js".to_string()),
                types: Some("./dist/index.d.ts".to_string()),
            },
        });
        builder.package.files = Some(vec!["dist".to_string()]);
        builder.package.side_effects = Some(false);
        
        // Default scripts
        let mut scripts = HashMap::new();
        scripts.insert("build".to_string(), "omnicraft build".to_string());
        scripts.insert("dev".to_string(), "omnicraft dev".to_string());
        builder.package.scripts = Some(scripts);
        
        // Peer dependencies
        let mut peer_deps = HashMap::new();
        peer_deps.insert("@omnicraft/runtime".to_string(), "^0.1.0".to_string());
        builder.package.peer_dependencies = Some(peer_deps);
        
        // Dev dependencies
        let mut dev_deps = HashMap::new();
        dev_deps.insert("@omnicraft/cli".to_string(), "^0.1.0".to_string());
        dev_deps.insert("typescript".to_string(), "^5.0.0".to_string());
        builder.package.dev_dependencies = Some(dev_deps);
        
        // Keywords
        builder.package.keywords = Some(vec![
            "omnicraft".to_string(),
            "component".to_string(),
            "wasm".to_string(),
        ]);
        
        builder
    }

    pub fn version(mut self, version: &str) -> Self {
        self.package.version = version.to_string();
        self
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.package.description = Some(desc.to_string());
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.package.author = Some(author.to_string());
        self
    }

    pub fn license(mut self, license: &str) -> Self {
        self.package.license = Some(license.to_string());
        self
    }

    pub fn repository(mut self, url: &str) -> Self {
        self.package.repository = Some(Repository {
            repo_type: "git".to_string(),
            url: url.to_string(),
        });
        self
    }

    pub fn add_dependency(mut self, name: &str, version: &str) -> Self {
        self.package
            .dependencies
            .get_or_insert_with(HashMap::new)
            .insert(name.to_string(), version.to_string());
        self
    }

    pub fn add_dev_dependency(mut self, name: &str, version: &str) -> Self {
        self.package
            .dev_dependencies
            .get_or_insert_with(HashMap::new)
            .insert(name.to_string(), version.to_string());
        self
    }

    pub fn add_script(mut self, name: &str, command: &str) -> Self {
        self.package
            .scripts
            .get_or_insert_with(HashMap::new)
            .insert(name.to_string(), command.to_string());
        self
    }

    pub fn build(self) -> PackageJson {
        self.package
    }

    /// Build and serialize to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.package).unwrap_or_default()
    }
}

impl PackageJson {
    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_json_builder() {
        let pkg = PackageJsonBuilder::new("my-component")
            .version("1.0.0")
            .description("A test component")
            .author("Test Author")
            .license("MIT")
            .build();

        assert_eq!(pkg.name, "my-component");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.description, Some("A test component".to_string()));
    }

    #[test]
    fn test_omnicraft_component_package() {
        let pkg = PackageJsonBuilder::omnicraft_component("my-app").build();

        assert_eq!(pkg.name, "my-app");
        assert!(pkg.main.is_some());
        assert!(pkg.types.is_some());
        assert!(pkg.exports.is_some());
        assert!(pkg.peer_dependencies.is_some());
    }

    #[test]
    fn test_package_json_serialization() {
        let pkg = PackageJsonBuilder::omnicraft_component("test-pkg").build();
        let json = pkg.to_json();
        
        assert!(json.contains("\"name\": \"test-pkg\""));
        assert!(json.contains("\"types\""));
    }
}
