//! ASC Evidence Artifact Generator CLI

use clap::Parser;
use evidence::{EvidenceArtifact, EvidenceGenerator};
use kernel::generated::*;
use kernel::runtime::{ExecutionTrace, Runtime};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "asc-evidence")]
#[command(about = "Generate evidence artifacts from execution traces")]
struct Cli {
    /// Path to execution trace JSON file
    #[arg(short, long)]
    trace_file: Option<PathBuf>,

    /// Output file for evidence artifact
    #[arg(short, long, default_value = "evidence/artifact.json")]
    output: PathBuf,

    /// Generate example evidence
    #[arg(short, long)]
    example: bool,

    /// Generate human-readable report
    #[arg(short, long)]
    report: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let artifact = if cli.example {
        generate_example_evidence(cli.verbose)?
    } else if let Some(trace_file) = cli.trace_file {
        load_and_generate(&trace_file, cli.verbose)?
    } else {
        println!("Please specify --example or --trace-file");
        println!("Run with --help for more information");
        return Ok(());
    };

    // Create output directory if it doesn't exist
    if let Some(parent) = cli.output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Export evidence artifact
    EvidenceGenerator::export_to_file(&artifact, &cli.output)?;
    println!("Evidence artifact written to: {}", cli.output.display());

    // Generate report if requested
    if cli.report {
        let report = EvidenceGenerator::generate_report(&artifact);
        println!("\n{}", report);

        let report_path = cli.output.with_extension("txt");
        std::fs::write(&report_path, report)?;
        println!("Report written to: {}", report_path.display());
    }

    Ok(())
}

fn generate_example_evidence(verbose: bool) -> anyhow::Result<EvidenceArtifact> {
    if verbose {
        println!("Generating example evidence...");
    }

    let mut runtime = Runtime::new();

    // Process a series of flight states
    let states = vec![
        FlightState {
            altitude: Altitude::new(10000.0).unwrap(),
            airspeed: Airspeed::new(250.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(10.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(5.0).unwrap(),
            active_sensors: 3,
            timestamp_ms: 1000,
        },
        FlightState {
            altitude: Altitude::new(10500.0).unwrap(),
            airspeed: Airspeed::new(255.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(12.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(6.0).unwrap(),
            active_sensors: 3,
            timestamp_ms: 2000,
        },
        FlightState {
            altitude: Altitude::new(11000.0).unwrap(),
            airspeed: Airspeed::new(260.0).unwrap(),
            aileron_deflection: ControlSurfaceDeflection::new(15.0).unwrap(),
            elevator_deflection: ControlSurfaceDeflection::new(7.0).unwrap(),
            active_sensors: 2,
            timestamp_ms: 3000,
        },
    ];

    for state in states {
        let _ = runtime.process_state(state);
    }

    let trace = runtime.get_trace();
    let artifact = EvidenceGenerator::generate(trace);

    if verbose {
        println!("Processed {} states", artifact.summary.total_states);
    }

    Ok(artifact)
}

fn load_and_generate(path: &PathBuf, verbose: bool) -> anyhow::Result<EvidenceArtifact> {
    if verbose {
        println!("Loading trace from: {}", path.display());
    }

    let content = std::fs::read_to_string(path)?;
    let trace: ExecutionTrace = serde_json::from_str(&content)?;

    if verbose {
        println!("Loaded trace with {} entries", trace.entries.len());
    }

    let artifact = EvidenceGenerator::generate(&trace);

    Ok(artifact)
}
