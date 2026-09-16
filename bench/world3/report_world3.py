#!/usr/bin/env python3
"""Benchmark 3.0 bottleneck report generator — v2.

Parses the async-profiler collapsed stacks (self-time = frame counts) into
bucketed rankings that directly answer "what do we replace with Rust next",
plus server-log stats (boot time, forceload count, TPS polls, tick warnings).

v2 additions (run#10 gap analysis, 2026-09-17):
  - GC stats from gc.log (pause counts/total/avg/max, heap ranges, Full GC flag)
  - tick-phase split via stack ancestry (leaf-first marker scan)
  - JVM-vs-native split (heuristic classifier + per-top-leaf tag)
  - wall/alloc collapsed profiles (top-20 each) when present
  - MSPT percentile windows from `paper mspt` console output
  - entity totals from `paper entity list` console output

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
    # run#10 lesson: HotSpot C++ self-time frames arrive WITHOUT the libjvm.so
    # module prefix — G1 barriers/stubs/vdso were drowning 39% of self-time in
    # "other". Recognize them by frame CONTENT (checked before the prefixes).
    ("OopOopIterateDispatch", "JVM internals (GC oop barriers)"),
    ("G1", "JVM internals (G1 GC)"),
    ("longest_match", "JVM internals (GC)"),
    (" stub", "JIT stubs (vtable/itable)"),
    ("[vdso]", "vdso (clock)"),
    ("[kernel]", "kernel syscalls"),
    ("net/minecraft/world/level/levelgen", "worldgen/noise (kernel)"),
    ("net/minecraft/world/entity", "entities/mobs (kernel)"),
    ("net/minecraft/world/level/chunk", "chunk system (kernel)"),
    ("net/minecraft/server/level/ServerChunkCache", "chunk system (kernel)"),
    ("net/minecraft/world/level/block/entity", "block entities/hoppers (kernel)"),
    ("net/minecraft/world/level/redstone", "redstone (kernel)"),
    ("net/minecraft/world/ticks", "tick scheduling (kernel)"),
    ("net/minecraft/network", "network (kernel)"),
    ("net/minecraft", "kernel: other"),
    ("ca/spottedleaf/moonrise", "moonrise/paper patches"),
    ("org/bukkit/craftbukkit", "craftbukkit glue"),
    ("org/bukkit", "bukkit api"),
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

# leaf-first tick-phase markers: (substring in ANY frame, phase label).
# Scanned leaf->root; first match wins (leaf-specific beats generic).
PHASES = [
    ("tickBlockEntities", "phase: block entities (hoppers/furnaces)"),
    ("optimiseRandomTick", "phase: random tick"),
    ("randomTick", "phase: random tick"),
    ("tickSpawning", "phase: mob spawning"),
    ("tickNonPassenger", "phase: entity tick (AI/movement)"),
    ("entityTick", "phase: entity tick (AI/movement)"),
    ("tickPassenger", "phase: entity tick (AI/movement)"),
    ("tickEntities", "phase: entities (manager)"),
    ("tickChunks", "phase: chunk tick"),
    ("ServerChunkCache.tick", "phase: chunk tick"),
    ("tickServerConnection", "phase: network"),
    ("sendChanges", "phase: network sync (ServerEntity)"),
    ("tickChildren", "phase: main tick (unclassified)"),
    ("runAllTasks", "phase: scheduler/mid-tick tasks"),
    ("doRunTask", "phase: scheduler/mid-tick tasks"),
    ("ChunkHolder", "phase: chunk system (off-main worker)"),
    ("ChunkTaskScheduler", "phase: chunk system (off-main worker)"),
    ("chunkSystem", "phase: chunk system (off-main worker)"),
    ("RenderThread", "phase: n/a (non-server thread)"),
]

NATIVE_EXTRA = {
    "read", "write", "longest_match", "fsync", "pread64", "pwrite64",
    "pthread_cond_wait", "pthread_mutex_lock", "epoll_wait", "poll",
    "__memcpy_avx_unaligned_erms", "memset",
}


def bucket_of(frame: str) -> str:
    for prefix, name in BUCKETS:
        if prefix in frame:  # content match (JVM C++ frames carry no path)
            return name
    return "other"


def phase_of(frames) -> str:
    """Leaf-first: the most specific phase marker wins."""
    for fr in reversed(frames):
        for marker, label in PHASES:
            if marker in fr:
                return label
    return "phase: unclassified"


def kind_of_leaf(frame: str) -> str:
    """JVM-Java vs native/JVM-internal leaf classification (heuristic)."""
    if frame.startswith("[") or frame.endswith(".so") or ".so!" in frame:
        return "native/JVM-internal"
    if " stub" in frame or frame.endswith("stub"):
        return "native/JVM-internal"
    if "::" in frame:  # HotSpot C++ symbols (OopOopIterateDispatch<..>::..)
        return "native/JVM-internal"
    if frame in NATIVE_EXTRA:
        return "native/JVM-internal"
    if "/" in frame:  # dotted Java packages use '/' in collapsed output
        return "JVM-Java"
    if re.match(r"^[a-z_][a-z0-9_]*$", frame):  # bare libc symbol e.g. `read`
        return "native/JVM-internal"
    return "other"


def parse_collapsed(path, limit_top=40):
    total = 0
    top_self = collections.Counter()
    leaf_samples = collections.Counter()
    bucket_self = collections.Counter()
    phase_self = collections.Counter()
    kind_self = collections.Counter()
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8", errors="replace") as f:
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
            if not frames:
                continue
            leaf = frames[-1]
            top_self[leaf] += n
            leaf_samples[leaf.split("(")[0]] += n
            bucket_self[bucket_of(leaf)] += n
            phase_self[phase_of(frames)] += n
            kind_self[kind_of_leaf(leaf)] += n
    return {
        "total": total,
        "top_self": top_self,
        "leaf_samples": leaf_samples,
        "bucket_self": bucket_self,
        "phase_self": phase_self,
        "kind_self": kind_self,
    }


def parse_gc(path):
    """-Xlog:gc* parse: pause events, heap ranges, Full-GC flag."""
    if not os.path.exists(path):
        return None
    pauses = collections.Counter()
    total_ms = 0.0
    max_ms = 0.0
    heap_before = heap_after = 0.0
    full_gcs = 0
    gc_events = 0
    up_first = up_last = None
    pat = re.compile(
        r"GC\(\d+\)\s+Pause\s+([A-Za-z0-9 ()]+?)\s+"
        r"([\d.]+)([MG])\(?[\d.]*%?\)?->([\d.]+)([MG])\(?[\d.]*[MG%]?\)?\s+([\d.]+)ms")
    up_pat = re.compile(r"\[uptime\]([^\]]*)\]")
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = pat.search(line)
            if m:
                gc_events += 1
                name = m.group(1).strip()
                pauses[name] += 1
                dur = float(m.group(6))
                total_ms += dur
                max_ms = max(max_ms, dur)
                b = float(m.group(2)) * (1024 if m.group(3) == "G" else 1)
                a = float(m.group(4)) * (1024 if m.group(5) == "G" else 1)
                heap_before = max(heap_before, b)
                heap_after = a if a else heap_after
                if "Full" in name:
                    full_gcs += 1
            mu = up_pat.search(line)
            if mu:
                if up_first is None:
                    up_first = mu.group(1).strip()
                up_last = mu.group(1).strip()
    return {
        "events": gc_events, "pauses": pauses, "total_ms": total_ms,
        "max_ms": max_ms, "heap_before_mb": heap_before,
        "heap_after_mb": heap_after, "full": full_gcs,
    }


def parse_mspt_windows(path):
    """`paper mspt` console output: window headers + percentile lines."""
    windows = []
    cur = None
    hdr = re.compile(r"(Last\s+\d+[smh]|All\s+time|Lifetime)", re.I)
    pct = re.compile(r"(Min|Median|P95|P99|Max|Avg|Average)\D{0,6}(\d+\.?\d*)", re.I)
    if not os.path.exists(path):
        return windows
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            if hdr.search(line) and "ms" not in line and "TPS" not in line:
                cur = {"window": line.strip(" -:"), "vals": {}}
                windows.append(cur)
                continue
            if cur is None:
                continue
            for m in pct.finditer(line):
                key = m.group(1).capitalize()
                if key == "Average":
                    key = "Avg"
                cur["vals"].setdefault(key, float(m.group(2)))
    return [w for w in windows if w["vals"]]


def parse_entity_totals(path):
    """`paper entity list`: 'Total entities: N' style lines + top types."""
    totals = []
    types = collections.Counter()
    if not os.path.exists(path):
        return totals, types
    tpat = re.compile(r"Total (?:ticking |loaded |spawnable )?entities[^:]*:\s*(\d+)", re.I)
    upat = re.compile(r"([a-z_]+:[a-z0-9_/]+)\s+(\d+)\s+\[")
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = tpat.search(line)
            if m:
                totals.append(int(m.group(1)))
            for m in upat.finditer(line):
                types[m.group(1)] = max(types[m.group(1)], int(m.group(2)))
    return totals, types


# --- parse everything -------------------------------------------------------
cpu = parse_collapsed(os.path.join(work, "cpu-collapsed.txt"))
wall = parse_collapsed(os.path.join(work, "wall-collapsed.txt"))
alloc = parse_collapsed(os.path.join(work, "alloc-collapsed.txt"))
gc = parse_gc(os.path.join(work, "gc.log"))
mspt_windows = parse_mspt_windows(os.path.join(work, "server-stdout.log"))
ent_totals, ent_types = parse_entity_totals(os.path.join(work, "server-stdout.log"))

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

# --- report -------------------------------------------------------------------
lines = []
lines.append("# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2")
lines.append("")
lines.append(f"- natives mode: **{natives_mode}**")
lines.append(f"- boot reached Done: **{seen_done}** (boot time {boot_s or 'n/a'} s)")
lines.append(f"- forceload commands issued: {forceloads} ({chunks_marked} chunks force-loaded)")
lines.append(f"- TPS polls captured: {tps_polls}" + (f", first-of-window values: {tps_values}" if tps_values else ""))
if mspt_avg or mspt_max:
    lines.append(f"- spark tick-monitor MSPT: avg **{mspt_avg}ms** / min {mspt_min}ms / max **{mspt_max}ms** (>50ms = TPS<20)")
if mspt_windows:
    lines.append("")
    lines.append("### MSPT percentile windows (`paper mspt`)")
    lines.append("")
    lines.append("| window | min | median | p95 | p99 | max | avg |")
    lines.append("|---|---|---|---|---|---|---|")
    for w in mspt_windows[-6:]:
        v = w["vals"]
        lines.append("| {window} | {min} | {median} | {p95} | {p99} | {max} | {avg} |".format(
            window=w["window"][:40],
            min=v.get("Min", "—"), median=v.get("Median", "—"), p95=v.get("P95", "—"),
            p99=v.get("P99", "—"), max=v.get("Max", "—"), avg=v.get("Avg", "—")))
if ent_totals or ent_types:
    lines.append("")
    lines.append(f"- entity totals seen: {ent_totals[-3:] if ent_totals else '—'}")
    top_types = ", ".join(f"{t}×{n}" for t, n in ent_types.most_common(12))
    if top_types:
        lines.append(f"- top entity types (max seen): {top_types}")
for code in spark_links:
    lines.append(f"- spark viewer report: https://spark.lucko.me/{code}")
lines.append(f"- tick-behind warnings in log: {warns}")

if gc and gc["events"]:
    lines.append("")
    lines.append("### GC (from gc.log)")
    lines.append("")
    lines.append(f"- pause events: **{gc['events']}** (Full GC: **{gc['full']}**)")
    lines.append(f"- total pause: **{gc['total_ms']:.1f} ms**, avg **{gc['total_ms']/max(gc['events'],1):.2f} ms**, max **{gc['max_ms']:.1f} ms**")
    lines.append(f"- heap high-water seen: **{gc['heap_before_mb']:.0f} MB** -> last-after: **{gc['heap_after_mb']:.0f} MB**")
    for name, n in gc["pauses"].most_common(6):
        lines.append(f"  - {name}: {n}")
elif seen_done == "1":
    lines.append("")
    lines.append("- GC: gc.log missing or no pause lines parsed")

for label, prof, topn in (("CPU", cpu, 40), ("WALL", wall, 20), ("ALLOC", alloc, 20)):
    if not prof:
        continue
    total = prof["total"]
    lines.append("")
    lines.append(f"### {label} profile — self-time by research bucket (total samples {total})")
    lines.append("")
    lines.append("| bucket | self-time samples | share |")
    lines.append("|---|---|---|")
    for name, n in prof["bucket_self"].most_common(20):
        share = (100.0 * n / total) if total else 0.0
        lines.append(f"| {name} | {n} | {share:.1f}% |")
    lines.append("")
    lines.append(f"### {label} profile — tick-phase split (stack ancestry, leaf-first)")
    lines.append("")
    lines.append("| phase | self-time samples | share |")
    lines.append("|---|---|---|")
    for name, n in prof["phase_self"].most_common(14):
        share = (100.0 * n / total) if total else 0.0
        lines.append(f"| {name} | {n} | {share:.1f}% |")
    lines.append("")
    kinds = prof["kind_self"]
    j = kinds.get("JVM-Java", 0)
    nn = kinds.get("native/JVM-internal", 0)
    o = kinds.get("other", 0)
    lines.append(f"**JVM-vs-native split (leaf self-time):** JVM-Java **{j}** ({100.0*j/total:.1f}%) · "
                 f"native/JVM-internal **{nn}** ({100.0*nn/total:.1f}%) · other **{o}** ({100.0*o/total:.1f}%)")
    lines.append("")
    lines.append(f"### {label} profile — top-{topn} leaf frames by self-time")
    lines.append("")
    lines.append("| leaf frame | kind | samples | share |")
    lines.append("|---|---|---|---|")
    for frame, n in prof["top_self"].most_common(topn):
        share = (100.0 * n / total) if total else 0.0
        lines.append(f"| `{frame}` | {kind_of_leaf(frame)} | {n} | {share:.1f}% |")

if not cpu and seen_done == "1":
    lines.append("")
    lines.append("> NOTE: boot reached Done but no CPU collapsed stacks were produced —")
    lines.append("> profiler attach failed; report ranks are INVALID for this run (log stats only).")
lines.append("")
lines.append("## Artifacts in this run")
lines.append("")
for pat in ("cpu-collapsed.txt", "wall-collapsed.txt", "alloc-collapsed.txt",
            "cpu-flamegraph.html", "server-stdout.log", "gc.log", "ap.log",
            "spark-report*"):
    for p in sorted(glob.glob(os.path.join(work, pat))):
        lines.append(f"- `{os.path.basename(p)}` ({os.path.getsize(p)} B)")
lines.append("")
lines.append("NEXT: research rounds attack the top kernel buckets in order —")
lines.append("each round = one pre-registered c-crussty task with a Rust replacement")
lines.append("or a native bridge, gated by the module's parity discipline.")

out = os.path.join(work, "BOTTLENECKS_3.md")
with open(out, "w", encoding="utf-8") as f:
    f.write("\n".join(lines) + "\n")
print(f"[world3-report] wrote {out} (cpu samples {cpu['total'] if cpu else 0}, "
      f"wall {wall['total'] if wall else 0}, alloc {alloc['total'] if alloc else 0})")
