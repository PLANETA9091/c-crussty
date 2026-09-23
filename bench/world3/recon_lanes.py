#!/usr/bin/env python3
"""recon_lanes.py — fresh-recon lane tool for bench-4 era (S7-99 prep, task170).

After the scenario switch (fake players), every lane share changes. This tool
consumes one or more run dirs (each holding cpu-collapsed.txt / wall-collapsed.txt /
alloc-collapsed.txt / run-env.txt as produced by the world-bench workflow) and:

  1. prints the MSPT-relevant lane table per run (same buckets as
     report_world3.py, % of CPU samples),
  2. prints per-lane top leaves (the STEP-0 anatomy input: replaceable-core
     sizing starts here),
  3. with --diff A B: prints the lane-share delta between two runs, together
     with the pairing keys (world_sha256, runner_cpu_index) from run-env.txt —
     the runner-variance law (S7-96d) requires pairing disclosure before any
     cross-run comparison is trusted.

Self-contained on purpose (does not import report_world3.py — that script has
no __main__ guard and runs its report on import; zero risk to CI infra).

Usage:
  python3 recon_lanes.py RUN_DIR [RUN_DIR ...]
  python3 recon_lanes.py --diff RUN_A RUN_B [--top N]
"""
import os
import re
import sys

BUCKETS = [
    ("OopOopIterateDispatch", "JVM internals (GC oop barriers)"),
    ("G1", "JVM internals (G1 GC)"),
    ("longest_match", "JVM internals (GC)"),
    (" stub", "JIT stubs (vtable/itable)"),
    ("[vdso]", "vdso (clock)"),
    ("[kernel]", "kernel syscalls"),
    ("net/minecraft/world/level/levelgen", "worldgen/noise (kernel)"),
    ("NaturalSpawner", "spawn lane (kernel)"),
    ("checkDespawn", "despawn lane (kernel)"),
    ("net/minecraft/world/entity", "entities/mobs (kernel)"),
    ("net/minecraft/world/level/chunk", "chunk system (kernel)"),
    ("net/minecraft/server/level/ServerChunkCache", "chunk system (kernel)"),
    ("net/minecraft/world/level/block/entity", "block entities/hoppers (kernel)"),
    ("net/minecraft/world/level/redstone", "redstone (kernel)"),
    ("net/minecraft/world/ticks", "tick scheduling (kernel)"),
    ("net/minecraft/server/players/PlayerList", "player lane (kernel)"),
    ("net/minecraft/server/network/ServerGamePacketListenerImpl", "player lane (kernel)"),
    ("net/minecraft/server/level/ServerPlayer", "player lane (kernel)"),
    ("net/minecraft/network", "network (kernel)"),
    ("net/minecraft", "kernel: other"),
    ("ca/spottedleaf/moonrise", "moonrise/paper patches"),
    ("org/bukkit/craftbukkit", "craftbukkit glue"),
    ("org/bukkit", "bukkit api"),
    ("bench/fakeplayers", "BenchFakePlayers fixture (must be ~0)"),
    ("libcrussty.so", "c-crussty module (Rust)"),
    ("libcrussty_runtime.so", "CRUSSTY engine runtime (Rust)"),
    ("libpaper_native", "Crussty CE natives (JNI)"),
    ("libjvm.so", "JVM internals (GC/JIT)"),
    ("java/util", "JDK collections"),
    ("it/unimi/dsi/fastutil", "fastutil collections"),
    ("java/lang/invoke", "JDK invokes/VarHandle"),
    ("java/", "JDK other"),
    ("jdk/", "JDK other"),
]

# natural-spawn lane markers (bench-4 profile is EXPECTED to open these)
SPAWN_LANES = [
    ("tickSpawning", "spawn: tickSpawning"),
    ("NaturalSpawner", "spawn: NaturalSpawner"),
    ("spawnForChunk", "spawn: spawnForChunk"),
    ("getFilteredSpawningCategories", "spawn: category filter"),
    ("createState", "spawn: createState"),
    ("checkDespawn", "despawn: checkDespawn"),
    ("findNearbyPlayer", "despawn: findNearbyPlayer"),
    ("canSpawn", "spawn: cap check"),
]


def bucket_of(frame):
    for prefix, name in BUCKETS:
        if prefix in frame:
            return name
    return "other"


def parse_collapsed(path):
    """frame-string -> sample count (async-profiler collapsed format)."""
    per_frame = {}
    total = 0
    if not os.path.isfile(path):
        return per_frame, 0
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line or ";" not in line:
                continue
            key, _, cnt = line.rpartition(" ")
            try:
                n = int(cnt)
            except ValueError:
                key, n = line, 1
            frames = key.split(";")
            leaf = frames[-1]
            per_frame[leaf] = per_frame.get(leaf, 0) + n
            total += n
    return per_frame, total


