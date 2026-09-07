# P500 benchmark report — Crussty CE native kernels (old vs optimized)

* Generated: 2026-09-07T16:46:41+00:00 (aggregator v2 — stem pairing, noise-model classification, baseline tracking)
* Raw data: `p500_raw.tsv` — one JVM fork per group, time-bounded batches (~120 ms), median of 5, identical synthesized args per group.
* Hardware note: shared 2-CPU sandbox; treat <±15% deltas as parity.
* Noise model: ratio = alt/old — **WIN** ≤ 0.85, **REGRESSION** ≥ 1.18, PARITY in between; repeated measurements of a kernel collapse via min-of-medians.
* stability = relative spread (max−min)/median of the underlying batch samples; for a pair the worse (larger) of the two kernels is shown; `n/a` when unavailable.

* Groups measured: 49, skipped: 0, crashed: 0, kernels measured: 129
* Pairs formed (P500 stem rule): 70 — unpaired kernels: 1, multi-pair warnings: 9
* Baseline diff vs: `baseline.tsv` (drift flagged when |Δratio| > 20%)

## Per-group results

### 0. `PaperNativeAquiferIndexStride` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 9.9 µs | 9.9 µs | 9.9 µs | 0.0% |
| `oldBatchSummary` | old | 10.6 µs | 10.6 µs | 10.6 µs | 0.0% |

### 1. `PaperNativeAquiferPositionalLocation` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directBatchSummary` | alt | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `oldBatchSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |

### 2. `PaperNativeAquiferSurfaceSampling` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 5.5 µs | 5.5 µs | 5.5 µs | 0.0% |
| `oldBatchSummary` | old | 6.0 µs | 6.0 µs | 6.0 µs | 0.0% |

### 3. `PaperNativeBlendedNoise` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedBatchSummary` | alt | 50.9 µs | 50.9 µs | 50.9 µs | 0.0% |
| `oldBatchSummary` | old | 51.9 µs | 51.9 µs | 51.9 µs | 0.0% |

### 4. `PaperNativeCaveCarverSkip` `(I[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directHelperSummary` | alt | 34.2 ms | 34.2 ms | 34.2 ms | 0.0% |
| `reusedCheckerSummary` | alt | 34.5 ms | 34.5 ms | 34.5 ms | 0.0% |
| `oldLambdaSummary` | old | 34.6 ms | 34.6 ms | 34.6 ms | 0.0% |

### 5. `PaperNativeChunkDependencies` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `arraySummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldImmutableListSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |

### 6. `PaperNativeClimateParameterDistance` `([J[J[J[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDistanceSum` | old | 522.8 ns | 522.8 ns | 522.8 ns | 0.0% |
| `subtractFirstDistanceSum` | alt | 525.3 ns | 525.3 ns | 525.3 ns | 0.0% |
| `branchDistanceSum` | alt | 526.8 ns | 526.8 ns | 526.8 ns | 0.0% |

### 7. `PaperNativeCubicSplineCreate` `(I[F[F[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldIteratorSummary` | old | 153.3 µs | 153.3 µs | 153.3 µs | 0.0% |
| `indexSummary` | alt | 153.5 µs | 153.5 µs | 153.5 µs | 0.0% |

### 8. `PaperNativeDensityAp2Fill` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchFlatSummary` | alt | 11.3 µs | 11.3 µs | 11.3 µs | 0.0% |
| `oldFlatSummary` | old | 13.0 µs | 13.0 µs | 13.0 µs | 0.0% |
| `scratchNestedSummary` | alt | 19.7 µs | 19.7 µs | 19.7 µs | 0.0% |
| `oldNestedSummary` | old | 22.2 µs | 22.2 µs | 22.2 µs | 0.0% |

### 9. `PaperNativeDensityAp2MinMaxFill` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 119.8 ns | 119.8 ns | 119.8 ns | 0.0% |
| `newSummary` | alt | 120.2 ns | 120.2 ns | 120.2 ns | 0.0% |

### 10. `PaperNativeDensitySplineContext` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldWrapperSummary` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `newDirectSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |

### 11. `PaperNativeDensityVisitorHook` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldUnwrappingSummary` | old | 672.4 ns | 672.4 ns | 672.4 ns | 0.0% |
| `hookedUnwrappingSummary` | alt | 678.6 ns | 678.6 ns | 678.6 ns | 0.0% |

### 12. `PaperNativeEntityBoundingBox` `(I[F[F[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldMakeThenSetSummary` | old | 5.7 µs | 5.7 µs | 5.7 µs | 0.0% |
| `directDimensionsSetSummary` | alt | 5.8 µs | 5.8 µs | 5.8 µs | 0.0% |

