//! ASC Conformance Test Runner CLI

use clap::Parser;
use conformance::{ConformanceRunner, ConformanceTestCase, ExpectedResult};
use kernel::generated::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "asc-conformance")]
#[command(about = "Run ASC conformance tests")]
struct Cli {
    /// Path to test case file (JSON)
    #[arg(short, long)]
    test_file: Option<PathBuf>,

    /// Run built-in example tests
    #[arg(short, long)]
    examples: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.examples {
        run_example_tests(cli.verbose)?;
    } else if let Some(test_file) = cli.test_file {
        run_test_file(&test_file, cli.verbose)?;
    } else {
        println!("Please specify --examples or --test-file");
        println!("Run with --help for more information");
    }

    Ok(())
}

fn run_example_tests(verbose: bool) -> anyhow::Result<()> {
    println!("Running built-in example conformance tests...\n");

    let test_case = ConformanceTestCase {
        name: "Flight Control Validation".to_string(),
        description: "Validates flight control contracts".to_string(),
        states: vec![
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
                active_sensors: 1, // Violates redundancy requirement
                timestamp_ms: 3000,
            },
        ],
        expected_results: vec![
            ExpectedResult::Success,
            ExpectedResult::Success,
            ExpectedResult::Failure {
                pattern: "redundancy_check".to_string(),
            },
        ],
    };

    let mut runner = ConformanceRunner::new();
    let result = runner.run_test(&test_case);

    println!("{}", result.summary());

    if verbose {
        println!("\nDetailed results:");
        for step in &result.steps {
            let status = if step.passed { "PASS" } else { "FAIL" };
            println!("  Step {}: {}", step.step, status);
        }

        println!("\nExecution trace:");
        let trace_json = runner.get_trace();
        let json = serde_json::to_string_pretty(&trace_json)?;
        println!("{}", json);
    }

    if result.all_passed() {
        println!("\n✓ All conformance tests passed");
        Ok(())
    } else {
        println!("\n✗ Some conformance tests failed");
        std::process::exit(1);
    }
}

fn run_test_file(path: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    println!("Loading test case from: {}", path.display());

    let content = std::fs::read_to_string(path)?;
    let test_case: ConformanceTestCase = serde_json::from_str(&content)?;

    let mut runner = ConformanceRunner::new();
    let result = runner.run_test(&test_case);

    println!("{}", result.summary());

    if verbose {
        println!("\nDetailed results:");
        for step in &result.steps {
            let status = if step.passed { "PASS" } else { "FAIL" };
            println!("  Step {}: {}", step.step, status);
        }
    }

    if result.all_passed() {
        println!("\n✓ All conformance tests passed");
        Ok(())
    } else {
        println!("\n✗ Some conformance tests failed");
        std::process::exit(1);
    }
}
