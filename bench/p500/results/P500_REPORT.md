# P500 benchmark report — Crussty CE native kernels (old vs optimized)

* Generated: 2026-09-08T12:20:57+00:00 (aggregator v2 — stem pairing, noise-model classification, baseline tracking)
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
| `newBatchSummary` | alt | 13.1 µs | 13.1 µs | 13.1 µs | 0.0% |
| `oldBatchSummary` | old | 14.1 µs | 14.1 µs | 14.1 µs | 0.0% |

### 1. `PaperNativeAquiferPositionalLocation` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directBatchSummary` | alt | 2.7 µs | 2.7 µs | 2.7 µs | 0.0% |
| `oldBatchSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |

### 2. `PaperNativeAquiferSurfaceSampling` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 7.1 µs | 7.1 µs | 7.1 µs | 0.0% |
| `oldBatchSummary` | old | 7.9 µs | 7.9 µs | 7.9 µs | 0.0% |

### 3. `PaperNativeBlendedNoise` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedBatchSummary` | alt | 63.3 µs | 63.3 µs | 63.3 µs | 0.0% |
| `oldBatchSummary` | old | 67.1 µs | 67.1 µs | 67.1 µs | 0.0% |

### 4. `PaperNativeCaveCarverSkip` `(I[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLambdaSummary` | old | 45.5 ms | 45.5 ms | 45.5 ms | 0.0% |
| `directHelperSummary` | alt | 45.8 ms | 45.8 ms | 45.8 ms | 0.0% |
| `reusedCheckerSummary` | alt | 45.8 ms | 45.8 ms | 45.8 ms | 0.0% |

### 5. `PaperNativeChunkDependencies` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `arraySummary` | alt | 1.4 µs | 1.4 µs | 1.4 µs | 0.0% |
| `oldImmutableListSummary` | old | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |

### 6. `PaperNativeClimateParameterDistance` `([J[J[J[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `branchDistanceSum` | alt | 671.8 ns | 671.8 ns | 671.8 ns | 0.0% |
| `oldDistanceSum` | old | 680.1 ns | 680.1 ns | 680.1 ns | 0.0% |
| `subtractFirstDistanceSum` | alt | 681.3 ns | 681.3 ns | 681.3 ns | 0.0% |

### 7. `PaperNativeCubicSplineCreate` `(I[F[F[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldIteratorSummary` | old | 189.9 µs | 189.9 µs | 189.9 µs | 0.0% |
| `indexSummary` | alt | 198.7 µs | 198.7 µs | 198.7 µs | 0.0% |

### 8. `PaperNativeDensityAp2Fill` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchFlatSummary` | alt | 13.9 µs | 13.9 µs | 13.9 µs | 0.0% |
| `oldFlatSummary` | old | 16.0 µs | 16.0 µs | 16.0 µs | 0.0% |
| `scratchNestedSummary` | alt | 24.4 µs | 24.4 µs | 24.4 µs | 0.0% |
| `oldNestedSummary` | old | 25.9 µs | 25.9 µs | 25.9 µs | 0.0% |

### 9. `PaperNativeDensityAp2MinMaxFill` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newSummary` | alt | 147.5 ns | 147.5 ns | 147.5 ns | 0.0% |
| `oldSummary` | old | 154.5 ns | 154.5 ns | 154.5 ns | 0.0% |

### 10. `PaperNativeDensitySplineContext` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldWrapperSummary` | old | 2.3 µs | 2.3 µs | 2.3 µs | 0.0% |
| `newDirectSummary` | alt | 2.4 µs | 2.4 µs | 2.4 µs | 0.0% |

### 11. `PaperNativeDensityVisitorHook` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldUnwrappingSummary` | old | 887.2 ns | 887.2 ns | 887.2 ns | 0.0% |
| `hookedUnwrappingSummary` | alt | 891.1 ns | 891.1 ns | 891.1 ns | 0.0% |

### 12. `PaperNativeEntityBoundingBox` `(I[F[F[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldMakeThenSetSummary` | old | 7.6 µs | 7.6 µs | 7.6 µs | 0.0% |
| `directDimensionsSetSummary` | alt | 7.7 µs | 7.7 µs | 7.7 µs | 0.0% |

### 13. `PaperNativeEntityChunkTransient` `(IIJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newMixedSummary` | alt | 363.0 ns | 363.0 ns | 363.0 ns | 0.0% |
| `oldMixedSummary` | old | 370.0 ns | 370.0 ns | 370.0 ns | 0.0% |

