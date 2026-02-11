# ASC Handbook

## Purpose

This handbook describes how to operate the ASC reference repository as a spec-first, evidence-producing engineering system.

## Repository operating model

1. Edit canonical spec (`ASC.md` + `spec/*.yaml`).
2. Regenerate derived artifacts with `tools/specgen`.
3. Run kernel conformance tests.
4. Produce evidence manifests (`tracecheck`, `hashlock`, `releasepack`).
5. Verify CI drift/evidence gates before merge.

## Key commands

```bash
cargo run --manifest-path tools/specgen/Cargo.toml -- --profile uas-small --repo-root .
cargo test --manifest-path reference/kernel/Cargo.toml --workspace
python3 tools/tracecheck/tracecheck.py --repo-root .
python3 tools/hashlock/hashlock.py --repo-root .
python3 tools/releasepack/releasepack.py --repo-root .
```

## Evidence expectations

- Every test ID in `safety-case/traceability/spec_to_test.csv` should map to evidence via `test_to_evidence.csv`.
- `evidence/manifests/spec-hash.txt` is the canonical reproducibility marker.
- `evidence/manifests/hashlock.json` records immutable checksums for evidence artifacts.
