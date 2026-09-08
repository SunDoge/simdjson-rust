#!/usr/bin/env python3
"""Time perf_sample stages serially in shuffled order; save individual runs."""
import argparse
import json
import random
import re
import statistics
import subprocess
from pathlib import Path
from typing import TypedDict


class ProfileArgs(argparse.Namespace):
    binary: str
    output: Path
    rounds: int
    seconds: float
    sizes: list[int]


class TimingRow(TypedDict):
    users: int
    bytes: int
    mode: str
    median_ns: float
    min_ns: float
    max_ns: float
    samples_ns: list[float]
    iterations: int


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', default='target/release/examples/perf_sample')
    parser.add_argument('--output', type=Path, default=Path('/tmp/simdjson-profile-timings.json'))
    parser.add_argument('--rounds', type=int, default=7)
    parser.add_argument('--seconds', type=float, default=0.15)
    parser.add_argument('--sizes', type=int, nargs='+', default=[1, 10, 500])
    args = parser.parse_args(namespace=ProfileArgs())
    if args.rounds < 1 or args.seconds <= 0 or any(n < 1 for n in args.sizes):
        parser.error('rounds, seconds and sizes must be positive')
    modes = ['reuse', 'fresh', 'padded', 'parse', 'ffi', 'view', 'serde', 'serde-json']
    cases = [(users, mode) for users in args.sizes for mode in modes]

    def run(users: int, mode: str, iters: int) -> tuple[float, int]:
        output = subprocess.check_output(
            [args.binary, str(iters), mode, str(users)], text=True
        )
        timing = re.search(r'ns_per_iter=([\d.]+)', output)
        size = re.search(r'bytes=(\d+)', output)
        if timing is None or size is None:
            raise ValueError(f'unexpected profiler output: {output!r}')
        return float(timing[1]), int(size[1])

    iterations: dict[tuple[int, str], int] = {}
    sizes: dict[int, int] = {}
    results: dict[tuple[int, str], list[float]] = {case: [] for case in cases}
    for case in cases:
        ns, size = run(*case, 1000)
        iterations[case] = max(100, int(args.seconds * 1e9 / ns))
        sizes[case[0]] = size
    rng = random.Random(42)
    for repeat in range(args.rounds):
        rng.shuffle(cases)
        for case in cases:
            ns, _ = run(*case, iterations[case])
            results[case].append(ns)
        print(f'round {repeat + 1} done', flush=True)
    rows: list[TimingRow] = []
    for users in args.sizes:
        for mode in modes:
            values = results[(users, mode)]
            row = TimingRow(users=users, bytes=sizes[users], mode=mode,
                       median_ns=statistics.median(values), min_ns=min(values),
                       max_ns=max(values), samples_ns=values,
                       iterations=iterations[(users, mode)])
            rows.append(row)
            print(f'{users:4} {mode:10} median={row["median_ns"]:.1f} ns '
                  f'range=[{min(values):.1f}, {max(values):.1f}]')
    args.output.write_text(json.dumps(rows, indent=2) + '\n')


if __name__ == '__main__':
    main()
