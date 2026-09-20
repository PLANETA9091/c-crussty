#!/usr/bin/env python3
import os, zipfile, subprocess, sys
RUN_DIR = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7196-p2pregate"
VERDICT = "/home/z/c-crussty/research/gc-recon-2026-09-19/RECON37_VERDICT.md"
Z = "/tmp/test_world3.zip"
def wline(thread, frames, n):
    return f"{thread};{';'.join(frames)} {n}\n"
env = ("region_threads: 4\nbatch_collector: 1\ninside_cache: 1\nflush_diet: 1\n"
       "travel_diet: 0\nrunner_cpu_index: 7123456\n")
rows = []
for i, act in enumerate([100, 90, 90, 90]):
    t = f"crussty-region-worker-{i}"
    rows.append(wline(t, ["RegionTickOps", "tickBucket", "EntityTickOps.tickEntities"], act))
    rows.append(wline(t, ["RegionTickOps", "CyclicBarrier.await", "parkAndCheckInterrupt"], 50))
rows.append(wline("Server thread", ["MinecraftServer", "spin", "tickChildren", "park"], 200))
rows.append(wline("Server thread", ["MinecraftServer", "spin", "tickChildren", "RegionTickOps"], 30))
open("/tmp/wall-collapsed.txt", "w").writelines(rows)
with zipfile.ZipFile(Z, "w") as z:
    z.writestr("run-env.txt", env)
    z.writestr("server-stdout.log", "no crash markers here\n")
    z.writestr("wall-collapsed.txt", "".join(rows))
r = subprocess.run([sys.executable, "/home/z/c-crussty/scripts/bench4_recon/absorb_s7196.py",
                    "35488526730", "--zip", Z], capture_output=True, text=True)
print(r.stdout[-600:]); print("exit:", r.returncode)
print("auto-doc OK:", os.path.exists(VERDICT) and "OFFLOAD-READY" in open(VERDICT).read())
