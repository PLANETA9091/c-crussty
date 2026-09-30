#!/usr/bin/env python3
"""seed_gate.py — seed-gate canon x516, registry v2 (x518 AG-19 audit).

Asserts a seed before dispatch-POST:
  1) seed not in the SEED-LEDGER axis 240..535 (bank legs own that range);
  2) seed not in any bench-v2 registry (BENCH_SEEDS + KNOWN_SEEDS v2: all
     in-flight/historical legs x515..x518, see docs/SEED_REGISTRY_518.md);
  3) seed != another live leg in the same dispatch (per-LEG concurrency groups
     key on seed — two legs with same seed on same ref = sibling-cancel).
Usage:
  python3 seed_gate.py <seed> [<seed> ...]   -> exit 0 = clear, 1 = collision
  python3 seed_gate.py --suggest <wave> <n>  -> print n free seeds for the wave

Canon re-use exception: 351515 = BENCH-V2 canon sanity seed (reruns of the
canon stand itself are SUPPOSED to reuse it for comparability; cross-ref
same-seed A/B pairs like fp0-vs-fp6 are legal, same-ref siblings are not).
"""
import sys

LEDGER_AXIS = range(240, 536)          # SEED-LEDGER bank axis (AG-173/340)
BENCH_SEEDS = {351515: "BENCH-V2 canon sanity (AG-433/AG-12)", 351601: "AG-12 leg FP4"}

# v2 registry (AG-19 x518): every seed known in-flight or historical.
# Conservative: seen ranges are blocked whole. Sources: BLACKBOARD x517
# IN-FLIGHT, WAVE_MEMORY.md, claims x518, payloads AG-14/AG-6 x517,
# docs/SEED_REGISTRY_518.md.
KNOWN_RANGES = [
    (1902, 1938),   # bank legs s1902/s1903 + GATE-3 replicas/ib 1908..1938
    (2110, 2111), (2180, 2181), (2191, 2192), (2210, 2211),  # GATE-3/compo R3
    (1834, 1843),   # compo R1/R2 (1842/1843) + ib (1834/1835)
    (1652, 1653),   # leg-2 i64 CSR bases
    (1768, 1777),   # leg-2 i64 CSR patch seeds
    (1669, 1670),   # s1669 tail + s1670 FIRE leg (+25.49)
    (1803, 1812),   # s1803 FIRE (+23.05), s1811/s1812 tails
]
KNOWN_SEEDS = {
    16: "s16b bank x515", 73: "s73/s73b pair x515", 170: "s170b bank x515 (FIRE-cand)",
    2201: "STAND-517-PRESS alt leg", 2202: "STAND-517-PRESS alt leg",
    303030: "STZ-102 base/patch x517", 515045: "STZ-101 probe x517",
    271828: "STZ-103/104 probe x517", 517014: "STAND-517-PRESS press-pack (AG-14 spec)",
    351515: "CANON sanity (exception)", 351601: "AG-12 FP4 leg",
}
WAVE_FREE = {518: (518001, 518099), 519: (519001, 519099)}

# registered seeds whose planned reuse is LEGAL (canon / preregistered same
# spec, cross-ref only): gate prints PASS+reuse-note instead of blocking.
REUSE_OK = {
    351515: "canon sanity (solo leg or cross-ref A/B)",
    351601: "AG-12 FP4 canon leg",
    517014: "STAND-517-PRESS prereg press-vs-base pair (cross-ref, AG-14 spec)",
}


def _in_known(s: int) -> str:
    if s in KNOWN_SEEDS:
        return KNOWN_SEEDS[s]
    for a, b in KNOWN_RANGES:
        if a <= s <= b:
            return f"in-flight/historical range {a}..{b}"
    if s in LEDGER_AXIS:
        return "SEED-LEDGER bank axis 240..535"
    return ""


def check(seeds) -> int:
    if len(set(seeds)) != len(seeds):
        print(f"SEED-GATE FAIL: duplicate seeds in dispatch {seeds}")
        return 1
    reused = []
    for s in seeds:
        hit = _in_known(s)
        if hit and s not in BENCH_SEEDS:
            if s in REUSE_OK and len(seeds) == 1:
                reused.append(s)
                continue
            print(f"SEED-GATE FAIL: {s} registered: {hit}")
            return 1
        if s in BENCH_SEEDS and len(seeds) > 1:
            # canon seed allowed only for a solo canon-sanity leg
            print(f"SEED-GATE FAIL: {s} canon seed reused in multi-leg dispatch")
            return 1
    if reused:
        print(f"SEED-GATE PASS (registered reuse, cross-ref only): {seeds}")
    else:
        print(f"SEED-GATE PASS: {seeds} (unique, unregistered, canon rules ok)")
    return 0


def suggest(wave: int, n: int) -> int:
    lo, hi = WAVE_FREE.get(wave, (0, 0))
    if not lo:
        print(f"no free range for wave {wave}; known: {sorted(WAVE_FREE)}")
        return 1
    free = [s for s in range(lo, hi + 1) if not _in_known(s)][:n]
    print(f"FREE seeds wave-{wave}: {free}")
    return 0


def main() -> int:
    args = sys.argv[1:]
    if args and args[0] == "--suggest":
        return suggest(int(args[1]), int(args[2])) if len(args) >= 3 else 1
    return check([int(a) for a in args])


if __name__ == "__main__":
    sys.exit(main())
