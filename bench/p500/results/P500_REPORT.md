# P500 benchmark report — Crussty CE native kernels (old vs optimized)

* Generated: 2026-09-08T00:17:53+00:00 (aggregator v2 — stem pairing, noise-model classification, baseline tracking)
* Raw data: `p500_raw.tsv` — one JVM fork per group, time-bounded batches (~120 ms), median of 5, identical synthesized args per group.
* Hardware note: shared 2-CPU sandbox; treat <±15% deltas as parity.
* Noise model: ratio = alt/old — **WIN** ≤ 0.85, **REGRESSION** ≥ 1.18, PARITY in between; repeated measurements of a kernel collapse to one primary row via min-of-medians (ties keep the earliest row); every collapsed repeat is retained as `method#variantK` — see 'Duplicate collapse (variant policy)'.
* stability = relative spread (max−min)/median of the underlying batch samples; for a pair the worse (larger) of the two kernels is shown; `n/a` when unavailable.

* Groups measured: 49, skipped: 0, crashed: 0, kernels measured: 129 (raw RESULT rows: 129, collapsed variant rows: 0 — duplicate policy: min-of-medians primary, repeats retained as #variantK)
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
| `newBatchSummary` | alt | 5.4 µs | 5.4 µs | 5.4 µs | 0.0% |
| `oldBatchSummary` | old | 6.0 µs | 6.0 µs | 6.0 µs | 0.0% |

### 3. `PaperNativeBlendedNoise` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedBatchSummary` | alt | 50.5 µs | 50.5 µs | 50.5 µs | 0.0% |
| `oldBatchSummary` | old | 51.4 µs | 51.4 µs | 51.4 µs | 0.0% |

### 4. `PaperNativeCaveCarverSkip` `(I[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLambdaSummary` | old | 34.3 ms | 34.3 ms | 34.3 ms | 0.0% |
| `reusedCheckerSummary` | alt | 34.4 ms | 34.4 ms | 34.4 ms | 0.0% |
| `directHelperSummary` | alt | 34.4 ms | 34.4 ms | 34.4 ms | 0.0% |

### 5. `PaperNativeChunkDependencies` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `arraySummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldImmutableListSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |

### 6. `PaperNativeClimateParameterDistance` `([J[J[J[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `subtractFirstDistanceSum` | alt | 521.5 ns | 521.5 ns | 521.5 ns | 0.0% |
| `oldDistanceSum` | old | 522.5 ns | 522.5 ns | 522.5 ns | 0.0% |
| `branchDistanceSum` | alt | 522.8 ns | 522.8 ns | 522.8 ns | 0.0% |

### 7. `PaperNativeCubicSplineCreate` `(I[F[F[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldIteratorSummary` | old | 152.7 µs | 152.7 µs | 152.7 µs | 0.0% |
| `indexSummary` | alt | 153.1 µs | 153.1 µs | 153.1 µs | 0.0% |

### 8. `PaperNativeDensityAp2Fill` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchFlatSummary` | alt | 11.2 µs | 11.2 µs | 11.2 µs | 0.0% |
| `oldFlatSummary` | old | 13.0 µs | 13.0 µs | 13.0 µs | 0.0% |
| `scratchNestedSummary` | alt | 19.6 µs | 19.6 µs | 19.6 µs | 0.0% |
| `oldNestedSummary` | old | 22.1 µs | 22.1 µs | 22.1 µs | 0.0% |

### 9. `PaperNativeDensityAp2MinMaxFill` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newSummary` | alt | 118.6 ns | 118.6 ns | 118.6 ns | 0.0% |
| `oldSummary` | old | 118.7 ns | 118.7 ns | 118.7 ns | 0.0% |

### 10. `PaperNativeDensitySplineContext` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldWrapperSummary` | old | 1.8 µs | 1.8 µs | 1.8 µs | 0.0% |
| `newDirectSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |

### 11. `PaperNativeDensityVisitorHook` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `hookedUnwrappingSummary` | alt | 674.2 ns | 674.2 ns | 674.2 ns | 0.0% |
| `oldUnwrappingSummary` | old | 674.9 ns | 674.9 ns | 674.9 ns | 0.0% |

### 12. `PaperNativeEntityBoundingBox` `(I[F[F[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldMakeThenSetSummary` | old | 5.7 µs | 5.7 µs | 5.7 µs | 0.0% |
| `directDimensionsSetSummary` | alt | 5.8 µs | 5.8 µs | 5.8 µs | 0.0% |

### 13. `PaperNativeEntityChunkTransient` `(IIJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newMixedSummary` | alt | 292.2 ns | 292.2 ns | 292.2 ns | 0.0% |
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
| `inlineDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `oldDerivativeSummary` | old | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |

### 16. `PaperNativeImprovedNoiseInline` `([BI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `switchGradientSummary` | alt | 7.6 µs | 7.6 µs | 7.6 µs | 0.0% |
| `arithmeticSummary` | alt | 8.0 µs | 8.0 µs | 8.0 µs | 0.0% |
| `flatGradientSummary` | alt | 8.1 µs | 8.1 µs | 8.1 µs | 0.0% |
| `oldPMethodSummary` | old | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |
| `inlineByteAccessSummary` | alt | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |

### 17. `PaperNativeJigsawCanAttach` `(I[I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `targetFirstBatchSummary` | alt | 625.8 ns | 625.8 ns | 625.8 ns | 0.0% |
| `oldBatchSummary` | old | 626.1 ns | 626.1 ns | 626.1 ns | 0.0% |
| `optimizedBatchSummary` | alt | 626.5 ns | 626.5 ns | 626.5 ns | 0.0% |

### 18. `PaperNativeLegacyProvidedAliasRemoval` `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newReverseAliasRemoveSummary` | alt | 87.8 ns | 87.8 ns | 87.8 ns | 0.0% |
| `oldValuesRemoveIfSummary` | old | 88.0 ns | 88.0 ns | 88.0 ns | 0.0% |

### 19. `PaperNativeLevelChunkHeightmap` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFourUpdateSummary` | old | 2.2 ms | 2.2 ms | 2.2 ms | 0.0% |
| `newCombinedUpdateSummary` | alt | 12.4 ms | 12.4 ms | 12.4 ms | 0.0% |

### 20. `PaperNativeMarkerCache` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 124.6 µs | 124.6 µs | 124.6 µs | 0.0% |
| `cachedSummary` | alt | 583.8 µs | 583.8 µs | 583.8 µs | 0.0% |

### 21. `PaperNativeNoiseChunkBlendCache` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newEmptyBlenderSummary` | alt | 232.5 ns | 232.5 ns | 232.5 ns | 0.0% |
| `oldEmptyBlenderSummary` | old | 73.2 µs | 73.2 µs | 73.2 µs | 0.0% |

### 22. `PaperNativeNoiseChunkFlatCacheContext` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newFalseContextSummary` | alt | 18.5 µs | 18.5 µs | 18.5 µs | 0.0% |
| `newTrueContextSummary` | alt | 18.9 µs | 18.9 µs | 18.9 µs | 0.0% |
| `oldFalseContextSummary` | old | 21.9 µs | 21.9 µs | 21.9 µs | 0.0% |
| `oldTrueContextSummary` | old | 23.5 µs | 23.5 µs | 23.5 µs | 0.0% |

### 23. `PaperNativeNoiseInterpolatorSlice` `(IIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatSummary` | alt | 1.8 ms | 1.8 ms | 1.8 ms | 0.0% |
| `oldJaggedSummary` | old | 6.1 ms | 6.1 ms | 6.1 ms | 0.0% |

### 24. `PaperNativeObfHelperMaps` `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStreamDefaultSummary` | old | 93.3 ns | 93.3 ns | 93.3 ns | 0.0% |
| `directMapsSummary` | alt | 93.4 ns | 93.4 ns | 93.4 ns | 0.0% |
| `presizedStringPoolSummary` | alt | 93.5 ns | 93.5 ns | 93.5 ns | 0.0% |

### 25. `PaperNativeOreFeatureLoop` `([D[D[D[D[I[I[I[I[I[IIIIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLoopSummary` | old | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `optimizedLoopSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |

### 26. `PaperNativeOwnableRule` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 3.7 µs | 3.7 µs | 3.7 µs | 0.0% |
| `oldStreamSummary` | old | 4.1 µs | 4.1 µs | 4.1 µs | 0.0% |

### 27. `PaperNativePalettedReencodeScratch` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchThreadLocalSummary` | alt | 408.9 µs | 408.9 µs | 408.9 µs | 0.0% |
| `oldNewArraySummary` | old | 486.7 µs | 486.7 µs | 486.7 µs | 0.0% |
| `directPackedSummary` | alt | 1.1 ms | 1.1 ms | 1.1 ms | 0.0% |

### 28. `PaperNativePluginClassLoaderGroup` `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLookupSummary` | old | 91.8 ns | 91.8 ns | 91.8 ns | 0.0% |
| `skipRequesterSummary` | alt | 91.9 ns | 91.9 ns | 91.9 ns | 0.0% |

### 29. `PaperNativePluginDirectoryScan` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newListSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `directoryStreamSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `oldWalkDepth1Summary` | old | 1.5 ms | 1.5 ms | 1.5 ms | 0.0% |

### 30. `PaperNativePluginLoadingAllocation` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLazyMissingSetSummary` | alt | 87.7 ns | 87.7 ns | 87.7 ns | 0.0% |
| `newPresizedSetupSummary` | alt | 88.5 ns | 88.5 ns | 88.5 ns | 0.0% |
| `oldEagerMissingSetSummary` | old | 88.5 ns | 88.5 ns | 88.5 ns | 0.0% |
| `newLazyValidateSummary` | alt | 88.7 ns | 88.7 ns | 88.7 ns | 0.0% |
| `oldDefaultCapacitySetupSummary` | old | 88.7 ns | 88.7 ns | 88.7 ns | 0.0% |
| `oldEagerValidateSummary` | old | 88.9 ns | 88.9 ns | 88.9 ns | 0.0% |

### 31. `PaperNativePluginMetaDependency` `(I[Ljava/lang/Object;[Z[Z[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedSummary` | alt | 92.0 ns | 92.0 ns | 92.0 ns | 0.0% |
| `newLoopSummary` | alt | 92.0 ns | 92.0 ns | 92.0 ns | 0.0% |
| `oldStreamSummary` | old | 92.5 ns | 92.5 ns | 92.5 ns | 0.0% |

### 32. `PaperNativePluginNameLog` `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldTreesetSummary` | old | 87.8 ns | 87.8 ns | 87.8 ns | 0.0% |
| `newArrayListSortSummary` | alt | 88.0 ns | 88.0 ns | 88.0 ns | 0.0% |

### 33. `PaperNativePluginStartupRollup` `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newSummary` | alt | 96.7 ns | 96.7 ns | 96.7 ns | 0.0% |
| `oldSummary` | old | 97.3 ns | 97.3 ns | 97.3 ns | 0.0% |

### 34. `PaperNativeProtoChunkHeightmap` `(I)J`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEnumSetForeachSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `newCachedContainsSummary` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |

### 35. `PaperNativeRangeChoice` `([D[I[I[II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFillArraySummary` | old | 81.5 ns | 81.5 ns | 81.5 ns | 0.0% |
| `optimizedFillArraySummary` | alt | 81.7 ns | 81.7 ns | 81.7 ns | 0.0% |

### 36. `PaperNativeRemapperIndexCleanup` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLazyCleanupSummary` | alt | 91.5 ns | 91.5 ns | 91.5 ns | 0.0% |
| `oldEagerCleanupSummary` | old | 91.5 ns | 91.5 ns | 91.5 ns | 0.0% |

### 37. `PaperNativeRemapperSkipHashes` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 33.6 µs | 33.6 µs | 33.6 µs | 0.0% |
| `oldStreamSummary` | old | 34.8 µs | 34.8 µs | 34.8 µs | 0.0% |

### 38. `PaperNativeServerEntityDeltaIdentity` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDistanceSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |

### 39. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoadAfterBuildSummary` | alt | 87.3 ns | 87.3 ns | 87.3 ns | 0.0% |
| `oldLoadAfterBuildSummary` | old | 87.6 ns | 87.6 ns | 87.6 ns | 0.0% |

### 40. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newRemovedCountSummary` | alt | 87.8 ns | 87.8 ns | 87.8 ns | 0.0% |
| `oldRemovedCountSummary` | old | 88.1 ns | 88.1 ns | 88.1 ns | 0.0% |

### 41. `PaperNativeSpringFeatureMutablePos` `(I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |
| `mutableBatchSummary` | alt | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |

### 42. `PaperNativeStaticCacheGet` `(IIIII[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 34.4 ns | 34.4 ns | 34.4 ns | 0.0% |
| `oldBatchSummary` | old | 34.4 ns | 34.4 ns | 34.4 ns | 0.0% |

### 43. `PaperNativeSurfaceRulesTestRuleState` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStateRuleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `newStateRuleSummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |

### 44. `PaperNativeTopographicGraphSortCapacity` `(I[I[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newPresizedSummary` | alt | 349.7 ns | 349.7 ns | 349.7 ns | 0.0% |
| `oldDefaultCapacitySummary` | old | 349.7 ns | 349.7 ns | 349.7 ns | 0.0% |

### 45. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedAtOrBeyondRangeSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs | 0.0% |
| `oldAtOrBeyondRangeSummary` | old | 9.6 µs | 9.6 µs | 9.6 µs | 0.0% |

### 46. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarSummary` | old | 8.3 µs | 8.3 µs | 8.3 µs | 0.0% |
| `guardedReallyFarSummary` | alt | 8.4 µs | 8.4 µs | 8.4 µs | 0.0% |

### 47. `PaperNativeWaypointHotPath` `(I)D`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarValue` | old | 943.8 ns | 943.8 ns | 943.8 ns | 0.0% |
| `guardedReallyFarValue` | alt | 1.0 µs | 1.0 µs | 1.0 µs | 0.0% |
| `oldAtOrBeyondRangeValue` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `guardedAtOrBeyondRangeValue` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `oldAzimuthValue` | old | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `directAzimuthValue` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `cachedChunkVisibleValue` | alt | 3.0 µs | 3.0 µs | 3.0 µs | 0.0% |
| `oldChunkVisibleValue` | old | 3.3 µs | 3.3 µs | 3.3 µs | 0.0% |
| `optimizedWaypointManagerValue` | alt | 93.3 µs | 93.3 µs | 93.3 µs | 0.0% |
| `oldWaypointManagerValue` | old | 99.5 µs | 99.5 µs | 99.5 µs | 0.0% |

### 48. `PaperNativeXoroshiroPositionalDirect` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDoubleBatchSummary` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `directDoubleBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `directFloatBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldFloatBatchSummary` | old | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |

## Duplicate collapse (variant policy)

Raw RESULT rows repeating a (group, method) kernel collapse to one primary row via min-of-medians (ties keep the earliest row — unchanged v2 semantics, so no previously reported median can change). Every collapsed repeat is retained below as `method#variantK`; variants are repeat measurements of the same kernel and never form pairs.

* DUP-VARIANT: none — every kernel measured exactly once.

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
| 314.89x | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 73.2 µs | 232.5 ns |
| 3.34x | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.1 ms | 1.8 ms |
| 1.25x | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.5 µs | 18.9 µs |
| 1.22x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs |
| 1.19x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 486.7 µs | 408.9 µs |
| 1.18x | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 21.9 µs | 18.5 µs |
| 1.16x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `arithmeticSummary` | 9.3 µs | 8.0 µs |
| 1.15x | `PaperNativeDensityAp2Fill` | `oldFlatSummary` | `scratchFlatSummary` | 13.0 µs | 11.2 µs |

## Regressions (optimized SLOWER than old) — optimization targets

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 0.56x | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` | `newCachedContainsSummary` | 1.2 µs | 2.1 µs |
| 0.43x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `directPackedSummary` | 486.7 µs | 1.1 ms |
| 0.21x | `PaperNativeMarkerCache` | `oldSummary` | `cachedSummary` | 124.6 µs | 583.8 µs |
| 0.18x | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` | `newCombinedUpdateSummary` | 2.2 ms | 12.4 ms |

## Parity (within ±15%)

`PaperNativeImprovedNoiseInline.flatGradientSummary` (1.14x), `PaperNativeDensityAp2Fill.scratchNestedSummary` (1.13x), `PaperNativeWaypointHotPath.cachedChunkVisibleValue` (1.13x), `PaperNativeOwnableRule.newLoopSummary` (1.12x), `PaperNativeAquiferSurfaceSampling.newBatchSummary` (1.11x), `PaperNativePluginDirectoryScan.newListSummary` (1.11x), `PaperNativePluginDirectoryScan.directoryStreamSummary` (1.11x), `PaperNativeAquiferIndexStride.newBatchSummary` (1.07x), `PaperNativeWaypointHotPath.optimizedWaypointManagerValue` (1.07x), `PaperNativeRemapperSkipHashes.newLoopSummary` (1.04x), `PaperNativeChunkDependencies.arraySummary` (1.03x), `PaperNativeBlendedNoise.cachedBatchSummary` (1.02x), `PaperNativeWaypointDistanceGuard.guardedAtOrBeyondRangeSummary` (1.02x), `PaperNativeImprovedNoiseDerivative.flatGradientDerivativeSummary` (1.01x), `PaperNativePluginLoadingAllocation.newLazyMissingSetSummary` (1.01x), `PaperNativePluginStartupRollup.newSummary` (1.01x), `PaperNativePluginMetaDependency.cachedSummary` (1.01x), `PaperNativePluginMetaDependency.newLoopSummary` (1.01x), `PaperNativeEntityChunkTransient.newMixedSummary` (1.01x), `PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary` (1.00x), `PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary` (1.00x), `PaperNativeEntityLookupStatus.directAccessibleSummary` (1.00x), `PaperNativeAquiferPositionalLocation.directBatchSummary` (1.00x), `PaperNativeLegacyProvidedAliasRemoval.newReverseAliasRemoveSummary` (1.00x), `PaperNativePluginLoadingAllocation.newPresizedSetupSummary` (1.00x), `PaperNativePluginLoadingAllocation.newLazyValidateSummary` (1.00x), `PaperNativeClimateParameterDistance.subtractFirstDistanceSum` (1.00x), `PaperNativeDensityVisitorHook.hookedUnwrappingSummary` (1.00x), `PaperNativeImprovedNoiseDerivative.intTableDerivativeSummary` (1.00x), `PaperNativeDensityAp2MinMaxFill.newSummary` (1.00x), `PaperNativeEntityLookupStatus.directStatusSummary` (1.00x), `PaperNativeImprovedNoiseDerivative.inlineDerivativeSummary` (1.00x), `PaperNativeJigsawCanAttach.targetFirstBatchSummary` (1.00x), `PaperNativeXoroshiroPositionalDirect.directFloatBatchSummary` (1.00x), `PaperNativeRemapperIndexCleanup.newLazyCleanupSummary` (1.00x), `PaperNativeStaticCacheGet.newBatchSummary` (1.00x), `PaperNativeTopographicGraphSortCapacity.newPresizedSummary` (1.00x), `PaperNativeImprovedNoiseInline.inlineByteAccessSummary` (1.00x), `PaperNativeSpringFeatureMutablePos.mutableBatchSummary` (1.00x), `PaperNativeWaypointHotPath.directAzimuthValue` (1.00x), `PaperNativeClimateParameterDistance.branchDistanceSum` (1.00x), `PaperNativeJigsawCanAttach.optimizedBatchSummary` (1.00x), `PaperNativeObfHelperMaps.directMapsSummary` (1.00x), `PaperNativePluginClassLoaderGroup.skipRequesterSummary` (1.00x), `PaperNativeCaveCarverSkip.reusedCheckerSummary` (1.00x), `PaperNativeXoroshiroPositionalDirect.directDoubleBatchSummary` (1.00x), `PaperNativeCaveCarverSkip.directHelperSummary` (1.00x), `PaperNativeObfHelperMaps.presizedStringPoolSummary` (1.00x), `PaperNativePluginNameLog.newArrayListSortSummary` (1.00x), `PaperNativeCubicSplineCreate.indexSummary` (1.00x), `PaperNativeRangeChoice.optimizedFillArraySummary` (1.00x), `PaperNativeOreFeatureLoop.optimizedLoopSummary` (1.00x), `PaperNativeWaypointDistanceGuard.guardedReallyFarSummary` (0.99x), `PaperNativeDensitySplineContext.newDirectSummary` (0.99x), `PaperNativeWaypointHotPath.guardedAtOrBeyondRangeValue` (0.98x), `PaperNativeEntityBoundingBox.directDimensionsSetSummary` (0.97x), `PaperNativeSurfaceRulesTestRuleState.newStateRuleSummary` (0.95x), `PaperNativeWaypointHotPath.guardedReallyFarValue` (0.94x)

## Regressions (do-not-wire)

ratio = alt/old ≥ 1.18 — beyond the ±15% noise floor. Do NOT wire these optimized variants into hot paths. '(noisy)' = pair stability > 50%.

| kernel (alt / paired old) | group | ratio | stability | verdict |
|---|---|---:|---:|---|
| `newCombinedUpdateSummary` / `oldFourUpdateSummary` | PaperNativeLevelChunkHeightmap (g19) | 5.616 | 0.0% | REGRESSION |
| `cachedSummary` / `oldSummary` | PaperNativeMarkerCache (g20) | 4.684 | 0.0% | REGRESSION |
| `directPackedSummary` / `oldNewArraySummary` | PaperNativePalettedReencodeScratch (g27) | 2.330 | 0.0% | REGRESSION |
| `newCachedContainsSummary` / `oldEnumSetForeachSummary` | PaperNativeProtoChunkHeightmap (g34) | 1.772 | 0.0% | REGRESSION |

## Wins (promotion candidates)

ratio = alt/old ≤ 0.85 (≥ ~1.18x speedup), sorted best-first. Candidates for engine hot paths; check stability before promoting.

| ratio | class | old kernel | alt kernel | old | alt | stability | verdict |
|---:|---|---|---|---:|---:|---:|---|
| 0.003 | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 73.2 µs | 232.5 ns | 0.0% | WIN |
| 0.299 | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.1 ms | 1.8 ms | 0.0% | WIN |
| 0.803 | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.5 µs | 18.9 µs | 0.0% | WIN |
| 0.822 | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs | 0.0% | WIN |
| 0.840 | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 486.7 µs | 408.9 µs | 0.0% | WIN |
| 0.845 | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 21.9 µs | 18.5 µs | 0.0% | WIN |

## JNI floor groups

Groups with at least one kernel median below 200 ns sit on the JNI-transition floor (canon 35–90 ns; anchors: StaticCacheGet 34.6 ns, RangeChoice 81.4 ns, (I)D @N=1 19.9–31.2 ns — TASK-33 canon; an earlier revision of this line cited '~115 ns measured in the scaling study', a pre-BATCH_NS-audit artifact, commit 3baa0f7): per-kernel micro-optimization is pointless there — batch more work per JNI call instead (engine-level batch-API candidates).

| group | class | fastest median | kernels below 200 ns |
|---|---|---:|---|
| g9 | `PaperNativeDensityAp2MinMaxFill` | 118.6 ns | `newSummary` (118.6 ns), `oldSummary` (118.7 ns) |
| g18 | `PaperNativeLegacyProvidedAliasRemoval` | 87.8 ns | `newReverseAliasRemoveSummary` (87.8 ns), `oldValuesRemoveIfSummary` (88.0 ns) |
| g24 | `PaperNativeObfHelperMaps` | 93.3 ns | `oldStreamDefaultSummary` (93.3 ns), `directMapsSummary` (93.4 ns), `presizedStringPoolSummary` (93.5 ns) |
| g28 | `PaperNativePluginClassLoaderGroup` | 91.8 ns | `oldLookupSummary` (91.8 ns), `skipRequesterSummary` (91.9 ns) |
| g30 | `PaperNativePluginLoadingAllocation` | 87.7 ns | `newLazyMissingSetSummary` (87.7 ns), `newPresizedSetupSummary` (88.5 ns), `oldEagerMissingSetSummary` (88.5 ns), `newLazyValidateSummary` (88.7 ns), `oldDefaultCapacitySetupSummary` (88.7 ns), `oldEagerValidateSummary` (88.9 ns) |
| g31 | `PaperNativePluginMetaDependency` | 92.0 ns | `cachedSummary` (92.0 ns), `newLoopSummary` (92.0 ns), `oldStreamSummary` (92.5 ns) |
| g32 | `PaperNativePluginNameLog` | 87.8 ns | `oldTreesetSummary` (87.8 ns), `newArrayListSortSummary` (88.0 ns) |
| g33 | `PaperNativePluginStartupRollup` | 96.7 ns | `newSummary` (96.7 ns), `oldSummary` (97.3 ns) |
| g35 | `PaperNativeRangeChoice` | 81.5 ns | `oldFillArraySummary` (81.5 ns), `optimizedFillArraySummary` (81.7 ns) |
| g36 | `PaperNativeRemapperIndexCleanup` | 91.5 ns | `newLazyCleanupSummary` (91.5 ns), `oldEagerCleanupSummary` (91.5 ns) |
| g39 | `PaperNativeSpigotLoadOrderDependency` | 87.3 ns | `newLoadAfterBuildSummary` (87.3 ns), `oldLoadAfterBuildSummary` (87.6 ns) |
| g40 | `PaperNativeSpigotLoadOrderDependency` | 87.8 ns | `newRemovedCountSummary` (87.8 ns), `oldRemovedCountSummary` (88.1 ns) |
| g42 | `PaperNativeStaticCacheGet` | 34.4 ns | `newBatchSummary` (34.4 ns), `oldBatchSummary` (34.4 ns) |

## Baseline diff vs `baseline.tsv`

Per-pair ratio change: drift = (current ratio − baseline ratio) / baseline ratio; flagged when |drift| > 20%.

| class | pair (old / alt) | baseline ratio | current ratio | drift | flag |
|---|---|---:|---:|---:|---|
| `PaperNativeAquiferIndexStride` | `oldBatchSummary` / `newBatchSummary` | 0.933 | 0.935 | +0.2% | ok |
| `PaperNativeAquiferPositionalLocation` | `oldBatchSummary` / `directBatchSummary` | 0.993 | 0.997 | +0.4% | ok |
| `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` / `newBatchSummary` | 0.906 | 0.900 | -0.6% | ok |
| `PaperNativeBlendedNoise` | `oldBatchSummary` / `cachedBatchSummary` | 0.980 | 0.983 | +0.3% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `directHelperSummary` | 0.987 | 1.002 | +1.5% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `reusedCheckerSummary` | 0.997 | 1.001 | +0.4% | ok |
| `PaperNativeChunkDependencies` | `oldImmutableListSummary` / `arraySummary` | 0.972 | 0.967 | -0.5% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `branchDistanceSum` | 1.008 | 1.001 | -0.7% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `subtractFirstDistanceSum` | 1.005 | 0.998 | -0.7% | ok |
| `PaperNativeCubicSplineCreate` | `oldIteratorSummary` / `indexSummary` | 1.001 | 1.002 | +0.1% | ok |
| `PaperNativeDensityAp2Fill` | `oldFlatSummary` / `scratchFlatSummary` | 0.869 | 0.868 | -0.1% | ok |
| `PaperNativeDensityAp2Fill` | `oldNestedSummary` / `scratchNestedSummary` | 0.888 | 0.887 | -0.0% | ok |
| `PaperNativeDensityAp2MinMaxFill` | `oldSummary` / `newSummary` | 1.003 | 0.999 | -0.4% | ok |
| `PaperNativeDensitySplineContext` | `oldWrapperSummary` / `newDirectSummary` | 1.004 | 1.011 | +0.8% | ok |
| `PaperNativeDensityVisitorHook` | `oldUnwrappingSummary` / `hookedUnwrappingSummary` | 1.009 | 0.999 | -1.0% | ok |
| `PaperNativeEntityBoundingBox` | `oldMakeThenSetSummary` / `directDimensionsSetSummary` | 1.029 | 1.026 | -0.3% | ok |
| `PaperNativeEntityChunkTransient` | `oldMixedSummary` / `newMixedSummary` | 0.993 | 0.995 | +0.2% | ok |
| `PaperNativeEntityLookupStatus` | `oldAccessibleSummary` / `directAccessibleSummary` | 0.999 | 0.997 | -0.2% | ok |
| `PaperNativeEntityLookupStatus` | `oldStatusSummary` / `directStatusSummary` | 0.999 | 0.999 | +0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `flatGradientDerivativeSummary` | 0.985 | 0.989 | +0.3% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `inlineDerivativeSummary` | 1.000 | 0.999 | -0.1% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `intTableDerivativeSummary` | 0.996 | 0.999 | +0.3% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `arithmeticSummary` | 0.864 | 0.863 | -0.1% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `flatGradientSummary` | 0.878 | 0.874 | -0.5% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `inlineByteAccessSummary` | 1.001 | 1.000 | -0.1% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `switchGradientSummary` | 0.819 | 0.822 | +0.4% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `optimizedBatchSummary` | 0.999 | 1.001 | +0.2% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `targetFirstBatchSummary` | 0.997 | 1.000 | +0.3% | ok |
| `PaperNativeLegacyProvidedAliasRemoval` | `oldValuesRemoveIfSummary` / `newReverseAliasRemoveSummary` | 1.048 | 0.998 | -4.8% | ok |
| `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` / `newCombinedUpdateSummary` | 5.700 | 5.616 | -1.5% | ok |
| `PaperNativeMarkerCache` | `oldSummary` / `cachedSummary` | 4.540 | 4.684 | +3.2% | ok |
| `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` / `newEmptyBlenderSummary` | 0.003 | 0.003 | +0.5% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` / `newFalseContextSummary` | 0.846 | 0.845 | -0.0% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` / `newTrueContextSummary` | 0.805 | 0.803 | -0.3% | ok |
| `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` / `flatSummary` | 0.301 | 0.299 | -0.6% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `directMapsSummary` | 0.998 | 1.001 | +0.3% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `presizedStringPoolSummary` | 1.001 | 1.002 | +0.1% | ok |
| `PaperNativeOreFeatureLoop` | `oldLoopSummary` / `optimizedLoopSummary` | 0.996 | 1.005 | +0.9% | ok |
| `PaperNativeOwnableRule` | `oldStreamSummary` / `newLoopSummary` | 0.894 | 0.892 | -0.2% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `directPackedSummary` | 2.346 | 2.330 | -0.7% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `scratchThreadLocalSummary` | 0.834 | 0.840 | +0.7% | ok |
| `PaperNativePluginClassLoaderGroup` | `oldLookupSummary` / `skipRequesterSummary` | 1.008 | 1.001 | -0.6% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `directoryStreamSummary` | 0.921 | 0.904 | -1.8% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `newListSummary` | 0.939 | 0.904 | -3.8% | ok |
| `PaperNativePluginLoadingAllocation` | `oldDefaultCapacitySetupSummary` / `newPresizedSetupSummary` | 0.970 | 0.998 | +2.9% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` / `newLazyMissingSetSummary` | 0.996 | 0.991 | -0.5% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` / `newLazyValidateSummary` | 0.993 | 0.998 | +0.5% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `cachedSummary` | 0.974 | 0.995 | +2.2% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `newLoopSummary` | 0.998 | 0.995 | -0.3% | ok |
| `PaperNativePluginNameLog` | `oldTreesetSummary` / `newArrayListSortSummary` | 0.944 | 1.002 | +6.2% | ok |
| `PaperNativePluginStartupRollup` | `oldSummary` / `newSummary` | 0.992 | 0.994 | +0.2% | ok |
| `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` / `newCachedContainsSummary` | 1.775 | 1.772 | -0.2% | ok |
| `PaperNativeRangeChoice` | `oldFillArraySummary` / `optimizedFillArraySummary` | 1.002 | 1.002 | -0.0% | ok |
| `PaperNativeRemapperIndexCleanup` | `oldEagerCleanupSummary` / `newLazyCleanupSummary` | 1.003 | 1.000 | -0.3% | ok |
| `PaperNativeRemapperSkipHashes` | `oldStreamSummary` / `newLoopSummary` | 0.956 | 0.966 | +1.0% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldLoadAfterBuildSummary` / `newLoadAfterBuildSummary` | 1.003 | 0.997 | -0.7% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldRemovedCountSummary` / `newRemovedCountSummary` | 0.996 | 0.997 | +0.1% | ok |
| `PaperNativeSpringFeatureMutablePos` | `oldBatchSummary` / `mutableBatchSummary` | 1.001 | 1.000 | -0.1% | ok |
| `PaperNativeStaticCacheGet` | `oldBatchSummary` / `newBatchSummary` | 0.997 | 1.000 | +0.3% | ok |
| `PaperNativeSurfaceRulesTestRuleState` | `oldStateRuleSummary` / `newStateRuleSummary` | 1.046 | 1.048 | +0.1% | ok |
| `PaperNativeTopographicGraphSortCapacity` | `oldDefaultCapacitySummary` / `newPresizedSummary` | 1.003 | 1.000 | -0.3% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldAtOrBeyondRangeSummary` / `guardedAtOrBeyondRangeSummary` | 0.985 | 0.985 | -0.0% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldReallyFarSummary` / `guardedReallyFarSummary` | 1.011 | 1.007 | -0.4% | ok |
| `PaperNativeWaypointHotPath` | `oldAtOrBeyondRangeValue` / `guardedAtOrBeyondRangeValue` | 1.018 | 1.023 | +0.4% | ok |
| `PaperNativeWaypointHotPath` | `oldAzimuthValue` / `directAzimuthValue` | 0.995 | 1.000 | +0.6% | ok |
| `PaperNativeWaypointHotPath` | `oldChunkVisibleValue` / `cachedChunkVisibleValue` | 0.896 | 0.888 | -0.9% | ok |
| `PaperNativeWaypointHotPath` | `oldReallyFarValue` / `guardedReallyFarValue` | 1.071 | 1.061 | -0.9% | ok |
| `PaperNativeWaypointHotPath` | `oldWaypointManagerValue` / `optimizedWaypointManagerValue` | 0.933 | 0.938 | +0.6% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldDoubleBatchSummary` / `directDoubleBatchSummary` | 1.001 | 1.001 | +0.1% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldFloatBatchSummary` / `directFloatBatchSummary` | 1.000 | 1.000 | +0.0% | ok |

* compared: 70, new pairs: 0, missing: 0