### 13. `PaperNativeEntityChunkTransient` `(IIJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newMixedSummary` | alt | 291.6 ns | 291.6 ns | 291.6 ns | 0.0% |
| `oldMixedSummary` | old | 293.7 ns | 293.7 ns | 293.7 ns | 0.0% |

### 14. `PaperNativeEntityLookupStatus` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directAccessibleSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldAccessibleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `directStatusSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldStatusSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |

### 15. `PaperNativeImprovedNoiseDerivative` `([B[I[I[I[D[D[DI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatGradientDerivativeSummary` | alt | 11.5 µs | 11.5 µs | 11.5 µs | 0.0% |
| `intTableDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `oldDerivativeSummary` | old | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `inlineDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |

### 16. `PaperNativeImprovedNoiseInline` `([BI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `switchGradientSummary` | alt | 7.6 µs | 7.6 µs | 7.6 µs | 0.0% |
| `arithmeticSummary` | alt | 8.0 µs | 8.0 µs | 8.0 µs | 0.0% |
| `flatGradientSummary` | alt | 8.2 µs | 8.2 µs | 8.2 µs | 0.0% |
| `oldPMethodSummary` | old | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |
| `inlineByteAccessSummary` | alt | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |

### 17. `PaperNativeJigsawCanAttach` `(I[I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `targetFirstBatchSummary` | alt | 626.4 ns | 626.4 ns | 626.4 ns | 0.0% |
| `optimizedBatchSummary` | alt | 627.7 ns | 627.7 ns | 627.7 ns | 0.0% |
| `oldBatchSummary` | old | 628.3 ns | 628.3 ns | 628.3 ns | 0.0% |

### 18. `PaperNativeLegacyProvidedAliasRemoval` `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldValuesRemoveIfSummary` | old | 109.6 ns | 109.6 ns | 109.6 ns | 0.0% |
| `newReverseAliasRemoveSummary` | alt | 114.9 ns | 114.9 ns | 114.9 ns | 0.0% |

### 19. `PaperNativeLevelChunkHeightmap` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFourUpdateSummary` | old | 2.8 ms | 2.8 ms | 2.8 ms | 0.0% |
| `newCombinedUpdateSummary` | alt | 15.8 ms | 15.8 ms | 15.8 ms | 0.0% |

### 20. `PaperNativeMarkerCache` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 150.9 µs | 150.9 µs | 150.9 µs | 0.0% |
| `cachedSummary` | alt | 685.0 µs | 685.0 µs | 685.0 µs | 0.0% |

### 21. `PaperNativeNoiseChunkBlendCache` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newEmptyBlenderSummary` | alt | 301.1 ns | 301.1 ns | 301.1 ns | 0.0% |
| `oldEmptyBlenderSummary` | old | 95.3 µs | 95.3 µs | 95.3 µs | 0.0% |

### 22. `PaperNativeNoiseChunkFlatCacheContext` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newFalseContextSummary` | alt | 18.7 µs | 18.7 µs | 18.7 µs | 0.0% |
| `newTrueContextSummary` | alt | 19.0 µs | 19.0 µs | 19.0 µs | 0.0% |
| `oldFalseContextSummary` | old | 22.1 µs | 22.1 µs | 22.1 µs | 0.0% |
| `oldTrueContextSummary` | old | 23.5 µs | 23.5 µs | 23.5 µs | 0.0% |

### 23. `PaperNativeNoiseInterpolatorSlice` `(IIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatSummary` | alt | 1.9 ms | 1.9 ms | 1.9 ms | 0.0% |
| `oldJaggedSummary` | old | 6.3 ms | 6.3 ms | 6.3 ms | 0.0% |

### 24. `PaperNativeObfHelperMaps` `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directMapsSummary` | alt | 93.3 ns | 93.3 ns | 93.3 ns | 0.0% |
| `oldStreamDefaultSummary` | old | 93.5 ns | 93.5 ns | 93.5 ns | 0.0% |
| `presizedStringPoolSummary` | alt | 93.6 ns | 93.6 ns | 93.6 ns | 0.0% |

### 25. `PaperNativeOreFeatureLoop` `([D[D[D[D[I[I[I[I[I[IIIIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `optimizedLoopSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `oldLoopSummary` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |

### 26. `PaperNativeOwnableRule` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 3.7 µs | 3.7 µs | 3.7 µs | 0.0% |
| `oldStreamSummary` | old | 4.2 µs | 4.2 µs | 4.2 µs | 0.0% |

### 27. `PaperNativePalettedReencodeScratch` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchThreadLocalSummary` | alt | 411.9 µs | 411.9 µs | 411.9 µs | 0.0% |
| `oldNewArraySummary` | old | 493.6 µs | 493.6 µs | 493.6 µs | 0.0% |
| `directPackedSummary` | alt | 1.2 ms | 1.2 ms | 1.2 ms | 0.0% |