### 14. `PaperNativeEntityLookupStatus` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldAccessibleSummary` | old | 1.4 µs | 1.4 µs | 1.4 µs | 0.0% |
| `directAccessibleSummary` | alt | 1.4 µs | 1.4 µs | 1.4 µs | 0.0% |
| `oldStatusSummary` | old | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |
| `directStatusSummary` | alt | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |

### 15. `PaperNativeImprovedNoiseDerivative` `([B[I[I[I[D[D[DI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatGradientDerivativeSummary` | alt | 15.2 µs | 15.2 µs | 15.2 µs | 0.0% |
| `oldDerivativeSummary` | old | 15.3 µs | 15.3 µs | 15.3 µs | 0.0% |
| `intTableDerivativeSummary` | alt | 15.4 µs | 15.4 µs | 15.4 µs | 0.0% |
| `inlineDerivativeSummary` | alt | 15.5 µs | 15.5 µs | 15.5 µs | 0.0% |

### 16. `PaperNativeImprovedNoiseInline` `([BI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `switchGradientSummary` | alt | 10.1 µs | 10.1 µs | 10.1 µs | 0.0% |
| `arithmeticSummary` | alt | 10.6 µs | 10.6 µs | 10.6 µs | 0.0% |
| `flatGradientSummary` | alt | 10.7 µs | 10.7 µs | 10.7 µs | 0.0% |
| `oldPMethodSummary` | old | 12.2 µs | 12.2 µs | 12.2 µs | 0.0% |
| `inlineByteAccessSummary` | alt | 12.3 µs | 12.3 µs | 12.3 µs | 0.0% |

### 17. `PaperNativeJigsawCanAttach` `(I[I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 777.8 ns | 777.8 ns | 777.8 ns | 0.0% |
| `optimizedBatchSummary` | alt | 798.9 ns | 798.9 ns | 798.9 ns | 0.0% |
| `targetFirstBatchSummary` | alt | 821.3 ns | 821.3 ns | 821.3 ns | 0.0% |

### 18. `PaperNativeLegacyProvidedAliasRemoval` `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newReverseAliasRemoveSummary` | alt | 116.5 ns | 116.5 ns | 116.5 ns | 0.0% |
| `oldValuesRemoveIfSummary` | old | 117.8 ns | 117.8 ns | 117.8 ns | 0.0% |

### 19. `PaperNativeLevelChunkHeightmap` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFourUpdateSummary` | old | 2.8 ms | 2.8 ms | 2.8 ms | 0.0% |
| `newCombinedUpdateSummary` | alt | 15.9 ms | 15.9 ms | 15.9 ms | 0.0% |

### 20. `PaperNativeMarkerCache` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 150.2 µs | 150.2 µs | 150.2 µs | 0.0% |
| `cachedSummary` | alt | 725.1 µs | 725.1 µs | 725.1 µs | 0.0% |

### 21. `PaperNativeNoiseChunkBlendCache` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newEmptyBlenderSummary` | alt | 312.2 ns | 312.2 ns | 312.2 ns | 0.0% |
| `oldEmptyBlenderSummary` | old | 90.2 µs | 90.2 µs | 90.2 µs | 0.0% |

### 22. `PaperNativeNoiseChunkFlatCacheContext` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newFalseContextSummary` | alt | 24.4 µs | 24.4 µs | 24.4 µs | 0.0% |
| `newTrueContextSummary` | alt | 24.5 µs | 24.5 µs | 24.5 µs | 0.0% |
| `oldFalseContextSummary` | old | 28.5 µs | 28.5 µs | 28.5 µs | 0.0% |
| `oldTrueContextSummary` | old | 29.5 µs | 29.5 µs | 29.5 µs | 0.0% |

### 23. `PaperNativeNoiseInterpolatorSlice` `(IIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatSummary` | alt | 2.4 ms | 2.4 ms | 2.4 ms | 0.0% |
| `oldJaggedSummary` | old | 7.8 ms | 7.8 ms | 7.8 ms | 0.0% |

### 24. `PaperNativeObfHelperMaps` `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `presizedStringPoolSummary` | alt | 116.3 ns | 116.3 ns | 116.3 ns | 0.0% |
| `oldStreamDefaultSummary` | old | 118.6 ns | 118.6 ns | 118.6 ns | 0.0% |
| `directMapsSummary` | alt | 123.4 ns | 123.4 ns | 123.4 ns | 0.0% |

### 25. `PaperNativeOreFeatureLoop` `([D[D[D[D[I[I[I[I[I[IIIIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLoopSummary` | old | 2.5 µs | 2.5 µs | 2.5 µs | 0.0% |
| `optimizedLoopSummary` | alt | 2.6 µs | 2.6 µs | 2.6 µs | 0.0% |

