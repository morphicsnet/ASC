//! Evidence Artifact Generation
//!
//! This crate generates first-class evidence artifacts for safety certification.

use kernel::runtime::{ExecutionTrace, VerificationResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Evidence artifact metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceMetadata {
    pub artifact_id: String,
    pub generated_at: String,
    pub spec_version: String,
    pub kernel_version: String,
}

/// Complete evidence artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceArtifact {
    pub metadata: EvidenceMetadata,
    pub execution_trace: ExecutionTrace,
    pub summary: EvidenceSummary,
}

/// Summary of execution evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSummary {
    pub total_states: usize,
    pub successful_verifications: usize,
    pub failed_verifications: usize,
    pub contract_violations: Vec<String>,
}

/// Evidence generator
pub struct EvidenceGenerator;

impl EvidenceGenerator {
    /// Generate evidence artifact from execution trace
    pub fn generate(trace: &ExecutionTrace) -> EvidenceArtifact {
        let mut successful = 0;
        let mut failed = 0;
        let mut violations = Vec::new();

        for entry in &trace.entries {
            match &entry.verification_result {
                VerificationResult::Success => successful += 1,
                VerificationResult::Failure { reason } => {
                    failed += 1;
                    violations.push(reason.clone());
                }
            }
        }

        let metadata = EvidenceMetadata {
            artifact_id: format!("evidence-{}", chrono_timestamp()),
            generated_at: chrono_timestamp(),
            spec_version: "1.0.0".to_string(),
            kernel_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let summary = EvidenceSummary {
            total_states: trace.entries.len(),
            successful_verifications: successful,
            failed_verifications: failed,
            contract_violations: violations,
        };

        EvidenceArtifact {
            metadata,
            execution_trace: trace.clone(),
            summary,
        }
    }

    /// Export evidence to JSON file
    pub fn export_to_file(artifact: &EvidenceArtifact, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(artifact)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Generate a human-readable report
    pub fn generate_report(artifact: &EvidenceArtifact) -> String {
        let mut report = String::new();

        report.push_str("=== ASC Evidence Artifact Report ===\n\n");

        report.push_str(&format!("Artifact ID: {}\n", artifact.metadata.artifact_id));
        report.push_str(&format!("Generated: {}\n", artifact.metadata.generated_at));
        report.push_str(&format!(
            "Spec Version: {}\n",
            artifact.metadata.spec_version
        ));
        report.push_str(&format!(
            "Kernel Version: {}\n\n",
            artifact.metadata.kernel_version
        ));

        report.push_str("=== Execution Summary ===\n");
        report.push_str(&format!(
            "Total States Processed: {}\n",
            artifact.summary.total_states
        ));
        report.push_str(&format!(
            "Successful Verifications: {}\n",
            artifact.summary.successful_verifications
        ));
        report.push_str(&format!(
            "Failed Verifications: {}\n\n",
            artifact.summary.failed_verifications
        ));

        if !artifact.summary.contract_violations.is_empty() {
            report.push_str("=== Contract Violations ===\n");
            for (idx, violation) in artifact.summary.contract_violations.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", idx + 1, violation));
            }
        } else {
            report.push_str("No contract violations detected.\n");
        }

        report
    }
}

/// Simple timestamp function (returns milliseconds since UNIX epoch as string)
fn chrono_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::generated::*;
    use kernel::runtime::Runtime;

    #[test]
    fn test_evidence_generation() {
        let mut runtime = Runtime::new();
        let state = FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors: 3,
            timestamp_ms: 1000,
        };

        runtime.process_state(state).unwrap();
        let trace = runtime.get_trace();
        let artifact = EvidenceGenerator::generate(trace);

        assert_eq!(artifact.summary.total_states, 1);
        assert_eq!(artifact.summary.successful_verifications, 1);
        assert_eq!(artifact.summary.failed_verifications, 0);
    }

    #[test]
    fn test_report_generation() {
        let mut runtime = Runtime::new();
        let state = FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors: 3,
            timestamp_ms: 1000,
        };

        runtime.process_state(state).unwrap();
        let trace = runtime.get_trace();
        let artifact = EvidenceGenerator::generate(trace);
        let report = EvidenceGenerator::generate_report(&artifact);

        assert!(report.contains("Evidence Artifact Report"));
        assert!(report.contains("Total States Processed: 1"));
    }
}
