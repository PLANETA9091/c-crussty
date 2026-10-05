#!/usr/bin/env python3
"""NCF P0.1/P0.2 — aggregate JFR ExecutionSample text into ChunkStatus-stage CPU buckets.

Input: `jfr print --events jdk.ExecutionSample` (and jdk.NativeMethodSample)
text dumps of the cold and warm burst recordings (profile_chunkstatus.sh).

Attribution semantics (documented honestly, see GOLDEN_HARNESS.md / owner
worklog section 6):
  - Each jdk.ExecutionSample nominally represents 10 ms of Java-thread CPU
    (JFR profile settings period). Buckets are shares of JAVA-executing
    samples; absolute CPU-seconds are estimated as share * measured
    cpu_burst (from /proc ticks, the primary CPU metric).
  - A sample is attributed to the stage of the FIRST (deepest) stack frame
    whose class/method matches a stage pattern ("nearest enclosing known
    worldgen stage"). Samples with no matching frame -> jvm_other.
  - jdk.NativeMethodSample gives the JVM-native share separately (Rust NCF
    will compete with that, not just the Java share).

Usage:
  parse_stage_profile.py [--chunks N] [--t-cold S --cpu-cold C --t-warm S --cpu-warm C]
                         cold.samples.txt cold.natives.txt warm.samples.txt warm.natives.txt
Output: TSV report on stdout.
"""
import argparse
import collections
import re
import sys

SAMPLE_PERIOD_MS = 10.0  # JFR "profile" settings default jdk.ExecutionFilter period

# Granular stages; FIRST match wins, so specific patterns precede generic.
# (case-insensitive substring match on the full frame text)
GRANULAR = [
    ("noise_core",        ["levelgen.synth", "improvednoise", "perlinnoise", "normalnoise", "blendednoise"]),
    ("ore_veins",         ["orefeature"]),
    ("aquifers",          ["aquifer"]),
    ("density_dag",       ["levelgen.noise", "noisechunk", "densityfunction", "densityfunctions"]),
    ("biome_climate",     ["climate", "multinoise", "biomesource", "biome.", "biomes"]),
    ("surface",           ["surfacesystem", "surfacerule", "surfacebuilder"]),
    ("carvers",           ["carver"]),
    ("structures",        ["structurestart", "structurepiece", "structure.", "structures.", "jigsaw", "templatepool", "poipuzzle", "structurecheck", "structurecalculate"]),
    ("features",          ["feature", "placement", "blockplacer", "treegrower"]),
    ("light",             ["lightengine", "lightenginelayer", "lighting", "skylight", "blocklight"]),
    ("io_nbt_compress",   ["regionfile", "serializablechunkdata", "chunkserializer", "nbt", "deflater", "inflater", "zlib", "compression", "chunkstorage", "iostore"]),
    ("scheduler_status",  ["chunktaskscheduler", "chunkstatus", "chunkpyramid", "chunkstep", "generationchunkholder", "mainthreadbox", "threadedtaskbox", "chunktaskpriority", "chunkholder", "executor"]),
]

# Merge granular -> section-6 budget rows of the owner worklog.
MERGE = {
    "noise_aquifers_oreveins": ["noise_core", "ore_veins", "aquifers", "density_dag"],
    "biomes":                  ["biome_climate"],
    "surface":                 ["surface"],
    "carvers":                 ["carvers"],
    "features":                ["features"],
    "light":                   ["light"],
    "nbt_compression":         ["io_nbt_compress"],
    "scheduler_status":        ["scheduler_status"],
}

SAMPLE_RE = re.compile(r"^(jdk\.(?:ExecutionSample|NativeMethodSample)) \{")
THREAD_RE = re.compile(r"javaThreadId = (\d+)")
WEIGHT_RE = re.compile(r"weight = ([0-9.]+) (ms|s|us)")
FRAME_RE = re.compile(r"^\s+(net\.|com\.|org\.|io\.|java\.|jdk\.|ca\.|dev\.|<|\S+)\S+\(")


