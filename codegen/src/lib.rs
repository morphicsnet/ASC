//! ASC Code Generation Library
//!
//! This crate provides utilities for generating Rust code from ASC specification files.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Specification loader
pub struct SpecLoader;

impl SpecLoader {
    /// Load a YAML specification file
    pub fn load_yaml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read spec file: {}", path.display()))?;

        let spec: T = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse spec file: {}", path.display()))?;

        Ok(spec)
    }
}

/// Base specification metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecMetadata {
    pub version: String,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_metadata() {
        let meta = SpecMetadata {
            version: "1.0.0".to_string(),
            name: "Test".to_string(),
            description: "Test spec".to_string(),
        };
        assert_eq!(meta.version, "1.0.0");
    }
}
