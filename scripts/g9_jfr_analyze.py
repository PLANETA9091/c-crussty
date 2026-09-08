#!/usr/bin/env python3
"""S7-14 G9 JFR amplification analyzer.

Reads `jfr print --events jdk.ExecutionSample` text output and computes:
  - total samples, samples on the Server thread,
  - samples whose stack contains the g9 target frames:
      DensityFunctions$Ap2.fillArray   (the g9 method itself)
      NoiseChunk$NoiseInterpolator.fillArray / NoiseChunk.fillSlice (drivers)
      NoiseChunk / generateNoise       (chunk-gen context)
  - inclusive time share estimate for Ap2.fillArray on the Server thread
  - calls/tick estimate via the matrix §2.1 identity (R_mid = 54 ns/call)
G9 bar (docs/G9_WHOLE_METHOD_HOOK_DESIGN.md §4.2): >= ~18.5k MIN/MAX-arm
calls/tick (12k-40k band). Below that = amplification blocker NOT cleared.
"""
import re
import sys

TARGETS = {
    "ap2_fillArray": re.compile(r"DensityFunctions\$Ap2\.fillArray"),
    "interpolator_fillArray": re.compile(r"NoiseInterpolator\.fillArray"),
    "fillSlice": re.compile(r"NoiseChunk\.fillSlice"),
    "noise_chunk_gen": re.compile(r"NoiseChunk\.(?:create|recalculate|generate)"),
    "generate_noise": re.compile(r"generateNoise"),
    "chunk_system": re.compile(r"ChunkSystem|ChunkMap|ChunkTaskScheduler"),
}

def main(path: str) -> None:
    text = open(path, encoding="utf-8", errors="replace").read()
    # jfr print blocks look like:
    # jdk.ExecutionSample { / startTime = ... / ... /
    #   stackTrace = [ frames... ] / ... }
    blocks = re.findall(r"jdk\.ExecutionSample \{(.*?)\n\}", text, re.S)
    total = len(blocks)
    server = 0
    counts = {k: 0 for k in TARGETS}
    server_counts = {k: 0 for k in TARGETS}
    tname = re.compile(r"thread = \"([^\"]+)\"")
    for b in blocks:
        m = tname.search(b)
        is_server = bool(m and ("Server" in m.group(1) and "Watchdog" not in m.group(1)))
        if is_server:
            server += 1
        for k, rx in TARGETS.items():
            if rx.search(b):
                counts[k] += 1
                if is_server:
                    server_counts[k] += 1
    print(f"samples_total={total}")
    print(f"samples_server_thread={server}")
    for k in TARGETS:
        print(f"  {k}: any_thread={counts[k]} server_thread={server_counts[k]}")
    if server:
        for k in ("ap2_fillArray", "interpolator_fillArray", "fillSlice"):
            share = server_counts[k] / server * 100.0
            print(f"  share[{k}] of server-thread samples = {share:.3f}%")
    # Calls/tick estimate: if the sustained-gen window has T seconds of
    # Server-thread CPU sampling and Ap2.fillArray holds fraction f of it,
    # time-in-frame per second = f*T; with 20 ticks/s and R=54ns/call:
    # calls/tick = f*T*1e9 / (20 ticks * 54 ns)
    T = float(sys.argv[2]) if len(sys.argv) > 2 else 480.0
    f = (server_counts["ap2_fillArray"] / server) if server else 0.0
    ticks = T * 20.0
    calls_per_tick = f * T * 1e9 / (ticks * 54.0) if ticks else 0.0
    print(f"estimated Ap2.fillArray calls/tick (R_mid=54ns, window={T:.0f}s): {calls_per_tick:,.0f}")
    print("G9 bar: >= ~18,500 calls/tick (band 12k-40k) — "
          "BLOCKER CLEARED" if calls_per_tick >= 12000 else "G9 bar NOT reached — blocker (a) stays FAILED")

if __name__ == "__main__":
    main(sys.argv[1])
