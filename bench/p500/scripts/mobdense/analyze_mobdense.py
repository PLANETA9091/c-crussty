#!/usr/bin/env python3
"""TASK-81 mob-dense census analyzer.

Parses `jfr print --events jdk.ExecutionSample` text output and produces:
  - total + server-thread sample counts
  - per-target-frame presence counts and shares (server-thread basis)

Usage: python3 analyze_mobdense.py <exec_samples.txt> [out.tsv]

Target frames: the TASK-81 census list (isInWall / sensors / memories /
GoalSelector / Brain / fluid-push / checkInsideBlocks / noCollision /
item-entity / grid queries) + umbrella ticks for context.
"""
import re
import sys
from collections import Counter

TARGETS = [
    ("Entity.isInWall",                        r"Entity\.isInWall"),
    ("NearestLivingEntitySensor.doTick",       r"NearestLivingEntitySensor\.doTick"),
    ("Sensor.doTick (any sensor)",             r"Sensor[s]?\.|Sensor\.doTick|net\.minecraft\.world\.entity\.ai\.sensor"),
    ("Brain.forgetOutdatedMemories",           r"Brain\.forgetOutdatedMemories"),
    ("Brain.tick (umbrella)",                  r"Brain\.tick\("),
    ("GoalSelector.tick",                      r"GoalSelector\.tick"),
    ("WrappedGoal.tick / Goal.tick",           r"(WrappedGoal|Goal)\.tick|net\.minecraft\.world\.entity\.ai\.goal\.[A-Z]\w+Goal\.(tick|canUse|canContinueToUse)"),
    ("Behavior/Brain scheduler",               r"behavior\.(Behavior|GateBehavior|OneShot)|net\.minecraft\.world\.entity\.ai\.behavior"),
    ("Entity.updateFluidHeightAndDoFluidPushing", r"updateFluidHeightAndDoFluidPushing"),
    ("Entity.checkInsideBlocks",               r"checkInsideBlocks"),
    ("CollisionGetter.noCollision",            r"noCollision"),
    ("EntityGetter.getEntitiesOfClass (grid)", r"getEntitiesOfClass"),
    ("ItemEntity.tick",                        r"ItemEntity\.tick"),
    ("Entity.move",                            r"(?<![\w$])Entity\.move\("),
    ("LivingEntity.tick (umbrella)",           r"LivingEntity\.tick\("),
    ("Entity.baseTick (umbrella)",             r"Entity\.baseTick\("),
    ("PalettedContainer.get (block read)",     r"PalettedContainer\.get"),
    ("EntitySelector / Predicate",             r"EntitySelector"),
]

EVENT_SPLIT = re.compile(r"(?m)^jdk\.ExecutionSample\s*\{", re.M)
THREAD_RE = re.compile(r'sampledThread\s*=\s*"([^"]+)"')
FRAME_RE = re.compile(r"^\s*[\w$.]+\.[\w$<>]+\(")

def main(path, out=None):
    text = open(path, encoding="utf-8", errors="replace").read()
    events = EVENT_SPLIT.split(text)[1:]
    total = len(events)
    server = []
    for ev in events:
        m = THREAD_RE.search(ev)
        if m and m.group(1) == "Server thread":
            server.append(ev)
    ns = len(server)
    print(f"events={total} server_thread={ns} ({ns/total*100:.1f}% of {total})")
    cnt = Counter()
    for name, pat in TARGETS:
        rx = re.compile(pat)
        c = sum(1 for ev in server if rx.search(ev))
        cnt[name] = c
    rows = sorted(cnt.items(), key=lambda kv: -kv[1])
    print(f"\n{'frame':<46}{'samples':>8}{'share%':>9}")
    for name, c in rows:
        print(f"{name:<46}{c:>8}{(c/ns*100 if ns else 0):>9.1f}")
    if out:
        with open(out, "w") as f:
            f.write("frame\tsamples\tshare_of_server_thread\n")
            for name, c in rows:
                f.write(f"{name}\t{c}\t{(c/ns*100 if ns else 0):.2f}\n")
        print(f"\nwrote {out}")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else None)