def lane_table(per_frame, total, top=14):
    lane_self = {}
    lane_frames = {}
    for leaf, n in per_frame.items():
        b = bucket_of(leaf)
        lane_self[b] = lane_self.get(b, 0) + n
        lane_frames.setdefault(b, []).append((leaf, n))
    rows = sorted(lane_self.items(), key=lambda x: -x[1])
    out = []
    for b, n in rows[:top]:
        out.append((b, n, 100.0 * n / total if total else 0.0, lane_frames[b]))
    rest = total - sum(n for _, n, _, _ in out)
    return out, rest


def spawn_series(per_frame, total):
    """Natural-spawn/despawn lane presence — the NEW profile's signature."""
    acc = {}
    for marker, label in SPAWN_LANES:
        n = sum(cnt for leaf, cnt in per_frame.items() if marker in leaf)
        if n:
            acc[label] = (n, 100.0 * n / total if total else 0.0)
    return acc


def run_env(path):
    env = {}
    if os.path.isfile(path):
        with open(path, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = re.match(r"([a-z_0-9]+):\s*(.*)", line.strip())
                if m:
                    env[m.group(1)] = m.group(2)
    return env


def report_run(d):
    per, total = parse_collapsed(os.path.join(d, "cpu-collapsed.txt"))
    env = run_env(os.path.join(d, "run-env.txt"))
    print(f"\n=== {os.path.basename(os.path.normpath(d))} "
          f"(cpu={total} samples; fake_players={env.get('fake_players','?').split()[0] if env.get('fake_players') else '?'}; "
          f"cpu_idx={env.get('runner_cpu_index','?').split()[0] if env.get('runner_cpu_index') else '?'}; world={env.get('world_sha256','?')[:12]}) ===")
    if not total:
        print("  (no cpu-collapsed.txt — run without profiles or failed attach)")
        return None
    rows, rest = lane_table(per, total)
    print("| lane (self-time) | samples | share |")
    print("|---|---|---|")
    for b, n, pct, _ in rows:
        print(f"| {b} | {n} | {pct:.2f}% |")
    print(f"| (other/unbucketed tail) | {rest} | {100.0*rest/total:.2f}% |")
    sp = spawn_series(per, total)
    if sp:
        print("\n  spawn/despawn lane (bench-4 signature):")
        for label, (n, pct) in sorted(sp.items(), key=lambda x: -x[1][1]):
            print(f"    - {label}: {n} samples = {pct:.2f}%")
    print("\n  top leaves per lane (STEP-0 input):")
    for b, n, pct, frames in rows[:8]:
        print(f"    [{b}] ({pct:.2f}%)")
        for leaf, ln in sorted(frames, key=lambda x: -x[1])[:5]:
            print(f"        {ln:7d}  {leaf[:140]}")
    return {"total": total, "lanes": {b: n for b, n, _, _ in rows}}


def diff(a, b):
    ra = report_run(a)
    rb = report_run(b)
    if not ra or not rb:
        print("\n(diff needs both runs to have cpu-collapsed.txt)")
        return
    enva, envb = run_env(os.path.join(a, "run-env.txt")), run_env(os.path.join(b, "run-env.txt"))
    print(f"\n=== PAIRING DISCLOSURE (runner-variance law S7-96d) ===")
    wa, wb = enva.get('world_sha256', '?'), envb.get('world_sha256', '?')
    ca = enva.get('runner_cpu_index', '?').split()[0]
    cb = envb.get('runner_cpu_index', '?').split()[0]
    fa = enva.get('fake_players', '?').split()[0]
    fb = envb.get('fake_players', '?').split()[0]
    print(f"  world_sha256:  {wa[:16]} vs {wb[:16]} "
          f"{'(same snapshot)' if wa == wb else '(DIFFERENT snapshot — cross-run deltas confounded by live-world drift)'}")
    print(f"  runner_cpu:    {ca} vs {cb}")
    print(f"  fake_players:  {fa} vs {fb}")
    print("\n=== LANE DELTA (share % of B minus A; scenario deltas NOT module wins) ===")
    keys = set(ra["lanes"]) | set(rb["lanes"])
    ta, tb = ra["total"], rb["total"]
    for k in sorted(keys, key=lambda k: -(100.0*rb["lanes"].get(k,0)/tb - 100.0*ra["lanes"].get(k,0)/ta)):
        pa, pb = 100.0*ra["lanes"].get(k,0)/ta, 100.0*rb["lanes"].get(k,0)/tb
        print(f"  {k:44s} {pa:6.2f}% -> {pb:6.2f}%  ({pb-pa:+.2f}pp)")


def main():
    argv = sys.argv[1:]
    diffmode = False
    if "--diff" in argv:
        diffmode = True
        argv.remove("--diff")
    if diffmode and len(argv) >= 2:
        diff(argv[0], argv[1])
    else:
        for d in argv:
            report_run(d)


if __name__ == "__main__":
    main()