### 26. `PaperNativeOwnableRule` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 4.9 µs | 4.9 µs | 4.9 µs | 0.0% |
| `oldStreamSummary` | old | 5.4 µs | 5.4 µs | 5.4 µs | 0.0% |

### 27. `PaperNativePalettedReencodeScratch` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchThreadLocalSummary` | alt | 539.7 µs | 539.7 µs | 539.7 µs | 0.0% |
| `oldNewArraySummary` | old | 609.4 µs | 609.4 µs | 609.4 µs | 0.0% |
| `directPackedSummary` | alt | 1.4 ms | 1.4 ms | 1.4 ms | 0.0% |

### 28. `PaperNativePluginClassLoaderGroup` `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `skipRequesterSummary` | alt | 115.0 ns | 115.0 ns | 115.0 ns | 0.0% |
| `oldLookupSummary` | old | 120.4 ns | 120.4 ns | 120.4 ns | 0.0% |

### 29. `PaperNativePluginDirectoryScan` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newListSummary` | alt | 1.8 ms | 1.8 ms | 1.8 ms | 0.0% |
| `directoryStreamSummary` | alt | 1.8 ms | 1.8 ms | 1.8 ms | 0.0% |
| `oldWalkDepth1Summary` | old | 1.9 ms | 1.9 ms | 1.9 ms | 0.0% |

### 30. `PaperNativePluginLoadingAllocation` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEagerMissingSetSummary` | old | 110.2 ns | 110.2 ns | 110.2 ns | 0.0% |
| `newLazyMissingSetSummary` | alt | 114.6 ns | 114.6 ns | 114.6 ns | 0.0% |
| `newPresizedSetupSummary` | alt | 115.8 ns | 115.8 ns | 115.8 ns | 0.0% |
| `newLazyValidateSummary` | alt | 116.4 ns | 116.4 ns | 116.4 ns | 0.0% |
| `oldDefaultCapacitySetupSummary` | old | 116.4 ns | 116.4 ns | 116.4 ns | 0.0% |
| `oldEagerValidateSummary` | old | 117.5 ns | 117.5 ns | 117.5 ns | 0.0% |

### 31. `PaperNativePluginMetaDependency` `(I[Ljava/lang/Object;[Z[Z[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedSummary` | alt | 114.2 ns | 114.2 ns | 114.2 ns | 0.0% |
| `newLoopSummary` | alt | 114.3 ns | 114.3 ns | 114.3 ns | 0.0% |
| `oldStreamSummary` | old | 121.3 ns | 121.3 ns | 121.3 ns | 0.0% |

### 32. `PaperNativePluginNameLog` `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newArrayListSortSummary` | alt | 109.7 ns | 109.7 ns | 109.7 ns | 0.0% |
| `oldTreesetSummary` | old | 110.9 ns | 110.9 ns | 110.9 ns | 0.0% |

### 33. `PaperNativePluginStartupRollup` `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 116.1 ns | 116.1 ns | 116.1 ns | 0.0% |
| `newSummary` | alt | 119.9 ns | 119.9 ns | 119.9 ns | 0.0% |

### 34. `PaperNativeProtoChunkHeightmap` `(I)J`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEnumSetForeachSummary` | old | 1.4 µs | 1.4 µs | 1.4 µs | 0.0% |
| `newCachedContainsSummary` | alt | 2.5 µs | 2.5 µs | 2.5 µs | 0.0% |

### 35. `PaperNativeRangeChoice` `([D[I[I[II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `optimizedFillArraySummary` | alt | 101.4 ns | 101.4 ns | 101.4 ns | 0.0% |
| `oldFillArraySummary` | old | 102.0 ns | 102.0 ns | 102.0 ns | 0.0% |

### 36. `PaperNativeRemapperIndexCleanup` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLazyCleanupSummary` | alt | 115.2 ns | 115.2 ns | 115.2 ns | 0.0% |
| `oldEagerCleanupSummary` | old | 119.6 ns | 119.6 ns | 119.6 ns | 0.0% |

### 37. `PaperNativeRemapperSkipHashes` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStreamSummary` | old | 44.4 µs | 44.4 µs | 44.4 µs | 0.0% |
| `newLoopSummary` | alt | 45.2 µs | 45.2 µs | 45.2 µs | 0.0% |

### 38. `PaperNativeServerEntityDeltaIdentity` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDistanceSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |

### 39. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLoadAfterBuildSummary` | old | 113.8 ns | 113.8 ns | 113.8 ns | 0.0% |
| `newLoadAfterBuildSummary` | alt | 115.1 ns | 115.1 ns | 115.1 ns | 0.0% |

