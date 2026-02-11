use asc_types::{
    model::{CheckOutcome, KernelInput},
    ReasonCode, Severity, Verdict,
};

use crate::generated_thresholds as t;

pub fn evaluate_checks(input: &KernelInput, inter_tick_ms: Option<u64>) -> Vec<CheckOutcome> {
    let mut outcomes = Vec::new();

    if input.state.frame != t::FRAME {
        outcomes.push(CheckOutcome {
            verdict: Verdict::Shutdown,
            reason: ReasonCode::StateInvalidFrame,
            severity: Severity::Critical,
        });
    }
    if input.state.velocity_mps > t::MAX_SPEED_MPS {
        outcomes.push(CheckOutcome {
            verdict: Verdict::Clamp,
            reason: ReasonCode::StateOutOfBounds,
            severity: Severity::Warning,
        });
    }
    if input.intent.desired_rates_dps[0].abs() > t::MAX_ROLL_RATE_DPS
        || input.intent.desired_rates_dps[1].abs() > t::MAX_PITCH_RATE_DPS
        || input.intent.desired_rates_dps[2].abs() > t::MAX_YAW_RATE_DPS
        || input.intent.desired_climb_mps.abs() > t::MAX_CLIMB_RATE_MPS
    {
        outcomes.push(CheckOutcome {
            verdict: Verdict::Clamp,
            reason: ReasonCode::FlowConstraintViolation,
            severity: Severity::Warning,
        });
    }
    if input.state.soc_percent < t::MIN_SOC_PERCENT {
        outcomes.push(CheckOutcome {
            verdict: Verdict::Hold,
            reason: ReasonCode::EnergyBudgetExceeded,
            severity: Severity::Critical,
        });
    }
    if input.state.input_age_ms > t::MAX_INPUT_AGE_MS {
        outcomes.push(CheckOutcome {
            verdict: Verdict::Hold,
            reason: ReasonCode::InputStale,
            severity: Severity::Critical,
        });
    }
    if let Some(delta_ms) = inter_tick_ms {
        if delta_ms > t::MAX_TICK_INTERVAL_MS {
            outcomes.push(CheckOutcome {
                verdict: Verdict::Override,
                reason: ReasonCode::TemporalGuaranteeViolation,
                severity: Severity::Critical,
            });
        }
        if delta_ms > t::DEADLINE_MS {
            outcomes.push(CheckOutcome {
                verdict: Verdict::Override,
                reason: ReasonCode::DeadlineMiss,
                severity: Severity::Critical,
            });
        }
    }
    if input.state.position_m[2] < t::MIN_ALTITUDE_M || input.state.bank_deg.abs() > t::MAX_BANK_DEG
    {
        outcomes.push(CheckOutcome {
            verdict: Verdict::Shutdown,
            reason: ReasonCode::InvariantViolation,
            severity: Severity::Critical,
        });
    }

    outcomes
}