### 28. `PaperNativePluginClassLoaderGroup` `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLookupSummary` | old | 91.7 ns | 91.7 ns | 91.7 ns | 0.0% |
| `skipRequesterSummary` | alt | 92.4 ns | 92.4 ns | 92.4 ns | 0.0% |

### 29. `PaperNativePluginDirectoryScan` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directoryStreamSummary` | alt | 1.9 ms | 1.9 ms | 1.9 ms | 0.0% |
| `newListSummary` | alt | 1.9 ms | 1.9 ms | 1.9 ms | 0.0% |
| `oldWalkDepth1Summary` | old | 2.1 ms | 2.1 ms | 2.1 ms | 0.0% |

### 30. `PaperNativePluginLoadingAllocation` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newPresizedSetupSummary` | alt | 112.7 ns | 112.7 ns | 112.7 ns | 0.0% |
| `newLazyMissingSetSummary` | alt | 114.6 ns | 114.6 ns | 114.6 ns | 0.0% |
| `oldEagerMissingSetSummary` | old | 115.1 ns | 115.1 ns | 115.1 ns | 0.0% |
| `newLazyValidateSummary` | alt | 115.3 ns | 115.3 ns | 115.3 ns | 0.0% |
| `oldEagerValidateSummary` | old | 116.1 ns | 116.1 ns | 116.1 ns | 0.0% |
| `oldDefaultCapacitySetupSummary` | old | 116.2 ns | 116.2 ns | 116.2 ns | 0.0% |

### 31. `PaperNativePluginMetaDependency` `(I[Ljava/lang/Object;[Z[Z[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedSummary` | alt | 117.8 ns | 117.8 ns | 117.8 ns | 0.0% |
| `newLoopSummary` | alt | 120.7 ns | 120.7 ns | 120.7 ns | 0.0% |
| `oldStreamSummary` | old | 121.0 ns | 121.0 ns | 121.0 ns | 0.0% |

### 32. `PaperNativePluginNameLog` `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newArrayListSortSummary` | alt | 109.0 ns | 109.0 ns | 109.0 ns | 0.0% |
| `oldTreesetSummary` | old | 115.5 ns | 115.5 ns | 115.5 ns | 0.0% |

### 33. `PaperNativePluginStartupRollup` `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newSummary` | alt | 95.3 ns | 95.3 ns | 95.3 ns | 0.0% |
| `oldSummary` | old | 96.1 ns | 96.1 ns | 96.1 ns | 0.0% |

### 34. `PaperNativeProtoChunkHeightmap` `(I)J`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEnumSetForeachSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `newCachedContainsSummary` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |

### 35. `PaperNativeRangeChoice` `([D[I[I[II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFillArraySummary` | old | 81.4 ns | 81.4 ns | 81.4 ns | 0.0% |
| `optimizedFillArraySummary` | alt | 81.6 ns | 81.6 ns | 81.6 ns | 0.0% |

### 36. `PaperNativeRemapperIndexCleanup` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEagerCleanupSummary` | old | 91.3 ns | 91.3 ns | 91.3 ns | 0.0% |
| `newLazyCleanupSummary` | alt | 91.6 ns | 91.6 ns | 91.6 ns | 0.0% |

### 37. `PaperNativeRemapperSkipHashes` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 33.0 µs | 33.0 µs | 33.0 µs | 0.0% |
| `oldStreamSummary` | old | 34.5 µs | 34.5 µs | 34.5 µs | 0.0% |

### 38. `PaperNativeServerEntityDeltaIdentity` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDistanceSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |

### 39. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLoadAfterBuildSummary` | old | 87.7 ns | 87.7 ns | 87.7 ns | 0.0% |
| `newLoadAfterBuildSummary` | alt | 88.0 ns | 88.0 ns | 88.0 ns | 0.0% |

### 40. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newRemovedCountSummary` | alt | 88.6 ns | 88.6 ns | 88.6 ns | 0.0% |
| `oldRemovedCountSummary` | old | 89.0 ns | 89.0 ns | 89.0 ns | 0.0% |

### 41. `PaperNativeSpringFeatureMutablePos` `(I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |
| `mutableBatchSummary` | alt | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |

### 42. `PaperNativeStaticCacheGet` `(IIIII[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 34.6 ns | 34.6 ns | 34.6 ns | 0.0% |
| `oldBatchSummary` | old | 34.7 ns | 34.7 ns | 34.7 ns | 0.0% |

### 43. `PaperNativeSurfaceRulesTestRuleState` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStateRuleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `newStateRuleSummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |

### 44. `PaperNativeTopographicGraphSortCapacity` `(I[I[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDefaultCapacitySummary` | old | 348.1 ns | 348.1 ns | 348.1 ns | 0.0% |
| `newPresizedSummary` | alt | 349.0 ns | 349.0 ns | 349.0 ns | 0.0% |

