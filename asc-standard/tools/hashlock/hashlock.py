#!/usr/bin/env python3
"""Create a deterministic hash manifest for evidence artifacts."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(8192):
            h.update(chunk)
    return h.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--input-dir", default="evidence/manifests")
    parser.add_argument("--output", default="evidence/manifests/hashlock.json")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    input_dir = repo_root / args.input_dir
    output = repo_root / args.output

    files = sorted([p for p in input_dir.glob("**/*") if p.is_file()])
    entries = []
    for f in files:
        entries.append(
            {
                "path": str(f.relative_to(repo_root)),
                "sha256": sha256_file(f),
            }
        )

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps({"entries": entries}, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
