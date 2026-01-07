//! Source Map Generation
//!
//! Generates source maps for debugging compiled OmniCraft code.
//! Maps generated Rust/JS code back to original `.omni` sources.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;

/// A source map following the Source Map V3 specification
/// https://sourcemaps.info/spec.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    /// Version (always 3)
    pub version: u32,
    /// Generated file name
    pub file: String,
    /// Source root 
    #[serde(rename = "sourceRoot", skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    /// Original source files
    pub sources: Vec<String>,
    /// Source content (optional, for inline source maps)
    #[serde(rename = "sourcesContent", skip_serializing_if = "Option::is_none")]
    pub sources_content: Option<Vec<Option<String>>>,
    /// Names array (used in VLQ mapping)
    pub names: Vec<String>,
    /// VLQ-encoded mappings
    pub mappings: String,
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl SourceMap {
    /// Create a new source map
    pub fn new(file: &str, source: &str) -> Self {
        Self {
            version: 3,
            file: file.to_string(),
            source_root: None,
            sources: vec![source.to_string()],
            sources_content: None,
            names: Vec::new(),
            mappings: String::new(),
        }
    }

    /// Include source content inline
    pub fn with_source_content(mut self, content: &str) -> Self {
        self.sources_content = Some(vec![Some(content.to_string())]);
        self
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Convert to pretty JSON string
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Create inline source map data URL
    pub fn to_data_url(&self) -> String {
        let json = self.to_json();
        let encoded = base64_encode(&json);
        format!("data:application/json;charset=utf-8;base64,{}", encoded)
    }

    /// Create source map comment for JS/TS
    pub fn to_js_comment(&self) -> String {
        format!("//# sourceMappingURL={}", self.to_data_url())
    }
}

/// Source map generator that builds mappings incrementally
#[derive(Debug, Default)]
pub struct SourceMapGenerator {
    file: String,
    sources: Vec<String>,
    sources_content: Vec<Option<String>>,
    names: Vec<String>,
    mappings: Vec<Mapping>,
    name_index: BTreeMap<String, u32>,
    source_index: BTreeMap<String, u32>,
}

/// A single mapping entry
#[derive(Debug, Clone)]
pub struct Mapping {
    pub generated_line: u32,
    pub generated_column: u32,
    pub source_index: u32,
    pub original_line: u32,
    pub original_column: u32,
    pub name_index: Option<u32>,
}

impl SourceMapGenerator {
    /// Create a new source map generator
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
            ..Default::default()
        }
    }

    /// Add a source file
    pub fn add_source(&mut self, source: &str) -> u32 {
        if let Some(&idx) = self.source_index.get(source) {
            return idx;
        }
        let idx = self.sources.len() as u32;
        self.sources.push(source.to_string());
        self.sources_content.push(None);
        self.source_index.insert(source.to_string(), idx);
        idx
    }

    /// Add a source file with content
    pub fn add_source_with_content(&mut self, source: &str, content: &str) -> u32 {
        let idx = self.add_source(source);
        self.sources_content[idx as usize] = Some(content.to_string());
        idx
    }

    /// Add a name
    pub fn add_name(&mut self, name: &str) -> u32 {
        if let Some(&idx) = self.name_index.get(name) {
            return idx;
        }
        let idx = self.names.len() as u32;
        self.names.push(name.to_string());
        self.name_index.insert(name.to_string(), idx);
        idx
    }

    /// Add a mapping
    pub fn add_mapping(
        &mut self,
        generated_line: u32,
        generated_column: u32,
        source: &str,
        original_line: u32,
        original_column: u32,
        name: Option<&str>,
    ) {
        let source_index = self.add_source(source);
        let name_index = name.map(|n| self.add_name(n));

        self.mappings.push(Mapping {
            generated_line,
            generated_column,
            source_index,
            original_line,
            original_column,
            name_index,
        });
    }

    /// Generate the source map
    pub fn generate(&self) -> SourceMap {
        let mappings = self.encode_mappings();
        
        SourceMap {
            version: 3,
            file: self.file.clone(),
            source_root: None,
            sources: self.sources.clone(),
            sources_content: if self.sources_content.iter().any(|c| c.is_some()) {
                Some(self.sources_content.clone())
            } else {
                None
            },
            names: self.names.clone(),
            mappings,
        }
    }

    /// Encode mappings to VLQ format
    fn encode_mappings(&self) -> String {
        if self.mappings.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut prev_generated_line = 0u32;
        let mut prev_generated_column = 0u32;
        let mut prev_source_index = 0u32;
        let mut prev_original_line = 0u32;
        let mut prev_original_column = 0u32;
        let mut prev_name_index = 0u32;

        // Sort mappings by generated position
        let mut sorted_mappings = self.mappings.clone();
        sorted_mappings.sort_by(|a, b| {
            a.generated_line.cmp(&b.generated_line)
                .then(a.generated_column.cmp(&b.generated_column))
        });

        for mapping in &sorted_mappings {
            // Add semicolons for line changes
            while prev_generated_line < mapping.generated_line {
                result.push(';');
                prev_generated_line += 1;
                prev_generated_column = 0;
            }

            // Add comma separator within a line
            if !result.is_empty() && !result.ends_with(';') {
                result.push(',');
            }

            // Encode generated column
            result.push_str(&vlq_encode(mapping.generated_column as i32 - prev_generated_column as i32));
            prev_generated_column = mapping.generated_column;

            // Encode source index
            result.push_str(&vlq_encode(mapping.source_index as i32 - prev_source_index as i32));
            prev_source_index = mapping.source_index;

            // Encode original line
            result.push_str(&vlq_encode(mapping.original_line as i32 - prev_original_line as i32));
            prev_original_line = mapping.original_line;

            // Encode original column
            result.push_str(&vlq_encode(mapping.original_column as i32 - prev_original_column as i32));
            prev_original_column = mapping.original_column;

            // Encode name index if present
            if let Some(name_idx) = mapping.name_index {
                result.push_str(&vlq_encode(name_idx as i32 - prev_name_index as i32));
                prev_name_index = name_idx;
            }
        }

        result
    }
}