### 40. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newRemovedCountSummary` | alt | 111.5 ns | 111.5 ns | 111.5 ns | 0.0% |
| `oldRemovedCountSummary` | old | 114.1 ns | 114.1 ns | 114.1 ns | 0.0% |

### 41. `PaperNativeSpringFeatureMutablePos` `(I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `mutableBatchSummary` | alt | 3.6 µs | 3.6 µs | 3.6 µs | 0.0% |
| `oldBatchSummary` | old | 3.8 µs | 3.8 µs | 3.8 µs | 0.0% |

### 42. `PaperNativeStaticCacheGet` `(IIIII[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 42.9 ns | 42.9 ns | 42.9 ns | 0.0% |
| `oldBatchSummary` | old | 44.9 ns | 44.9 ns | 44.9 ns | 0.0% |

### 43. `PaperNativeSurfaceRulesTestRuleState` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newStateRuleSummary` | alt | 1.4 µs | 1.4 µs | 1.4 µs | 0.0% |
| `oldStateRuleSummary` | old | 1.4 µs | 1.4 µs | 1.4 µs | 0.0% |

### 44. `PaperNativeTopographicGraphSortCapacity` `(I[I[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newPresizedSummary` | alt | 435.9 ns | 435.9 ns | 435.9 ns | 0.0% |
| `oldDefaultCapacitySummary` | old | 449.4 ns | 449.4 ns | 449.4 ns | 0.0% |

### 45. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedAtOrBeyondRangeSummary` | alt | 12.4 µs | 12.4 µs | 12.4 µs | 0.0% |
| `oldAtOrBeyondRangeSummary` | old | 12.5 µs | 12.5 µs | 12.5 µs | 0.0% |

### 46. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedReallyFarSummary` | alt | 10.5 µs | 10.5 µs | 10.5 µs | 0.0% |
| `oldReallyFarSummary` | old | 10.7 µs | 10.7 µs | 10.7 µs | 0.0% |

### 47. `PaperNativeWaypointHotPath` `(I)D`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldReallyFarValue` | old | 1.3 µs | 1.3 µs | 1.3 µs | 0.0% |
| `guardedReallyFarValue` | alt | 1.3 µs | 1.3 µs | 1.3 µs | 0.0% |
| `oldAtOrBeyondRangeValue` | old | 1.5 µs | 1.5 µs | 1.5 µs | 0.0% |
| `guardedAtOrBeyondRangeValue` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `directAzimuthValue` | alt | 2.7 µs | 2.7 µs | 2.7 µs | 0.0% |
| `oldAzimuthValue` | old | 2.8 µs | 2.8 µs | 2.8 µs | 0.0% |
| `cachedChunkVisibleValue` | alt | 3.9 µs | 3.9 µs | 3.9 µs | 0.0% |
| `oldChunkVisibleValue` | old | 4.4 µs | 4.4 µs | 4.4 µs | 0.0% |
| `optimizedWaypointManagerValue` | alt | 123.0 µs | 123.0 µs | 123.0 µs | 0.0% |
| `oldWaypointManagerValue` | old | 132.8 µs | 132.8 µs | 132.8 µs | 0.0% |

### 48. `PaperNativeXoroshiroPositionalDirect` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directDoubleBatchSummary` | alt | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |
| `directFloatBatchSummary` | alt | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |
| `oldFloatBatchSummary` | old | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |
| `oldDoubleBatchSummary` | old | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |

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
| 288.87x | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 90.2 µs | 312.2 ns |
| 3.31x | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 7.8 ms | 2.4 ms |
| 1.21x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 12.2 µs | 10.1 µs |
| 1.20x | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 29.5 µs | 24.5 µs |
| 1.17x | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 28.5 µs | 24.4 µs |
| 1.16x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `arithmeticSummary` | 12.2 µs | 10.6 µs |
| 1.15x | `PaperNativeDensityAp2Fill` | `oldFlatSummary` | `scratchFlatSummary` | 16.0 µs | 13.9 µs |

## Regressions (optimized SLOWER than old) — optimization targets

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 0.57x | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` | `newCachedContainsSummary` | 1.4 µs | 2.5 µs |
| 0.44x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `directPackedSummary` | 609.4 µs | 1.4 ms |
| 0.21x | `PaperNativeMarkerCache` | `oldSummary` | `cachedSummary` | 150.2 µs | 725.1 µs |
| 0.18x | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` | `newCombinedUpdateSummary` | 2.8 ms | 15.9 ms |

## Parity (within ±15%)

