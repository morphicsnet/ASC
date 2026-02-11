# ASC Tutorials

## Tutorial 1: Spec change to generated code

1. Update a field in `spec/asc/*.yaml`.
2. Run:

```bash
cargo run --manifest-path tools/specgen/Cargo.toml -- --profile uas-small --repo-root .
```

3. Inspect generated files in `reference/kernel/crates/*/src/generated_*.rs`.
4. Run tests and confirm deterministic outputs.

## Tutorial 2: Build evidence bundle

1. Run traceability and hash locking:

```bash
python3 tools/tracecheck/tracecheck.py --repo-root .
python3 tools/hashlock/hashlock.py --repo-root .
```

2. Build release package:

```bash
python3 tools/releasepack/releasepack.py --repo-root .
```

3. Inspect:
- `evidence/manifests/tracecheck-report.json`
- `evidence/manifests/hashlock.json`
- `evidence/manifests/releasepack.tgz`