// VLQ encoding for source maps

const VLQ_BASE_SHIFT: u32 = 5;
const VLQ_BASE: i32 = 1 << VLQ_BASE_SHIFT;
const VLQ_BASE_MASK: i32 = VLQ_BASE - 1;
const VLQ_CONTINUATION_BIT: i32 = VLQ_BASE;

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn vlq_encode(value: i32) -> String {
    let mut encoded = String::new();
    let mut v = if value < 0 {
        ((-value) << 1) + 1
    } else {
        value << 1
    };

    loop {
        let mut digit = v & VLQ_BASE_MASK;
        v >>= VLQ_BASE_SHIFT;
        if v > 0 {
            digit |= VLQ_CONTINUATION_BIT;
        }
        encoded.push(BASE64_CHARS[digit as usize] as char);
        if v == 0 {
            break;
        }
    }

    encoded
}

fn base64_encode(input: &str) -> String {
    use std::io::Write;
    let mut result = Vec::new();
    {
        let mut encoder = Base64Encoder::new(&mut result);
        let _ = encoder.write_all(input.as_bytes());
    }
    String::from_utf8(result).unwrap_or_default()
}

struct Base64Encoder<W: std::io::Write> {
    writer: W,
    buffer: [u8; 3],
    buffer_len: usize,
}

impl<W: std::io::Write> Base64Encoder<W> {
    fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: [0; 3],
            buffer_len: 0,
        }
    }

    fn encode_chunk(&mut self) -> std::io::Result<()> {
        if self.buffer_len == 0 {
            return Ok(());
        }

        let mut chunk = [0u8; 4];
        let n = self.buffer_len;
        
        chunk[0] = BASE64_CHARS[(self.buffer[0] >> 2) as usize];
        chunk[1] = BASE64_CHARS[((self.buffer[0] & 0x03) << 4 | self.buffer[1] >> 4) as usize];
        chunk[2] = if n > 1 {
            BASE64_CHARS[((self.buffer[1] & 0x0f) << 2 | self.buffer[2] >> 6) as usize]
        } else {
            b'='
        };
        chunk[3] = if n > 2 {
            BASE64_CHARS[(self.buffer[2] & 0x3f) as usize]
        } else {
            b'='
        };

        self.writer.write_all(&chunk)?;
        self.buffer = [0; 3];
        self.buffer_len = 0;
        Ok(())
    }
}

impl<W: std::io::Write> std::io::Write for Base64Encoder<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for &byte in buf {
            self.buffer[self.buffer_len] = byte;
            self.buffer_len += 1;
            if self.buffer_len == 3 {
                self.encode_chunk()?;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.buffer_len > 0 {
            self.encode_chunk()?;
        }
        self.writer.flush()
    }
}

impl<W: std::io::Write> Drop for Base64Encoder<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlq_encode() {
        assert_eq!(vlq_encode(0), "A");
        assert_eq!(vlq_encode(1), "C");
        assert_eq!(vlq_encode(-1), "D");
        assert_eq!(vlq_encode(15), "e");
    }

    #[test]
    fn test_source_map_generator() {
        let mut generator = SourceMapGenerator::new("output.js");
        generator.add_source_with_content("input.omni", "<canvas></canvas>");
        generator.add_mapping(0, 0, "input.omni", 0, 0, None);
        generator.add_mapping(1, 0, "input.omni", 0, 8, None);

        let map = generator.generate();
        assert_eq!(map.version, 3);
        assert_eq!(map.sources.len(), 1);
        assert!(!map.mappings.is_empty());
    }

    #[test]
    fn test_source_map_to_json() {
        let map = SourceMap::new("output.js", "input.omni");
        let json = map.to_json();
        assert!(json.contains("\"version\":3"));
        assert!(json.contains("\"sources\""));
    }
}
