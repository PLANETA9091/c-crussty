#!/usr/bin/env python3
"""RECON-13b (TASK-325): attribution of the chunk-(re)load parse churn.

Question (TASK-323 NEXT): WHO calls SerializableChunkData.parse in steady
state and WHEN. Findings hardcoded as verification tool:

- parse path: BalancedPrioritisedThreadPool -> ChunkLoadTask$ChunkDataLoadTask
  -> GenericDataLoadTask$ProcessOffMainTask -> SerializableChunkData.parse
- 10G base s7165: 10,186/30,516 alloc samples = 33.38% of ALL alloc bytes
  (DataResult-linked subset was 19.4% — full parse path is wider)
- WINDOW ANOMALY: cpu window (0-55%) = 0.00%, wall window (55-80%) = 0.00%,
  alloc window (80-100%) = 45.03% -> chunk (re)load BURST at end of soak,
  NOT steady-state churn. RECON-13c must find the trigger (rate instrument).

Usage: python3 scripts/bench4_recon/recon13b_chunk_load.py <alloc-collapsed.txt>
"""
import sys, re, collections


def main(path):
    paths = collections.Counter()
    total = 0
    grand = 0
    for line in open(path, errors="ignore"):
        stack, _, cnt = line.rstrip("\n").rpartition(" ")
        try:
            n = int(cnt)
        except ValueError:
            continue
        grand += n
        if "SerializableChunkData" not in stack:
            continue
        total += n
        fr = stack.split(";")
        for k, f in enumerate(fr):
            if "parse" in f and "SerializableChunkData" in f:
                up = [x.split("/")[-1][:38] for x in fr[max(0, k - 3):k]]
                paths[" <- ".join(up)] += n
                break
    print(f"parse-linked alloc: {total:,}/{grand:,} = {100*total/grand:.2f}%")
    for k, v in paths.most_common(6):
        print(f"{100*v/total:5.1f}%  {v:7,}  {k}")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else
         "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7165-recon-diag/alloc-collapsed.txt")
