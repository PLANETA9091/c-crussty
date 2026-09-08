# P500 benchmark report — Crussty CE native kernels (old vs optimized)

* Generated: 2026-09-08T06:45:29+00:00 (aggregator v2 — stem pairing, noise-model classification, baseline tracking)
* Raw data: `p500_raw.tsv` — one JVM fork per group, time-bounded batches (~120 ms), median of 5, identical synthesized args per group.
* Hardware note: shared 2-CPU sandbox; treat <±15% deltas as parity.
* Noise model: ratio = alt/old — **WIN** ≤ 0.85, **REGRESSION** ≥ 1.18, PARITY in between; repeated measurements of a kernel collapse to one primary row via min-of-medians (ties keep the earliest row); every collapsed repeat is retained as `method#variantK` — see 'Duplicate collapse (variant policy)'.
* stability = relative spread (max−min)/median of the underlying batch samples; for a pair the worse (larger) of the two kernels is shown; `n/a` when unavailable.

* Groups measured: 49, skipped: 0, crashed: 0, kernels measured: 129 (raw RESULT rows: 258, collapsed variant rows: 129 — duplicate policy: min-of-medians primary, repeats retained as #variantK)
* Pairs formed (P500 stem rule): 70 — unpaired kernels: 1, multi-pair warnings: 9
* Baseline diff vs: `baseline.tsv` (drift flagged when |Δratio| > 20%)

## Per-group results

### 0. `PaperNativeAquiferIndexStride` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 10.0 µs | 10.0 µs | 10.0 µs | 0.0% |
| `newBatchSummary#variant2` | alt | 10.0 µs | 10.0 µs | 10.0 µs | 0.0% |
| `oldBatchSummary` | old | 10.7 µs | 10.7 µs | 10.7 µs | 0.0% |
| `oldBatchSummary#variant2` | old | 10.7 µs | 10.7 µs | 10.7 µs | 0.0% |

### 1. `PaperNativeAquiferPositionalLocation` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directBatchSummary` | alt | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `oldBatchSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `directBatchSummary#variant2` | alt | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `oldBatchSummary#variant2` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |

### 2. `PaperNativeAquiferSurfaceSampling` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 5.5 µs | 5.5 µs | 5.5 µs | 0.0% |
| `newBatchSummary#variant2` | alt | 5.5 µs | 5.5 µs | 5.5 µs | 0.0% |
| `oldBatchSummary` | old | 6.1 µs | 6.1 µs | 6.1 µs | 0.0% |
| `oldBatchSummary#variant2` | old | 6.1 µs | 6.1 µs | 6.1 µs | 0.0% |

### 3. `PaperNativeBlendedNoise` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedBatchSummary` | alt | 50.5 µs | 50.5 µs | 50.5 µs | 0.0% |
| `cachedBatchSummary#variant2` | alt | 50.8 µs | 50.8 µs | 50.8 µs | 0.0% |
| `oldBatchSummary` | old | 51.4 µs | 51.4 µs | 51.4 µs | 0.0% |
| `oldBatchSummary#variant2` | old | 51.5 µs | 51.5 µs | 51.5 µs | 0.0% |

### 4. `PaperNativeCaveCarverSkip` `(I[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `reusedCheckerSummary` | alt | 34.2 ms | 34.2 ms | 34.2 ms | 0.0% |
| `oldLambdaSummary` | old | 34.2 ms | 34.2 ms | 34.2 ms | 0.0% |
| `directHelperSummary` | alt | 34.3 ms | 34.3 ms | 34.3 ms | 0.0% |
| `reusedCheckerSummary#variant2` | alt | 34.4 ms | 34.4 ms | 34.4 ms | 0.0% |
| `directHelperSummary#variant2` | alt | 34.4 ms | 34.4 ms | 34.4 ms | 0.0% |
| `oldLambdaSummary#variant2` | old | 34.5 ms | 34.5 ms | 34.5 ms | 0.0% |

### 5. `PaperNativeChunkDependencies` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `arraySummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `arraySummary#variant2` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldImmutableListSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldImmutableListSummary#variant2` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |

### 6. `PaperNativeClimateParameterDistance` `([J[J[J[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `subtractFirstDistanceSum` | alt | 524.2 ns | 524.2 ns | 524.2 ns | 0.0% |
| `oldDistanceSum` | old | 524.4 ns | 524.4 ns | 524.4 ns | 0.0% |
| `branchDistanceSum` | alt | 525.4 ns | 525.4 ns | 525.4 ns | 0.0% |
| `branchDistanceSum#variant2` | alt | 534.0 ns | 534.0 ns | 534.0 ns | 0.0% |
| `oldDistanceSum#variant2` | old | 534.3 ns | 534.3 ns | 534.3 ns | 0.0% |
| `subtractFirstDistanceSum#variant2` | alt | 536.4 ns | 536.4 ns | 536.4 ns | 0.0% |

### 7. `PaperNativeCubicSplineCreate` `(I[F[F[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldIteratorSummary` | old | 152.8 µs | 152.8 µs | 152.8 µs | 0.0% |
| `indexSummary` | alt | 153.2 µs | 153.2 µs | 153.2 µs | 0.0% |
| `oldIteratorSummary#variant2` | old | 153.4 µs | 153.4 µs | 153.4 µs | 0.0% |
| `indexSummary#variant2` | alt | 153.6 µs | 153.6 µs | 153.6 µs | 0.0% |

### 8. `PaperNativeDensityAp2Fill` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchFlatSummary` | alt | 11.3 µs | 11.3 µs | 11.3 µs | 0.0% |
| `scratchFlatSummary#variant2` | alt | 11.3 µs | 11.3 µs | 11.3 µs | 0.0% |
| `oldFlatSummary` | old | 13.0 µs | 13.0 µs | 13.0 µs | 0.0% |
| `oldFlatSummary#variant2` | old | 13.0 µs | 13.0 µs | 13.0 µs | 0.0% |
| `scratchNestedSummary` | alt | 19.7 µs | 19.7 µs | 19.7 µs | 0.0% |
| `scratchNestedSummary#variant2` | alt | 19.7 µs | 19.7 µs | 19.7 µs | 0.0% |
| `oldNestedSummary` | old | 21.9 µs | 21.9 µs | 21.9 µs | 0.0% |
| `oldNestedSummary#variant2` | old | 22.2 µs | 22.2 µs | 22.2 µs | 0.0% |

### 9. `PaperNativeDensityAp2MinMaxFill` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 118.7 ns | 118.7 ns | 118.7 ns | 0.0% |
| `newSummary` | alt | 118.9 ns | 118.9 ns | 118.9 ns | 0.0% |
| `oldSummary#variant2` | old | 118.9 ns | 118.9 ns | 118.9 ns | 0.0% |
| `newSummary#variant2` | alt | 119.3 ns | 119.3 ns | 119.3 ns | 0.0% |

### 10. `PaperNativeDensitySplineContext` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldWrapperSummary` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `oldWrapperSummary#variant2` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `newDirectSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `newDirectSummary#variant2` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |

### 11. `PaperNativeDensityVisitorHook` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldUnwrappingSummary` | old | 672.4 ns | 672.4 ns | 672.4 ns | 0.0% |
| `hookedUnwrappingSummary` | alt | 676.2 ns | 676.2 ns | 676.2 ns | 0.0% |
| `oldUnwrappingSummary#variant2` | old | 676.8 ns | 676.8 ns | 676.8 ns | 0.0% |
| `hookedUnwrappingSummary#variant2` | alt | 677.5 ns | 677.5 ns | 677.5 ns | 0.0% |

### 12. `PaperNativeEntityBoundingBox` `(I[F[F[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldMakeThenSetSummary` | old | 5.7 µs | 5.7 µs | 5.7 µs | 0.0% |
| `oldMakeThenSetSummary#variant2` | old | 5.7 µs | 5.7 µs | 5.7 µs | 0.0% |
| `directDimensionsSetSummary` | alt | 5.8 µs | 5.8 µs | 5.8 µs | 0.0% |
| `directDimensionsSetSummary#variant2` | alt | 5.8 µs | 5.8 µs | 5.8 µs | 0.0% |

### 13. `PaperNativeEntityChunkTransient` `(IIJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newMixedSummary` | alt | 291.1 ns | 291.1 ns | 291.1 ns | 0.0% |
| `newMixedSummary#variant2` | alt | 291.2 ns | 291.2 ns | 291.2 ns | 0.0% |
| `oldMixedSummary` | old | 292.9 ns | 292.9 ns | 292.9 ns | 0.0% |
| `oldMixedSummary#variant2` | old | 293.4 ns | 293.4 ns | 293.4 ns | 0.0% |

### 14. `PaperNativeEntityLookupStatus` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directAccessibleSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `directAccessibleSummary#variant2` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldAccessibleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldStatusSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldStatusSummary#variant2` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldAccessibleSummary#variant2` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `directStatusSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `directStatusSummary#variant2` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |

### 15. `PaperNativeImprovedNoiseDerivative` `([B[I[I[I[D[D[DI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatGradientDerivativeSummary` | alt | 11.5 µs | 11.5 µs | 11.5 µs | 0.0% |
| `flatGradientDerivativeSummary#variant2` | alt | 11.6 µs | 11.6 µs | 11.6 µs | 0.0% |
| `inlineDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `oldDerivativeSummary` | old | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `intTableDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `oldDerivativeSummary#variant2` | old | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `intTableDerivativeSummary#variant2` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `inlineDerivativeSummary#variant2` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |

### 16. `PaperNativeImprovedNoiseInline` `([BI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `switchGradientSummary` | alt | 7.6 µs | 7.6 µs | 7.6 µs | 0.0% |
| `switchGradientSummary#variant2` | alt | 7.7 µs | 7.7 µs | 7.7 µs | 0.0% |
| `arithmeticSummary` | alt | 8.0 µs | 8.0 µs | 8.0 µs | 0.0% |
| `arithmeticSummary#variant2` | alt | 8.0 µs | 8.0 µs | 8.0 µs | 0.0% |
| `flatGradientSummary` | alt | 8.1 µs | 8.1 µs | 8.1 µs | 0.0% |
| `flatGradientSummary#variant2` | alt | 8.1 µs | 8.1 µs | 8.1 µs | 0.0% |
| `inlineByteAccessSummary` | alt | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |
| `oldPMethodSummary` | old | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |
| `oldPMethodSummary#variant2` | old | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |
| `inlineByteAccessSummary#variant2` | alt | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |

### 17. `PaperNativeJigsawCanAttach` `(I[I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 625.6 ns | 625.6 ns | 625.6 ns | 0.0% |
| `targetFirstBatchSummary` | alt | 626.3 ns | 626.3 ns | 626.3 ns | 0.0% |
| `optimizedBatchSummary` | alt | 626.8 ns | 626.8 ns | 626.8 ns | 0.0% |
| `targetFirstBatchSummary#variant2` | alt | 627.3 ns | 627.3 ns | 627.3 ns | 0.0% |
| `oldBatchSummary#variant2` | old | 627.4 ns | 627.4 ns | 627.4 ns | 0.0% |
| `optimizedBatchSummary#variant2` | alt | 628.1 ns | 628.1 ns | 628.1 ns | 0.0% |

### 18. `PaperNativeLegacyProvidedAliasRemoval` `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newReverseAliasRemoveSummary` | alt | 88.1 ns | 88.1 ns | 88.1 ns | 0.0% |
| `oldValuesRemoveIfSummary` | old | 88.2 ns | 88.2 ns | 88.2 ns | 0.0% |
| `oldValuesRemoveIfSummary#variant2` | old | 88.3 ns | 88.3 ns | 88.3 ns | 0.0% |
| `newReverseAliasRemoveSummary#variant2` | alt | 88.8 ns | 88.8 ns | 88.8 ns | 0.0% |

### 19. `PaperNativeLevelChunkHeightmap` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFourUpdateSummary` | old | 2.2 ms | 2.2 ms | 2.2 ms | 0.0% |
| `oldFourUpdateSummary#variant2` | old | 2.2 ms | 2.2 ms | 2.2 ms | 0.0% |
| `newCombinedUpdateSummary` | alt | 12.4 ms | 12.4 ms | 12.4 ms | 0.0% |
| `newCombinedUpdateSummary#variant2` | alt | 12.4 ms | 12.4 ms | 12.4 ms | 0.0% |

### 20. `PaperNativeMarkerCache` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 118.7 µs | 118.7 µs | 118.7 µs | 0.0% |
| `oldSummary#variant2` | old | 119.9 µs | 119.9 µs | 119.9 µs | 0.0% |
| `cachedSummary` | alt | 552.3 µs | 552.3 µs | 552.3 µs | 0.0% |
| `cachedSummary#variant2` | alt | 553.8 µs | 553.8 µs | 553.8 µs | 0.0% |

### 21. `PaperNativeNoiseChunkBlendCache` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newEmptyBlenderSummary` | alt | 232.4 ns | 232.4 ns | 232.4 ns | 0.0% |
| `newEmptyBlenderSummary#variant2` | alt | 232.5 ns | 232.5 ns | 232.5 ns | 0.0% |
| `oldEmptyBlenderSummary` | old | 73.1 µs | 73.1 µs | 73.1 µs | 0.0% |
| `oldEmptyBlenderSummary#variant2` | old | 74.2 µs | 74.2 µs | 74.2 µs | 0.0% |

### 22. `PaperNativeNoiseChunkFlatCacheContext` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newFalseContextSummary` | alt | 18.7 µs | 18.7 µs | 18.7 µs | 0.0% |
| `newFalseContextSummary#variant2` | alt | 18.8 µs | 18.8 µs | 18.8 µs | 0.0% |
| `newTrueContextSummary` | alt | 18.8 µs | 18.8 µs | 18.8 µs | 0.0% |
| `newTrueContextSummary#variant2` | alt | 19.1 µs | 19.1 µs | 19.1 µs | 0.0% |
| `oldFalseContextSummary` | old | 21.8 µs | 21.8 µs | 21.8 µs | 0.0% |
| `oldFalseContextSummary#variant2` | old | 21.9 µs | 21.9 µs | 21.9 µs | 0.0% |
| `oldTrueContextSummary` | old | 23.6 µs | 23.6 µs | 23.6 µs | 0.0% |
| `oldTrueContextSummary#variant2` | old | 23.6 µs | 23.6 µs | 23.6 µs | 0.0% |

### 23. `PaperNativeNoiseInterpolatorSlice` `(IIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatSummary` | alt | 1.8 ms | 1.8 ms | 1.8 ms | 0.0% |
| `flatSummary#variant2` | alt | 1.9 ms | 1.9 ms | 1.9 ms | 0.0% |
| `oldJaggedSummary` | old | 6.1 ms | 6.1 ms | 6.1 ms | 0.0% |
| `oldJaggedSummary#variant2` | old | 6.2 ms | 6.2 ms | 6.2 ms | 0.0% |

### 24. `PaperNativeObfHelperMaps` `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directMapsSummary` | alt | 93.3 ns | 93.3 ns | 93.3 ns | 0.0% |
| `oldStreamDefaultSummary` | old | 93.5 ns | 93.5 ns | 93.5 ns | 0.0% |
| `presizedStringPoolSummary` | alt | 93.5 ns | 93.5 ns | 93.5 ns | 0.0% |
| `oldStreamDefaultSummary#variant2` | old | 93.5 ns | 93.5 ns | 93.5 ns | 0.0% |
| `directMapsSummary#variant2` | alt | 93.6 ns | 93.6 ns | 93.6 ns | 0.0% |
| `presizedStringPoolSummary#variant2` | alt | 94.4 ns | 94.4 ns | 94.4 ns | 0.0% |

### 25. `PaperNativeOreFeatureLoop` `([D[D[D[D[I[I[I[I[I[IIIIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `optimizedLoopSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `oldLoopSummary` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `optimizedLoopSummary#variant2` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `oldLoopSummary#variant2` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |

### 26. `PaperNativeOwnableRule` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 3.7 µs | 3.7 µs | 3.7 µs | 0.0% |
| `newLoopSummary#variant2` | alt | 3.7 µs | 3.7 µs | 3.7 µs | 0.0% |
| `oldStreamSummary` | old | 4.2 µs | 4.2 µs | 4.2 µs | 0.0% |
| `oldStreamSummary#variant2` | old | 4.2 µs | 4.2 µs | 4.2 µs | 0.0% |

### 27. `PaperNativePalettedReencodeScratch` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchThreadLocalSummary` | alt | 411.6 µs | 411.6 µs | 411.6 µs | 0.0% |
| `scratchThreadLocalSummary#variant2` | alt | 412.9 µs | 412.9 µs | 412.9 µs | 0.0% |
| `oldNewArraySummary` | old | 489.5 µs | 489.5 µs | 489.5 µs | 0.0% |
| `oldNewArraySummary#variant2` | old | 492.2 µs | 492.2 µs | 492.2 µs | 0.0% |
| `directPackedSummary` | alt | 1.1 ms | 1.1 ms | 1.1 ms | 0.0% |
| `directPackedSummary#variant2` | alt | 1.1 ms | 1.1 ms | 1.1 ms | 0.0% |

### 28. `PaperNativePluginClassLoaderGroup` `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLookupSummary` | old | 91.3 ns | 91.3 ns | 91.3 ns | 0.0% |
| `skipRequesterSummary` | alt | 91.9 ns | 91.9 ns | 91.9 ns | 0.0% |
| `oldLookupSummary#variant2` | old | 92.0 ns | 92.0 ns | 92.0 ns | 0.0% |
| `skipRequesterSummary#variant2` | alt | 92.4 ns | 92.4 ns | 92.4 ns | 0.0% |

### 29. `PaperNativePluginDirectoryScan` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newListSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `directoryStreamSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `newListSummary#variant2` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `directoryStreamSummary#variant2` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `oldWalkDepth1Summary` | old | 1.5 ms | 1.5 ms | 1.5 ms | 0.0% |
| `oldWalkDepth1Summary#variant2` | old | 1.5 ms | 1.5 ms | 1.5 ms | 0.0% |

### 30. `PaperNativePluginLoadingAllocation` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLazyMissingSetSummary` | alt | 88.0 ns | 88.0 ns | 88.0 ns | 0.0% |
| `newLazyMissingSetSummary#variant2` | alt | 88.5 ns | 88.5 ns | 88.5 ns | 0.0% |
| `newLazyValidateSummary` | alt | 88.6 ns | 88.6 ns | 88.6 ns | 0.0% |
| `newLazyValidateSummary#variant2` | alt | 88.6 ns | 88.6 ns | 88.6 ns | 0.0% |
| `newPresizedSetupSummary` | alt | 88.9 ns | 88.9 ns | 88.9 ns | 0.0% |
| `oldDefaultCapacitySetupSummary` | old | 89.0 ns | 89.0 ns | 89.0 ns | 0.0% |
| `oldEagerMissingSetSummary` | old | 89.1 ns | 89.1 ns | 89.1 ns | 0.0% |
| `newPresizedSetupSummary#variant2` | alt | 89.4 ns | 89.4 ns | 89.4 ns | 0.0% |
| `oldDefaultCapacitySetupSummary#variant2` | old | 89.4 ns | 89.4 ns | 89.4 ns | 0.0% |
| `oldEagerValidateSummary` | old | 89.7 ns | 89.7 ns | 89.7 ns | 0.0% |
| `oldEagerMissingSetSummary#variant2` | old | 89.9 ns | 89.9 ns | 89.9 ns | 0.0% |
| `oldEagerValidateSummary#variant2` | old | 91.3 ns | 91.3 ns | 91.3 ns | 0.0% |

### 31. `PaperNativePluginMetaDependency` `(I[Ljava/lang/Object;[Z[Z[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedSummary` | alt | 91.7 ns | 91.7 ns | 91.7 ns | 0.0% |
| `oldStreamSummary` | old | 92.0 ns | 92.0 ns | 92.0 ns | 0.0% |
| `newLoopSummary` | alt | 92.2 ns | 92.2 ns | 92.2 ns | 0.0% |
| `cachedSummary#variant2` | alt | 92.3 ns | 92.3 ns | 92.3 ns | 0.0% |
| `newLoopSummary#variant2` | alt | 92.3 ns | 92.3 ns | 92.3 ns | 0.0% |
| `oldStreamSummary#variant2` | old | 92.6 ns | 92.6 ns | 92.6 ns | 0.0% |

### 32. `PaperNativePluginNameLog` `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newArrayListSortSummary` | alt | 87.7 ns | 87.7 ns | 87.7 ns | 0.0% |
| `oldTreesetSummary` | old | 88.3 ns | 88.3 ns | 88.3 ns | 0.0% |
| `newArrayListSortSummary#variant2` | alt | 88.4 ns | 88.4 ns | 88.4 ns | 0.0% |
| `oldTreesetSummary#variant2` | old | 88.6 ns | 88.6 ns | 88.6 ns | 0.0% |

### 33. `PaperNativePluginStartupRollup` `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 95.3 ns | 95.3 ns | 95.3 ns | 0.0% |
| `newSummary` | alt | 95.5 ns | 95.5 ns | 95.5 ns | 0.0% |
| `oldSummary#variant2` | old | 95.8 ns | 95.8 ns | 95.8 ns | 0.0% |
| `newSummary#variant2` | alt | 95.9 ns | 95.9 ns | 95.9 ns | 0.0% |

### 34. `PaperNativeProtoChunkHeightmap` `(I)J`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEnumSetForeachSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `oldEnumSetForeachSummary#variant2` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `newCachedContainsSummary` | alt | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |
| `newCachedContainsSummary#variant2` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |

### 35. `PaperNativeRangeChoice` `([D[I[I[II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFillArraySummary` | old | 81.4 ns | 81.4 ns | 81.4 ns | 0.0% |
| `optimizedFillArraySummary` | alt | 81.6 ns | 81.6 ns | 81.6 ns | 0.0% |
| `oldFillArraySummary#variant2` | old | 81.6 ns | 81.6 ns | 81.6 ns | 0.0% |
| `optimizedFillArraySummary#variant2` | alt | 81.6 ns | 81.6 ns | 81.6 ns | 0.0% |

### 36. `PaperNativeRemapperIndexCleanup` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEagerCleanupSummary` | old | 91.2 ns | 91.2 ns | 91.2 ns | 0.0% |
| `newLazyCleanupSummary` | alt | 91.3 ns | 91.3 ns | 91.3 ns | 0.0% |
| `newLazyCleanupSummary#variant2` | alt | 91.4 ns | 91.4 ns | 91.4 ns | 0.0% |
| `oldEagerCleanupSummary#variant2` | old | 91.5 ns | 91.5 ns | 91.5 ns | 0.0% |

### 37. `PaperNativeRemapperSkipHashes` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 32.8 µs | 32.8 µs | 32.8 µs | 0.0% |
| `newLoopSummary#variant2` | alt | 33.2 µs | 33.2 µs | 33.2 µs | 0.0% |
| `oldStreamSummary` | old | 34.7 µs | 34.7 µs | 34.7 µs | 0.0% |
| `oldStreamSummary#variant2` | old | 35.0 µs | 35.0 µs | 35.0 µs | 0.0% |

### 38. `PaperNativeServerEntityDeltaIdentity` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDistanceSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `oldDistanceSummary#variant2` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |

### 39. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoadAfterBuildSummary` | alt | 87.5 ns | 87.5 ns | 87.5 ns | 0.0% |
| `oldLoadAfterBuildSummary` | old | 87.6 ns | 87.6 ns | 87.6 ns | 0.0% |
| `oldLoadAfterBuildSummary#variant2` | old | 87.8 ns | 87.8 ns | 87.8 ns | 0.0% |
| `newLoadAfterBuildSummary#variant2` | alt | 87.9 ns | 87.9 ns | 87.9 ns | 0.0% |

### 40. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newRemovedCountSummary` | alt | 87.8 ns | 87.8 ns | 87.8 ns | 0.0% |
| `newRemovedCountSummary#variant2` | alt | 88.1 ns | 88.1 ns | 88.1 ns | 0.0% |
| `oldRemovedCountSummary` | old | 88.3 ns | 88.3 ns | 88.3 ns | 0.0% |
| `oldRemovedCountSummary#variant2` | old | 88.5 ns | 88.5 ns | 88.5 ns | 0.0% |

### 41. `PaperNativeSpringFeatureMutablePos` `(I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |
| `oldBatchSummary#variant2` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |
| `mutableBatchSummary` | alt | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |
| `mutableBatchSummary#variant2` | alt | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |

### 42. `PaperNativeStaticCacheGet` `(IIIII[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 34.4 ns | 34.4 ns | 34.4 ns | 0.0% |
| `newBatchSummary` | alt | 34.5 ns | 34.5 ns | 34.5 ns | 0.0% |
| `newBatchSummary#variant2` | alt | 34.6 ns | 34.6 ns | 34.6 ns | 0.0% |
| `oldBatchSummary#variant2` | old | 34.6 ns | 34.6 ns | 34.6 ns | 0.0% |

### 43. `PaperNativeSurfaceRulesTestRuleState` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStateRuleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldStateRuleSummary#variant2` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `newStateRuleSummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `newStateRuleSummary#variant2` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |

### 44. `PaperNativeTopographicGraphSortCapacity` `(I[I[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDefaultCapacitySummary` | old | 349.2 ns | 349.2 ns | 349.2 ns | 0.0% |
| `newPresizedSummary` | alt | 349.3 ns | 349.3 ns | 349.3 ns | 0.0% |
| `newPresizedSummary#variant2` | alt | 349.5 ns | 349.5 ns | 349.5 ns | 0.0% |
| `oldDefaultCapacitySummary#variant2` | old | 350.1 ns | 350.1 ns | 350.1 ns | 0.0% |

### 45. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedAtOrBeyondRangeSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs | 0.0% |
| `guardedAtOrBeyondRangeSummary#variant2` | alt | 9.5 µs | 9.5 µs | 9.5 µs | 0.0% |
| `oldAtOrBeyondRangeSummary` | old | 9.6 µs | 9.6 µs | 9.6 µs | 0.0% |
| `oldAtOrBeyondRangeSummary#variant2` | old | 9.6 µs | 9.6 µs | 9.6 µs | 0.0% |

### 46. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarSummary` | old | 8.3 µs | 8.3 µs | 8.3 µs | 0.0% |
| `oldReallyFarSummary#variant2` | old | 8.3 µs | 8.3 µs | 8.3 µs | 0.0% |
| `guardedReallyFarSummary` | alt | 8.4 µs | 8.4 µs | 8.4 µs | 0.0% |
| `guardedReallyFarSummary#variant2` | alt | 8.4 µs | 8.4 µs | 8.4 µs | 0.0% |

### 47. `PaperNativeWaypointHotPath` `(I)D`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarValue` | old | 944.2 ns | 944.2 ns | 944.2 ns | 0.0% |
| `oldReallyFarValue#variant2` | old | 947.1 ns | 947.1 ns | 947.1 ns | 0.0% |
| `guardedReallyFarValue` | alt | 1.0 µs | 1.0 µs | 1.0 µs | 0.0% |
| `guardedReallyFarValue#variant2` | alt | 1.0 µs | 1.0 µs | 1.0 µs | 0.0% |
| `oldAtOrBeyondRangeValue` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `guardedAtOrBeyondRangeValue` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `oldAtOrBeyondRangeValue#variant2` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `guardedAtOrBeyondRangeValue#variant2` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `directAzimuthValue` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `directAzimuthValue#variant2` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `oldAzimuthValue` | old | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `oldAzimuthValue#variant2` | old | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `cachedChunkVisibleValue` | alt | 3.0 µs | 3.0 µs | 3.0 µs | 0.0% |
| `cachedChunkVisibleValue#variant2` | alt | 3.1 µs | 3.1 µs | 3.1 µs | 0.0% |
| `oldChunkVisibleValue` | old | 3.4 µs | 3.4 µs | 3.4 µs | 0.0% |
| `oldChunkVisibleValue#variant2` | old | 3.4 µs | 3.4 µs | 3.4 µs | 0.0% |
| `optimizedWaypointManagerValue` | alt | 92.4 µs | 92.4 µs | 92.4 µs | 0.0% |
| `optimizedWaypointManagerValue#variant2` | alt | 93.7 µs | 93.7 µs | 93.7 µs | 0.0% |
| `oldWaypointManagerValue` | old | 98.9 µs | 98.9 µs | 98.9 µs | 0.0% |
| `oldWaypointManagerValue#variant2` | old | 99.9 µs | 99.9 µs | 99.9 µs | 0.0% |

### 48. `PaperNativeXoroshiroPositionalDirect` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directDoubleBatchSummary` | alt | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |
| `directFloatBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldDoubleBatchSummary` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `directDoubleBatchSummary#variant2` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldDoubleBatchSummary#variant2` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldFloatBatchSummary` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `directFloatBatchSummary#variant2` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldFloatBatchSummary#variant2` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |

## Duplicate collapse (variant policy)

Raw RESULT rows repeating a (group, method) kernel collapse to one primary row via min-of-medians (ties keep the earliest row — unchanged v2 semantics, so no previously reported median can change). Every collapsed repeat is retained below as `method#variantK`; variants are repeat measurements of the same kernel and never form pairs.

* DUP-VARIANT g0 PaperNativeAquiferIndexStride newBatchSummary#variant2 kind=alt med=10.0 µs kept-primary=10.0 µs policy=min-of-medians
* DUP-VARIANT g0 PaperNativeAquiferIndexStride oldBatchSummary#variant2 kind=old med=10.7 µs kept-primary=10.7 µs policy=min-of-medians
* DUP-VARIANT g1 PaperNativeAquiferPositionalLocation directBatchSummary#variant2 kind=alt med=2.2 µs kept-primary=2.2 µs policy=min-of-medians
* DUP-VARIANT g1 PaperNativeAquiferPositionalLocation oldBatchSummary#variant2 kind=old med=2.2 µs kept-primary=2.2 µs policy=min-of-medians
* DUP-VARIANT g2 PaperNativeAquiferSurfaceSampling newBatchSummary#variant2 kind=alt med=5.5 µs kept-primary=5.5 µs policy=min-of-medians
* DUP-VARIANT g2 PaperNativeAquiferSurfaceSampling oldBatchSummary#variant2 kind=old med=6.1 µs kept-primary=6.1 µs policy=min-of-medians
* DUP-VARIANT g3 PaperNativeBlendedNoise cachedBatchSummary#variant2 kind=alt med=50.8 µs kept-primary=50.5 µs policy=min-of-medians
* DUP-VARIANT g3 PaperNativeBlendedNoise oldBatchSummary#variant2 kind=old med=51.5 µs kept-primary=51.4 µs policy=min-of-medians
* DUP-VARIANT g4 PaperNativeCaveCarverSkip directHelperSummary#variant2 kind=alt med=34.4 ms kept-primary=34.3 ms policy=min-of-medians
* DUP-VARIANT g4 PaperNativeCaveCarverSkip oldLambdaSummary#variant2 kind=old med=34.5 ms kept-primary=34.2 ms policy=min-of-medians
* DUP-VARIANT g4 PaperNativeCaveCarverSkip reusedCheckerSummary#variant2 kind=alt med=34.4 ms kept-primary=34.2 ms policy=min-of-medians
* DUP-VARIANT g5 PaperNativeChunkDependencies arraySummary#variant2 kind=alt med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g5 PaperNativeChunkDependencies oldImmutableListSummary#variant2 kind=old med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g6 PaperNativeClimateParameterDistance branchDistanceSum#variant2 kind=alt med=534.0 ns kept-primary=525.4 ns policy=min-of-medians
* DUP-VARIANT g6 PaperNativeClimateParameterDistance oldDistanceSum#variant2 kind=old med=534.3 ns kept-primary=524.4 ns policy=min-of-medians
* DUP-VARIANT g6 PaperNativeClimateParameterDistance subtractFirstDistanceSum#variant2 kind=alt med=536.4 ns kept-primary=524.2 ns policy=min-of-medians
* DUP-VARIANT g7 PaperNativeCubicSplineCreate indexSummary#variant2 kind=alt med=153.6 µs kept-primary=153.2 µs policy=min-of-medians
* DUP-VARIANT g7 PaperNativeCubicSplineCreate oldIteratorSummary#variant2 kind=old med=153.4 µs kept-primary=152.8 µs policy=min-of-medians
* DUP-VARIANT g8 PaperNativeDensityAp2Fill oldFlatSummary#variant2 kind=old med=13.0 µs kept-primary=13.0 µs policy=min-of-medians
* DUP-VARIANT g8 PaperNativeDensityAp2Fill oldNestedSummary#variant2 kind=old med=22.2 µs kept-primary=21.9 µs policy=min-of-medians
* DUP-VARIANT g8 PaperNativeDensityAp2Fill scratchFlatSummary#variant2 kind=alt med=11.3 µs kept-primary=11.3 µs policy=min-of-medians
* DUP-VARIANT g8 PaperNativeDensityAp2Fill scratchNestedSummary#variant2 kind=alt med=19.7 µs kept-primary=19.7 µs policy=min-of-medians
* DUP-VARIANT g9 PaperNativeDensityAp2MinMaxFill newSummary#variant2 kind=alt med=119.3 ns kept-primary=118.9 ns policy=min-of-medians
* DUP-VARIANT g9 PaperNativeDensityAp2MinMaxFill oldSummary#variant2 kind=old med=118.9 ns kept-primary=118.7 ns policy=min-of-medians
* DUP-VARIANT g10 PaperNativeDensitySplineContext newDirectSummary#variant2 kind=alt med=1.9 µs kept-primary=1.9 µs policy=min-of-medians
* DUP-VARIANT g10 PaperNativeDensitySplineContext oldWrapperSummary#variant2 kind=old med=1.9 µs kept-primary=1.9 µs policy=min-of-medians
* DUP-VARIANT g11 PaperNativeDensityVisitorHook hookedUnwrappingSummary#variant2 kind=alt med=677.5 ns kept-primary=676.2 ns policy=min-of-medians
* DUP-VARIANT g11 PaperNativeDensityVisitorHook oldUnwrappingSummary#variant2 kind=old med=676.8 ns kept-primary=672.4 ns policy=min-of-medians
* DUP-VARIANT g12 PaperNativeEntityBoundingBox directDimensionsSetSummary#variant2 kind=alt med=5.8 µs kept-primary=5.8 µs policy=min-of-medians
* DUP-VARIANT g12 PaperNativeEntityBoundingBox oldMakeThenSetSummary#variant2 kind=old med=5.7 µs kept-primary=5.7 µs policy=min-of-medians
* DUP-VARIANT g13 PaperNativeEntityChunkTransient newMixedSummary#variant2 kind=alt med=291.2 ns kept-primary=291.1 ns policy=min-of-medians
* DUP-VARIANT g13 PaperNativeEntityChunkTransient oldMixedSummary#variant2 kind=old med=293.4 ns kept-primary=292.9 ns policy=min-of-medians
* DUP-VARIANT g14 PaperNativeEntityLookupStatus directAccessibleSummary#variant2 kind=alt med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g14 PaperNativeEntityLookupStatus directStatusSummary#variant2 kind=alt med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g14 PaperNativeEntityLookupStatus oldAccessibleSummary#variant2 kind=old med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g14 PaperNativeEntityLookupStatus oldStatusSummary#variant2 kind=old med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g15 PaperNativeImprovedNoiseDerivative flatGradientDerivativeSummary#variant2 kind=alt med=11.6 µs kept-primary=11.5 µs policy=min-of-medians
* DUP-VARIANT g15 PaperNativeImprovedNoiseDerivative inlineDerivativeSummary#variant2 kind=alt med=11.7 µs kept-primary=11.7 µs policy=min-of-medians
* DUP-VARIANT g15 PaperNativeImprovedNoiseDerivative intTableDerivativeSummary#variant2 kind=alt med=11.7 µs kept-primary=11.7 µs policy=min-of-medians
* DUP-VARIANT g15 PaperNativeImprovedNoiseDerivative oldDerivativeSummary#variant2 kind=old med=11.7 µs kept-primary=11.7 µs policy=min-of-medians
* DUP-VARIANT g16 PaperNativeImprovedNoiseInline arithmeticSummary#variant2 kind=alt med=8.0 µs kept-primary=8.0 µs policy=min-of-medians
* DUP-VARIANT g16 PaperNativeImprovedNoiseInline flatGradientSummary#variant2 kind=alt med=8.1 µs kept-primary=8.1 µs policy=min-of-medians
* DUP-VARIANT g16 PaperNativeImprovedNoiseInline inlineByteAccessSummary#variant2 kind=alt med=9.3 µs kept-primary=9.3 µs policy=min-of-medians
* DUP-VARIANT g16 PaperNativeImprovedNoiseInline oldPMethodSummary#variant2 kind=old med=9.3 µs kept-primary=9.3 µs policy=min-of-medians
* DUP-VARIANT g16 PaperNativeImprovedNoiseInline switchGradientSummary#variant2 kind=alt med=7.7 µs kept-primary=7.6 µs policy=min-of-medians
* DUP-VARIANT g17 PaperNativeJigsawCanAttach oldBatchSummary#variant2 kind=old med=627.4 ns kept-primary=625.6 ns policy=min-of-medians
* DUP-VARIANT g17 PaperNativeJigsawCanAttach optimizedBatchSummary#variant2 kind=alt med=628.1 ns kept-primary=626.8 ns policy=min-of-medians
* DUP-VARIANT g17 PaperNativeJigsawCanAttach targetFirstBatchSummary#variant2 kind=alt med=627.3 ns kept-primary=626.3 ns policy=min-of-medians
* DUP-VARIANT g18 PaperNativeLegacyProvidedAliasRemoval newReverseAliasRemoveSummary#variant2 kind=alt med=88.8 ns kept-primary=88.1 ns policy=min-of-medians
* DUP-VARIANT g18 PaperNativeLegacyProvidedAliasRemoval oldValuesRemoveIfSummary#variant2 kind=old med=88.3 ns kept-primary=88.2 ns policy=min-of-medians
* DUP-VARIANT g19 PaperNativeLevelChunkHeightmap newCombinedUpdateSummary#variant2 kind=alt med=12.4 ms kept-primary=12.4 ms policy=min-of-medians
* DUP-VARIANT g19 PaperNativeLevelChunkHeightmap oldFourUpdateSummary#variant2 kind=old med=2.2 ms kept-primary=2.2 ms policy=min-of-medians
* DUP-VARIANT g20 PaperNativeMarkerCache cachedSummary#variant2 kind=alt med=553.8 µs kept-primary=552.3 µs policy=min-of-medians
* DUP-VARIANT g20 PaperNativeMarkerCache oldSummary#variant2 kind=old med=119.9 µs kept-primary=118.7 µs policy=min-of-medians
* DUP-VARIANT g21 PaperNativeNoiseChunkBlendCache newEmptyBlenderSummary#variant2 kind=alt med=232.5 ns kept-primary=232.4 ns policy=min-of-medians
* DUP-VARIANT g21 PaperNativeNoiseChunkBlendCache oldEmptyBlenderSummary#variant2 kind=old med=74.2 µs kept-primary=73.1 µs policy=min-of-medians
* DUP-VARIANT g22 PaperNativeNoiseChunkFlatCacheContext newFalseContextSummary#variant2 kind=alt med=18.8 µs kept-primary=18.7 µs policy=min-of-medians
* DUP-VARIANT g22 PaperNativeNoiseChunkFlatCacheContext newTrueContextSummary#variant2 kind=alt med=19.1 µs kept-primary=18.8 µs policy=min-of-medians
* DUP-VARIANT g22 PaperNativeNoiseChunkFlatCacheContext oldFalseContextSummary#variant2 kind=old med=21.9 µs kept-primary=21.8 µs policy=min-of-medians
* DUP-VARIANT g22 PaperNativeNoiseChunkFlatCacheContext oldTrueContextSummary#variant2 kind=old med=23.6 µs kept-primary=23.6 µs policy=min-of-medians
* DUP-VARIANT g23 PaperNativeNoiseInterpolatorSlice flatSummary#variant2 kind=alt med=1.9 ms kept-primary=1.8 ms policy=min-of-medians
* DUP-VARIANT g23 PaperNativeNoiseInterpolatorSlice oldJaggedSummary#variant2 kind=old med=6.2 ms kept-primary=6.1 ms policy=min-of-medians
* DUP-VARIANT g24 PaperNativeObfHelperMaps directMapsSummary#variant2 kind=alt med=93.6 ns kept-primary=93.3 ns policy=min-of-medians
* DUP-VARIANT g24 PaperNativeObfHelperMaps oldStreamDefaultSummary#variant2 kind=old med=93.5 ns kept-primary=93.5 ns policy=min-of-medians
* DUP-VARIANT g24 PaperNativeObfHelperMaps presizedStringPoolSummary#variant2 kind=alt med=94.4 ns kept-primary=93.5 ns policy=min-of-medians
* DUP-VARIANT g25 PaperNativeOreFeatureLoop oldLoopSummary#variant2 kind=old med=1.9 µs kept-primary=1.9 µs policy=min-of-medians
* DUP-VARIANT g25 PaperNativeOreFeatureLoop optimizedLoopSummary#variant2 kind=alt med=1.9 µs kept-primary=1.9 µs policy=min-of-medians
* DUP-VARIANT g26 PaperNativeOwnableRule newLoopSummary#variant2 kind=alt med=3.7 µs kept-primary=3.7 µs policy=min-of-medians
* DUP-VARIANT g26 PaperNativeOwnableRule oldStreamSummary#variant2 kind=old med=4.2 µs kept-primary=4.2 µs policy=min-of-medians
* DUP-VARIANT g27 PaperNativePalettedReencodeScratch directPackedSummary#variant2 kind=alt med=1.1 ms kept-primary=1.1 ms policy=min-of-medians
* DUP-VARIANT g27 PaperNativePalettedReencodeScratch oldNewArraySummary#variant2 kind=old med=492.2 µs kept-primary=489.5 µs policy=min-of-medians
* DUP-VARIANT g27 PaperNativePalettedReencodeScratch scratchThreadLocalSummary#variant2 kind=alt med=412.9 µs kept-primary=411.6 µs policy=min-of-medians
* DUP-VARIANT g28 PaperNativePluginClassLoaderGroup oldLookupSummary#variant2 kind=old med=92.0 ns kept-primary=91.3 ns policy=min-of-medians
* DUP-VARIANT g28 PaperNativePluginClassLoaderGroup skipRequesterSummary#variant2 kind=alt med=92.4 ns kept-primary=91.9 ns policy=min-of-medians
* DUP-VARIANT g29 PaperNativePluginDirectoryScan directoryStreamSummary#variant2 kind=alt med=1.3 ms kept-primary=1.3 ms policy=min-of-medians
* DUP-VARIANT g29 PaperNativePluginDirectoryScan newListSummary#variant2 kind=alt med=1.3 ms kept-primary=1.3 ms policy=min-of-medians
* DUP-VARIANT g29 PaperNativePluginDirectoryScan oldWalkDepth1Summary#variant2 kind=old med=1.5 ms kept-primary=1.5 ms policy=min-of-medians
* DUP-VARIANT g30 PaperNativePluginLoadingAllocation newLazyMissingSetSummary#variant2 kind=alt med=88.5 ns kept-primary=88.0 ns policy=min-of-medians
* DUP-VARIANT g30 PaperNativePluginLoadingAllocation newLazyValidateSummary#variant2 kind=alt med=88.6 ns kept-primary=88.6 ns policy=min-of-medians
* DUP-VARIANT g30 PaperNativePluginLoadingAllocation newPresizedSetupSummary#variant2 kind=alt med=89.4 ns kept-primary=88.9 ns policy=min-of-medians
* DUP-VARIANT g30 PaperNativePluginLoadingAllocation oldDefaultCapacitySetupSummary#variant2 kind=old med=89.4 ns kept-primary=89.0 ns policy=min-of-medians
* DUP-VARIANT g30 PaperNativePluginLoadingAllocation oldEagerMissingSetSummary#variant2 kind=old med=89.9 ns kept-primary=89.1 ns policy=min-of-medians
* DUP-VARIANT g30 PaperNativePluginLoadingAllocation oldEagerValidateSummary#variant2 kind=old med=91.3 ns kept-primary=89.7 ns policy=min-of-medians
* DUP-VARIANT g31 PaperNativePluginMetaDependency cachedSummary#variant2 kind=alt med=92.3 ns kept-primary=91.7 ns policy=min-of-medians
* DUP-VARIANT g31 PaperNativePluginMetaDependency newLoopSummary#variant2 kind=alt med=92.3 ns kept-primary=92.2 ns policy=min-of-medians
* DUP-VARIANT g31 PaperNativePluginMetaDependency oldStreamSummary#variant2 kind=old med=92.6 ns kept-primary=92.0 ns policy=min-of-medians
* DUP-VARIANT g32 PaperNativePluginNameLog newArrayListSortSummary#variant2 kind=alt med=88.4 ns kept-primary=87.7 ns policy=min-of-medians
* DUP-VARIANT g32 PaperNativePluginNameLog oldTreesetSummary#variant2 kind=old med=88.6 ns kept-primary=88.3 ns policy=min-of-medians
* DUP-VARIANT g33 PaperNativePluginStartupRollup newSummary#variant2 kind=alt med=95.9 ns kept-primary=95.5 ns policy=min-of-medians
* DUP-VARIANT g33 PaperNativePluginStartupRollup oldSummary#variant2 kind=old med=95.8 ns kept-primary=95.3 ns policy=min-of-medians
* DUP-VARIANT g34 PaperNativeProtoChunkHeightmap newCachedContainsSummary#variant2 kind=alt med=2.1 µs kept-primary=2.0 µs policy=min-of-medians
* DUP-VARIANT g34 PaperNativeProtoChunkHeightmap oldEnumSetForeachSummary#variant2 kind=old med=1.2 µs kept-primary=1.2 µs policy=min-of-medians
* DUP-VARIANT g35 PaperNativeRangeChoice oldFillArraySummary#variant2 kind=old med=81.6 ns kept-primary=81.4 ns policy=min-of-medians
* DUP-VARIANT g35 PaperNativeRangeChoice optimizedFillArraySummary#variant2 kind=alt med=81.6 ns kept-primary=81.6 ns policy=min-of-medians
* DUP-VARIANT g36 PaperNativeRemapperIndexCleanup newLazyCleanupSummary#variant2 kind=alt med=91.4 ns kept-primary=91.3 ns policy=min-of-medians
* DUP-VARIANT g36 PaperNativeRemapperIndexCleanup oldEagerCleanupSummary#variant2 kind=old med=91.5 ns kept-primary=91.2 ns policy=min-of-medians
* DUP-VARIANT g37 PaperNativeRemapperSkipHashes newLoopSummary#variant2 kind=alt med=33.2 µs kept-primary=32.8 µs policy=min-of-medians
* DUP-VARIANT g37 PaperNativeRemapperSkipHashes oldStreamSummary#variant2 kind=old med=35.0 µs kept-primary=34.7 µs policy=min-of-medians
* DUP-VARIANT g38 PaperNativeServerEntityDeltaIdentity oldDistanceSummary#variant2 kind=old med=2.2 µs kept-primary=2.2 µs policy=min-of-medians
* DUP-VARIANT g39 PaperNativeSpigotLoadOrderDependency newLoadAfterBuildSummary#variant2 kind=alt med=87.9 ns kept-primary=87.5 ns policy=min-of-medians
* DUP-VARIANT g39 PaperNativeSpigotLoadOrderDependency oldLoadAfterBuildSummary#variant2 kind=old med=87.8 ns kept-primary=87.6 ns policy=min-of-medians
* DUP-VARIANT g40 PaperNativeSpigotLoadOrderDependency newRemovedCountSummary#variant2 kind=alt med=88.1 ns kept-primary=87.8 ns policy=min-of-medians
* DUP-VARIANT g40 PaperNativeSpigotLoadOrderDependency oldRemovedCountSummary#variant2 kind=old med=88.5 ns kept-primary=88.3 ns policy=min-of-medians
* DUP-VARIANT g41 PaperNativeSpringFeatureMutablePos mutableBatchSummary#variant2 kind=alt med=2.9 µs kept-primary=2.9 µs policy=min-of-medians
* DUP-VARIANT g41 PaperNativeSpringFeatureMutablePos oldBatchSummary#variant2 kind=old med=2.9 µs kept-primary=2.9 µs policy=min-of-medians
* DUP-VARIANT g42 PaperNativeStaticCacheGet newBatchSummary#variant2 kind=alt med=34.6 ns kept-primary=34.5 ns policy=min-of-medians
* DUP-VARIANT g42 PaperNativeStaticCacheGet oldBatchSummary#variant2 kind=old med=34.6 ns kept-primary=34.4 ns policy=min-of-medians
* DUP-VARIANT g43 PaperNativeSurfaceRulesTestRuleState newStateRuleSummary#variant2 kind=alt med=1.2 µs kept-primary=1.2 µs policy=min-of-medians
* DUP-VARIANT g43 PaperNativeSurfaceRulesTestRuleState oldStateRuleSummary#variant2 kind=old med=1.1 µs kept-primary=1.1 µs policy=min-of-medians
* DUP-VARIANT g44 PaperNativeTopographicGraphSortCapacity newPresizedSummary#variant2 kind=alt med=349.5 ns kept-primary=349.3 ns policy=min-of-medians
* DUP-VARIANT g44 PaperNativeTopographicGraphSortCapacity oldDefaultCapacitySummary#variant2 kind=old med=350.1 ns kept-primary=349.2 ns policy=min-of-medians
* DUP-VARIANT g45 PaperNativeWaypointDistanceGuard guardedAtOrBeyondRangeSummary#variant2 kind=alt med=9.5 µs kept-primary=9.5 µs policy=min-of-medians
* DUP-VARIANT g45 PaperNativeWaypointDistanceGuard oldAtOrBeyondRangeSummary#variant2 kind=old med=9.6 µs kept-primary=9.6 µs policy=min-of-medians
* DUP-VARIANT g46 PaperNativeWaypointDistanceGuard guardedReallyFarSummary#variant2 kind=alt med=8.4 µs kept-primary=8.4 µs policy=min-of-medians
* DUP-VARIANT g46 PaperNativeWaypointDistanceGuard oldReallyFarSummary#variant2 kind=old med=8.3 µs kept-primary=8.3 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath cachedChunkVisibleValue#variant2 kind=alt med=3.1 µs kept-primary=3.0 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath directAzimuthValue#variant2 kind=alt med=2.1 µs kept-primary=2.1 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath guardedAtOrBeyondRangeValue#variant2 kind=alt med=1.2 µs kept-primary=1.2 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath guardedReallyFarValue#variant2 kind=alt med=1.0 µs kept-primary=1.0 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath oldAtOrBeyondRangeValue#variant2 kind=old med=1.2 µs kept-primary=1.2 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath oldAzimuthValue#variant2 kind=old med=2.1 µs kept-primary=2.1 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath oldChunkVisibleValue#variant2 kind=old med=3.4 µs kept-primary=3.4 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath oldReallyFarValue#variant2 kind=old med=947.1 ns kept-primary=944.2 ns policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath oldWaypointManagerValue#variant2 kind=old med=99.9 µs kept-primary=98.9 µs policy=min-of-medians
* DUP-VARIANT g47 PaperNativeWaypointHotPath optimizedWaypointManagerValue#variant2 kind=alt med=93.7 µs kept-primary=92.4 µs policy=min-of-medians
* DUP-VARIANT g48 PaperNativeXoroshiroPositionalDirect directDoubleBatchSummary#variant2 kind=alt med=1.6 µs kept-primary=1.5 µs policy=min-of-medians
* DUP-VARIANT g48 PaperNativeXoroshiroPositionalDirect directFloatBatchSummary#variant2 kind=alt med=1.6 µs kept-primary=1.6 µs policy=min-of-medians
* DUP-VARIANT g48 PaperNativeXoroshiroPositionalDirect oldDoubleBatchSummary#variant2 kind=old med=1.6 µs kept-primary=1.6 µs policy=min-of-medians
* DUP-VARIANT g48 PaperNativeXoroshiroPositionalDirect oldFloatBatchSummary#variant2 kind=old med=1.6 µs kept-primary=1.6 µs policy=min-of-medians

## Pairing diagnostics (P500 stem rule)

An alt kernel pairs with the old kernel whose stem (name after stripping one leading kind prefix: old/new/direct/cached/scratch/reused/lazy/index/branch/array/helper/lambda/…) shares the longest common suffix. Cross-stem comparisons are refused.

**UNPAIRED kernels** (excluded from old-vs-alt verdicts; grep `UNPAIRED g<gid>` / use --strict to gate):

* UNPAIRED g38 PaperNativeServerEntityDeltaIdentity oldDistanceSummary kind=old reason="no OK alt kernel in group"

**MULTI-pair warnings** (one old claimed by several alts, or ambiguous ties; grep `MULTI-PAIR g<gid>`):

* MULTI-PAIR g4 PaperNativeCaveCarverSkip type=many-to-one old=oldLambdaSummary alts=2 directHelperSummary,reusedCheckerSummary
* MULTI-PAIR g6 PaperNativeClimateParameterDistance type=many-to-one old=oldDistanceSum alts=2 branchDistanceSum,subtractFirstDistanceSum
* MULTI-PAIR g15 PaperNativeImprovedNoiseDerivative type=many-to-one old=oldDerivativeSummary alts=3 flatGradientDerivativeSummary,inlineDerivativeSummary,intTableDerivativeSummary
* MULTI-PAIR g16 PaperNativeImprovedNoiseInline type=many-to-one old=oldPMethodSummary alts=4 arithmeticSummary,flatGradientSummary,inlineByteAccessSummary,switchGradientSummary
* MULTI-PAIR g17 PaperNativeJigsawCanAttach type=many-to-one old=oldBatchSummary alts=2 optimizedBatchSummary,targetFirstBatchSummary
* MULTI-PAIR g24 PaperNativeObfHelperMaps type=many-to-one old=oldStreamDefaultSummary alts=2 directMapsSummary,presizedStringPoolSummary
* MULTI-PAIR g27 PaperNativePalettedReencodeScratch type=many-to-one old=oldNewArraySummary alts=2 directPackedSummary,scratchThreadLocalSummary
* MULTI-PAIR g29 PaperNativePluginDirectoryScan type=many-to-one old=oldWalkDepth1Summary alts=2 directoryStreamSummary,newListSummary
* MULTI-PAIR g31 PaperNativePluginMetaDependency type=many-to-one old=oldStreamSummary alts=2 cachedSummary,newLoopSummary

## Top wins (optimized faster than old)

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 314.45x | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 73.1 µs | 232.4 ns |
| 3.34x | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.1 ms | 1.8 ms |
| 1.25x | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.6 µs | 18.8 µs |
| 1.22x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs |
| 1.19x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 489.5 µs | 411.6 µs |
| 1.17x | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 21.8 µs | 18.7 µs |
| 1.16x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `arithmeticSummary` | 9.3 µs | 8.0 µs |
| 1.15x | `PaperNativeDensityAp2Fill` | `oldFlatSummary` | `scratchFlatSummary` | 13.0 µs | 11.3 µs |

## Regressions (optimized SLOWER than old) — optimization targets

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 0.57x | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` | `newCachedContainsSummary` | 1.2 µs | 2.0 µs |
| 0.44x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `directPackedSummary` | 489.5 µs | 1.1 ms |
| 0.21x | `PaperNativeMarkerCache` | `oldSummary` | `cachedSummary` | 118.7 µs | 552.3 µs |
| 0.18x | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` | `newCombinedUpdateSummary` | 2.2 ms | 12.4 ms |

## Parity (within ±15%)

`PaperNativeImprovedNoiseInline.flatGradientSummary` (1.14x), `PaperNativeOwnableRule.newLoopSummary` (1.13x), `PaperNativeWaypointHotPath.cachedChunkVisibleValue` (1.12x), `PaperNativeDensityAp2Fill.scratchNestedSummary` (1.11x), `PaperNativeAquiferSurfaceSampling.newBatchSummary` (1.11x), `PaperNativePluginDirectoryScan.newListSummary` (1.10x), `PaperNativePluginDirectoryScan.directoryStreamSummary` (1.10x), `PaperNativeAquiferIndexStride.newBatchSummary` (1.07x), `PaperNativeWaypointHotPath.optimizedWaypointManagerValue` (1.07x), `PaperNativeRemapperSkipHashes.newLoopSummary` (1.06x), `PaperNativeChunkDependencies.arraySummary` (1.03x), `PaperNativeBlendedNoise.cachedBatchSummary` (1.02x), `PaperNativeImprovedNoiseDerivative.flatGradientDerivativeSummary` (1.01x), `PaperNativePluginLoadingAllocation.newLazyMissingSetSummary` (1.01x), `PaperNativePluginLoadingAllocation.newLazyValidateSummary` (1.01x), `PaperNativeWaypointDistanceGuard.guardedAtOrBeyondRangeSummary` (1.01x), `PaperNativePluginNameLog.newArrayListSortSummary` (1.01x), `PaperNativeEntityChunkTransient.newMixedSummary` (1.01x), `PaperNativeAquiferPositionalLocation.directBatchSummary` (1.01x), `PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary` (1.01x), `PaperNativeXoroshiroPositionalDirect.directDoubleBatchSummary` (1.00x), `PaperNativePluginMetaDependency.cachedSummary` (1.00x), `PaperNativeEntityLookupStatus.directAccessibleSummary` (1.00x), `PaperNativeOreFeatureLoop.optimizedLoopSummary` (1.00x), `PaperNativeWaypointHotPath.directAzimuthValue` (1.00x), `PaperNativeObfHelperMaps.directMapsSummary` (1.00x), `PaperNativeCaveCarverSkip.reusedCheckerSummary` (1.00x), `PaperNativeXoroshiroPositionalDirect.directFloatBatchSummary` (1.00x), `PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary` (1.00x), `PaperNativeLegacyProvidedAliasRemoval.newReverseAliasRemoveSummary` (1.00x), `PaperNativePluginLoadingAllocation.newPresizedSetupSummary` (1.00x), `PaperNativeImprovedNoiseInline.inlineByteAccessSummary` (1.00x), `PaperNativeClimateParameterDistance.subtractFirstDistanceSum` (1.00x), `PaperNativeImprovedNoiseDerivative.inlineDerivativeSummary` (1.00x), `PaperNativeObfHelperMaps.presizedStringPoolSummary` (1.00x), `PaperNativeTopographicGraphSortCapacity.newPresizedSummary` (1.00x), `PaperNativeImprovedNoiseDerivative.intTableDerivativeSummary` (1.00x), `PaperNativeCaveCarverSkip.directHelperSummary` (1.00x), `PaperNativeRemapperIndexCleanup.newLazyCleanupSummary` (1.00x), `PaperNativeJigsawCanAttach.targetFirstBatchSummary` (1.00x), `PaperNativeDensityAp2MinMaxFill.newSummary` (1.00x), `PaperNativeEntityLookupStatus.directStatusSummary` (1.00x), `PaperNativeClimateParameterDistance.branchDistanceSum` (1.00x), `PaperNativeJigsawCanAttach.optimizedBatchSummary` (1.00x), `PaperNativePluginStartupRollup.newSummary` (1.00x), `PaperNativePluginMetaDependency.newLoopSummary` (1.00x), `PaperNativeRangeChoice.optimizedFillArraySummary` (1.00x), `PaperNativeCubicSplineCreate.indexSummary` (1.00x), `PaperNativeStaticCacheGet.newBatchSummary` (1.00x), `PaperNativeSpringFeatureMutablePos.mutableBatchSummary` (1.00x), `PaperNativeDensityVisitorHook.hookedUnwrappingSummary` (0.99x), `PaperNativeWaypointDistanceGuard.guardedReallyFarSummary` (0.99x), `PaperNativePluginClassLoaderGroup.skipRequesterSummary` (0.99x), `PaperNativeDensitySplineContext.newDirectSummary` (0.99x), `PaperNativeWaypointHotPath.guardedAtOrBeyondRangeValue` (0.98x), `PaperNativeEntityBoundingBox.directDimensionsSetSummary` (0.97x), `PaperNativeSurfaceRulesTestRuleState.newStateRuleSummary` (0.95x), `PaperNativeWaypointHotPath.guardedReallyFarValue` (0.94x)

## Regressions (do-not-wire)

ratio = alt/old ≥ 1.18 — beyond the ±15% noise floor. Do NOT wire these optimized variants into hot paths. '(noisy)' = pair stability > 50%.

| kernel (alt / paired old) | group | ratio | stability | verdict |
|---|---|---:|---:|---|
| `newCombinedUpdateSummary` / `oldFourUpdateSummary` | PaperNativeLevelChunkHeightmap (g19) | 5.621 | 0.0% | REGRESSION |
| `cachedSummary` / `oldSummary` | PaperNativeMarkerCache (g20) | 4.655 | 0.0% | REGRESSION |
| `directPackedSummary` / `oldNewArraySummary` | PaperNativePalettedReencodeScratch (g27) | 2.294 | 0.0% | REGRESSION |
| `newCachedContainsSummary` / `oldEnumSetForeachSummary` | PaperNativeProtoChunkHeightmap (g34) | 1.760 | 0.0% | REGRESSION |

## Wins (promotion candidates)

ratio = alt/old ≤ 0.85 (≥ ~1.18x speedup), sorted best-first. Candidates for engine hot paths; check stability before promoting.

| ratio | class | old kernel | alt kernel | old | alt | stability | verdict |
|---:|---|---|---|---:|---:|---:|---|
| 0.003 | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 73.1 µs | 232.4 ns | 0.0% | WIN |
| 0.300 | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.1 ms | 1.8 ms | 0.0% | WIN |
| 0.799 | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.6 µs | 18.8 µs | 0.0% | WIN |
| 0.821 | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs | 0.0% | WIN |
| 0.841 | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 489.5 µs | 411.6 µs | 0.0% | WIN |

## JNI floor groups

Groups with at least one kernel median below 200 ns sit on the JNI-transition floor (canon 35–90 ns; anchors: StaticCacheGet 34.6 ns, RangeChoice 81.4 ns, (I)D @N=1 19.9–31.2 ns — TASK-33 canon; an earlier revision of this line cited '~115 ns measured in the scaling study', a pre-BATCH_NS-audit artifact, commit 3baa0f7): per-kernel micro-optimization is pointless there — batch more work per JNI call instead (engine-level batch-API candidates).

| group | class | fastest median | kernels below 200 ns |
|---|---|---:|---|
| g9 | `PaperNativeDensityAp2MinMaxFill` | 118.7 ns | `oldSummary` (118.7 ns), `newSummary` (118.9 ns) |
| g18 | `PaperNativeLegacyProvidedAliasRemoval` | 88.1 ns | `newReverseAliasRemoveSummary` (88.1 ns), `oldValuesRemoveIfSummary` (88.2 ns) |
| g24 | `PaperNativeObfHelperMaps` | 93.3 ns | `directMapsSummary` (93.3 ns), `oldStreamDefaultSummary` (93.5 ns), `presizedStringPoolSummary` (93.5 ns) |
| g28 | `PaperNativePluginClassLoaderGroup` | 91.3 ns | `oldLookupSummary` (91.3 ns), `skipRequesterSummary` (91.9 ns) |
| g30 | `PaperNativePluginLoadingAllocation` | 88.0 ns | `newLazyMissingSetSummary` (88.0 ns), `newLazyValidateSummary` (88.6 ns), `newPresizedSetupSummary` (88.9 ns), `oldDefaultCapacitySetupSummary` (89.0 ns), `oldEagerMissingSetSummary` (89.1 ns), `oldEagerValidateSummary` (89.7 ns) |
| g31 | `PaperNativePluginMetaDependency` | 91.7 ns | `cachedSummary` (91.7 ns), `oldStreamSummary` (92.0 ns), `newLoopSummary` (92.2 ns) |
| g32 | `PaperNativePluginNameLog` | 87.7 ns | `newArrayListSortSummary` (87.7 ns), `oldTreesetSummary` (88.3 ns) |
| g33 | `PaperNativePluginStartupRollup` | 95.3 ns | `oldSummary` (95.3 ns), `newSummary` (95.5 ns) |
| g35 | `PaperNativeRangeChoice` | 81.4 ns | `oldFillArraySummary` (81.4 ns), `optimizedFillArraySummary` (81.6 ns) |
| g36 | `PaperNativeRemapperIndexCleanup` | 91.2 ns | `oldEagerCleanupSummary` (91.2 ns), `newLazyCleanupSummary` (91.3 ns) |
| g39 | `PaperNativeSpigotLoadOrderDependency` | 87.5 ns | `newLoadAfterBuildSummary` (87.5 ns), `oldLoadAfterBuildSummary` (87.6 ns) |
| g40 | `PaperNativeSpigotLoadOrderDependency` | 87.8 ns | `newRemovedCountSummary` (87.8 ns), `oldRemovedCountSummary` (88.3 ns) |
| g42 | `PaperNativeStaticCacheGet` | 34.4 ns | `oldBatchSummary` (34.4 ns), `newBatchSummary` (34.5 ns) |

## Baseline diff vs `baseline.tsv`

Per-pair ratio change: drift = (current ratio − baseline ratio) / baseline ratio; flagged when |drift| > 20%.

| class | pair (old / alt) | baseline ratio | current ratio | drift | flag |
|---|---|---:|---:|---:|---|
| `PaperNativeAquiferIndexStride` | `oldBatchSummary` / `newBatchSummary` | 0.933 | 0.935 | +0.2% | ok |
| `PaperNativeAquiferPositionalLocation` | `oldBatchSummary` / `directBatchSummary` | 0.993 | 0.994 | +0.1% | ok |
| `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` / `newBatchSummary` | 0.906 | 0.902 | -0.5% | ok |
| `PaperNativeBlendedNoise` | `oldBatchSummary` / `cachedBatchSummary` | 0.980 | 0.983 | +0.3% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `directHelperSummary` | 0.987 | 1.001 | +1.4% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `reusedCheckerSummary` | 0.997 | 0.998 | +0.1% | ok |
| `PaperNativeChunkDependencies` | `oldImmutableListSummary` / `arraySummary` | 0.972 | 0.968 | -0.4% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `branchDistanceSum` | 1.008 | 1.002 | -0.6% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `subtractFirstDistanceSum` | 1.005 | 1.000 | -0.5% | ok |
| `PaperNativeCubicSplineCreate` | `oldIteratorSummary` / `indexSummary` | 1.001 | 1.003 | +0.2% | ok |
| `PaperNativeDensityAp2Fill` | `oldFlatSummary` / `scratchFlatSummary` | 0.869 | 0.868 | -0.1% | ok |
| `PaperNativeDensityAp2Fill` | `oldNestedSummary` / `scratchNestedSummary` | 0.888 | 0.898 | +1.2% | ok |
| `PaperNativeDensityAp2MinMaxFill` | `oldSummary` / `newSummary` | 1.003 | 1.002 | -0.2% | ok |
| `PaperNativeDensitySplineContext` | `oldWrapperSummary` / `newDirectSummary` | 1.004 | 1.007 | +0.4% | ok |
| `PaperNativeDensityVisitorHook` | `oldUnwrappingSummary` / `hookedUnwrappingSummary` | 1.009 | 1.006 | -0.4% | ok |
| `PaperNativeEntityBoundingBox` | `oldMakeThenSetSummary` / `directDimensionsSetSummary` | 1.029 | 1.029 | -0.1% | ok |
| `PaperNativeEntityChunkTransient` | `oldMixedSummary` / `newMixedSummary` | 0.993 | 0.994 | +0.1% | ok |
| `PaperNativeEntityLookupStatus` | `oldAccessibleSummary` / `directAccessibleSummary` | 0.999 | 0.997 | -0.1% | ok |
| `PaperNativeEntityLookupStatus` | `oldStatusSummary` / `directStatusSummary` | 0.999 | 1.002 | +0.3% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `flatGradientDerivativeSummary` | 0.985 | 0.988 | +0.2% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `inlineDerivativeSummary` | 1.000 | 1.000 | -0.1% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `intTableDerivativeSummary` | 0.996 | 1.001 | +0.5% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `arithmeticSummary` | 0.864 | 0.859 | -0.6% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `flatGradientSummary` | 0.878 | 0.874 | -0.4% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `inlineByteAccessSummary` | 1.001 | 0.999 | -0.2% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `switchGradientSummary` | 0.819 | 0.821 | +0.2% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `optimizedBatchSummary` | 0.999 | 1.002 | +0.3% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `targetFirstBatchSummary` | 0.997 | 1.001 | +0.4% | ok |
| `PaperNativeLegacyProvidedAliasRemoval` | `oldValuesRemoveIfSummary` / `newReverseAliasRemoveSummary` | 1.048 | 0.999 | -4.7% | ok |
| `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` / `newCombinedUpdateSummary` | 5.700 | 5.621 | -1.4% | ok |
| `PaperNativeMarkerCache` | `oldSummary` / `cachedSummary` | 4.540 | 4.655 | +2.5% | ok |
| `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` / `newEmptyBlenderSummary` | 0.003 | 0.003 | +0.6% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` / `newFalseContextSummary` | 0.846 | 0.856 | +1.2% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` / `newTrueContextSummary` | 0.805 | 0.799 | -0.8% | ok |
| `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` / `flatSummary` | 0.301 | 0.300 | -0.4% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `directMapsSummary` | 0.998 | 0.998 | -0.0% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `presizedStringPoolSummary` | 1.001 | 1.000 | -0.1% | ok |
| `PaperNativeOreFeatureLoop` | `oldLoopSummary` / `optimizedLoopSummary` | 0.996 | 0.998 | +0.2% | ok |
| `PaperNativeOwnableRule` | `oldStreamSummary` / `newLoopSummary` | 0.894 | 0.883 | -1.1% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `directPackedSummary` | 2.346 | 2.294 | -2.2% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `scratchThreadLocalSummary` | 0.834 | 0.841 | +0.8% | ok |
| `PaperNativePluginClassLoaderGroup` | `oldLookupSummary` / `skipRequesterSummary` | 1.008 | 1.007 | -0.1% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `directoryStreamSummary` | 0.921 | 0.906 | -1.6% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `newListSummary` | 0.939 | 0.906 | -3.6% | ok |
| `PaperNativePluginLoadingAllocation` | `oldDefaultCapacitySetupSummary` / `newPresizedSetupSummary` | 0.970 | 0.999 | +3.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` / `newLazyMissingSetSummary` | 0.996 | 0.988 | -0.8% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` / `newLazyValidateSummary` | 0.993 | 0.988 | -0.5% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `cachedSummary` | 0.974 | 0.997 | +2.4% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `newLoopSummary` | 0.998 | 1.002 | +0.5% | ok |
| `PaperNativePluginNameLog` | `oldTreesetSummary` / `newArrayListSortSummary` | 0.944 | 0.993 | +5.2% | ok |
| `PaperNativePluginStartupRollup` | `oldSummary` / `newSummary` | 0.992 | 1.002 | +1.1% | ok |
| `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` / `newCachedContainsSummary` | 1.775 | 1.760 | -0.9% | ok |
| `PaperNativeRangeChoice` | `oldFillArraySummary` / `optimizedFillArraySummary` | 1.002 | 1.002 | +0.0% | ok |
| `PaperNativeRemapperIndexCleanup` | `oldEagerCleanupSummary` / `newLazyCleanupSummary` | 1.003 | 1.001 | -0.2% | ok |
| `PaperNativeRemapperSkipHashes` | `oldStreamSummary` / `newLoopSummary` | 0.956 | 0.946 | -1.1% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldLoadAfterBuildSummary` / `newLoadAfterBuildSummary` | 1.003 | 0.999 | -0.5% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldRemovedCountSummary` / `newRemovedCountSummary` | 0.996 | 0.994 | -0.1% | ok |
| `PaperNativeSpringFeatureMutablePos` | `oldBatchSummary` / `mutableBatchSummary` | 1.001 | 1.004 | +0.2% | ok |
| `PaperNativeStaticCacheGet` | `oldBatchSummary` / `newBatchSummary` | 0.997 | 1.003 | +0.6% | ok |
| `PaperNativeSurfaceRulesTestRuleState` | `oldStateRuleSummary` / `newStateRuleSummary` | 1.046 | 1.047 | +0.1% | ok |
| `PaperNativeTopographicGraphSortCapacity` | `oldDefaultCapacitySummary` / `newPresizedSummary` | 1.003 | 1.000 | -0.2% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldAtOrBeyondRangeSummary` / `guardedAtOrBeyondRangeSummary` | 0.985 | 0.990 | +0.5% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldReallyFarSummary` / `guardedReallyFarSummary` | 1.011 | 1.006 | -0.5% | ok |
| `PaperNativeWaypointHotPath` | `oldAtOrBeyondRangeValue` / `guardedAtOrBeyondRangeValue` | 1.018 | 1.024 | +0.5% | ok |
| `PaperNativeWaypointHotPath` | `oldAzimuthValue` / `directAzimuthValue` | 0.995 | 0.998 | +0.3% | ok |
| `PaperNativeWaypointHotPath` | `oldChunkVisibleValue` / `cachedChunkVisibleValue` | 0.896 | 0.894 | -0.2% | ok |
| `PaperNativeWaypointHotPath` | `oldReallyFarValue` / `guardedReallyFarValue` | 1.071 | 1.065 | -0.5% | ok |
| `PaperNativeWaypointHotPath` | `oldWaypointManagerValue` / `optimizedWaypointManagerValue` | 0.933 | 0.935 | +0.3% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldDoubleBatchSummary` / `directDoubleBatchSummary` | 1.001 | 0.997 | -0.4% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldFloatBatchSummary` / `directFloatBatchSummary` | 1.000 | 0.998 | -0.1% | ok |

* compared: 70, new pairs: 0, missing: 0

