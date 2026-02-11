//! Runtime system for deterministic contract execution

use crate::contracts::FlightControlContract;
use crate::generated::FlightState;
use crate::{KernelError, Result};
use serde::{Deserialize, Serialize};

/// Execution trace for deterministic replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    pub entries: Vec<TraceEntry>,
}

impl ExecutionTrace {
    pub fn new() -> Self {
        ExecutionTrace {
            entries: Vec::new(),
        }
    }

    pub fn record(&mut self, entry: TraceEntry) {
        self.entries.push(entry);
    }
}

impl Default for ExecutionTrace {
    fn default() -> Self {
        Self::new()
    }
}

/// Single trace entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub timestamp_ms: u64,
    pub state: FlightState,
    pub verification_result: VerificationResult,
}

/// Result of contract verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationResult {
    Success,
    Failure { reason: String },
}

/// Runtime for executing and verifying flight control operations
pub struct Runtime {
    trace: ExecutionTrace,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            trace: ExecutionTrace::new(),
        }
    }

    /// Process a flight state with contract verification
    pub fn process_state(&mut self, state: FlightState) -> Result<()> {
        let verification_result = match FlightControlContract::verify_all(&state) {
            Ok(_) => VerificationResult::Success,
            Err(e) => VerificationResult::Failure {
                reason: e.to_string(),
            },
        };

        let entry = TraceEntry {
            timestamp_ms: state.timestamp_ms,
            state: state.clone(),
            verification_result: verification_result.clone(),
        };

        self.trace.record(entry);

        match verification_result {
            VerificationResult::Success => Ok(()),
            VerificationResult::Failure { reason } => Err(KernelError::ContractViolation(reason)),
        }
    }

    /// Get the execution trace for evidence generation
    pub fn get_trace(&self) -> &ExecutionTrace {
        &self.trace
    }

    /// Export trace for deterministic replay
    pub fn export_trace(&self) -> String {
        serde_json::to_string_pretty(&self.trace).unwrap()
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::*;

    #[test]
    fn test_runtime_processes_valid_state() {
        let mut runtime = Runtime::new();
        let state = FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors: 3,
            timestamp_ms: 1000,
        };

        assert!(runtime.process_state(state).is_ok());
        assert_eq!(runtime.get_trace().entries.len(), 1);
    }

    #[test]
    fn test_runtime_records_contract_violation() {
        let mut runtime = Runtime::new();
        let state = FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors: 1, // Violates redundancy requirement
            timestamp_ms: 1000,
        };

        assert!(runtime.process_state(state).is_err());
        assert_eq!(runtime.get_trace().entries.len(), 1);
    }

    #[test]
    fn test_trace_export() {
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
        let trace_json = runtime.export_trace();
        assert!(trace_json.contains("entries"));
        assert!(trace_json.contains("Success"));
    }
}
