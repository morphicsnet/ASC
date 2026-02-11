//! ASC Kernel - Core avionics safety contract runtime
//!
//! This crate provides the foundational types and runtime for the Avionics Safety Contract system.
//! All types in this module are generated from canonical specifications in the spec/ directory.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod contracts;
pub mod generated;
pub mod runtime;
pub mod types;

/// Core error type for the ASC kernel
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum KernelError {
    #[error("Contract violation: {0}")]
    ContractViolation(String),

    #[error("Sensor error: {0}")]
    SensorError(String),

    #[error("State validation failed: {0}")]
    ValidationError(String),

    #[error("Bounds check failed: {0}")]
    BoundsError(String),
}

/// Result type for kernel operations
pub type Result<T> = std::result::Result<T, KernelError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_error_serialization() {
        let error = KernelError::ContractViolation("test".to_string());
        let json = serde_json::to_string(&error).unwrap();
        // thiserror serializes as {"ContractViolation":"test"}
        assert!(json.contains("ContractViolation"));
        assert!(json.contains("test"));
    }
}