### 45. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedAtOrBeyondRangeSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs | 0.0% |
| `oldAtOrBeyondRangeSummary` | old | 9.6 µs | 9.6 µs | 9.6 µs | 0.0% |

### 46. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarSummary` | old | 8.4 µs | 8.4 µs | 8.4 µs | 0.0% |
| `guardedReallyFarSummary` | alt | 8.5 µs | 8.5 µs | 8.5 µs | 0.0% |

### 47. `PaperNativeWaypointHotPath` `(I)D`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarValue` | old | 938.4 ns | 938.4 ns | 938.4 ns | 0.0% |
| `guardedReallyFarValue` | alt | 1.0 µs | 1.0 µs | 1.0 µs | 0.0% |
| `oldAtOrBeyondRangeValue` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `guardedAtOrBeyondRangeValue` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `directAzimuthValue` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `oldAzimuthValue` | old | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `cachedChunkVisibleValue` | alt | 3.1 µs | 3.1 µs | 3.1 µs | 0.0% |
| `oldChunkVisibleValue` | old | 3.4 µs | 3.4 µs | 3.4 µs | 0.0% |
| `optimizedWaypointManagerValue` | alt | 93.3 µs | 93.3 µs | 93.3 µs | 0.0% |
| `oldWaypointManagerValue` | old | 100.0 µs | 100.0 µs | 100.0 µs | 0.0% |

### 48. `PaperNativeXoroshiroPositionalDirect` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDoubleBatchSummary` | old | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |
| `directDoubleBatchSummary` | alt | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |
| `directFloatBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldFloatBatchSummary` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |

## Pairing diagnostics (P500 stem rule)

An alt kernel pairs with the old kernel whose stem (name after stripping one leading kind prefix: old/new/direct/cached/scratch/reused/lazy/index/branch/array/helper/lambda/…) shares the longest common suffix. Cross-stem comparisons are refused.

**UNPAIRED kernels** (excluded from old-vs-alt verdicts):

* g38 `PaperNativeServerEntityDeltaIdentity.oldDistanceSummary` (old) — no OK alt kernel in group

**MULTI-pair warnings** (one old claimed by several alts, or ambiguous ties):

* PaperNativeCaveCarverSkip (g4): old kernel `oldLambdaSummary` is the pair for 2 alt kernels: `directHelperSummary` `reusedCheckerSummary`
* PaperNativeClimateParameterDistance (g6): old kernel `oldDistanceSum` is the pair for 2 alt kernels: `branchDistanceSum` `subtractFirstDistanceSum`
* PaperNativeImprovedNoiseDerivative (g15): old kernel `oldDerivativeSummary` is the pair for 3 alt kernels: `flatGradientDerivativeSummary` `inlineDerivativeSummary` `intTableDerivativeSummary`
* PaperNativeImprovedNoiseInline (g16): old kernel `oldPMethodSummary` is the pair for 4 alt kernels: `arithmeticSummary` `flatGradientSummary` `inlineByteAccessSummary` `switchGradientSummary`
* PaperNativeJigsawCanAttach (g17): old kernel `oldBatchSummary` is the pair for 2 alt kernels: `optimizedBatchSummary` `targetFirstBatchSummary`
* PaperNativeObfHelperMaps (g24): old kernel `oldStreamDefaultSummary` is the pair for 2 alt kernels: `directMapsSummary` `presizedStringPoolSummary`
* PaperNativePalettedReencodeScratch (g27): old kernel `oldNewArraySummary` is the pair for 2 alt kernels: `directPackedSummary` `scratchThreadLocalSummary`
* PaperNativePluginDirectoryScan (g29): old kernel `oldWalkDepth1Summary` is the pair for 2 alt kernels: `directoryStreamSummary` `newListSummary`
* PaperNativePluginMetaDependency (g31): old kernel `oldStreamSummary` is the pair for 2 alt kernels: `cachedSummary` `newLoopSummary`

## Top wins (optimized faster than old)

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 316.45x | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 95.3 µs | 301.1 ns |
| 3.32x | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.3 ms | 1.9 ms |
| 1.24x | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.5 µs | 19.0 µs |
| 1.22x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs |
| 1.20x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 493.6 µs | 411.9 µs |
| 1.18x | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 22.1 µs | 18.7 µs |
| 1.16x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `arithmeticSummary` | 9.3 µs | 8.0 µs |
| 1.15x | `PaperNativeDensityAp2Fill` | `oldFlatSummary` | `scratchFlatSummary` | 13.0 µs | 11.3 µs |

