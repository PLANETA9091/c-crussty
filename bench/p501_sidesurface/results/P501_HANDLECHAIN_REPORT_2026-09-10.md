# P501 HANDLE-CHAIN — FIRST MEASUREMENT REPORT (TASK-157, agent-7625532f)

In-JVM build→sentinel-check→consume→free chain for the handle families
TASK-156 crashed on (per-group JVMs cannot pass handles). Synthetic args,
canonical batch discipline (8 calls/iter, ~120ms, WARM=2/ROUNDS=5 median;
single-call medians for destructive free over untimed pools).

**FIRST MEASUREMENT — no baseline, no drift claims, no wiring.**

## Environment
```
date: 2026-09-09T17:05:34+00:00
jvm: openjdk version "21.0.12.1" 2026-08-18
javac: /home/z/bin/javac
host: Linux 5.10.134-013.8.3.kangaroo.al8.x86_64 x86_64
libs: /home/z/c-crussty/native/libpaper_native_jni.so:/home/z/c-crussty/native/libpaper_native_chunk_encode_jni.so
```

## Family perlin — net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise (nativeBuildHandle/nativeGetValue/nativeGetValueNoYScale/nativeFreeHandle)
| op | ns/op | status |
|---|---:|---|
| build | - | BUILD-SENTINEL h=0 |
| consumers | - | SKIPPED-SENTINEL (build failed — handle lane closed for synthetic probing) |

## Family rtree — net.minecraft.world.level.biome.PaperNativeClimateRTree (nativeBuildTreeHandle/nativeChecksumTreeHandle/nativeSearch*/nativeFreeTreeHandle)
| op | ns/op | status |
|---|---:|---|
| build | - | BUILD-SENTINEL h=0 |
| consumers | - | SKIPPED-SENTINEL (build failed — handle lane closed for synthetic probing) |

## Verdicts (pre-registered tree)

- perlin: **H-SENTINEL**
- rtree: **H-SENTINEL**

## Summary

- SINK lines (DCE-proof): 2 (perlin,rtree)
- INJECTS-ONLY: 0 server boots, 0 product changes; sidecar-own probe,
  canonical p500 + generated trees untouched

