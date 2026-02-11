#!/usr/bin/env python3
"""Build a release evidence tarball and a manifest summary."""

from __future__ import annotations

import argparse
import json
import tarfile
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--output", default="evidence/manifests/releasepack.tgz")
    parser.add_argument(
        "--include",
        nargs="*",
        default=[
            "evidence/manifests/spec-hash.txt",
            "evidence/manifests/tracecheck-report.json",
            "evidence/manifests/hashlock.json",
        ],
    )
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    out_path = repo_root / args.output
    out_path.parent.mkdir(parents=True, exist_ok=True)

    included = []
    with tarfile.open(out_path, "w:gz") as archive:
        for rel in args.include:
            p = repo_root / rel
            if p.exists() and p.is_file():
                archive.add(p, arcname=rel)
                included.append(rel)

    manifest = {
        "package": str(Path(args.output)),
        "included": included,
    }
    (repo_root / "evidence/manifests/releasepack.json").write_text(
        json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
    )
    print(f"built {out_path} with {len(included)} files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
