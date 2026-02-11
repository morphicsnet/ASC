mod io;
mod model;
mod normalize;
mod render;

use anyhow::{bail, Context, Result};
use clap::Parser;
use model::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "uas-small")]
    profile: String,
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<()> {
    let spec_root = args.repo_root.join("spec");

    let tuple_raw = io::read(&spec_root.join("asc/tuple.yaml"))?;
    let state_raw = io::read(&spec_root.join("asc/state-se3.yaml"))?;
    let flow_raw = io::read(&spec_root.join("asc/flow-phs.yaml"))?;
    let energy_raw = io::read(&spec_root.join("asc/energy-contract.yaml"))?;
    let guarantees_raw = io::read(&spec_root.join("asc/guarantees-stl.yaml"))?;
    let inv_raw = io::read(&spec_root.join("asc/invariants-rcbf.yaml"))?;
    let profile_raw = io::read(&spec_root.join(format!("profiles/{}.yaml", args.profile)))
        .with_context(|| "profile file missing")?;

    let tuple: TupleSpec = serde_yaml::from_str(&tuple_raw)?;
    let state: StateSpec = serde_yaml::from_str(&state_raw)?;
    let flow: FlowSpec = serde_yaml::from_str(&flow_raw)?;
    let energy: EnergySpec = serde_yaml::from_str(&energy_raw)?;
    let guarantees: GuaranteesSpec = serde_yaml::from_str(&guarantees_raw)?;
    let inv: InvariantsSpec = serde_yaml::from_str(&inv_raw)?;
    let profile: ProfileSpec = serde_yaml::from_str(&profile_raw)?;

    validate(&tuple, &state, &flow, &energy, &guarantees, &inv, &profile)?;

    let canonical = [
        normalize::canonicalize(&tuple_raw),
        normalize::canonicalize(&state_raw),
        normalize::canonicalize(&flow_raw),
        normalize::canonicalize(&energy_raw),
        normalize::canonicalize(&guarantees_raw),
        normalize::canonicalize(&inv_raw),
        normalize::canonicalize(&profile_raw),
    ]
    .join("\n");

    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let digest = hex::encode(hasher.finalize());
    println!("spec-hash:{}", digest);

    let reason_out = render::render_reason_codes(&tuple);
    let thresholds_out = render::render_thresholds(&state, &flow, &energy, &guarantees, &inv);
    let profile_out = render::render_profile(&profile);

    io::write_if_changed(
        &args
            .repo_root
            .join("reference/kernel/crates/asc-types/src/generated_reason_codes.rs"),
        &reason_out,
    )?;
    io::write_if_changed(
        &args
            .repo_root
            .join("reference/kernel/crates/asc-kernel-model/src/generated_thresholds.rs"),
        &thresholds_out,
    )?;
    io::write_if_changed(
        &args
            .repo_root
            .join("reference/kernel/crates/asc-kernel-model/src/generated_profile.rs"),
        &profile_out,
    )?;
    io::write_if_changed(
        &args.repo_root.join("evidence/manifests/spec-hash.txt"),
        &format!("{}\n", digest),
    )?;

    Ok(())
}

fn validate(
    tuple: &TupleSpec,
    state: &StateSpec,
    flow: &FlowSpec,
    energy: &EnergySpec,
    guarantees: &GuaranteesSpec,
    inv: &InvariantsSpec,
    profile: &ProfileSpec,
) -> Result<()> {
    const REQUIRED_REASONS: [&str; 8] = [
        "StateInvalidFrame",
        "StateOutOfBounds",
        "FlowConstraintViolation",
        "EnergyBudgetExceeded",
        "TemporalGuaranteeViolation",
        "InvariantViolation",
        "InputStale",
        "DeadlineMiss",
    ];

    let unique = tuple.reason_codes.iter().collect::<BTreeSet<_>>();
    if unique.len() != tuple.reason_codes.len() {
        bail!("tuple.reason_codes contains duplicates")
    }
    for required in REQUIRED_REASONS {
        if !tuple.reason_codes.iter().any(|r| r == required) {
            bail!("tuple.reason_codes missing required value: {required}")
        }
    }
    if !tuple.severities.iter().any(|s| s == "Critical") {
        bail!("tuple.severities must include Critical")
    }
    if state.max_speed_mps <= 0.0 {
        bail!("state.max_speed_mps must be > 0")
    }
    if flow.max_roll_rate_dps <= 0.0
        || flow.max_pitch_rate_dps <= 0.0
        || flow.max_yaw_rate_dps <= 0.0
        || flow.max_climb_rate_mps <= 0.0
    {
        bail!("flow limits must be > 0")
    }
    if !(0.0..=100.0).contains(&energy.min_soc_percent) {
        bail!("energy.min_soc_percent must be in [0, 100]")
    }
    if guarantees.deadline_ms == 0 || guarantees.max_tick_interval_ms == 0 {
        bail!("guarantee timings must be > 0")
    }
    if guarantees.deadline_ms > guarantees.max_tick_interval_ms {
        bail!("guarantees.deadline_ms must be <= max_tick_interval_ms")
    }
    if inv.min_altitude_m < 0.0 || inv.max_bank_deg <= 0.0 {
        bail!("invariant bounds invalid")
    }
    if profile.timing.control_hz == 0 || profile.timing.deadline_ms == 0 {
        bail!("profile timing values must be > 0")
    }

    Ok(())
}