`PaperNativeImprovedNoiseInline.flatGradientSummary` (1.14x), `PaperNativeWaypointHotPath.cachedChunkVisibleValue` (1.14x), `PaperNativePalettedReencodeScratch.scratchThreadLocalSummary` (1.13x), `PaperNativeOwnableRule.newLoopSummary` (1.12x), `PaperNativeAquiferSurfaceSampling.newBatchSummary` (1.11x), `PaperNativePluginDirectoryScan.newListSummary` (1.10x), `PaperNativePluginDirectoryScan.directoryStreamSummary` (1.10x), `PaperNativeChunkDependencies.arraySummary` (1.08x), `PaperNativeWaypointHotPath.optimizedWaypointManagerValue` (1.08x), `PaperNativeAquiferIndexStride.newBatchSummary` (1.07x), `PaperNativePluginMetaDependency.cachedSummary` (1.06x), `PaperNativePluginMetaDependency.newLoopSummary` (1.06x), `PaperNativeDensityAp2Fill.scratchNestedSummary` (1.06x), `PaperNativeBlendedNoise.cachedBatchSummary` (1.06x), `PaperNativeAquiferPositionalLocation.directBatchSummary` (1.06x), `PaperNativeWaypointHotPath.directAzimuthValue` (1.05x), `PaperNativeDensityAp2MinMaxFill.newSummary` (1.05x), `PaperNativePluginClassLoaderGroup.skipRequesterSummary` (1.05x), `PaperNativeStaticCacheGet.newBatchSummary` (1.05x), `PaperNativeSpringFeatureMutablePos.mutableBatchSummary` (1.04x), `PaperNativeRemapperIndexCleanup.newLazyCleanupSummary` (1.04x), `PaperNativeTopographicGraphSortCapacity.newPresizedSummary` (1.03x), `PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary` (1.02x), `PaperNativeWaypointDistanceGuard.guardedReallyFarSummary` (1.02x), `PaperNativeObfHelperMaps.presizedStringPoolSummary` (1.02x), `PaperNativeEntityChunkTransient.newMixedSummary` (1.02x), `PaperNativeWaypointDistanceGuard.guardedAtOrBeyondRangeSummary` (1.01x), `PaperNativeClimateParameterDistance.branchDistanceSum` (1.01x), `PaperNativeLegacyProvidedAliasRemoval.newReverseAliasRemoveSummary` (1.01x), `PaperNativePluginNameLog.newArrayListSortSummary` (1.01x), `PaperNativePluginLoadingAllocation.newLazyValidateSummary` (1.01x), `PaperNativeImprovedNoiseDerivative.flatGradientDerivativeSummary` (1.01x), `PaperNativeRangeChoice.optimizedFillArraySummary` (1.01x), `PaperNativePluginLoadingAllocation.newPresizedSetupSummary` (1.01x), `PaperNativeXoroshiroPositionalDirect.directDoubleBatchSummary` (1.00x), `PaperNativeSurfaceRulesTestRuleState.newStateRuleSummary` (1.00x), `PaperNativeXoroshiroPositionalDirect.directFloatBatchSummary` (1.00x), `PaperNativeClimateParameterDistance.subtractFirstDistanceSum` (1.00x), `PaperNativeEntityLookupStatus.directStatusSummary` (1.00x), `PaperNativeImprovedNoiseInline.inlineByteAccessSummary` (1.00x), `PaperNativeImprovedNoiseDerivative.intTableDerivativeSummary` (1.00x), `PaperNativeDensityVisitorHook.hookedUnwrappingSummary` (1.00x), `PaperNativeWaypointHotPath.guardedReallyFarValue` (0.99x), `PaperNativeCaveCarverSkip.directHelperSummary` (0.99x), `PaperNativeCaveCarverSkip.reusedCheckerSummary` (0.99x), `PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary` (0.99x), `PaperNativeEntityBoundingBox.directDimensionsSetSummary` (0.99x), `PaperNativeImprovedNoiseDerivative.inlineDerivativeSummary` (0.99x), `PaperNativeEntityLookupStatus.directAccessibleSummary` (0.98x), `PaperNativeRemapperSkipHashes.newLoopSummary` (0.98x), `PaperNativeJigsawCanAttach.optimizedBatchSummary` (0.97x), `PaperNativeOreFeatureLoop.optimizedLoopSummary` (0.97x), `PaperNativePluginStartupRollup.newSummary` (0.97x), `PaperNativePluginLoadingAllocation.newLazyMissingSetSummary` (0.96x), `PaperNativeObfHelperMaps.directMapsSummary` (0.96x), `PaperNativeDensitySplineContext.newDirectSummary` (0.96x), `PaperNativeCubicSplineCreate.indexSummary` (0.96x), `PaperNativeWaypointHotPath.guardedAtOrBeyondRangeValue` (0.95x), `PaperNativeJigsawCanAttach.targetFirstBatchSummary` (0.95x)

