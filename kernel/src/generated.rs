//! Generated types from specification files
//!
//! This module is automatically generated from spec/*.yaml files by the codegen crate.
//! DO NOT EDIT MANUALLY - your changes will be overwritten.

use crate::KernelError;
use serde::{Deserialize, Serialize};

/// Altitude in feet (generated from spec/types/core.yaml)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Altitude(f64);

impl Altitude {
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 50000.0;

    pub fn new(value: f64) -> Result<Self, KernelError> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Altitude(value))
        } else {
            Err(KernelError::BoundsError(format!(
                "Altitude {} is out of bounds [{}, {}]",
                value,
                Self::MIN,
                Self::MAX
            )))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Airspeed in knots (generated from spec/types/core.yaml)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Airspeed(f64);

impl Airspeed {
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 600.0;

    pub fn new(value: f64) -> Result<Self, KernelError> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Airspeed(value))
        } else {
            Err(KernelError::BoundsError(format!(
                "Airspeed {} is out of bounds [{}, {}]",
                value,
                Self::MIN,
                Self::MAX
            )))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Control surface deflection in degrees (generated from spec/types/core.yaml)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ControlSurfaceDeflection(f64);

impl ControlSurfaceDeflection {
    pub const MIN: f64 = -30.0;
    pub const MAX: f64 = 30.0;

    pub fn new(value: f64) -> Result<Self, KernelError> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(ControlSurfaceDeflection(value))
        } else {
            Err(KernelError::BoundsError(format!(
                "ControlSurfaceDeflection {} is out of bounds [{}, {}]",
                value,
                Self::MIN,
                Self::MAX
            )))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Sensor status enumeration (generated from spec/types/core.yaml)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorStatus {
    Active,
    Inactive,
    Failed,
    Degraded,
}

/// Flight state structure (generated from spec/types/core.yaml)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightState {
    pub altitude: Altitude,
    pub airspeed: Airspeed,
    pub aileron_deflection: ControlSurfaceDeflection,
    pub elevator_deflection: ControlSurfaceDeflection,
    pub active_sensors: u8,
    pub timestamp_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_altitude_bounds() {
        assert!(Altitude::new(1000.0).is_ok());
        assert!(Altitude::new(-1.0).is_err());
        assert!(Altitude::new(60000.0).is_err());
    }

    #[test]
    fn test_airspeed_bounds() {
        assert!(Airspeed::new(250.0).is_ok());
        assert!(Airspeed::new(-1.0).is_err());
        assert!(Airspeed::new(700.0).is_err());
    }

    #[test]
    fn test_control_surface_bounds() {
        assert!(ControlSurfaceDeflection::new(15.0).is_ok());
        assert!(ControlSurfaceDeflection::new(-40.0).is_err());
        assert!(ControlSurfaceDeflection::new(40.0).is_err());
    }
}
