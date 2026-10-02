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
c-crussty HEAD: c4570ee
JVM forks (SINK lines): 42
```

## Per-group results
### g0 — PaperNativeBeardifierBury `(I[D[D[D[J)I`
| method | ns/op | status |
|---|---:|---|
| currentBatchSummary | 1.6us | OK s0 |
| optimizedBatchSummary | 1.6us | OK s0 |
spread (max/min of OK rows): 0.4% — no direction claim

### g1 — PaperNativeBiomeGetBiome `(I[J[I[I[I[J)I`
| method | ns/op | status |
|---|---:|---|
| currentBatchSummary | 18.0us | OK s0 |
| optimizedBatchSummary | 16.4us | OK s0 |
spread (max/min of OK rows): 9.8% — no direction claim

### g2 — PaperNativeCarverIteration `(I[I[I[J)I`
| method | ns/op | status |
|---|---:|---|
| foreachSummary | 212.9ns | OK s0 |
| indexedSummary | 214.4ns | OK s0 |
spread (max/min of OK rows): 0.7% — no direction claim

### g3 — PaperNativeChunkExpireCount `(IIII[J)I`
| method | ns/op | status |
|---|---:|---|
| hotSummary | 114.1ns | OK s0 |

### g4 — PaperNativeChunkExpireCount `(III[J)I`
**CRASH** (exit=134) — no usable rows; see logs/g4.log

### g5 — PaperNativeChunkTicketStage `([J[J[B[JII[J)I`
| method | ns/op | status |
|---|---:|---|
| runBatch | 190.9us | OK s0 |

### g6 — PaperNativeCompressionThresholdShape `([I[II[J)I`
| method | ns/op | status |
|---|---:|---|
| thresholdSummary | 80.30ms | OK s0 |

### g7 — PaperNativeCraftPlayerCanSee `(I[J)I`
NOTE: State-dependent (needs player/world view); synthetic args — expect rc-errors or semantically meaningless values; as-is, no wiring.
| method | ns/op | status |
|---|---:|---|
| emptyCandidateSummary | 2.2us | OK s0 |
| emptyChunkMapCandidateSummary | 5.9us | OK s0 |
| emptyCurrentSummary | 2.5us | OK s0 |
| emptyGuardedSummary | 2.5us | OK s0 |
| populatedCandidateSummary | 5.0us | OK s0 |
| populatedChunkMapCandidateSummary | 8.0us | OK s0 |
| populatedCurrentSummary | 5.0us | OK s0 |
| populatedGuardedSummary | 5.0us | OK s0 |
spread (max/min of OK rows): 257.3% — no direction claim

### g8 — PaperNativeDeflaterInputShape `([B[I[II[J)I`
| method | ns/op | status |
|---|---:|---|
| copiedSummary | 368.9ns | OK s0 |
| sliceSummary | 370.8ns | OK s0 |
spread (max/min of OK rows): 0.5% — no direction claim

### g9 — PaperNativeHash `([B[B)I`
| method | ns/op | status |
|---|---:|---|
| sha256Digest | 339.1ns | OK s0 |

### g10 — PaperNativeHashPath `([Ljava/lang/Object;I[J)I`
| method | ns/op | status |
|---|---:|---|
| streamingSummary | 86.4ns | OK s0 |

### g11 — PaperNativeHashPath `([Ljava/lang/Object;[J)I`
| method | ns/op | status |
|---|---:|---|
| readAllSummary | 86.6ns | OK s0 |

### g12 — PaperNativeImprovedNoiseFloor `([BI[J)I`
| method | ns/op | status |
|---|---:|---|
| currentMthFloorSummary | 9.6us | OK s0 |
| mathFloorSummary | 9.5us | OK s0 |
spread (max/min of OK rows): 1.3% — no direction claim

### g13 — PaperNativeLz4 `([BI[B)I`
| method | ns/op | status |
|---|---:|---|
| lz4BlockCompress | 53.8ns | OK s0 |

### g14 — PaperNativeLz4 `([B[B)I`
| method | ns/op | status |
|---|---:|---|
| lz4BlockDecompress | 159.3ns | OK s0 |

### g15 — PaperNativeLz4StreamRoundtrip `([BII[J)I`
| method | ns/op | status |
|---|---:|---|
| roundtripSummary | 73.7us | OK s0 |

### g16 — PaperNativeNbtCompoundMapCapacity `([B[I[II[J)I`
| method | ns/op | status |
|---|---:|---|
| parseCapacitySummary | 356.8ns | OK s0 |

### g17 — PaperNativeNbtGzipBufferShape `([IIII[J)I`
| method | ns/op | status |
|---|---:|---|
| shapeSummary | 538.3us | OK s0 |

### g18 — PaperNativeNearbyPlayerMap `(II[J)I`
| method | ns/op | status |
|---|---:|---|
| defaultCapacitySummary | 3.6us | OK s0 |
| presizedCapacitySummary | 2.9us | OK s0 |
spread (max/min of OK rows): 23.5% — no direction claim

### g19 — PaperNativeNoiseChunkInterpolatorArray `(IIII[J)I`
| method | ns/op | status |
|---|---:|---|
| arraySummary | 34.5us | OK s0 |
| indexedListSummary | 34.3us | OK s0 |
| listSummary | 35.5us | OK s0 |
spread (max/min of OK rows): 3.5% — no direction claim

### g20 — PaperNativeNoiseGeneratorSettings `([I[I[II[I)I`
| method | ns/op | status |
|---|---:|---|
| cachedIntSettings | 601.8ns | OK s0 |
| holderValueSettings | 602.5ns | OK s0 |
| lazyPrimitiveSettings | 602.0ns | OK s0 |
| manualLazyObjectSettings | 601.3ns | OK s0 |
| memoizedSupplierSettings | 602.1ns | OK s0 |
spread (max/min of OK rows): 0.2% — no direction claim

### g21 — PaperNativePalettedReencodeRemapCache `(I[J)I`
| method | ns/op | status |
|---|---:|---|
| cachedPaletteIdsSummary | 617.9us | OK s0 |
| currentPreviousOnlySummary | 782.2us | OK s0 |
spread (max/min of OK rows): 26.6% — no direction claim

### g22 — PaperNativePerlinGetValue `([B[B[D[D[D[D[DDDII[J)I`
| method | ns/op | status |
|---|---:|---|
| getValueVariantBatchSummary | 22.0ns | OK s0 |

### g23 — PaperNativePlacedFeatureTraversal `(JI[J)I`
| method | ns/op | status |
|---|---:|---|
| recursiveSummary | 3.1us | OK s0 |

### g24 — PaperNativePluginNameJoin `(I[Ljava/lang/Object;Ljava/lang/String;[J)I`
| method | ns/op | status |
|---|---:|---|
| manualJoinSummary | 86.8ns | OK s0 |
| stringJoinSummary | 87.2ns | OK s0 |
spread (max/min of OK rows): 0.5% — no direction claim

### g25 — PaperNativeRemapperHashThreshold `(I[Ljava/lang/Object;[J)I`
| method | ns/op | status |
|---|---:|---|
| computeIfAbsentSummary | 87.5ns | OK s0 |
| hybridSummary | 87.5ns | OK s0 |
| parallelSummary | 86.4ns | OK s0 |
| putSummary | 87.3ns | OK s0 |
spread (max/min of OK rows): 1.3% — no direction claim

### g26 — PaperNativeShiftNoiseDirect `(I[I[I[I[J)I`
| method | ns/op | status |
|---|---:|---|
| currentASummary | 1.5us | OK s0 |
| currentBSummary | 1.5us | OK s0 |
| currentDefaultSummary | 1.5us | OK s0 |
| directASummary | 1.5us | OK s0 |
| directBSummary | 1.5us | OK s0 |
| directDefaultSummary | 1.5us | OK s0 |
spread (max/min of OK rows): 1.1% — no direction claim

### g27 — PaperNativeSurfaceRulesSequenceArray `(II[J)I`
| method | ns/op | status |
|---|---:|---|
| arrayForeachSummary | 5.2us | OK s0 |
| arrayIndexedSummary | 2.8us | OK s0 |
| listEnhancedSummary | 2.8us | OK s0 |
| listIndexedSummary | 2.8us | OK s0 |
spread (max/min of OK rows): 88.8% — no direction claim

### g28 — PaperNativeTicketCompare `([I[J[B[I[I[II[J)I`
| method | ns/op | status |
|---|---:|---|
| compareIndexedBatch | 670.0ns | OK s0 |

### g29 — PaperNativeTicketPack `([J[B[III[J)I`
| method | ns/op | status |
|---|---:|---|
| packSummary | 54.1ns | OK s0 |

### g30 — PaperNativeWaypointChunkUpdate `(I[J)I`
NOTE: State-dependent (waypoint storage); synthetic args — as-is.
| method | ns/op | status |
|---|---:|---|
| distanceSummary | 565.4ns | OK s0 |
| longKeySummary | 296.7ns | OK s0 |
spread (max/min of OK rows): 90.6% — no direction claim

### g31 — PaperNativeWaypointManagerSkip `(I[J)I`
NOTE: State-dependent (waypoint manager internals); synthetic args — as-is.
| method | ns/op | status |
|---|---:|---|
| currentPlayerFullSummary | 175.0ns | OK s0 |
| currentPlayerPartialSummary | 175.2ns | OK s0 |
| currentWaypointFullSummary | 176.7ns | OK s0 |
| currentWaypointPartialSummary | 176.8ns | OK s0 |
| skipPlayerFullSummary | 175.5ns | OK s0 |
| skipPlayerPartialSummary | 175.8ns | OK s0 |
| skipWaypointFullSummary | 176.3ns | OK s0 |
| skipWaypointPartialSummary | 176.3ns | OK s0 |
spread (max/min of OK rows): 1.0% — no direction claim

### g32 — PaperNativeWaypointSnapshot `(I[J)I`
| method | ns/op | status |
|---|---:|---|
| manualSummary | 192.1ns | OK s0 |
| sizedArraySummary | 192.2ns | OK s0 |
| toArraySummary | 194.3ns | OK s0 |
spread (max/min of OK rows): 1.1% — no direction claim

### g33 — PaperNativeWaypointTableView `(I[J)I`
| method | ns/op | status |
|---|---:|---|
| columnSummary | 195.2ns | OK s0 |
| transposeRowSummary | 196.7ns | OK s0 |
spread (max/min of OK rows): 0.8% — no direction claim

### g34 — PaperNativeYClampedGradient `(I[I[I[I[D[D[J)I`
| method | ns/op | status |
|---|---:|---|
| currentBatchSummary | 1.7us | OK s0 |
| optimizedBatchSummary | 1.7us | OK s0 |
spread (max/min of OK rows): 0.3% — no direction claim

### g35 — net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode `(I[S[B[I[BI[I[JI[B[I[BI[I[JI[BI)I`
NOTE: CLOSED chunk-encode surface (§97): standalone rc=-3 fast-fail is the documented expectation (well-formed input, zero output bytes); in-server gate probe = TASK-150 phase-2a, pending /home/z/server+jdk21. ns/op of an error path is not a useful perf number — reported as-is for completeness.
| method | ns/op | status |
|---|---:|---|
| nativeEncodeSectionDataSized | 207.0ns | OK s0 |

### g36 — net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode `([J[J[J[J[BI[BI[B)I`
NOTE: CLOSED chunk-encode surface (§97): standalone rc=-3 fast-fail is the documented expectation (well-formed input, zero output bytes); in-server gate probe = TASK-150 phase-2a, pending /home/z/server+jdk21. ns/op of an error path is not a useful perf number — reported as-is for completeness.
| method | ns/op | status |
|---|---:|---|
| nativeEncodeLightData | 115.9ns | OK s0 |

### g37 — net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode `([S[B[I[B[I[J[B[I[B[I[J[B)I`
NOTE: CLOSED chunk-encode surface (§97): standalone rc=-3 fast-fail is the documented expectation (well-formed input, zero output bytes); in-server gate probe = TASK-150 phase-2a, pending /home/z/server+jdk21. ns/op of an error path is not a useful perf number — reported as-is for completeness.
| method | ns/op | status |
|---|---:|---|
| nativeEncodeSectionData | 276.8ns | OK s0 |

### g38 — net.minecraft.world.level.biome.PaperNativeClimate `([J[J[J[I[J)I`
| method | ns/op | status |
|---|---:|---|
| nodeBestMatchUniqueBatch | 40.7ns | OK s0 |

### g39 — net.minecraft.world.level.biome.PaperNativeClimateRTree `(J)J`
NOTE: Handle-based family: build/checksum/free/search share no state across separate JVMs; scalar handle args are synthetic constants — expect errors or crashes on search/free/checksum groups; as-is.
**CRASH** (exit=134) — no usable rows; see logs/g39.log

### g40 — net.minecraft.world.level.biome.PaperNativeClimateRTree `(J)V`
NOTE: Handle-based family: build/checksum/free/search share no state across separate JVMs; scalar handle args are synthetic constants — expect errors or crashes on search/free/checksum groups; as-is.
**CRASH** (exit=134) — no usable rows; see logs/g40.log

### g41 — net.minecraft.world.level.biome.PaperNativeClimateRTree `(JJJJJJJJI)I`
NOTE: Handle-based family: build/checksum/free/search share no state across separate JVMs; scalar handle args are synthetic constants — expect errors or crashes on search/free/checksum groups; as-is.
**CRASH** (exit=134) — no usable rows; see logs/g41.log

### g42 — net.minecraft.world.level.biome.PaperNativeClimateRTree `(J[JI[I[J)I`
NOTE: Handle-based family: build/checksum/free/search share no state across separate JVMs; scalar handle args are synthetic constants — expect errors or crashes on search/free/checksum groups; as-is.
| method | ns/op | status |
|---|---:|---|
| nativeSearchBoundedBatchPacked | 29.9ns | OK s0 |

### g43 — net.minecraft.world.level.biome.PaperNativeClimateRTree `([J[J)J`
NOTE: Handle-based family: build/checksum/free/search share no state across separate JVMs; scalar handle args are synthetic constants — expect errors or crashes on search/free/checksum groups; as-is.
| method | ns/op | status |
|---|---:|---|
| nativeBuildTreeHandle | 40.4ns | OK s0 |

### g44 — net.minecraft.world.level.levelgen.PaperNativeNoiseChunkWrapCapacity `([I[I[FI[J)I`
| method | ns/op | status |
|---|---:|---|
| shapeSummary | 378.4ns | OK s0 |

### g45 — net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise `(J)V`
**CRASH** (exit=134) — no usable rows; see logs/g45.log

### g46 — net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise `(JDDD)D`
**CRASH** (exit=134) — no usable rows; see logs/g46.log

### g47 — net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise `(JDDDDDZ)D`
**CRASH** (exit=134) — no usable rows; see logs/g47.log

### g48 — net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise `([B[B[D[D[D[DDD)J`
| method | ns/op | status |
|---|---:|---|
| nativeBuildHandle | 40.8ns | OK s0 |

## §103-registered in-group pair ratios

ratio = t_base / t_opt; >1 means the optimized kernel is faster.

- g0 `currentBatchSummary` vs `optimizedBatchSummary` [optimized/current]: **1.00x** (1.6us vs 1.6us)
- g1 `currentBatchSummary` vs `optimizedBatchSummary` [optimized/current]: **1.10x** (18.0us vs 16.4us)
- g2 `foreachSummary` vs `indexedSummary` [indexed/foreach]: **0.99x** (212.9ns vs 214.4ns)
- g34 `currentBatchSummary` vs `optimizedBatchSummary` [optimized/current]: **1.00x** (1.7us vs 1.7us)

## Cross-group same-class siblings (listed, never ratioed — different sigs)

- PaperNativeChunkExpireCount: g3 `(IIII[J)I` hotSummary; g4 `(III[J)I` coldSummary
- PaperNativeHashPath: g10 `([Ljava/lang/Object;I[J)I` streamingSummary; g11 `([Ljava/lang/Object;[J)I` readAllSummary
- PaperNativeLz4: g13 `([BI[B)I` lz4BlockCompress; g14 `([B[B)I` lz4BlockDecompress
- net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode: g35 `(I[S[B[I[BI[I[JI[B[I[BI[I[JI[BI)I` nativeEncodeSectionDataSized; g36 `([J[J[J[J[BI[BI[B)I` nativeEncodeLightData; g37 `([S[B[I[B[I[J[B[I[B[I[J[B)I` nativeEncodeSectionData
- net.minecraft.world.level.biome.PaperNativeClimateRTree: g39 `(J)J` nativeChecksumTreeHandle; g40 `(J)V` nativeFreeTreeHandle; g41 `(JJJJJJJJI)I` nativeSearchBoundedOnePacked,nativeSearchCurrentOnePacked; g42 `(J[JI[I[J)I` nativeSearchBoundedBatchPacked; g43 `([J[J)J` nativeBuildTreeHandle
- net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise: g45 `(J)V` nativeFreeHandle; g46 `(JDDD)D` nativeGetValueNoYScale; g47 `(JDDDDDZ)D` nativeGetValue; g48 `([B[B[D[D[D[DDD)J` nativeBuildHandle

## Summary

- groups attempted: 49 (groups.tsv: 49)
- RESULT rows: 86 OK/SLOW, 0 ERR
- CRASH groups: 7
- DCE-proof sink: every surviving JVM emitted one SINK line (42 total)
- INJECTS-ONLY: 0 server boots, 0 product changes, canonical p500 untouched

