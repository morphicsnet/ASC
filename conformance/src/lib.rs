//! ASC Conformance Testing Framework
//!
//! Provides deterministic replay and conformance testing capabilities.

use kernel::generated::FlightState;
use kernel::runtime::{ExecutionTrace, Runtime};
use kernel::Result;
use serde::{Deserialize, Serialize};

/// Test case for conformance testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformanceTestCase {
    pub name: String,
    pub description: String,
    pub states: Vec<FlightState>,
    pub expected_results: Vec<ExpectedResult>,
}

/// Expected result for a test step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpectedResult {
    Success,
    Failure { pattern: String },
}

/// Conformance test runner
pub struct ConformanceRunner {
    runtime: Runtime,
}

impl ConformanceRunner {
    pub fn new() -> Self {
        ConformanceRunner {
            runtime: Runtime::new(),
        }
    }

    /// Run a conformance test case
    pub fn run_test(&mut self, test_case: &ConformanceTestCase) -> TestResult {
        let mut results = Vec::new();

        for (idx, state) in test_case.states.iter().enumerate() {
            let result = self.runtime.process_state(state.clone());
            let expected = &test_case.expected_results[idx];

            let passed = match (result, expected) {
                (Ok(_), ExpectedResult::Success) => true,
                (Err(e), ExpectedResult::Failure { pattern }) => e.to_string().contains(pattern),
                _ => false,
            };

            results.push(StepResult { step: idx, passed });
        }

        TestResult {
            test_name: test_case.name.clone(),
            steps: results,
        }
    }

    /// Get the execution trace
    pub fn get_trace(&self) -> &ExecutionTrace {
        self.runtime.get_trace()
    }
}

impl Default for ConformanceRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a single test step
#[derive(Debug, Clone)]
pub struct StepResult {
    pub step: usize,
    pub passed: bool,
}

/// Result of a complete test case
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub steps: Vec<StepResult>,
}

impl TestResult {
    pub fn all_passed(&self) -> bool {
        self.steps.iter().all(|s| s.passed)
    }

    pub fn summary(&self) -> String {
        let passed = self.steps.iter().filter(|s| s.passed).count();
        let total = self.steps.len();
        format!("{}: {}/{} steps passed", self.test_name, passed, total)
    }
}

/// Replay an execution trace deterministically
pub fn replay_trace(trace: &ExecutionTrace) -> Result<()> {
    let mut runtime = Runtime::new();

    for entry in &trace.entries {
        runtime.process_state(entry.state.clone())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::generated::*;

    fn create_test_state(active_sensors: u8) -> FlightState {
        FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors,
            timestamp_ms: 1000,
        }
    }

    #[test]
    fn test_conformance_runner() {
        let test_case = ConformanceTestCase {
            name: "Basic test".to_string(),
            description: "Test basic functionality".to_string(),
            states: vec![create_test_state(3), create_test_state(1)],
            expected_results: vec![
                ExpectedResult::Success,
                ExpectedResult::Failure {
                    pattern: "redundancy_check".to_string(),
                },
            ],
        };

        let mut runner = ConformanceRunner::new();
        let result = runner.run_test(&test_case);
        assert!(result.all_passed());
    }

    #[test]
    fn test_trace_replay() {
        let mut runtime = Runtime::new();
        let state = create_test_state(3);
        runtime.process_state(state).unwrap();

        let trace = runtime.get_trace();
        assert!(replay_trace(trace).is_ok());
    }
}