## Regressions (optimized SLOWER than old) — optimization targets

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 0.56x | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` | `newCachedContainsSummary` | 1.2 µs | 2.1 µs |
| 0.43x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `directPackedSummary` | 493.6 µs | 1.2 ms |
| 0.22x | `PaperNativeMarkerCache` | `oldSummary` | `cachedSummary` | 150.9 µs | 685.0 µs |
| 0.18x | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` | `newCombinedUpdateSummary` | 2.8 ms | 15.8 ms |

## Parity (within ±15%)

`PaperNativeImprovedNoiseInline.flatGradientSummary` (1.14x), `PaperNativeDensityAp2Fill.scratchNestedSummary` (1.13x), `PaperNativeOwnableRule.newLoopSummary` (1.12x), `PaperNativeWaypointHotPath.cachedChunkVisibleValue` (1.12x), `PaperNativeAquiferSurfaceSampling.newBatchSummary` (1.10x), `PaperNativePluginDirectoryScan.directoryStreamSummary` (1.09x), `PaperNativeWaypointHotPath.optimizedWaypointManagerValue` (1.07x), `PaperNativeAquiferIndexStride.newBatchSummary` (1.07x), `PaperNativePluginDirectoryScan.newListSummary` (1.06x), `PaperNativePluginNameLog.newArrayListSortSummary` (1.06x), `PaperNativeRemapperSkipHashes.newLoopSummary` (1.05x), `PaperNativePluginLoadingAllocation.newPresizedSetupSummary` (1.03x), `PaperNativeChunkDependencies.arraySummary` (1.03x), `PaperNativePluginMetaDependency.cachedSummary` (1.03x), `PaperNativeBlendedNoise.cachedBatchSummary` (1.02x), `PaperNativeWaypointDistanceGuard.guardedAtOrBeyondRangeSummary` (1.02x), `PaperNativeImprovedNoiseDerivative.flatGradientDerivativeSummary` (1.01x), `PaperNativeCaveCarverSkip.directHelperSummary` (1.01x), `PaperNativePluginStartupRollup.newSummary` (1.01x), `PaperNativeEntityChunkTransient.newMixedSummary` (1.01x), `PaperNativePluginLoadingAllocation.newLazyValidateSummary` (1.01x), `PaperNativeAquiferPositionalLocation.directBatchSummary` (1.01x), `PaperNativeWaypointHotPath.directAzimuthValue` (1.01x), `PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary` (1.00x), `PaperNativeOreFeatureLoop.optimizedLoopSummary` (1.00x), `PaperNativePluginLoadingAllocation.newLazyMissingSetSummary` (1.00x), `PaperNativeImprovedNoiseDerivative.intTableDerivativeSummary` (1.00x), `PaperNativeCaveCarverSkip.reusedCheckerSummary` (1.00x), `PaperNativeJigsawCanAttach.targetFirstBatchSummary` (1.00x), `PaperNativeStaticCacheGet.newBatchSummary` (1.00x), `PaperNativePluginMetaDependency.newLoopSummary` (1.00x), `PaperNativeObfHelperMaps.directMapsSummary` (1.00x), `PaperNativeEntityLookupStatus.directAccessibleSummary` (1.00x), `PaperNativeEntityLookupStatus.directStatusSummary` (1.00x), `PaperNativeJigsawCanAttach.optimizedBatchSummary` (1.00x), `PaperNativeXoroshiroPositionalDirect.directFloatBatchSummary` (1.00x), `PaperNativeImprovedNoiseDerivative.inlineDerivativeSummary` (1.00x), `PaperNativeXoroshiroPositionalDirect.directDoubleBatchSummary` (1.00x), `PaperNativeObfHelperMaps.presizedStringPoolSummary` (1.00x), `PaperNativeImprovedNoiseInline.inlineByteAccessSummary` (1.00x), `PaperNativeCubicSplineCreate.indexSummary` (1.00x), `PaperNativeSpringFeatureMutablePos.mutableBatchSummary` (1.00x), `PaperNativeRangeChoice.optimizedFillArraySummary` (1.00x), `PaperNativeTopographicGraphSortCapacity.newPresizedSummary` (1.00x), `PaperNativeRemapperIndexCleanup.newLazyCleanupSummary` (1.00x), `PaperNativeDensityAp2MinMaxFill.newSummary` (1.00x), `PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary` (1.00x), `PaperNativeDensitySplineContext.newDirectSummary` (1.00x), `PaperNativeClimateParameterDistance.subtractFirstDistanceSum` (1.00x), `PaperNativePluginClassLoaderGroup.skipRequesterSummary` (0.99x), `PaperNativeClimateParameterDistance.branchDistanceSum` (0.99x), `PaperNativeDensityVisitorHook.hookedUnwrappingSummary` (0.99x), `PaperNativeWaypointDistanceGuard.guardedReallyFarSummary` (0.99x), `PaperNativeWaypointHotPath.guardedAtOrBeyondRangeValue` (0.98x), `PaperNativeEntityBoundingBox.directDimensionsSetSummary` (0.97x), `PaperNativeSurfaceRulesTestRuleState.newStateRuleSummary` (0.96x), `PaperNativeLegacyProvidedAliasRemoval.newReverseAliasRemoveSummary` (0.95x), `PaperNativeWaypointHotPath.guardedReallyFarValue` (0.93x)

