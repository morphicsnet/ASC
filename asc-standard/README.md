# ASC Standard Monorepo

This repository is a spec-first reference implementation for the Avionics Safety Contract (ASC).

## Quickstart

```bash
cargo run --manifest-path tools/specgen/Cargo.toml -- --profile uas-small --repo-root .
cargo test --manifest-path reference/kernel/Cargo.toml --workspace
cargo run --manifest-path tools/specgen/Cargo.toml -- --profile uas-small --repo-root . && git diff --exit-code
```

## Principles

- Spec before code.
- Deterministic replay required.
- No evidence, no merge.