## Regressions (do-not-wire)

ratio = alt/old ≥ 1.18 — beyond the ±15% noise floor. Do NOT wire these optimized variants into hot paths. '(noisy)' = pair stability > 50%.

| kernel (alt / paired old) | group | ratio | stability | verdict |
|---|---|---:|---:|---|
| `newCombinedUpdateSummary` / `oldFourUpdateSummary` | PaperNativeLevelChunkHeightmap (g19) | 5.640 | 0.0% | REGRESSION |
| `cachedSummary` / `oldSummary` | PaperNativeMarkerCache (g20) | 4.829 | 0.0% | REGRESSION |
| `directPackedSummary` / `oldNewArraySummary` | PaperNativePalettedReencodeScratch (g27) | 2.271 | 0.0% | REGRESSION |
| `newCachedContainsSummary` / `oldEnumSetForeachSummary` | PaperNativeProtoChunkHeightmap (g34) | 1.739 | 0.0% | REGRESSION |

## Wins (promotion candidates)

ratio = alt/old ≤ 0.85 (≥ ~1.18x speedup), sorted best-first. Candidates for engine hot paths; check stability before promoting.

| ratio | class | old kernel | alt kernel | old | alt | stability | verdict |
|---:|---|---|---|---:|---:|---:|---|
| 0.003 | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 90.2 µs | 312.2 ns | 0.0% | WIN |
| 0.302 | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 7.8 ms | 2.4 ms | 0.0% | WIN |
| 0.824 | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 12.2 µs | 10.1 µs | 0.0% | WIN |
| 0.831 | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 29.5 µs | 24.5 µs | 0.0% | WIN |

## JNI floor groups

Groups with at least one kernel median below 200 ns sit on the JNI-transition floor (canon 35–90 ns; anchors: StaticCacheGet 34.6 ns, RangeChoice 81.4 ns, (I)D @N=1 19.9–31.2 ns — TASK-33 canon; an earlier revision of this line cited '~115 ns measured in the scaling study', a pre-BATCH_NS-audit artifact, commit 3baa0f7): per-kernel micro-optimization is pointless there — batch more work per JNI call instead (engine-level batch-API candidates).

| group | class | fastest median | kernels below 200 ns |
|---|---|---:|---|
| g9 | `PaperNativeDensityAp2MinMaxFill` | 147.5 ns | `newSummary` (147.5 ns), `oldSummary` (154.5 ns) |
| g18 | `PaperNativeLegacyProvidedAliasRemoval` | 116.5 ns | `newReverseAliasRemoveSummary` (116.5 ns), `oldValuesRemoveIfSummary` (117.8 ns) |
| g24 | `PaperNativeObfHelperMaps` | 116.3 ns | `presizedStringPoolSummary` (116.3 ns), `oldStreamDefaultSummary` (118.6 ns), `directMapsSummary` (123.4 ns) |
| g28 | `PaperNativePluginClassLoaderGroup` | 115.0 ns | `skipRequesterSummary` (115.0 ns), `oldLookupSummary` (120.4 ns) |
| g30 | `PaperNativePluginLoadingAllocation` | 110.2 ns | `oldEagerMissingSetSummary` (110.2 ns), `newLazyMissingSetSummary` (114.6 ns), `newPresizedSetupSummary` (115.8 ns), `newLazyValidateSummary` (116.4 ns), `oldDefaultCapacitySetupSummary` (116.4 ns), `oldEagerValidateSummary` (117.5 ns) |
| g31 | `PaperNativePluginMetaDependency` | 114.2 ns | `cachedSummary` (114.2 ns), `newLoopSummary` (114.3 ns), `oldStreamSummary` (121.3 ns) |
| g32 | `PaperNativePluginNameLog` | 109.7 ns | `newArrayListSortSummary` (109.7 ns), `oldTreesetSummary` (110.9 ns) |
| g33 | `PaperNativePluginStartupRollup` | 116.1 ns | `oldSummary` (116.1 ns), `newSummary` (119.9 ns) |
| g35 | `PaperNativeRangeChoice` | 101.4 ns | `optimizedFillArraySummary` (101.4 ns), `oldFillArraySummary` (102.0 ns) |
| g36 | `PaperNativeRemapperIndexCleanup` | 115.2 ns | `newLazyCleanupSummary` (115.2 ns), `oldEagerCleanupSummary` (119.6 ns) |
| g39 | `PaperNativeSpigotLoadOrderDependency` | 113.8 ns | `oldLoadAfterBuildSummary` (113.8 ns), `newLoadAfterBuildSummary` (115.1 ns) |
| g40 | `PaperNativeSpigotLoadOrderDependency` | 111.5 ns | `newRemovedCountSummary` (111.5 ns), `oldRemovedCountSummary` (114.1 ns) |
| g42 | `PaperNativeStaticCacheGet` | 42.9 ns | `newBatchSummary` (42.9 ns), `oldBatchSummary` (44.9 ns) |

