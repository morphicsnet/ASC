# ASC - Avionics Safety Contract

A spec-first, standards-grade Rust monorepo for an Avionics Safety Contract (ASC) reference implementation.

## Overview

This repository implements a safety-critical avionics system with:
- **Spec-first design**: Canonical specifications in YAML drive code generation
- **Contract-based verification**: Runtime enforcement of safety contracts
- **Deterministic execution**: Reproducible builds and execution traces
- **Evidence artifacts**: First-class outputs for safety certification

## Architecture

```
ASC/
├── spec/               # Canonical specification files (YAML)
│   ├── contracts/     # Safety contract definitions
│   ├── types/         # Type specifications
│   └── interfaces/    # Interface specifications
├── kernel/            # Core runtime and generated types
├── codegen/           # Specification-to-code generator
├── conformance/       # Conformance testing framework
└── evidence/          # Evidence artifact generation
```

## Quick Start

### Prerequisites

- Rust 1.93.0 or later
- Cargo

### Build

```bash
# Build entire workspace
cargo build --workspace

# Build with optimizations
cargo build --release --workspace
```

### Test

```bash
# Run unit tests
cargo test --workspace

# Run conformance tests
cargo run --bin asc-conformance -- --examples --verbose
```

### Generate Evidence

```bash
# Generate example evidence artifact
cargo run --bin asc-evidence -- --example --output evidence/artifacts/example.json --report
```

## Workflow

1. **Define Specifications**: Write safety contracts, types, and interfaces in `spec/` directory
2. **Generate Code**: Use `codegen` to generate Rust types and contract verification code
3. **Implement Logic**: Write application logic using generated types in `kernel`
4. **Verify Contracts**: Runtime verifies all safety contracts during execution
5. **Test Conformance**: Use `conformance` to run deterministic conformance tests
6. **Generate Evidence**: Export execution traces and evidence artifacts for certification

## Key Features

### Spec-First Design

All code is derived from canonical specifications in the `spec/` directory:

```yaml
# spec/types/core.yaml
types:
  - name: "Altitude"
    kind: "newtype"
    base_type: "f64"
    unit: "feet"
    constraints:
      min: 0.0
      max: 50000.0
```

### Contract Verification

Safety contracts are automatically verified at runtime:

```yaml
# spec/contracts/flight_control.yaml
invariants:
  - name: "redundancy_check"
    expression: "active_sensors >= 2"
    description: "At least two sensors must be active"
```

### Deterministic Replay

All execution is traced and can be deterministically replayed:

```rust
let mut runtime = Runtime::new();
runtime.process_state(flight_state)?;
let trace = runtime.get_trace();
replay_trace(trace)?; // Deterministic replay
```

### Evidence Artifacts

First-class evidence artifacts for safety certification:

```bash
$ cargo run --bin asc-evidence -- --example --report

Evidence artifact written to: evidence/artifact.json
Report written to: evidence/artifact.txt

=== ASC Evidence Artifact Report ===
Total States Processed: 3
Successful Verifications: 3
Failed Verifications: 0
```

## CI/CD

The repository includes GitHub Actions workflows for:
- ✅ Automated testing
- ✅ Conformance verification
- ✅ Evidence artifact generation
- ✅ Deterministic build verification
- ✅ Code quality checks (clippy, fmt)
- ✅ Documentation generation

See `.github/workflows/ci.yml` for details.

## Documentation

- [Specification Format](spec/README.md)
- [Kernel Documentation](kernel/README.md) _(to be generated)_
- API Documentation: `cargo doc --workspace --open`

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please ensure:
1. All tests pass: `cargo test --workspace`
2. Code is formatted: `cargo fmt --all`
3. No clippy warnings: `cargo clippy --workspace -- -D warnings`
4. Conformance tests pass: `cargo run --bin asc-conformance -- --examples`

## Safety and Certification

⚠️ **Important**: This is a reference implementation for educational and research purposes. 
For use in safety-critical systems, additional validation, verification, and certification 
activities are required per applicable standards (DO-178C, DO-254, etc.).