def parse_samples(path):
    """Yield (kind, thread_id, weight_ms, frames[list of str]) per sample block."""
    try:
        with open(path, "r", errors="replace") as f:
            lines = f.readlines()
    except FileNotFoundError:
        return
    kind = tid = None
    weight = SAMPLE_PERIOD_MS
    frames = []
    in_stack = False
    for ln in lines:
        m = SAMPLE_RE.match(ln)
        if m:
            if kind:
                yield kind, tid, weight, frames
            kind = m.group(1)
            tid = None
            weight = SAMPLE_PERIOD_MS
            frames = []
            in_stack = False
            continue
        if kind is None:
            continue
        if ln.startswith("}"):
            yield kind, tid, weight, frames
            kind = None
            continue
        mt = THREAD_RE.search(ln)
        if mt:
            tid = int(mt.group(1))
            continue
        mw = WEIGHT_RE.search(ln)
        if mw:
            v = float(mw.group(1))
            weight = v / 1000.0 if mw.group(2) == "us" else (v * 1000.0 if mw.group(2) == "s" else v)
            continue
        if "stackTrace = [" in ln:
            in_stack = True
            continue
        if in_stack:
            if ln.strip() == "]":
                in_stack = False
                continue
            s = ln.strip()
            if s:
                frames.append(s)
    if kind:
        yield kind, tid, weight, frames


def stage_of(frames):
    """Nearest-enclosing-known-stage attribution: first frame (deepest) matching."""
    for fr in frames:
        low = fr.lower()
        for name, pats in GRANULAR:
            for p in pats:
                if p in low:
                    return name
    return "jvm_other"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cold_samples")
    ap.add_argument("cold_natives")
    ap.add_argument("warm_samples")
    ap.add_argument("warm_natives")
    ap.add_argument("--chunks", type=int, default=128)
    ap.add_argument("--t-cold", type=float, default=0)
    ap.add_argument("--cpu-cold", type=float, default=0)
    ap.add_argument("--t-warm", type=float, default=0)
    ap.add_argument("--cpu-warm", type=float, default=0)
    a = ap.parse_args()

    out = []
    for label, samples_path, natives_path, t_burst, cpu_burst in (
        ("cold", a.cold_samples, a.cold_natives, a.t_cold, a.cpu_cold),
        ("warm", a.warm_samples, a.warm_natives, a.t_warm, a.cpu_warm),
    ):
        gran = collections.Counter()
        threads = collections.Counter()
        leaf = collections.Counter()
        native_ms = 0.0
        java_ms = 0.0
        for kind, tid, weight, frames in parse_samples(samples_path):
            if kind == "jdk.NativeMethodSample":
                native_ms += weight
                continue
            java_ms += weight
            gran[stage_of(frames)] += weight
            threads[tid or 0] += weight
            if frames:
                leaf[frames[0][:110]] += weight
        for _k, _tid, w, _f in parse_samples(natives_path):
            native_ms += w

        total = sum(gran.values()) or 1.0
        out.append(f"#{label}_burst: t_burst={t_burst:.0f}s cpu_burst={cpu_burst:.2f}CPU-s "
                   f"chunks={a.chunks} wall_per_chunk={t_burst/max(a.chunks,1)*1000:.1f}ms "
                   f"cpu_per_chunk={cpu_burst/max(a.chunks,1)*1000:.1f}CPU-ms "
                   f"chunks_per_s_per_core={a.chunks/max(cpu_burst,1e-9):.2f} "
                   f"java_sample_s={java_ms/1000:.1f} native_sample_s={native_ms/1000:.1f}")

        out.append(f"#stage\t{label}_ms\t{label}_pct_java\test_cpu_s")
        merged = collections.Counter()
        for name, ms in gran.items():
            merged[name] += ms
        for bucket, parts in MERGE.items():
            ms = sum(merged.pop(p, 0.0) for p in parts)
            if ms:
                out.append(f"{bucket}\t{ms:.0f}\t{100.0*ms/total:.2f}\t{cpu_burst*ms/total:.2f}")
        for name, ms in sorted(merged.items(), key=lambda kv: -kv[1]):
            out.append(f"{name}\t{ms:.0f}\t{100.0*ms/total:.2f}\t{cpu_burst*ms/total:.2f}")
        out.append(f"native_jvm_internal\t{native_ms:.0f}\t-\t{cpu_burst*native_ms/max(total+native_ms,1e-9):.2f}")

        out.append(f"#threads_{label} (top 10 by sampled Java ms; parallelism view)")
        for tid, ms in threads.most_common(10):
            out.append(f"thread_{tid}\t{ms:.0f}")
        out.append(f"#leaf_hotspots_{label} (top 15 leaf frames)")
        for fr, ms in leaf.most_common(15):
            out.append(f"{ms:.0f}\t{fr}")
        out.append("")

    print("\n".join(out))


if __name__ == "__main__":
    sys.exit(main())