## Baseline diff vs `baseline.tsv`

Per-pair ratio change: drift = (current ratio − baseline ratio) / baseline ratio; flagged when |drift| > 20%.

| class | pair (old / alt) | baseline ratio | current ratio | drift | flag |
|---|---|---:|---:|---:|---|
| `PaperNativeAquiferIndexStride` | `oldBatchSummary` / `newBatchSummary` | 0.933 | 0.931 | -0.3% | ok |
| `PaperNativeAquiferPositionalLocation` | `oldBatchSummary` / `directBatchSummary` | 0.993 | 0.947 | -4.6% | ok |
| `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` / `newBatchSummary` | 0.906 | 0.904 | -0.2% | ok |
| `PaperNativeBlendedNoise` | `oldBatchSummary` / `cachedBatchSummary` | 0.980 | 0.943 | -3.8% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `directHelperSummary` | 0.987 | 1.007 | +1.9% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `reusedCheckerSummary` | 0.997 | 1.007 | +1.1% | ok |
| `PaperNativeChunkDependencies` | `oldImmutableListSummary` / `arraySummary` | 0.972 | 0.923 | -5.0% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `branchDistanceSum` | 1.008 | 0.988 | -2.0% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `subtractFirstDistanceSum` | 1.005 | 1.002 | -0.3% | ok |
| `PaperNativeCubicSplineCreate` | `oldIteratorSummary` / `indexSummary` | 1.001 | 1.046 | +4.5% | ok |
| `PaperNativeDensityAp2Fill` | `oldFlatSummary` / `scratchFlatSummary` | 0.869 | 0.866 | -0.3% | ok |
| `PaperNativeDensityAp2Fill` | `oldNestedSummary` / `scratchNestedSummary` | 0.888 | 0.943 | +6.2% | ok |
| `PaperNativeDensityAp2MinMaxFill` | `oldSummary` / `newSummary` | 1.003 | 0.955 | -4.8% | ok |
| `PaperNativeDensitySplineContext` | `oldWrapperSummary` / `newDirectSummary` | 1.004 | 1.045 | +4.1% | ok |
| `PaperNativeDensityVisitorHook` | `oldUnwrappingSummary` / `hookedUnwrappingSummary` | 1.009 | 1.004 | -0.5% | ok |
| `PaperNativeEntityBoundingBox` | `oldMakeThenSetSummary` / `directDimensionsSetSummary` | 1.029 | 1.013 | -1.6% | ok |
| `PaperNativeEntityChunkTransient` | `oldMixedSummary` / `newMixedSummary` | 0.993 | 0.981 | -1.2% | ok |
| `PaperNativeEntityLookupStatus` | `oldAccessibleSummary` / `directAccessibleSummary` | 0.999 | 1.018 | +1.9% | ok |
| `PaperNativeEntityLookupStatus` | `oldStatusSummary` / `directStatusSummary` | 0.999 | 1.002 | +0.4% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `flatGradientDerivativeSummary` | 0.985 | 0.991 | +0.6% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `inlineDerivativeSummary` | 1.000 | 1.013 | +1.3% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `intTableDerivativeSummary` | 0.996 | 1.004 | +0.8% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `arithmeticSummary` | 0.864 | 0.865 | +0.1% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `flatGradientSummary` | 0.878 | 0.875 | -0.3% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `inlineByteAccessSummary` | 1.001 | 1.003 | +0.2% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `switchGradientSummary` | 0.819 | 0.824 | +0.6% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `optimizedBatchSummary` | 0.999 | 1.027 | +2.8% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `targetFirstBatchSummary` | 0.997 | 1.056 | +5.9% | ok |
| `PaperNativeLegacyProvidedAliasRemoval` | `oldValuesRemoveIfSummary` / `newReverseAliasRemoveSummary` | 1.048 | 0.989 | -5.7% | ok |
| `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` / `newCombinedUpdateSummary` | 5.700 | 5.640 | -1.1% | ok |
| `PaperNativeMarkerCache` | `oldSummary` / `cachedSummary` | 4.540 | 4.829 | +6.4% | ok |
| `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` / `newEmptyBlenderSummary` | 0.003 | 0.003 | +9.6% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` / `newFalseContextSummary` | 0.846 | 0.858 | +1.4% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` / `newTrueContextSummary` | 0.805 | 0.831 | +3.2% | ok |
| `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` / `flatSummary` | 0.301 | 0.302 | +0.5% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `directMapsSummary` | 0.998 | 1.040 | +4.3% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `presizedStringPoolSummary` | 1.001 | 0.981 | -2.0% | ok |
| `PaperNativeOreFeatureLoop` | `oldLoopSummary` / `optimizedLoopSummary` | 0.996 | 1.028 | +3.2% | ok |
| `PaperNativeOwnableRule` | `oldStreamSummary` / `newLoopSummary` | 0.894 | 0.893 | -0.1% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `directPackedSummary` | 2.346 | 2.271 | -3.2% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `scratchThreadLocalSummary` | 0.834 | 0.886 | +6.1% | ok |
| `PaperNativePluginClassLoaderGroup` | `oldLookupSummary` / `skipRequesterSummary` | 1.008 | 0.955 | -5.2% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `directoryStreamSummary` | 0.921 | 0.910 | -1.2% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `newListSummary` | 0.939 | 0.909 | -3.3% | ok |
| `PaperNativePluginLoadingAllocation` | `oldDefaultCapacitySetupSummary` / `newPresizedSetupSummary` | 0.970 | 0.995 | +2.6% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` / `newLazyMissingSetSummary` | 0.996 | 1.040 | +4.4% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` / `newLazyValidateSummary` | 0.993 | 0.991 | -0.2% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `cachedSummary` | 0.974 | 0.941 | -3.3% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `newLoopSummary` | 0.998 | 0.942 | -5.5% | ok |
| `PaperNativePluginNameLog` | `oldTreesetSummary` / `newArrayListSortSummary` | 0.944 | 0.989 | +4.8% | ok |
| `PaperNativePluginStartupRollup` | `oldSummary` / `newSummary` | 0.992 | 1.033 | +4.1% | ok |
| `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` / `newCachedContainsSummary` | 1.775 | 1.739 | -2.0% | ok |
| `PaperNativeRangeChoice` | `oldFillArraySummary` / `optimizedFillArraySummary` | 1.002 | 0.994 | -0.8% | ok |
| `PaperNativeRemapperIndexCleanup` | `oldEagerCleanupSummary` / `newLazyCleanupSummary` | 1.003 | 0.963 | -4.0% | ok |
| `PaperNativeRemapperSkipHashes` | `oldStreamSummary` / `newLoopSummary` | 0.956 | 1.019 | +6.5% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldLoadAfterBuildSummary` / `newLoadAfterBuildSummary` | 1.003 | 1.011 | +0.8% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldRemovedCountSummary` / `newRemovedCountSummary` | 0.996 | 0.977 | -1.8% | ok |
| `PaperNativeSpringFeatureMutablePos` | `oldBatchSummary` / `mutableBatchSummary` | 1.001 | 0.957 | -4.4% | ok |
| `PaperNativeStaticCacheGet` | `oldBatchSummary` / `newBatchSummary` | 0.997 | 0.955 | -4.2% | ok |
| `PaperNativeSurfaceRulesTestRuleState` | `oldStateRuleSummary` / `newStateRuleSummary` | 1.046 | 0.997 | -4.7% | ok |
| `PaperNativeTopographicGraphSortCapacity` | `oldDefaultCapacitySummary` / `newPresizedSummary` | 1.003 | 0.970 | -3.3% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldAtOrBeyondRangeSummary` / `guardedAtOrBeyondRangeSummary` | 0.985 | 0.987 | +0.2% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldReallyFarSummary` / `guardedReallyFarSummary` | 1.011 | 0.978 | -3.3% | ok |
| `PaperNativeWaypointHotPath` | `oldAtOrBeyondRangeValue` / `guardedAtOrBeyondRangeValue` | 1.018 | 1.052 | +3.3% | ok |
| `PaperNativeWaypointHotPath` | `oldAzimuthValue` / `directAzimuthValue` | 0.995 | 0.948 | -4.7% | ok |
| `PaperNativeWaypointHotPath` | `oldChunkVisibleValue` / `cachedChunkVisibleValue` | 0.896 | 0.880 | -1.8% | ok |
| `PaperNativeWaypointHotPath` | `oldReallyFarValue` / `guardedReallyFarValue` | 1.071 | 1.005 | -6.1% | ok |
| `PaperNativeWaypointHotPath` | `oldWaypointManagerValue` / `optimizedWaypointManagerValue` | 0.933 | 0.926 | -0.7% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldDoubleBatchSummary` / `directDoubleBatchSummary` | 1.001 | 0.995 | -0.6% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldFloatBatchSummary` / `directFloatBatchSummary` | 1.000 | 0.999 | -0.1% | ok |

* compared: 70, new pairs: 0, missing: 0

