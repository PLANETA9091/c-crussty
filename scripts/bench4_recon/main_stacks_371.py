#!/usr/bin/env python3
"""TASK-371: что делает Server thread в s7196 threaded-профиле (агрегат по top-фреймам)."""
import re, sys, collections

path = sys.argv[1] if len(sys.argv) > 1 else "research/gc-recon-2026-09-19/run-s7196-p2pregate/wall-collapsed.txt"
MAIN = re.compile(r"^\[Server thread(?:\s+tid=\d+)?\];")

tot = 0
top_frames = collections.Counter()
leaf_frames = collections.Counter()
mid_buckets = collections.Counter()

for line in open(path, encoding="utf-8", errors="replace"):
    line = line.rstrip("\n")
    if not MAIN.match(line):
        continue
    m = re.match(r"^(.*?)\s+(\d+)$", line)
    if not m:
        continue
    parts = m.group(1).split(";")
    n = int(m.group(2))
    tot += n
    # parts[0] = thread token; frames = parts[1:]
    frames = parts[1:]
    if frames:
        leaf_frames[frames[0]] += n
    for f in frames[1:7]:
        top_frames[f] += n
    # bucket-маркеры RegionTickOps в стеке
    for f in frames:
        if "RegionTickOps" in f or "TickTask" in f or "tickBucket" in f:
            mid_buckets[f] += n
            break

print(f"== MAIN (Server thread): всего {tot} сэмплов")
print("\n-- TOP листовых фреймов (self):")
for f, c in leaf_frames.most_common(18):
    print(f"  {c:5d}  {f}")
print("\n-- TOP вложенных фреймов (крит-путь):")
for f, c in top_frames.most_common(22):
    print(f"  {c:5d}  {f}")
print("\n-- Bucket-маркеры в стеке:")
for f, c in mid_buckets.most_common(10):
    print(f"  {c:5d}  {f}")
