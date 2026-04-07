#!/usr/bin/env python3
"""Collect criterion benchmark results into a JSON summary file.

Walks criterion's output directory structure and extracts mean cycle counts
and cycles-per-byte (cpb) metrics for each benchmark group and algorithm.

Usage:
    python3 collect_results.py \
        --criterion-dir target/criterion \
        --output /tmp/bench-results/2026-04-03.json \
        --date 2026-04-03 \
        --commit abc1234
"""

import argparse
import json
import os
import sys
from pathlib import Path


def collect_results(criterion_dir):
    """Walk criterion output directory and extract benchmark metrics."""
    results = {}
    criterion_path = Path(criterion_dir)

    if not criterion_path.exists():
        return results

    for group_dir in sorted(criterion_path.iterdir()):
        if not group_dir.is_dir() or group_dir.name == "report":
            continue

        group_name = group_dir.name
        results[group_name] = {}

        for bench_dir in sorted(group_dir.iterdir()):
            if not bench_dir.is_dir() or bench_dir.name == "report":
                continue

            bench_name = bench_dir.name
            estimates_file = bench_dir / "new" / "estimates.json"
            benchmark_file = bench_dir / "new" / "benchmark.json"

            if not estimates_file.exists():
                continue

            with open(estimates_file) as f:
                estimates = json.load(f)

            throughput_bytes = None
            if benchmark_file.exists():
                with open(benchmark_file) as f:
                    benchmark = json.load(f)
                tp = benchmark.get("throughput")
                if tp and "Bytes" in tp:
                    throughput_bytes = tp["Bytes"]

            mean_cycles = estimates["mean"]["point_estimate"]

            entry = {"cycles": round(mean_cycles, 2)}
            if throughput_bytes:
                entry["bytes"] = throughput_bytes
                entry["cpb"] = round(mean_cycles / throughput_bytes, 4)

            results[group_name][bench_name] = entry

    return results


def main():
    parser = argparse.ArgumentParser(
        description="Collect criterion benchmark results into JSON"
    )
    parser.add_argument(
        "--criterion-dir",
        default="target/criterion",
        help="Path to criterion output directory",
    )
    parser.add_argument("--output", required=True, help="Output JSON file path")
    parser.add_argument(
        "--date", required=True, help="Date for the results (YYYY-MM-DD)"
    )
    parser.add_argument(
        "--commit", default="unknown", help="Git commit SHA"
    )
    args = parser.parse_args()

    results = collect_results(args.criterion_dir)

    if not results:
        print("Error: no benchmark results found in", args.criterion_dir, file=sys.stderr)
        sys.exit(1)

    output = {
        "date": args.date,
        "commit": args.commit,
        "results": results,
    }

    os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
    with open(args.output, "w") as f:
        json.dump(output, f, indent=2)

    total = sum(len(v) for v in results.values())
    print(f"Collected {total} benchmarks across {len(results)} groups -> {args.output}")


if __name__ == "__main__":
    main()
