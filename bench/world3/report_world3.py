#!/usr/bin/env python3
"""Benchmark 3.0 bottleneck report generator.

Parses the async-profiler collapsed stacks (self-time = frame counts) into
bucketed rankings that directly answer "what do we replace with Rust next",
plus server-log stats (boot time, forceload count, TPS polls, tick warnings).
Output: $WORK/BOTTLENECKS_3.md — the artifact the research rounds consume.
"""
import collections
import glob
import os
import re
import sys

work = sys.argv[1] if len(sys.argv) > 1 else "."
natives_mode = sys.argv[2] if len(sys.argv) > 2 else "unknown"
seen_done = sys.argv[3] if len(sys.argv) > 3 else "0"

# frame-prefix -> research bucket
BUCKETS = [
    ("net/minecraft/world/level/levelgen", "worldgen/noise (kernel)"),
    ("net/minecraft/world/entity", "entities/mobs (kernel)"),
    ("net/minecraft/world/level/chunk", "chunk system (kernel)"),
    ("net/minecraft/server/level/ServerChunkCache", "chunk system (kernel)"),
    ("net/minecraft/world/level/block/entity", "block entities/hoppers (kernel)"),
    ("net/minecraft/world/level/redstone", "redstone (kernel)"),
    ("net/minecraft/world/ticks", "tick scheduling (kernel)"),
    ("net/minecraft/network", "network (kernel)"),
    ("net/minecraft", "kernel: other"),
    ("org/bukkit/craftbukkit", "craftbukkit glue"),
    ("org/bukkit", "bukkit api"),
    ("libcrussty.so", "c-crussty module (Rust)"),
    ("libcrussty_runtime.so", "CRUSSTY engine runtime (Rust)"),
    ("libpaper_native", "Crussty CE natives (JNI)"),
    ("libjvm.so", "JVM internals (GC/JIT)"),
    ("java/util", "JDK collections"),
    ("java/", "JDK other"),
    ("jdk/", "JDK other"),
]


def bucket_of(frame: str) -> str:
    for prefix, name in BUCKETS:
        if frame.startswith(prefix):
            return name
    return "other"


collapsed = os.path.join(work, "cpu-collapsed.txt")
total = 0
bucket_self = collections.Counter()
top_self = collections.Counter()
leaf_samples = collections.Counter()

if os.path.exists(collapsed):
    with open(collapsed, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line or " " not in line:
                continue
            stack, _, count = line.rpartition(" ")
            try:
                n = int(count)
            except ValueError:
                continue
            total += n
            frames = stack.split(";")
            if frames:
                leaf = frames[-1]
                top_self[leaf] += n
                leaf_samples[leaf.split("(")[0]] += n
                for fr in frames:
                    bucket_self[bucket_of(fr)] += 0  # presence marker
                # self time attributed to the leaf's bucket
                bucket_self[bucket_of(frames[-1])] += n

log_path = os.path.join(work, "server-stdout.log")
boot_s = tps_polls = forceloads = warns = 0
chunks_marked = 0
tps_values = []
mspt_max = mspt_min = mspt_avg = 0.0
spark_links = []
if os.path.exists(log_path):
    with open(log_path, encoding="utf-8", errors="replace") as f:
        for line in f:
            if "Done (" in line:
                m = re.search(r"Done \(([\d.]+)s\)", line)
                if m:
                    boot_s = float(m.group(1))
            # run#5 lesson: console output is "Marked 256 chunks in minecraft:overworld
            # from [x, z] to [x, z] to be force loaded" — the old line-start prefix
            # never matched the echoed command
            m = re.search(r"Marked (\d+) chunks .* to be force loaded", line)
            if m:
                forceloads += 1
                chunks_marked += int(m.group(1))
            if "Can't keep up" in line or "Running .*ms behind" in line:
                warns += 1
            # run#5 lesson: paper 1.21.10 prints "TPS from last 5s, 1m, 5m, 15m: ..."
            m = re.search(r"TPS from last[^:]*:\s*([\d.]+)", line)
            if m:
                tps_values.append(float(m.group(1)))
                tps_polls += 1
            # spark tick-monitor analysis lines — REAL MSPT evidence
            m = re.search(r">\s*Max:\s*([\d.]+)ms", line)
            if m:
                mspt_max = max(mspt_max, float(m.group(1)))
            m = re.search(r">\s*Min:\s*([\d.]+)ms", line)
            if m:
                mspt_min = float(m.group(1)) if not mspt_min else min(mspt_min, float(m.group(1)))
            m = re.search(r">\s*Average:\s*([\d.]+)ms", line)
            if m:
                mspt_avg = max(mspt_avg, float(m.group(1)))
            m = re.search(r"https://spark\.lucko\.me/(\w+)", line)
            if m and m.group(1) not in spark_links:
                spark_links.append(m.group(1))

lines = []
lines.append("# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players)")
lines.append("")
lines.append(f"- natives mode: **{natives_mode}**")
lines.append(f"- boot reached Done: **{seen_done}** (boot time {boot_s or 'n/a'} s)")
lines.append(f"- forceload commands issued: {forceloads} ({chunks_marked} chunks force-loaded)")
lines.append(f"- TPS polls captured: {tps_polls}" + (f", first-of-window values: {tps_values}" if tps_values else ""))
if mspt_avg or mspt_max:
    lines.append(f"- spark tick-monitor MSPT: avg **{mspt_avg}ms** / min {mspt_min}ms / max **{mspt_max}ms** (>50ms = TPS<20)")
for code in spark_links:
    lines.append(f"- spark viewer report: https://spark.lucko.me/{code}")
lines.append(f"- tick-behind warnings in log: {warns}")
lines.append(f"- CPU samples total: {total}")
if not total and seen_done == "1":
    lines.append("")
    lines.append("> NOTE: boot reached Done but no CPU collapsed stacks were produced —")
    lines.append("> profiler attach failed; report ranks are INVALID for this run (log stats only).")
lines.append("")
lines.append("## Self-time by research bucket (what to replace with Rust next)")
lines.append("")
lines.append("| bucket | self-time samples | share |")
lines.append("|---|---|---|")
for name, n in bucket_self.most_common(20):
    share = (100.0 * n / total) if total else 0.0
    lines.append(f"| {name} | {n} | {share:.1f}% |")
lines.append("")
lines.append("## Top-40 leaf frames by self-time")
lines.append("")
lines.append("| leaf frame | samples | share |")
lines.append("|---|---|---|")
for frame, n in top_self.most_common(40):
    share = (100.0 * n / total) if total else 0.0
    lines.append(f"| `{frame}` | {n} | {share:.1f}% |")
lines.append("")
lines.append("## Artifacts in this run")
lines.append("")
for pat in ("cpu-collapsed.txt", "cpu-flamegraph.html", "server-stdout.log",
            "ap.log", "spark-report*"):
    for p in sorted(glob.glob(os.path.join(work, pat))):
        lines.append(f"- `{os.path.basename(p)}` ({os.path.getsize(p)} B)")
lines.append("")
lines.append("NEXT: research rounds attack the top kernel buckets in order —")
lines.append("each round = one pre-registered c-crussty task with a Rust replacement")
lines.append("or a native bridge, gated by the module's parity discipline.")

out = os.path.join(work, "BOTTLENECKS_3.md")
with open(out, "w", encoding="utf-8") as f:
    f.write("\n".join(lines) + "\n")
print(f"[world3-report] wrote {out} (total samples {total})")
