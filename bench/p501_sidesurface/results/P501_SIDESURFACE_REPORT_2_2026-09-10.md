# P501 SIDESURFACE — FIRST MEASUREMENT REPORT (TASK-156, agent-7625532f)

Scope: the 94 zero-evidence exports (TASK-155 census) in a self-contained
sidecar rig; canonical bench/p500/ untouched. Driver = canonical Bench.java
byte-identical (WARM=2, ROUNDS=5, ~120ms batches, min-of-two-medians,
DCE-proof sink, one JVM per group, 600s timeout, crash ladder N=16→1).

**FIRST MEASUREMENT — no baseline exists for this surface. No drift claims,
no wiring, no gate tuning. Ratios only for §103-registered conventions.**

## Environment
```
date: 2026-09-09T16:51:09+00:00
jvm: openjdk version "21.0.12.1" 2026-08-18
javac: /home/z/bin/javac
host: Linux 5.10.134-013.8.3.kangaroo.al8.x86_64 x86_64
N: 256
libs: /home/z/c-crussty/native/libpaper_native_jni.so:/home/z/c-crussty/native/libpaper_native_chunk_encode_jni.so
c-crussty HEAD: 5aaccec
JVM forks (SINK lines): 4
```

## Per-group results
### g0 — PaperNativeBeardifierBury `(I[D[D[D[J)I`
| method | ns/op | status |
|---|---:|---|
| currentBatchSummary | 1.6us | OK s0 |
| optimizedBatchSummary | 1.6us | OK s0 |
spread (max/min of OK rows): 0.2% — no direction claim

### g1 — PaperNativeBiomeGetBiome `(I[J[I[I[I[J)I`
| method | ns/op | status |
|---|---:|---|
| currentBatchSummary | 18.0us | OK s0 |
| optimizedBatchSummary | 16.2us | OK s0 |
spread (max/min of OK rows): 10.5% — no direction claim

### g2 — PaperNativeCarverIteration `(I[I[I[J)I`
| method | ns/op | status |
|---|---:|---|
| foreachSummary | 210.3ns | OK s0 |
| indexedSummary | 211.2ns | OK s0 |
spread (max/min of OK rows): 0.4% — no direction claim

### g34 — PaperNativeYClampedGradient `(I[I[I[I[D[D[J)I`
| method | ns/op | status |
|---|---:|---|
| currentBatchSummary | 1.7us | OK s0 |
| optimizedBatchSummary | 1.7us | OK s0 |
spread (max/min of OK rows): 0.1% — no direction claim

## §103-registered in-group pair ratios

ratio = t_base / t_opt; >1 means the optimized kernel is faster.

- g0 `currentBatchSummary` vs `optimizedBatchSummary` [optimized/current]: **1.00x** (1.6us vs 1.6us)
- g1 `currentBatchSummary` vs `optimizedBatchSummary` [optimized/current]: **1.11x** (18.0us vs 16.2us)
- g2 `foreachSummary` vs `indexedSummary` [indexed/foreach]: **1.00x** (210.3ns vs 211.2ns)
- g34 `currentBatchSummary` vs `optimizedBatchSummary` [optimized/current]: **1.00x** (1.7us vs 1.7us)

## Cross-group same-class siblings (listed, never ratioed — different sigs)

- PaperNativeChunkExpireCount: g3 `(IIII[J)I` hotSummary; g4 `(III[J)I` coldSummary
- PaperNativeHashPath: g10 `([Ljava/lang/Object;I[J)I` streamingSummary; g11 `([Ljava/lang/Object;[J)I` readAllSummary
- PaperNativeLz4: g13 `([BI[B)I` lz4BlockCompress; g14 `([B[B)I` lz4BlockDecompress
- net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode: g35 `(I[S[B[I[BI[I[JI[B[I[BI[I[JI[BI)I` nativeEncodeSectionDataSized; g36 `([J[J[J[J[BI[BI[B)I` nativeEncodeLightData; g37 `([S[B[I[B[I[J[B[I[B[I[J[B)I` nativeEncodeSectionData
- net.minecraft.world.level.biome.PaperNativeClimateRTree: g39 `(J)J` nativeChecksumTreeHandle; g40 `(J)V` nativeFreeTreeHandle; g41 `(JJJJJJJJI)I` nativeSearchBoundedOnePacked,nativeSearchCurrentOnePacked; g42 `(J[JI[I[J)I` nativeSearchBoundedBatchPacked; g43 `([J[J)J` nativeBuildTreeHandle
- net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise: g45 `(J)V` nativeFreeHandle; g46 `(JDDD)D` nativeGetValueNoYScale; g47 `(JDDDDDZ)D` nativeGetValue; g48 `([B[B[D[D[D[DDD)J` nativeBuildHandle

## Summary

- groups attempted: 4 (groups.tsv: 49)
- RESULT rows: 8 OK/SLOW, 0 ERR
- CRASH groups: 0
- DCE-proof sink: every surviving JVM emitted one SINK line (4 total)
- INJECTS-ONLY: 0 server boots, 0 product changes, canonical p500 untouched

