# ASC Quick Start Guide

This guide will help you get started with the Avionics Safety Contract (ASC) reference implementation.

## Prerequisites

- Rust 1.93.0 or later
- Cargo (included with Rust)

## Quick Start

### 1. Build the Project

```bash
# Build all workspace members
cargo build --workspace

# Build with optimizations
cargo build --release --workspace
```

### 2. Run Tests

```bash
# Run all unit tests
cargo test --workspace

# Run conformance tests
cargo run --bin asc-conformance -- --examples --verbose
```

### 3. Generate Evidence Artifacts

```bash
# Generate example evidence
cargo run --bin asc-evidence -- --example --output evidence/artifacts/demo.json --report
```

### 4. Run Code Generation Tool

```bash
# Run the code generator (currently informational)
cargo run --bin asc-codegen -- --verbose
```

## Understanding the Architecture

### Specification Files (`spec/`)

The `spec/` directory contains the canonical source of truth:

- **contracts/** - Safety contract definitions with pre/post conditions and invariants
- **types/** - Type specifications with constraints
- **interfaces/** - Interface definitions between components

Example spec file structure:

```yaml
version: "1.0.0"
name: "FlightControlContract"
preconditions:
  - name: "altitude_bounds"
    expression: "altitude >= 0 && altitude <= 50000"
```

### Kernel (`kernel/`)

The kernel crate implements:

- **Runtime** - Deterministic execution and tracing
- **Generated Types** - Bounded types with compile-time safety
- **Contract Verification** - Runtime verification of safety contracts

Example usage:

```rust
use kernel::runtime::Runtime;
use kernel::generated::*;

let mut runtime = Runtime::new();
let state = FlightState {
    altitude: Altitude::new(10000.0)?,
    airspeed: Airspeed::new(250.0)?,
    // ... more fields
};

runtime.process_state(state)?;
let trace = runtime.get_trace();
```

### Conformance Testing (`conformance/`)

Run conformance tests to verify system behavior:

```rust
use conformance::{ConformanceTestCase, ConformanceRunner};

let test_case = ConformanceTestCase {
    name: "Test Name".to_string(),
    description: "Test description".to_string(),
    states: vec![/* flight states */],
    expected_results: vec![/* expected outcomes */],
};

let mut runner = ConformanceRunner::new();
let result = runner.run_test(&test_case);
```

### Evidence Generation (`evidence/`)

Generate certification evidence:

```bash
# Generate evidence from execution trace
cargo run --bin asc-evidence -- \
    --trace-file path/to/trace.json \
    --output evidence/artifacts/cert.json \
    --report
```

## Development Workflow

1. **Define Specifications** - Write YAML specs in `spec/`
2. **Generate Code** - Use `codegen` to generate types and contracts
3. **Implement Logic** - Write application code in `kernel`
4. **Test** - Run unit tests and conformance tests
5. **Generate Evidence** - Create certification artifacts

## CI/CD Pipeline

The repository includes GitHub Actions workflows for:

- ✅ Code quality checks (fmt, clippy)
- ✅ Automated testing
- ✅ Conformance verification
- ✅ Evidence artifact generation
- ✅ Deterministic build verification

See `.github/workflows/ci.yml` for details.

## Common Commands

```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace --all-targets -- -D warnings

# Run specific test
cargo test --package kernel test_name

# Build release binaries
cargo build --release --workspace

# Generate documentation
cargo doc --workspace --open
```

## Troubleshooting

### Build Errors

If you encounter build errors, ensure you have the latest Rust version:

```bash
rustup update
```

### Test Failures

Run tests with verbose output:

```bash
cargo test --workspace -- --nocapture
```

### Conformance Test Issues

Run conformance tests with verbose flag:

```bash
cargo run --bin asc-conformance -- --examples --verbose
```

## Next Steps

- Review the [specification format](spec/README.md)
- Explore example specifications in `spec/`
- Read the kernel documentation: `cargo doc --package kernel --open`
- Implement your own safety contracts
- Create custom conformance tests

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- Repository: https://github.com/morphicsnet/ASC

## Safety Notice

⚠️ This is a reference implementation for educational purposes. For use in safety-critical systems, additional validation, verification, and certification activities are required per applicable standards (DO-178C, DO-254, etc.).
