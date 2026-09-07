#!/usr/bin/env python3
r"""Out-of-server property test for the area_map bridge contract.

Mirrors -- byte for byte -- the deterministic stream that the LIVE self-test
in `src/area_map.rs::bridge_selftest` feeds to the native bridge:

    seed  = 0x9E3779B97F4A7C15
    next: seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17  (u64 wrap)

    per rect, drawn in THIS order:
        from_x = (next() % 21) - 10
        from_z = (next() % 21) - 10
        to_x   = (next() % 21) - 10
        to_z   = (next() % 21) - 10
        old_d  =  next() % 7          # 0..6
        new_d  =  next() % 7          # 0..6

and computes the expected difference with the SAME naive reference the Rust
self-test compares against (`naive_set_difference`):

    old square = [from_x-old_d, from_x+old_d] x [from_z-old_d, from_z+old_d]
    new square = [to_x-new_d,   to_x+new_d]   x [to_z-new_d,   to_z+new_d]
    adds       = new \ old      (bridge op byte 0)
    removes    = old \ new      (bridge op byte != 0)

This script is pure math (no JVM, no Rust, no natives). It:
  1. regenerates the rect stream (default 10_000 rects, seeded/reproducible),
  2. runs the in-process PYTHON reference implementation of naive set
     difference,
  3. asserts the full invariant set the bridge must satisfy (see
     area_map_stress_spec.md, I1..I8) -- including the bridge's buffer cap
     2*(2*6+1)^2 = 338 and the op/key encoding round-trip,
  4. writes machine-consumable artifacts for a FUTURE OFFLINE HARNESS that
     will replay the same rects against the real native
     (`PaperNativeAreaMap.nativeUpdateOpsBatch`, JNI manifest line:
     `...|nativeUpdateOpsBatch|(IIIIII[B[J)I|...`):
        generated/area_map_cases.tsv        all rects + expected op counts
        generated/area_map_ops_first64.jsonl full expected op lists for the
                                             first 64 rects -- EXACTLY the
                                             rects the live in-server self-test
                                             replays (same seed, same order)
        generated/area_map_manifest.json    metadata / provenance

Usage:
    python3 tests/extended/area_map_stress.py
    python3 tests/extended/area_map_stress.py --rects 100000 --out DIR

Exit code 0 = all invariants hold; 1 = failure (prints the offending rect).
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from collections import Counter

# ---------------------------------------------------------------------------
# The PRNG: exact mirror of the Rust closure in bridge_selftest (xorshift64,
# Marsaglia's shift register; the code comments call it "fixed LCG").
# ---------------------------------------------------------------------------

MASK64 = (1 << 64) - 1
SELFTEST_SEED = 0x9E3779B97F4A7C15  # literal from src/area_map.rs


class SelftestLcg:
    """xorshift64 with u64 wrap-around semantics, identical to the Rust code:

        let mut seed: u64 = 0x9E3779B97F4A7C15;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

    `>>` on u64 is a LOGICAL shift (no sign extension) -- mirrored by the
    unsigned Python ints below. `<<` wraps modulo 2^64 (mirrored by MASK64).
    """

    __slots__ = ("state",)

    def __init__(self, seed: int = SELFTEST_SEED) -> None:
        self.state = seed & MASK64

    def next(self) -> int:
        s = self.state
        s ^= (s << 13) & MASK64
        s ^= s >> 7
        s ^= (s << 17) & MASK64
        self.state = s & MASK64  # already < 2**64; mask kept for clarity
        return self.state


# ---------------------------------------------------------------------------
# Bridge constants (src/area_map.rs / SingleUserAreaMapOps.java)
# ---------------------------------------------------------------------------

COORD_RANGE = 21          # next() % 21  -> coords in [-10, +10]
DIST_DOMAIN = 7           # next() % 7   -> d in [0, 6]
MAX_D = DIST_DOMAIN - 1   # 6
# Buffer cap used by bridge_selftest: max ops for d<=6 in both squares.
# (SingleUserAreaMapOps.maxOps computes oldSide^2 + newSide^2, which is
#  <= 2*(2*max_d+1)^2 -- equal only when old_d == new_d == max_d.)
SELFTEST_CAP = 2 * (2 * MAX_D + 1) * (2 * MAX_D + 1)  # 338

OP_ADD = 0                # ops[i] == 0    -> addCallback
OP_REMOVE = 1             # ops[i] != 0    -> removeCallback


# ---------------------------------------------------------------------------
# Reference implementation (pure math) -- mirrors naive_set_difference
# ---------------------------------------------------------------------------

def old_square(fx: int, fz: int, d: int) -> set[tuple[int, int]]:
    return {(x, z) for x in range(fx - d, fx + d + 1)
                   for z in range(fz - d, fz + d + 1)}


def naive_set_difference(
    from_x: int, from_z: int, old_d: int,
    to_x: int, to_z: int, new_d: int,
) -> tuple[set[tuple[int, int]], set[tuple[int, int]]]:
    """adds = new \\ old, removes = old \\ new (mirrors src/area_map.rs)."""
    o = old_square(from_x, from_z, old_d)
    n = old_square(to_x, to_z, new_d)
    return n - o, o - n


# ---------------------------------------------------------------------------
# Op/key encoding contract (SingleUserAreaMapOps.run + chunk_as_long)
# ---------------------------------------------------------------------------

def chunk_key(x: int, z: int) -> int:
    """Bridge long key: x in the low 32 bits, z in the high 32 bits.

    Java decode:  int x = (int) key;  int z = (int) (key >>> 32);
    """
    return ((z & 0xFFFFFFFF) << 32) | (x & 0xFFFFFFFF)


def key_x(key: int) -> int:
    low = key & 0xFFFFFFFF
    return low - (1 << 32) if low >= (1 << 31) else low


def key_z(key: int) -> int:
    return key_x(key >> 32)


def expected_op_stream(
    adds: set[tuple[int, int]], removes: set[tuple[int, int]]
) -> list[tuple[int, int, int]]:
    """Canonical (op, x, z) stream in the encoding the bridge must produce.

    The bridge is free to emit the ops in ANY order (the Java apply loop is
    order-agnostic); the harness therefore always compares decoded SETS.
    The canonical order here (adds then removes, cell-sorted) is only for
    deterministic artifacts.
    """
    stream = [(OP_ADD, x, z) for (x, z) in sorted(adds)]
    stream += [(OP_REMOVE, x, z) for (x, z) in sorted(removes)]
    return stream


def decode_stream(stream: list[tuple[int, int, int]]) -> tuple[set, set, bool]:
    """Mirror of the Java apply loop's decode: op==0 -> add else remove,
    x=(int)key, z=(int)(key>>>32); also flags duplicate cells."""
    adds, removes = set(), set()
    dup = False
    for op, x, z in stream:
        key = chunk_key(x, z)
        cell = (key_x(key), key_z(key))
        assert cell == (x, z), f"key round-trip broken for {(x, z)}"
        if op == OP_ADD:
            dup |= not (cell not in adds)
            adds.add(cell)
        else:
            dup |= not (cell not in removes)
            removes.add(cell)
    return adds, removes, dup


# ---------------------------------------------------------------------------
# Rect stream generator (draw order identical to bridge_selftest)
# ---------------------------------------------------------------------------

def gen_rects(rng: SelftestLcg, count: int):
    for _ in range(count):
        from_x = (rng.next() % COORD_RANGE) - 10
        from_z = (rng.next() % COORD_RANGE) - 10
        to_x = (rng.next() % COORD_RANGE) - 10
        to_z = (rng.next() % COORD_RANGE) - 10
        old_d = rng.next() % DIST_DOMAIN
        new_d = rng.next() % DIST_DOMAIN
        yield from_x, from_z, old_d, to_x, to_z, new_d


# ---------------------------------------------------------------------------
# Hand-verified spot cases (independent of the PRNG)
# ---------------------------------------------------------------------------

def spot_cases():
    """Small exact cases with hand-computed answers (pure-math oracle)."""
    return [
        # identical squares: nothing to do
        ((5, 5, 3, 5, 5, 3), set(), set()),
        # d=0 squares are single cells
        ((0, 0, 0, 0, 0, 0), set(), set()),
        # one-step move
        ((0, 0, 0, 1, 0, 0), {(1, 0)}, {(0, 0)}),
        # diagonal move, d=1: 5 adds, 5 removes
        ((0, 0, 1, 1, 1, 1),
         {(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)},
         {(-1, -1), (-1, 0), (-1, 1), (0, -1), (1, -1)}),
        # shrink in place: pure removes (ring of 25-9=16 cells)
        ((0, 0, 2, 0, 0, 1),
         set(),
         {(x, z) for x in range(-2, 3) for z in range(-2, 3)}
         - {(x, z) for x in range(-1, 2) for z in range(-1, 2)}),
        # grow: pure adds
        ((0, 0, 1, 0, 0, 2),
         {(x, z) for x in range(-2, 3) for z in range(-2, 3)}
         - {(x, z) for x in range(-1, 2) for z in range(-1, 2)},
         set()),
        # big jump d=6 -> d=6: 13x13 squares, 7x7 overlap -> 120+120
        ((-3, -3, 6, 3, 3, 6),
         None, None),  # counts only: adds=120, removes=120
    ]


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--rects", type=int, default=10_000)
    ap.add_argument("--seed", type=lambda v: int(v, 0), default=SELFTEST_SEED,
                    help="xorshift64 seed (default: bridge_selftest literal)")
    ap.add_argument("--out", default=os.path.join(os.path.dirname(
        os.path.abspath(__file__)), "generated"))
    ap.add_argument("--emit-ops", type=int, default=64,
                    help="how many leading rects get full expected op lists "
                         "in the JSONL sidecar (default 64 = the live "
                         "in-server self-test footprint)")
    args = ap.parse_args()

    failures: list[str] = []
    ok = lambda cond, msg: None if cond else failures.append(msg)

    # ---- 1. spot cases (pure-math oracle) --------------------------------
    for rect, exp_adds, exp_removes in spot_cases():
        adds, removes = naive_set_difference(*rect)
        if exp_adds is not None:
            ok(adds == exp_adds, f"spot {rect}: adds {sorted(adds)} != {sorted(exp_adds)}")
            ok(removes == exp_removes, f"spot {rect}: removes mismatch")
        else:  # counts-only case
            ok(len(adds) == 120 and len(removes) == 120,
               f"spot {rect}: expected 120+120, got {len(adds)}+{len(removes)}")
        # invariants hold on every spot case too
        _check_invariants(*rect, adds, removes, failures)

    # ---- 2. key-encoding spot checks --------------------------------------
    ok(chunk_key(-1, 2) == 0x00000002FFFFFFFF, "chunk_key(-1,2) packing")
    ok((key_x(chunk_key(1, -1)), key_z(chunk_key(1, -1))) == (1, -1),
        "key decode with negative z")
    ok((key_x(chunk_key(-2**31, 2**31 - 1)),
        key_z(chunk_key(-2**31, 2**31 - 1))) == (-2**31, 2**31 - 1),
        "key decode at i32 extremes")

    # ---- 3. the big stream ------------------------------------------------
    rng = SelftestLcg(args.seed)
    rows: list[tuple[int, ...]] = []
    hist = Counter()
    zero_ops = 0
    max_ops = 0
    total_adds = total_removes = 0

    for i, rect in enumerate(gen_rects(rng, args.rects)):
        fx, fz, od, tx, tz, nd = rect
        adds, removes = naive_set_difference(*rect)
        _check_invariants(*rect, adds, removes, failures, ctx=f"rect {i}")

        # op-stream encoding round-trip (bridge encoding contract)
        d_adds, d_removes, dup = decode_stream(expected_op_stream(adds, removes))
        ok(not dup, f"rect {i}: duplicate cell in op stream")
        ok(d_adds == adds and d_removes == removes,
           f"rect {i}: op stream decode mismatch")

        n = len(adds) + len(removes)
        ok(n <= SELFTEST_CAP,
           f"rect {i}: {n} ops exceed selftest cap {SELFTEST_CAP}")
        hist[min(n // 32, 10)] += 1
        zero_ops += n == 0
        max_ops = max(max_ops, n)
        total_adds += len(adds)
        total_removes += len(removes)
        rows.append((i, fx, fz, od, tx, tz, nd, len(adds), len(removes)))

        if i == 0:
            first_rect = rect
            first_adds, first_removes = adds, removes

    # ---- 4. write artifacts ----------------------------------------------
    os.makedirs(args.out, exist_ok=True)
    tsv_path = os.path.join(args.out, "area_map_cases.tsv")
    with open(tsv_path, "w", encoding="utf-8") as f:
        f.write("# c-crussty area_map offline-harness input\n")
        f.write(f"# prng=xorshift64 seed=0x{args.seed:016X} "
                "(src/area_map.rs bridge_selftest literal)\n")
        f.write("# draw order per rect: from_x, from_z, to_x, to_z, old_d, new_d\n")
        f.write("# domains: coords (next()%21)-10 in [-10,10]; d next()%7 in [0,6]\n")
        f.write("# op encoding: op==0 -> addCallback, else removeCallback; "
                "key = (z & 0xFFFFFFFF) << 32 | (x & 0xFFFFFFFF)\n")
        f.write("# expected ops: adds = new\\old (op 0), removes = old\\new (op 1); "
                "native may emit in any order; compare as sets\n")
        f.write("i\tfrom_x\tfrom_z\told_d\tto_x\tto_z\tnew_d\tn_adds\tn_removes\n")
        for r in rows:
            f.write("\t".join(str(v) for v in r) + "\n")

    jsonl_path = os.path.join(args.out, "area_map_ops_first64.jsonl")
    n_emit = min(args.emit_ops, len(rows))
    with open(jsonl_path, "w", encoding="utf-8") as f:
        for i, rect in enumerate(gen_rects(SelftestLcg(args.seed), n_emit)):
            adds, removes = naive_set_difference(*rect)
            f.write(json.dumps({
                "i": i,
                "from_x": rect[0], "from_z": rect[1], "old_d": rect[2],
                "to_x": rect[3], "to_z": rect[4], "new_d": rect[5],
                "adds": [list(c) for c in sorted(adds)],
                "removes": [list(c) for c in sorted(removes)],
            }, separators=(",", ":")) + "\n")

    manifest = {
        "tool": "tests/extended/area_map_stress.py",
        "prng": "xorshift64 (seed^=seed<<13; seed^=seed>>7; seed^=seed<<17, u64 wrap)",
        "seed_hex": f"0x{args.seed:016X}",
        "draw_order": ["from_x", "from_z", "to_x", "to_z", "old_d", "new_d"],
        "domains": {"coords": "[-10,10] via (next()%21)-10",
                    "distance": "[0,6] via next()%7"},
        "rects": args.rects,
        "selftest_cap": SELFTEST_CAP,
        "op_encoding": {"add": 0, "remove": "nonzero (1)",
                        "key": "x low 32 bits, z high 32 bits (chunk_as_long)"},
        "total_expected_adds": total_adds,
        "total_expected_removes": total_removes,
        "zero_op_rects": zero_ops,
        "max_ops_single_rect": max_ops,
        "histogram_bucket_32": dict(sorted(hist.items())),
        "first_64_note": "rects 0..63 of this stream are EXACTLY the rects "
                         "the live bridge_selftest replays in-server (same "
                         "seed, same draw order); see area_map_ops_first64.jsonl",
        "bridge_target": "ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap"
                         ".nativeUpdateOpsBatch(IIIIII[B[J)I",
    }
    manifest_path = os.path.join(args.out, "area_map_manifest.json")
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, sort_keys=True)
        f.write("\n")

    # ---- 5. report --------------------------------------------------------
    print("=== area_map_stress (out-of-server property test) ===")
    print(f"seed              : 0x{args.seed:016X} (xorshift64, mirror of "
          f"src/area_map.rs bridge_selftest)")
    print(f"rects             : {args.rects}")
    print(f"first rect        : {first_rect}")
    print(f"total expected    : {total_adds} adds + {total_removes} removes")
    print(f"zero-op rects     : {zero_ops}")
    print(f"max ops (1 rect)  : {max_ops}  (cap {SELFTEST_CAP})")
    print(f"op-count buckets  : {{'0-31': {hist[0]}, '32-63': {hist[1]}, "
          f"'64-95': {hist[2]}, '96-127': {hist[3]}, '128-159': {hist[4]}, "
          f"'160-191': {hist[5]}, '192-223': {hist[6]}, '224-255': {hist[7]}, "
          f"'256-287': {hist[8]}, '288-319': {hist[9]}, '320+': {hist[10]}}}")
    print(f"artifacts         : {tsv_path}")
    print(f"                    {jsonl_path} (first {n_emit} rects, full op lists)")
    print(f"                    {manifest_path}")
    if failures:
        print(f"FAIL ({len(failures)} invariant violations):")
        for m in failures[:20]:
            print(f"  - {m}")
        return 1
    print("PASS: all invariants hold "
          "(I1 counts, I2 disjoint, I3 symmetric-difference identity, "
          "I4 cap 2*(2d+1)^2, I5 no dups, I6 op/key round-trip, "
          "I7/I8 domain guards documented in spec)")
    return 0


def _check_invariants(fx, fz, od, tx, tz, nd, adds, removes, failures, ctx=""):
    """I1..I5 -- the set-algebra invariants, independent of how `adds` /
    `removes` were computed (recomputed here from scratch)."""
    o = old_square(fx, fz, od)
    n = old_square(tx, tz, nd)
    tag = f"{ctx} ({fx},{fz},d{od})->({tx},{tz},d{nd})"

    # I1: exactly the naive set difference
    if adds != n - o:
        failures.append(f"{tag}: I1 adds != new\\old")
    if removes != o - n:
        failures.append(f"{tag}: I1 removes != old\\new")
    # I2: disjoint
    if not adds.isdisjoint(removes):
        failures.append(f"{tag}: I2 adds ∩ removes != ∅")
    # I3: union == symmetric difference == (old ∪ new) \ (old ∩ new)
    sym = n ^ o
    if (adds | removes) != sym:
        failures.append(f"{tag}: I3 adds ∪ removes != symmetric difference")
    if (adds | removes) != (o | n) - (o & n):
        failures.append(f"{tag}: I3 union != (old ∪ new) \\ (old ∩ new)")
    # I4: count bound oldSide^2 + newSide^2, and the selftest cap 2*(2*6+1)^2
    if len(adds) + len(removes) > (2 * od + 1) ** 2 + (2 * nd + 1) ** 2:
        failures.append(f"{tag}: I4 count > oldSide^2+newSide^2")
    if len(adds) + len(removes) > SELFTEST_CAP:
        failures.append(f"{tag}: I4 count > 2*(2*6+1)^2 = {SELFTEST_CAP}")
    # I5: no duplicates possible (sets) -- vacuous here; enforced on streams
    # by decode_stream() in the caller.


if __name__ == "__main__":
    sys.exit(main())