## Regressions (do-not-wire)

ratio = alt/old ≥ 1.18 — beyond the ±15% noise floor. Do NOT wire these optimized variants into hot paths. '(noisy)' = pair stability > 50%.

| kernel (alt / paired old) | group | ratio | stability | verdict |
|---|---|---:|---:|---|
| `newCombinedUpdateSummary` / `oldFourUpdateSummary` | PaperNativeLevelChunkHeightmap (g19) | 5.700 | 0.0% | REGRESSION |
| `cachedSummary` / `oldSummary` | PaperNativeMarkerCache (g20) | 4.540 | 0.0% | REGRESSION |
| `directPackedSummary` / `oldNewArraySummary` | PaperNativePalettedReencodeScratch (g27) | 2.346 | 0.0% | REGRESSION |
| `newCachedContainsSummary` / `oldEnumSetForeachSummary` | PaperNativeProtoChunkHeightmap (g34) | 1.775 | 0.0% | REGRESSION |

## Wins (promotion candidates)

ratio = alt/old ≤ 0.85 (≥ ~1.18x speedup), sorted best-first. Candidates for engine hot paths; check stability before promoting.

| ratio | class | old kernel | alt kernel | old | alt | stability | verdict |
|---:|---|---|---|---:|---:|---:|---|
| 0.003 | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 95.3 µs | 301.1 ns | 0.0% | WIN |
| 0.301 | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.3 ms | 1.9 ms | 0.0% | WIN |
| 0.805 | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.5 µs | 19.0 µs | 0.0% | WIN |
| 0.819 | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs | 0.0% | WIN |
| 0.834 | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 493.6 µs | 411.9 µs | 0.0% | WIN |
| 0.846 | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 22.1 µs | 18.7 µs | 0.0% | WIN |

## JNI floor groups

Groups with at least one kernel median below 200 ns sit on the JNI-transition floor (canon **35–90 ns**: global min `StaticCacheGet` 34.6 ns, floor-resident cluster 81.4–89.0 ns, `(I)D` N=1 19.9–31.2 ns — TASK-33 errata: an earlier revision of this line cited "~115 ns measured in the scaling study", a pre-BATCH_NS-audit artifact, commit `3baa0f7`; counts + batch applicability: `docs/BATCH_ADOPTION_MATRIX.md`): per-kernel micro-optimization is pointless there — batch more work per JNI call instead (engine-level batch-API candidates).

| group | class | fastest median | kernels below 200 ns |
|---|---|---:|---|
| g9 | `PaperNativeDensityAp2MinMaxFill` | 119.8 ns | `oldSummary` (119.8 ns), `newSummary` (120.2 ns) |
| g18 | `PaperNativeLegacyProvidedAliasRemoval` | 109.6 ns | `oldValuesRemoveIfSummary` (109.6 ns), `newReverseAliasRemoveSummary` (114.9 ns) |
| g24 | `PaperNativeObfHelperMaps` | 93.3 ns | `directMapsSummary` (93.3 ns), `oldStreamDefaultSummary` (93.5 ns), `presizedStringPoolSummary` (93.6 ns) |
| g28 | `PaperNativePluginClassLoaderGroup` | 91.7 ns | `oldLookupSummary` (91.7 ns), `skipRequesterSummary` (92.4 ns) |
| g30 | `PaperNativePluginLoadingAllocation` | 112.7 ns | `newPresizedSetupSummary` (112.7 ns), `newLazyMissingSetSummary` (114.6 ns), `oldEagerMissingSetSummary` (115.1 ns), `newLazyValidateSummary` (115.3 ns), `oldEagerValidateSummary` (116.1 ns), `oldDefaultCapacitySetupSummary` (116.2 ns) |
| g31 | `PaperNativePluginMetaDependency` | 117.8 ns | `cachedSummary` (117.8 ns), `newLoopSummary` (120.7 ns), `oldStreamSummary` (121.0 ns) |
| g32 | `PaperNativePluginNameLog` | 109.0 ns | `newArrayListSortSummary` (109.0 ns), `oldTreesetSummary` (115.5 ns) |
| g33 | `PaperNativePluginStartupRollup` | 95.3 ns | `newSummary` (95.3 ns), `oldSummary` (96.1 ns) |
| g35 | `PaperNativeRangeChoice` | 81.4 ns | `oldFillArraySummary` (81.4 ns), `optimizedFillArraySummary` (81.6 ns) |
| g36 | `PaperNativeRemapperIndexCleanup` | 91.3 ns | `oldEagerCleanupSummary` (91.3 ns), `newLazyCleanupSummary` (91.6 ns) |
| g39 | `PaperNativeSpigotLoadOrderDependency` | 87.7 ns | `oldLoadAfterBuildSummary` (87.7 ns), `newLoadAfterBuildSummary` (88.0 ns) |
| g40 | `PaperNativeSpigotLoadOrderDependency` | 88.6 ns | `newRemovedCountSummary` (88.6 ns), `oldRemovedCountSummary` (89.0 ns) |
| g42 | `PaperNativeStaticCacheGet` | 34.6 ns | `newBatchSummary` (34.6 ns), `oldBatchSummary` (34.7 ns) |

