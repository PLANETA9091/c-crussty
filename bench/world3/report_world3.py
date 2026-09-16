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

# BENCH-4 fixture mode (task170, S7-99): run-env.txt carries fake_players=N.
# N>0 => fake players were injected => fixture-validity gate applies
# (spawnable chunks > 0 + churn ACTIVE with summon_sweeps=0 + alive-check
# steady). bench-3 runs (N=0) skip the gate entirely (back-compat).
fake_players = 0
_run_env = os.path.join(work, "run-env.txt")
if os.path.isfile(_run_env):
    with open(_run_env, encoding="utf-8", errors="replace") as _f:
        _m = re.search(r"fake_players:\s*(\d+)", _f.read())
    if _m:
        fake_players = int(_m.group(1))

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
    presence_by_frame = collections.Counter()
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
            for fr in frames:
                presence_by_frame[fr.split("(")[0]] += n
    return {
        "total": total,
        "top_self": top_self,
        "leaf_samples": leaf_samples,
        "bucket_self": bucket_self,
        "phase_self": phase_self,
        "kind_self": kind_self,
        "presence_by_frame": presence_by_frame,
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
    if not windows:
        # S7-96b fallback (run#10-12 root-cause: `paper mspt` does not exist on
        # Purpur 1.21.10 — every poll answered Usage-error; real MSPT arrives
        # from `spark tickmonitor` [⚡] Max/Min/Average lines instead).
        tmax = re.compile(r">\s*Max:\s*(\d+\.?\d*)\s*ms", re.I)
        tmin = re.compile(r">\s*Min:\s*(\d+\.?\d*)\s*ms", re.I)
        tavg = re.compile(r">\s*Average:\s*(\d+\.?\d*)\s*ms", re.I)
        vmax = vmin = None
        vavg = []
        with open(path, encoding="utf-8", errors="replace") as f:
            for line in f:
                for pat, dest in ((tmax, "Max"), (tmin, "Min"), (tavg, "Avg")):
                    m = pat.search(line)
                    if m:
                        v = float(m.group(1))
                        if dest == "Max":
                            vmax = v if vmax is None else max(vmax, v)
                        elif dest == "Min":
                            vmin = v if vmin is None else min(vmin, v)
                        else:
                            vavg.append(v)
        if vmax is not None:
            avg = (sum(vavg) / len(vavg)) if vavg else 0.0
            windows = [{"window": "spark tickmonitor (whole run, [⚡] lines)",
                        "vals": {"Max": vmax, "Min": vmin or 0.0, "Avg": avg or 0.0}}]
    return [w for w in windows if w["vals"]]


def parse_entity_totals(path):
    """`paper entity list`: 'Total entities: N' style lines + top types."""
    totals = []
    types = collections.Counter()
    if not os.path.exists(path):
        return totals, types
    # run#15 format: "Total Ticking: 9201, Total Non-Ticking: 707" (Paper 1.21.10);
    # older guesses ("Total entities: N") never matched any real output
    tpat = re.compile(r"Total Ticking:\s*(\d+),\s*Total Non-Ticking:\s*(\d+)", re.I)
    upat = re.compile(r"(\d+)\s+\((\d+)\)\s+:\s+([a-z_]+:[a-z0-9_/]+)")
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = tpat.search(line)
            if m:
                totals.append(int(m.group(1)))
            for m in upat.finditer(line):
                types[m.group(3)] = max(types[m.group(3)], int(m.group(1)))
    return totals, types


def parse_entity_churn(path):
    """F4 (task165/S7-96b, owner 20-TPS scenario): per-poll entity counts ->
    spawn/despawn churn evidence. Owner directive: mobs must spawn AND despawn
    'as if players are present' — this metric measures whether the bench
    condition actually exercises the spawn/despawn lifecycle (summon sweeps,
    natural churn) or the world entity population is stagnant across the run.
    Returns (blocks, summon_count): blocks = per-poll {type: count} dicts in
    order; summon_count = 'Summoned new' console confirmations.
    """
    blocks = []
    summon = 0
    if not os.path.exists(path):
        return blocks, summon
    # run#15 format: "Total Ticking: 9201, Total Non-Ticking: 707" (Paper 1.21.10);
    # older guesses ("Total entities: N") never matched any real output
    tpat = re.compile(r"Total Ticking:\s*(\d+),\s*Total Non-Ticking:\s*(\d+)", re.I)
    upat = re.compile(r"(\d+)\s+\((\d+)\)\s+:\s+([a-z_]+:[a-z0-9_/]+)")
    spat = re.compile(r"Summoned new \w+", re.I)
    cur = None
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            if spat.search(line):
                summon += 1
            m = tpat.search(line)
            if m:
                cur = {}
                blocks.append(cur)
                continue
            if cur is not None:
                for m in upat.finditer(line):
                    cur[m.group(3)] = int(m.group(1))
    return blocks, summon


def parse_spawnable_series(path):
    """BENCH-4 fixture evidence (task170): `paper mobcaps world` header lines.
    The header is composed with SpawnState.getSpawnableChunkCount() (javap
    bench4-recon C7) — parse the first integer after the marker as the
    spawnable-chunk count for that poll. None when the line carried no digits.
    """
    out = []
    if not os.path.exists(path):
        return out
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            if "Mobcaps for world" in line:
                nums = re.findall(r"\d+", line)
                out.append(int(nums[-1]) if nums else None)
    return out


def parse_alive_series(path):
    """BENCH-4 fixture evidence: '[BenchFakePlayers] alive-check:' heartbeat
    lines -> (level.players() count, injected count) per poll. A drop proves
    the stub lost players (keepalive timeout / disconnect) => fixture INVALID.
    """
    out = []
    if not os.path.exists(path):
        return out
    rx = re.compile(r"\[BenchFakePlayers\] alive-check: level\.players\(\)=(\d+) injected=(\d+)")
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = rx.search(line)
            if m:
                out.append((int(m.group(1)), int(m.group(2))))
    return out


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
# S7-96d: run-environment pairing (world snapshot + runner CPU speed)
env_path = os.path.join(work, "run-env.txt")
if os.path.exists(env_path):
    lines.append("### Run environment (pairing discipline, S7-96d)")
    lines.append("")
    for ln in open(env_path, encoding="utf-8", errors="replace"):
        lines.append(f"- {ln.rstrip()}")
    lines.append("")
    lines.append("> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave")
    lines.append("> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are")
    lines.append("> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use")
    lines.append("> same-boot A/B only.")
    lines.append("")
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

# --- preregistered CI metrics (task164/S7-95: F1, F2, F3) -------------------
lines.append("")
lines.append("## Preregistered CI metrics (TASK-230 F1/F2/F3)")
lines.append("")
# F1 — JNI/module self-time share (kernel §8 law: < 2%)
if cpu:
    tot = cpu["total"]
    f1 = sum(n for name, n in cpu["bucket_self"].items()
             if name in ("c-crussty module (Rust)", "CRUSSTY engine runtime (Rust)",
                         "Crussty CE natives (JNI)"))
    verdict = "PASS (< 2%)" if f1 / tot < 0.02 else "FAIL (>= 2% — investigate)"
    lines.append(f"- **F1 module/JNI self-time share:** {f1} / {tot} = **{100.0*f1/tot:.2f}%** — §8 {verdict}")
# F3 — per-class entity tick split (presence over CPU stacks)
if cpu:
    ent = re.compile(r"net/minecraft/world/entity/[a-z]+/[A-Za-z0-9_$]+\.tick")
    per = collections.Counter()
    for frame, n in cpu["presence_by_frame"].items():
        if ent.match(frame):
            per[frame] = n
    if per:
        lines.append("- **F3 per-class entity tick split (top-12 by stack presence):**")
        lines.append("")
        lines.append("| entity class tick | presence samples | share of CPU |")
        lines.append("|---|---|---|")
        for frame, n in per.most_common(12):
            lines.append(f"| `{frame}` | {n} | {100.0*n/cpu['total']:.2f}% |")
# F2 — allocation profile top sites + GC-based MB/s estimate
if alloc:
    lines.append("- **F2 allocation profile (top-10 sites by alloc-event samples; "
                 "interval-relative shares):**")
    lines.append("")
    lines.append("| alloc site | samples | share |")
    lines.append("|---|---|---|")
    for frame, n in alloc["top_self"].most_common(10):
        lines.append(f"| `{frame}` | {n} | {100.0*n/alloc['total']:.1f}% |")
if gc and gc["events"]:
    lines.append(f"- **F2 GC-churn estimate:** {gc['events']} pauses / total {gc['total_ms']:.0f} ms STW "
                 f"(see GC section above; MB/s needs region-size constants — wired next tick)")
# F4 — entity spawn/despawn churn (owner 20-TPS scenario fidelity, S7-96b)
churn_blocks, summon_count = parse_entity_churn(os.path.join(work, "server-stdout.log"))
f4_line = "- **F4 entity spawn/despawn churn (owner scenario):**"
lines.append(f4_line)
if ent_totals and len(ent_totals) >= 2:
    lo, hi = min(ent_totals), max(ent_totals)
    avg = sum(ent_totals) / len(ent_totals)
    churn_pct = 100.0 * (hi - lo) / avg if avg else 0.0
    lines[-1] = (f4_line + f" polls={len(ent_totals)} "
                 f"total={lo}..{hi} (delta {hi-lo}, churn {churn_pct:.1f}%), summons={summon_count}")
    if churn_blocks:
        type_max = collections.Counter()
        for b in churn_blocks:
            for t, c in b.items():
                type_max[t] = max(type_max[t], c)
        type_min = {}
        for b in churn_blocks:
            for t in type_max:
                type_min[t] = min(type_min.get(t, 10**9), b.get(t, 0))
        movers = sorted(((t, type_min[t], type_max[t]) for t in type_max),
                        key=lambda x: x[2] - x[1], reverse=True)[:8]
        moved = [f"{t} {lo2}->{hi2}" for t, lo2, hi2 in movers if hi2 - lo2 > 0]
        lines.append("  - top movers (max-min across polls): " + (", ".join(moved) if moved else "NONE"))
    if summon_count > 0 or hi > lo:
        lines.append("  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)")
    else:
        lines.append("  - verdict: **churn STAGNANT** — world entity population static; natural spawning "
                     "idle (0 players). bench-4 fake-player leg required for the owner's as-if-players condition")
else:
    lines[-1] = f4_line + " insufficient polls (need >=2 `paper entity list` outputs)"

# BENCH-4 fixture-validity gate (task170, S7-99; preregistered in
# docs/BENCH4_FAKE_PLAYERS_DESIGN.md §4):
#   gate 1 fixture-validity: spawnable chunks > 0 AND churn ACTIVE
#     (with summon_sweeps=0 — churn must be NATURAL) AND alive-check steady;
#   gate 2 baseline: bench-4 baseline (N=4) measured min-of-2 paired; the
#     delta to bench-3 is a SCENARIO delta, not a module win (enforced by
#     research discipline, documented here).
if fake_players > 0:
    lines.append("")
    lines.append("## BENCH-4 fixture validity (fake_players=%d)" % fake_players)
    lines.append("")
    spawnable = parse_spawnable_series(os.path.join(work, "server-stdout.log"))
    alive = parse_alive_series(os.path.join(work, "server-stdout.log"))
    spawnable_ok = any(v is not None and v > 0 for v in spawnable)
    alive_ok = bool(alive) and all(p >= fake_players for p, _ in alive) \
        and all(i == fake_players for _, i in alive)
    natural_churn_ok = (summon_count == 0 and ent_totals and len(ent_totals) >= 2
                        and max(ent_totals) > min(ent_totals))
    if spawnable:
        vals = [v for v in spawnable if v is not None]
        lines.append("- spawnable-chunk polls (mobcaps header): %s" % vals)
    else:
        lines.append("- spawnable-chunk polls: NONE parsed from `paper mobcaps world` output")
    if alive:
        lines.append("- alive-check heartbeat series: %s" % [f"{p}/{i}" for p, i in alive])
    else:
        lines.append("- alive-check heartbeat: NONE parsed (plugin heartbeat missing — check plugin log)")
    lines.append(f"- gate 1a spawnable chunks > 0: {'PASS' if spawnable_ok else 'FAIL'}")
    lines.append(f"- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): "
                 f"{'PASS' if natural_churn_ok else 'FAIL'} (summons={summon_count}, "
                 f"polls={len(ent_totals)}, delta={(max(ent_totals) - min(ent_totals)) if ent_totals and len(ent_totals) >= 2 else 'n/a'})")
    lines.append(f"- gate 1c alive-check steady at N={fake_players}: {'PASS' if alive_ok else 'FAIL'}")
    fixture_valid = spawnable_ok and natural_churn_ok and alive_ok and seen_done == "1"
    lines.append("")
    lines.append("- **FIXTURE-VALIDITY: %s**" % ("VALID" if fixture_valid else "INVALID"))
    if not fixture_valid:
        lines.append("  - bench-4 run INVALID as owner-scenario evidence: do NOT use its "
                     "MSPT as the canonical-scenario baseline; fix the fixture first.")
else:
    lines.append("")
    lines.append("- BENCH-4 fixture gate: N/A (fake_players=0, bench-3 mode)")

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
