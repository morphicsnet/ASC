use asc_types::model::{ConstrainedCommand, KernelInput, KernelOutput};
use asc_types::Verdict;

use crate::{arbitration::decide, checks::evaluate_checks, generated_thresholds as t};

pub fn constrain(input: &KernelInput, inter_tick_ms: Option<u64>) -> KernelOutput {
    let outcomes = evaluate_checks(input, inter_tick_ms);
    let verdict = decide(&outcomes);

    let mut rates = input.intent.desired_rates_dps;
    rates[0] = rates[0].clamp(-t::MAX_ROLL_RATE_DPS, t::MAX_ROLL_RATE_DPS);
    rates[1] = rates[1].clamp(-t::MAX_PITCH_RATE_DPS, t::MAX_PITCH_RATE_DPS);
    rates[2] = rates[2].clamp(-t::MAX_YAW_RATE_DPS, t::MAX_YAW_RATE_DPS);
    let climb = input
        .intent
        .desired_climb_mps
        .clamp(-t::MAX_CLIMB_RATE_MPS, t::MAX_CLIMB_RATE_MPS);

    let command = match verdict {
        Verdict::Allow | Verdict::Clamp => ConstrainedCommand {
            applied_rates_dps: rates,
            applied_climb_mps: climb,
            shutdown: false,
        },
        Verdict::Hold => ConstrainedCommand {
            applied_rates_dps: [0.0, 0.0, 0.0],
            applied_climb_mps: 0.0,
            shutdown: false,
        },
        Verdict::Override => ConstrainedCommand {
            applied_rates_dps: [0.0, 0.0, 0.0],
            applied_climb_mps: -1.0,
            shutdown: false,
        },
        Verdict::Shutdown => ConstrainedCommand {
            applied_rates_dps: [0.0, 0.0, 0.0],
            applied_climb_mps: 0.0,
            shutdown: true,
        },
    };

    KernelOutput {
        verdict,
        reasons: outcomes.iter().map(|o| o.reason).collect(),
        command,
        contract_fingerprint: String::new(),
    }
}