## Baseline diff vs `baseline.tsv`

Per-pair ratio change: drift = (current ratio − baseline ratio) / baseline ratio; flagged when |drift| > 20%.

| class | pair (old / alt) | baseline ratio | current ratio | drift | flag |
|---|---|---:|---:|---:|---|
| `PaperNativeAquiferIndexStride` | `oldBatchSummary` / `newBatchSummary` | 0.933 | 0.933 | +0.0% | ok |
| `PaperNativeAquiferPositionalLocation` | `oldBatchSummary` / `directBatchSummary` | 0.993 | 0.993 | -0.0% | ok |
| `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` / `newBatchSummary` | 0.906 | 0.906 | +0.0% | ok |
| `PaperNativeBlendedNoise` | `oldBatchSummary` / `cachedBatchSummary` | 0.980 | 0.980 | +0.0% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `directHelperSummary` | 0.987 | 0.987 | +0.0% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `reusedCheckerSummary` | 0.997 | 0.997 | +0.0% | ok |
| `PaperNativeChunkDependencies` | `oldImmutableListSummary` / `arraySummary` | 0.972 | 0.972 | -0.0% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `branchDistanceSum` | 1.008 | 1.008 | +0.0% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `subtractFirstDistanceSum` | 1.005 | 1.005 | -0.0% | ok |
| `PaperNativeCubicSplineCreate` | `oldIteratorSummary` / `indexSummary` | 1.001 | 1.001 | -0.0% | ok |
| `PaperNativeDensityAp2Fill` | `oldFlatSummary` / `scratchFlatSummary` | 0.869 | 0.869 | +0.0% | ok |
| `PaperNativeDensityAp2Fill` | `oldNestedSummary` / `scratchNestedSummary` | 0.888 | 0.888 | +0.0% | ok |
| `PaperNativeDensityAp2MinMaxFill` | `oldSummary` / `newSummary` | 1.003 | 1.003 | -0.0% | ok |
| `PaperNativeDensitySplineContext` | `oldWrapperSummary` / `newDirectSummary` | 1.004 | 1.004 | +0.0% | ok |
| `PaperNativeDensityVisitorHook` | `oldUnwrappingSummary` / `hookedUnwrappingSummary` | 1.009 | 1.009 | -0.0% | ok |
| `PaperNativeEntityBoundingBox` | `oldMakeThenSetSummary` / `directDimensionsSetSummary` | 1.029 | 1.029 | +0.0% | ok |
| `PaperNativeEntityChunkTransient` | `oldMixedSummary` / `newMixedSummary` | 0.993 | 0.993 | -0.0% | ok |
| `PaperNativeEntityLookupStatus` | `oldAccessibleSummary` / `directAccessibleSummary` | 0.999 | 0.999 | -0.0% | ok |
| `PaperNativeEntityLookupStatus` | `oldStatusSummary` / `directStatusSummary` | 0.999 | 0.999 | +0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `flatGradientDerivativeSummary` | 0.985 | 0.985 | -0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `inlineDerivativeSummary` | 1.000 | 1.000 | +0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `intTableDerivativeSummary` | 0.996 | 0.996 | -0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `arithmeticSummary` | 0.864 | 0.864 | +0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `flatGradientSummary` | 0.878 | 0.878 | -0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `inlineByteAccessSummary` | 1.001 | 1.001 | -0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `switchGradientSummary` | 0.819 | 0.819 | +0.0% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `optimizedBatchSummary` | 0.999 | 0.999 | +0.0% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `targetFirstBatchSummary` | 0.997 | 0.997 | -0.0% | ok |
| `PaperNativeLegacyProvidedAliasRemoval` | `oldValuesRemoveIfSummary` / `newReverseAliasRemoveSummary` | 1.048 | 1.048 | -0.0% | ok |
| `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` / `newCombinedUpdateSummary` | 5.700 | 5.700 | +0.0% | ok |
| `PaperNativeMarkerCache` | `oldSummary` / `cachedSummary` | 4.540 | 4.540 | +0.0% | ok |
| `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` / `newEmptyBlenderSummary` | 0.003 | 0.003 | +0.0% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` / `newFalseContextSummary` | 0.846 | 0.846 | +0.0% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` / `newTrueContextSummary` | 0.805 | 0.805 | +0.0% | ok |
| `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` / `flatSummary` | 0.301 | 0.301 | +0.0% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `directMapsSummary` | 0.998 | 0.998 | -0.0% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `presizedStringPoolSummary` | 1.001 | 1.001 | -0.0% | ok |
| `PaperNativeOreFeatureLoop` | `oldLoopSummary` / `optimizedLoopSummary` | 0.996 | 0.996 | +0.0% | ok |
| `PaperNativeOwnableRule` | `oldStreamSummary` / `newLoopSummary` | 0.894 | 0.894 | +0.0% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `directPackedSummary` | 2.346 | 2.346 | +0.0% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `scratchThreadLocalSummary` | 0.834 | 0.834 | +0.0% | ok |
| `PaperNativePluginClassLoaderGroup` | `oldLookupSummary` / `skipRequesterSummary` | 1.008 | 1.008 | -0.0% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `directoryStreamSummary` | 0.921 | 0.921 | +0.0% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `newListSummary` | 0.939 | 0.939 | +0.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldDefaultCapacitySetupSummary` / `newPresizedSetupSummary` | 0.970 | 0.970 | -0.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` / `newLazyMissingSetSummary` | 0.996 | 0.996 | -0.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` / `newLazyValidateSummary` | 0.993 | 0.993 | +0.0% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `cachedSummary` | 0.974 | 0.974 | -0.0% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `newLoopSummary` | 0.998 | 0.998 | -0.0% | ok |
| `PaperNativePluginNameLog` | `oldTreesetSummary` / `newArrayListSortSummary` | 0.944 | 0.944 | -0.0% | ok |
| `PaperNativePluginStartupRollup` | `oldSummary` / `newSummary` | 0.992 | 0.992 | +0.0% | ok |
| `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` / `newCachedContainsSummary` | 1.775 | 1.775 | +0.0% | ok |
| `PaperNativeRangeChoice` | `oldFillArraySummary` / `optimizedFillArraySummary` | 1.002 | 1.002 | +0.0% | ok |
| `PaperNativeRemapperIndexCleanup` | `oldEagerCleanupSummary` / `newLazyCleanupSummary` | 1.003 | 1.003 | -0.0% | ok |
| `PaperNativeRemapperSkipHashes` | `oldStreamSummary` / `newLoopSummary` | 0.956 | 0.956 | -0.0% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldLoadAfterBuildSummary` / `newLoadAfterBuildSummary` | 1.003 | 1.003 | -0.0% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldRemovedCountSummary` / `newRemovedCountSummary` | 0.996 | 0.996 | -0.0% | ok |
| `PaperNativeSpringFeatureMutablePos` | `oldBatchSummary` / `mutableBatchSummary` | 1.001 | 1.001 | -0.0% | ok |
| `PaperNativeStaticCacheGet` | `oldBatchSummary` / `newBatchSummary` | 0.997 | 0.997 | +0.0% | ok |
| `PaperNativeSurfaceRulesTestRuleState` | `oldStateRuleSummary` / `newStateRuleSummary` | 1.046 | 1.046 | -0.0% | ok |
| `PaperNativeTopographicGraphSortCapacity` | `oldDefaultCapacitySummary` / `newPresizedSummary` | 1.003 | 1.003 | +0.0% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldAtOrBeyondRangeSummary` / `guardedAtOrBeyondRangeSummary` | 0.985 | 0.985 | -0.0% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldReallyFarSummary` / `guardedReallyFarSummary` | 1.011 | 1.011 | -0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldAtOrBeyondRangeValue` / `guardedAtOrBeyondRangeValue` | 1.018 | 1.018 | -0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldAzimuthValue` / `directAzimuthValue` | 0.995 | 0.995 | +0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldChunkVisibleValue` / `cachedChunkVisibleValue` | 0.896 | 0.896 | -0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldReallyFarValue` / `guardedReallyFarValue` | 1.071 | 1.071 | +0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldWaypointManagerValue` / `optimizedWaypointManagerValue` | 0.933 | 0.933 | -0.0% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldDoubleBatchSummary` / `directDoubleBatchSummary` | 1.001 | 1.001 | -0.0% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldFloatBatchSummary` / `directFloatBatchSummary` | 1.000 | 1.000 | -0.0% | ok |

* compared: 70, new pairs: 0, missing: 0

