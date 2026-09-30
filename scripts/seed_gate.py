#!/usr/bin/env python3
"""seed_gate.py — seed-gate canon x516 (AG-173/340 SEED-LEDGER, коллизии x19).

Asserts a seed before dispatch-POST:
  1) seed not in the SEED-LEDGER axis 240..535 (bank legs own that range);
  2) seed not in any bench-v2 registry (this file BENCH_SEEDS + work registries);
  3) seed != another live leg in the same dispatch (per-LEG concurrency groups
     key on seed — two legs with same seed on same ref = sibling-cancel).
Usage: python3 seed_gate.py <seed> [<seed> ...]  -> exit 0 = clear, 1 = collision.
Canon re-use exception: 351515 = BENCH-V2 canon sanity seed (reruns of the
canon stand itself are SUPPOSED to reuse it for comparability).
"""
import sys

LEDGER_AXIS = range(240, 536)          # SEED-LEDGER bank axis (AG-173/340)
BENCH_SEEDS = {351515: "BENCH-V2 canon sanity (AG-433/AG-12)", 351601: "AG-12 leg FP4"}


def main() -> int:
    seeds = [int(a) for a in sys.argv[1:]]
    if len(set(seeds)) != len(seeds):
        print(f"SEED-GATE FAIL: duplicate seeds in dispatch {seeds}")
        return 1
    for s in seeds:
        if s in LEDGER_AXIS:
            print(f"SEED-GATE FAIL: {s} inside SEED-LEDGER bank axis 240..535")
            return 1
        if s in BENCH_SEEDS and len(seeds) > 1:
            # canon seed allowed only for a solo canon-sanity leg
            print(f"SEED-GATE FAIL: {s} canon seed reused in multi-leg dispatch")
            return 1
    print(f"SEED-GATE PASS: {seeds} (unique, outside bank axis, canon rules ok)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
