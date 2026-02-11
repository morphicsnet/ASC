//! Safety contract verification
//!
//! This module implements runtime verification of safety contracts defined in spec/contracts/

use crate::generated::FlightState;
use crate::{KernelError, Result};

/// Flight control contract verification (generated from spec/contracts/flight_control.yaml)
pub struct FlightControlContract;

impl FlightControlContract {
    /// Verify preconditions before control computation
    pub fn verify_preconditions(state: &FlightState) -> Result<()> {
        // altitude_bounds: altitude >= 0 && altitude <= 50000
        let altitude = state.altitude.value();
        if !(0.0..=50000.0).contains(&altitude) {
            return Err(KernelError::ContractViolation(format!(
                "Precondition 'altitude_bounds' violated: altitude = {}",
                altitude
            )));
        }

        // speed_bounds: airspeed >= 0 && airspeed <= 600
        let airspeed = state.airspeed.value();
        if !(0.0..=600.0).contains(&airspeed) {
            return Err(KernelError::ContractViolation(format!(
                "Precondition 'speed_bounds' violated: airspeed = {}",
                airspeed
            )));
        }

        Ok(())
    }

    /// Verify postconditions after control computation
    pub fn verify_postconditions(state: &FlightState) -> Result<()> {
        // control_surface_limits: abs(aileron_deflection) <= 25 && abs(elevator_deflection) <= 30
        let aileron = state.aileron_deflection.value().abs();
        let elevator = state.elevator_deflection.value().abs();

        if aileron > 25.0 {
            return Err(KernelError::ContractViolation(format!(
                "Postcondition 'control_surface_limits' violated: aileron = {}",
                aileron
            )));
        }

        if elevator > 30.0 {
            return Err(KernelError::ContractViolation(format!(
                "Postcondition 'control_surface_limits' violated: elevator = {}",
                elevator
            )));
        }

        Ok(())
    }

    /// Verify invariants throughout execution
    pub fn verify_invariants(state: &FlightState) -> Result<()> {
        // redundancy_check: active_sensors >= 2
        if state.active_sensors < 2 {
            return Err(KernelError::ContractViolation(format!(
                "Invariant 'redundancy_check' violated: active_sensors = {}",
                state.active_sensors
            )));
        }

        Ok(())
    }

    /// Verify all contracts for a given state
    pub fn verify_all(state: &FlightState) -> Result<()> {
        Self::verify_preconditions(state)?;
        Self::verify_postconditions(state)?;
        Self::verify_invariants(state)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::*;

    fn create_valid_state() -> FlightState {
        FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors: 3,
            timestamp_ms: 1000,
        }
    }

    #[test]
    fn test_valid_state_passes_all_checks() {
        let state = create_valid_state();
        assert!(FlightControlContract::verify_all(&state).is_ok());
    }

    #[test]
    fn test_insufficient_sensors_fails_invariant() {
        let mut state = create_valid_state();
        state.active_sensors = 1;
        assert!(FlightControlContract::verify_invariants(&state).is_err());
    }

    #[test]
    fn test_excessive_control_deflection_fails_postcondition() {
        let mut state = create_valid_state();
        state.aileron_deflection = ControlSurfaceDeflection::new(26.0).unwrap();
        assert!(FlightControlContract::verify_postconditions(&state).is_err());
    }
}
