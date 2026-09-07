# P500 benchmark report — Crussty CE native kernels (old vs optimized)

* Generated: 2026-09-07T16:08:28+00:00 (aggregator v2 — stem pairing, noise-model classification, baseline tracking)
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
| `oldBatchSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `directBatchSummary` | alt | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |

### 2. `PaperNativeAquiferSurfaceSampling` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newBatchSummary` | alt | 5.5 µs | 5.5 µs | 5.5 µs | 0.0% |
| `oldBatchSummary` | old | 6.3 µs | 6.3 µs | 6.3 µs | 0.0% |

### 3. `PaperNativeBlendedNoise` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `cachedBatchSummary` | alt | 50.6 µs | 50.6 µs | 50.6 µs | 0.0% |
| `oldBatchSummary` | old | 50.7 µs | 50.7 µs | 50.7 µs | 0.0% |

### 4. `PaperNativeCaveCarverSkip` `(I[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directHelperSummary` | alt | 34.3 ms | 34.3 ms | 34.3 ms | 0.0% |
| `oldLambdaSummary` | old | 34.4 ms | 34.4 ms | 34.4 ms | 0.0% |
| `reusedCheckerSummary` | alt | 34.5 ms | 34.5 ms | 34.5 ms | 0.0% |

### 5. `PaperNativeChunkDependencies` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `arraySummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `oldImmutableListSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |

### 6. `PaperNativeClimateParameterDistance` `([J[J[J[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `subtractFirstDistanceSum` | alt | 651.8 ns | 651.8 ns | 651.8 ns | 0.0% |
| `oldDistanceSum` | old | 670.1 ns | 670.1 ns | 670.1 ns | 0.0% |
| `branchDistanceSum` | alt | 682.3 ns | 682.3 ns | 682.3 ns | 0.0% |

### 7. `PaperNativeCubicSplineCreate` `(I[F[F[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldIteratorSummary` | old | 153.1 µs | 153.1 µs | 153.1 µs | 0.0% |
| `indexSummary` | alt | 153.8 µs | 153.8 µs | 153.8 µs | 0.0% |

### 8. `PaperNativeDensityAp2Fill` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchFlatSummary` | alt | 11.4 µs | 11.4 µs | 11.4 µs | 0.0% |
| `oldFlatSummary` | old | 12.9 µs | 12.9 µs | 12.9 µs | 0.0% |
| `scratchNestedSummary` | alt | 19.1 µs | 19.1 µs | 19.1 µs | 0.0% |
| `oldNestedSummary` | old | 20.4 µs | 20.4 µs | 20.4 µs | 0.0% |

### 9. `PaperNativeDensityAp2MinMaxFill` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newSummary` | alt | 220.8 ns | 220.8 ns | 220.8 ns | 0.0% |
| `oldSummary` | old | 221.9 ns | 221.9 ns | 221.9 ns | 0.0% |

### 10. `PaperNativeDensitySplineContext` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newDirectSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs | 0.0% |
| `oldWrapperSummary` | old | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |

### 11. `PaperNativeDensityVisitorHook` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldUnwrappingSummary` | old | 775.0 ns | 775.0 ns | 775.0 ns | 0.0% |
| `hookedUnwrappingSummary` | alt | 781.9 ns | 781.9 ns | 781.9 ns | 0.0% |

### 12. `PaperNativeEntityBoundingBox` `(I[F[F[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldMakeThenSetSummary` | old | 5.7 µs | 5.7 µs | 5.7 µs | 0.0% |
| `directDimensionsSetSummary` | alt | 5.9 µs | 5.9 µs | 5.9 µs | 0.0% |

### 13. `PaperNativeEntityChunkTransient` `(IIJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldMixedSummary` | old | 394.8 ns | 394.8 ns | 394.8 ns | 0.0% |
| `newMixedSummary` | alt | 401.3 ns | 401.3 ns | 401.3 ns | 0.0% |

### 14. `PaperNativeEntityLookupStatus` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldAccessibleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `directAccessibleSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `oldStatusSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |
| `directStatusSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs | 0.0% |

### 15. `PaperNativeImprovedNoiseDerivative` `([B[I[I[I[D[D[DI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatGradientDerivativeSummary` | alt | 11.5 µs | 11.5 µs | 11.5 µs | 0.0% |
| `inlineDerivativeSummary` | alt | 11.6 µs | 11.6 µs | 11.6 µs | 0.0% |
| `intTableDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs | 0.0% |
| `oldDerivativeSummary` | old | 11.9 µs | 11.9 µs | 11.9 µs | 0.0% |

### 16. `PaperNativeImprovedNoiseInline` `([BI[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `switchGradientSummary` | alt | 7.6 µs | 7.6 µs | 7.6 µs | 0.0% |
| `arithmeticSummary` | alt | 8.0 µs | 8.0 µs | 8.0 µs | 0.0% |
| `flatGradientSummary` | alt | 8.3 µs | 8.3 µs | 8.3 µs | 0.0% |
| `oldPMethodSummary` | old | 9.3 µs | 9.3 µs | 9.3 µs | 0.0% |
| `inlineByteAccessSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs | 0.0% |

### 17. `PaperNativeJigsawCanAttach` `(I[I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 655.3 ns | 655.3 ns | 655.3 ns | 0.0% |
| `targetFirstBatchSummary` | alt | 656.1 ns | 656.1 ns | 656.1 ns | 0.0% |
| `optimizedBatchSummary` | alt | 658.0 ns | 658.0 ns | 658.0 ns | 0.0% |

### 18. `PaperNativeLegacyProvidedAliasRemoval` `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newReverseAliasRemoveSummary` | alt | 191.7 ns | 191.7 ns | 191.7 ns | 0.0% |
| `oldValuesRemoveIfSummary` | old | 192.5 ns | 192.5 ns | 192.5 ns | 0.0% |

### 19. `PaperNativeLevelChunkHeightmap` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFourUpdateSummary` | old | 2.2 ms | 2.2 ms | 2.2 ms | 0.0% |
| `newCombinedUpdateSummary` | alt | 12.4 ms | 12.4 ms | 12.4 ms | 0.0% |

### 20. `PaperNativeMarkerCache` `(III[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldSummary` | old | 119.2 µs | 119.2 µs | 119.2 µs | 0.0% |
| `cachedSummary` | alt | 558.6 µs | 558.6 µs | 558.6 µs | 0.0% |

### 21. `PaperNativeNoiseChunkBlendCache` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newEmptyBlenderSummary` | alt | 274.5 ns | 274.5 ns | 274.5 ns | 0.0% |
| `oldEmptyBlenderSummary` | old | 67.0 µs | 67.0 µs | 67.0 µs | 0.0% |

### 22. `PaperNativeNoiseChunkFlatCacheContext` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newFalseContextSummary` | alt | 18.7 µs | 18.7 µs | 18.7 µs | 0.0% |
| `newTrueContextSummary` | alt | 19.0 µs | 19.0 µs | 19.0 µs | 0.0% |
| `oldTrueContextSummary` | old | 19.0 µs | 19.0 µs | 19.0 µs | 0.0% |
| `oldFalseContextSummary` | old | 20.5 µs | 20.5 µs | 20.5 µs | 0.0% |

### 23. `PaperNativeNoiseInterpolatorSlice` `(IIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `flatSummary` | alt | 1.9 ms | 1.9 ms | 1.9 ms | 0.0% |
| `oldJaggedSummary` | old | 6.2 ms | 6.2 ms | 6.2 ms | 0.0% |

### 24. `PaperNativeObfHelperMaps` `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStreamDefaultSummary` | old | 214.0 ns | 214.0 ns | 214.0 ns | 0.0% |
| `directMapsSummary` | alt | 214.6 ns | 214.6 ns | 214.6 ns | 0.0% |
| `presizedStringPoolSummary` | alt | 216.8 ns | 216.8 ns | 216.8 ns | 0.0% |

### 25. `PaperNativeOreFeatureLoop` `([D[D[D[D[I[I[I[I[I[IIIIII[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLoopSummary` | old | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |
| `optimizedLoopSummary` | alt | 2.0 µs | 2.0 µs | 2.0 µs | 0.0% |

### 26. `PaperNativeOwnableRule` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 3.8 µs | 3.8 µs | 3.8 µs | 0.0% |
| `oldStreamSummary` | old | 4.2 µs | 4.2 µs | 4.2 µs | 0.0% |

### 27. `PaperNativePalettedReencodeScratch` `(I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `scratchThreadLocalSummary` | alt | 403.2 µs | 403.2 µs | 403.2 µs | 0.0% |
| `oldNewArraySummary` | old | 483.9 µs | 483.9 µs | 483.9 µs | 0.0% |
| `directPackedSummary` | alt | 1.1 ms | 1.1 ms | 1.1 ms | 0.0% |

### 28. `PaperNativePluginClassLoaderGroup` `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldLookupSummary` | old | 146.7 ns | 146.7 ns | 146.7 ns | 0.0% |
| `skipRequesterSummary` | alt | 153.2 ns | 153.2 ns | 153.2 ns | 0.0% |

### 29. `PaperNativePluginDirectoryScan` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newListSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `directoryStreamSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms | 0.0% |
| `oldWalkDepth1Summary` | old | 1.5 ms | 1.5 ms | 1.5 ms | 0.0% |

### 30. `PaperNativePluginLoadingAllocation` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLazyValidateSummary` | alt | 140.6 ns | 140.6 ns | 140.6 ns | 0.0% |
| `newPresizedSetupSummary` | alt | 142.2 ns | 142.2 ns | 142.2 ns | 0.0% |
| `newLazyMissingSetSummary` | alt | 142.5 ns | 142.5 ns | 142.5 ns | 0.0% |
| `oldDefaultCapacitySetupSummary` | old | 154.0 ns | 154.0 ns | 154.0 ns | 0.0% |
| `oldEagerValidateSummary` | old | 217.9 ns | 217.9 ns | 217.9 ns | 0.0% |
| `oldEagerMissingSetSummary` | old | 218.3 ns | 218.3 ns | 218.3 ns | 0.0% |

### 31. `PaperNativePluginMetaDependency` `(I[Ljava/lang/Object;[Z[Z[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldStreamSummary` | old | 192.1 ns | 192.1 ns | 192.1 ns | 0.0% |
| `newLoopSummary` | alt | 192.5 ns | 192.5 ns | 192.5 ns | 0.0% |
| `cachedSummary` | alt | 196.6 ns | 196.6 ns | 196.6 ns | 0.0% |

### 32. `PaperNativePluginNameLog` `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newArrayListSortSummary` | alt | 112.7 ns | 112.7 ns | 112.7 ns | 0.0% |
| `oldTreesetSummary` | old | 117.8 ns | 117.8 ns | 117.8 ns | 0.0% |

### 33. `PaperNativePluginStartupRollup` `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newSummary` | alt | 143.1 ns | 143.1 ns | 143.1 ns | 0.0% |
| `oldSummary` | old | 145.7 ns | 145.7 ns | 145.7 ns | 0.0% |

### 34. `PaperNativeProtoChunkHeightmap` `(I)J`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEnumSetForeachSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `newCachedContainsSummary` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |

### 35. `PaperNativeRangeChoice` `([D[I[I[II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldFillArraySummary` | old | 111.9 ns | 111.9 ns | 111.9 ns | 0.0% |
| `optimizedFillArraySummary` | alt | 114.6 ns | 114.6 ns | 114.6 ns | 0.0% |

### 36. `PaperNativeRemapperIndexCleanup` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldEagerCleanupSummary` | old | 196.7 ns | 196.7 ns | 196.7 ns | 0.0% |
| `newLazyCleanupSummary` | alt | 205.9 ns | 205.9 ns | 205.9 ns | 0.0% |

### 37. `PaperNativeRemapperSkipHashes` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoopSummary` | alt | 33.3 µs | 33.3 µs | 33.3 µs | 0.0% |
| `oldStreamSummary` | old | 35.6 µs | 35.6 µs | 35.6 µs | 0.0% |

### 38. `PaperNativeServerEntityDeltaIdentity` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDistanceSummary` | old | 2.4 µs | 2.4 µs | 2.4 µs | 0.0% |

### 39. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newLoadAfterBuildSummary` | alt | 114.5 ns | 114.5 ns | 114.5 ns | 0.0% |
| `oldLoadAfterBuildSummary` | old | 114.8 ns | 114.8 ns | 114.8 ns | 0.0% |

### 40. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldRemovedCountSummary` | old | 193.6 ns | 193.6 ns | 193.6 ns | 0.0% |
| `newRemovedCountSummary` | alt | 195.9 ns | 195.9 ns | 195.9 ns | 0.0% |

### 41. `PaperNativeSpringFeatureMutablePos` `(I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs | 0.0% |
| `mutableBatchSummary` | alt | 3.0 µs | 3.0 µs | 3.0 µs | 0.0% |

### 42. `PaperNativeStaticCacheGet` `(IIIII[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldBatchSummary` | old | 138.2 ns | 138.2 ns | 138.2 ns | 0.0% |
| `newBatchSummary` | alt | 139.5 ns | 139.5 ns | 139.5 ns | 0.0% |

### 43. `PaperNativeSurfaceRulesTestRuleState` `(II[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `newStateRuleSummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `oldStateRuleSummary` | old | 1.3 µs | 1.3 µs | 1.3 µs | 0.0% |

### 44. `PaperNativeTopographicGraphSortCapacity` `(I[I[I[I[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `oldDefaultCapacitySummary` | old | 375.3 ns | 375.3 ns | 375.3 ns | 0.0% |
| `newPresizedSummary` | alt | 379.8 ns | 379.8 ns | 379.8 ns | 0.0% |

### 45. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedAtOrBeyondRangeSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs | 0.0% |
| `oldAtOrBeyondRangeSummary` | old | 9.6 µs | 9.6 µs | 9.6 µs | 0.0% |

### 46. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedReallyFarSummary` | alt | 8.4 µs | 8.4 µs | 8.4 µs | 0.0% |
| `oldReallyFarSummary` | old | 8.5 µs | 8.5 µs | 8.5 µs | 0.0% |

### 47. `PaperNativeWaypointHotPath` `(I)D`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `guardedReallyFarValue` | alt | 1.0 µs | 1.0 µs | 1.0 µs | 0.0% |
| `oldReallyFarValue` | old | 1.0 µs | 1.0 µs | 1.0 µs | 0.0% |
| `guardedAtOrBeyondRangeValue` | alt | 1.2 µs | 1.2 µs | 1.2 µs | 0.0% |
| `oldAtOrBeyondRangeValue` | old | 1.3 µs | 1.3 µs | 1.3 µs | 0.0% |
| `directAzimuthValue` | alt | 2.1 µs | 2.1 µs | 2.1 µs | 0.0% |
| `oldAzimuthValue` | old | 2.2 µs | 2.2 µs | 2.2 µs | 0.0% |
| `cachedChunkVisibleValue` | alt | 3.2 µs | 3.2 µs | 3.2 µs | 0.0% |
| `oldChunkVisibleValue` | old | 3.6 µs | 3.6 µs | 3.6 µs | 0.0% |
| `optimizedWaypointManagerValue` | alt | 92.6 µs | 92.6 µs | 92.6 µs | 0.0% |
| `oldWaypointManagerValue` | old | 100.0 µs | 100.0 µs | 100.0 µs | 0.0% |

### 48. `PaperNativeXoroshiroPositionalDirect` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max | stability |
|---|---|---:|---:|---:|---:|
| `directFloatBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `directDoubleBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs | 0.0% |
| `oldFloatBatchSummary` | old | 1.7 µs | 1.7 µs | 1.7 µs | 0.0% |
| `oldDoubleBatchSummary` | old | 1.7 µs | 1.7 µs | 1.7 µs | 0.0% |

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
| 244.00x | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 67.0 µs | 274.5 ns |
| 3.29x | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.2 ms | 1.9 ms |
| 1.55x | `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` | `newLazyValidateSummary` | 217.9 ns | 140.6 ns |
| 1.53x | `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` | `newLazyMissingSetSummary` | 218.3 ns | 142.5 ns |
| 1.22x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs |
| 1.20x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 483.9 µs | 403.2 µs |
| 1.16x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `arithmeticSummary` | 9.3 µs | 8.0 µs |
| 1.15x | `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` | `newBatchSummary` | 6.3 µs | 5.5 µs |

## Regressions (optimized SLOWER than old) — optimization targets

| speedup | class | old kernel | optimized kernel | old | optimized |
|---|---:|---|---|---:|---:|
| 0.57x | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` | `newCachedContainsSummary` | 1.2 µs | 2.1 µs |
| 0.44x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `directPackedSummary` | 483.9 µs | 1.1 ms |
| 0.21x | `PaperNativeMarkerCache` | `oldSummary` | `cachedSummary` | 119.2 µs | 558.6 µs |
| 0.18x | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` | `newCombinedUpdateSummary` | 2.2 ms | 12.4 ms |

## Parity (within ±15%)

`PaperNativeDensityAp2Fill.scratchFlatSummary` (1.13x), `PaperNativeOwnableRule.newLoopSummary` (1.12x), `PaperNativeImprovedNoiseInline.flatGradientSummary` (1.12x), `PaperNativeWaypointHotPath.cachedChunkVisibleValue` (1.12x), `PaperNativeNoiseChunkFlatCacheContext.newFalseContextSummary` (1.10x), `PaperNativePluginDirectoryScan.newListSummary` (1.09x), `PaperNativePluginDirectoryScan.directoryStreamSummary` (1.09x), `PaperNativePluginLoadingAllocation.newPresizedSetupSummary` (1.08x), `PaperNativeWaypointHotPath.optimizedWaypointManagerValue` (1.08x), `PaperNativeAquiferIndexStride.newBatchSummary` (1.07x), `PaperNativeDensitySplineContext.newDirectSummary` (1.07x), `PaperNativeDensityAp2Fill.scratchNestedSummary` (1.07x), `PaperNativeRemapperSkipHashes.newLoopSummary` (1.07x), `PaperNativeXoroshiroPositionalDirect.directFloatBatchSummary` (1.05x), `PaperNativePluginNameLog.newArrayListSortSummary` (1.05x), `PaperNativeChunkDependencies.arraySummary` (1.04x), `PaperNativeWaypointHotPath.directAzimuthValue` (1.03x), `PaperNativeImprovedNoiseDerivative.flatGradientDerivativeSummary` (1.03x), `PaperNativeWaypointHotPath.guardedAtOrBeyondRangeValue` (1.03x), `PaperNativeClimateParameterDistance.subtractFirstDistanceSum` (1.03x), `PaperNativeXoroshiroPositionalDirect.directDoubleBatchSummary` (1.03x), `PaperNativeSurfaceRulesTestRuleState.newStateRuleSummary` (1.02x), `PaperNativeImprovedNoiseDerivative.inlineDerivativeSummary` (1.02x), `PaperNativeWaypointHotPath.guardedReallyFarValue` (1.02x), `PaperNativePluginStartupRollup.newSummary` (1.02x), `PaperNativeImprovedNoiseDerivative.intTableDerivativeSummary` (1.02x), `PaperNativeWaypointDistanceGuard.guardedReallyFarSummary` (1.01x), `PaperNativeWaypointDistanceGuard.guardedAtOrBeyondRangeSummary` (1.01x), `PaperNativeDensityAp2MinMaxFill.newSummary` (1.00x), `PaperNativeLegacyProvidedAliasRemoval.newReverseAliasRemoveSummary` (1.00x), `PaperNativeCaveCarverSkip.directHelperSummary` (1.00x), `PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary` (1.00x), `PaperNativeBlendedNoise.cachedBatchSummary` (1.00x), `PaperNativeNoiseChunkFlatCacheContext.newTrueContextSummary` (1.00x), `PaperNativeEntityLookupStatus.directAccessibleSummary` (1.00x), `PaperNativeAquiferPositionalLocation.directBatchSummary` (1.00x), `PaperNativeJigsawCanAttach.targetFirstBatchSummary` (1.00x), `PaperNativePluginMetaDependency.newLoopSummary` (1.00x), `PaperNativeObfHelperMaps.directMapsSummary` (1.00x), `PaperNativeCaveCarverSkip.reusedCheckerSummary` (1.00x), `PaperNativeJigsawCanAttach.optimizedBatchSummary` (1.00x), `PaperNativeOreFeatureLoop.optimizedLoopSummary` (1.00x), `PaperNativeCubicSplineCreate.indexSummary` (1.00x), `PaperNativeSpringFeatureMutablePos.mutableBatchSummary` (0.99x), `PaperNativeDensityVisitorHook.hookedUnwrappingSummary` (0.99x), `PaperNativeStaticCacheGet.newBatchSummary` (0.99x), `PaperNativeEntityLookupStatus.directStatusSummary` (0.99x), `PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary` (0.99x), `PaperNativeTopographicGraphSortCapacity.newPresizedSummary` (0.99x), `PaperNativeObfHelperMaps.presizedStringPoolSummary` (0.99x), `PaperNativeEntityChunkTransient.newMixedSummary` (0.98x), `PaperNativeClimateParameterDistance.branchDistanceSum` (0.98x), `PaperNativeEntityBoundingBox.directDimensionsSetSummary` (0.98x), `PaperNativePluginMetaDependency.cachedSummary` (0.98x), `PaperNativeRangeChoice.optimizedFillArraySummary` (0.98x), `PaperNativeImprovedNoiseInline.inlineByteAccessSummary` (0.97x), `PaperNativePluginClassLoaderGroup.skipRequesterSummary` (0.96x), `PaperNativeRemapperIndexCleanup.newLazyCleanupSummary` (0.96x)

## Regressions (do-not-wire)

ratio = alt/old ≥ 1.18 — beyond the ±15% noise floor. Do NOT wire these optimized variants into hot paths. '(noisy)' = pair stability > 50%.

| kernel (alt / paired old) | group | ratio | stability | verdict |
|---|---|---:|---:|---|
| `newCombinedUpdateSummary` / `oldFourUpdateSummary` | PaperNativeLevelChunkHeightmap (g19) | 5.550 | 0.0% | REGRESSION |
| `cachedSummary` / `oldSummary` | PaperNativeMarkerCache (g20) | 4.687 | 0.0% | REGRESSION |
| `directPackedSummary` / `oldNewArraySummary` | PaperNativePalettedReencodeScratch (g27) | 2.297 | 0.0% | REGRESSION |
| `newCachedContainsSummary` / `oldEnumSetForeachSummary` | PaperNativeProtoChunkHeightmap (g34) | 1.752 | 0.0% | REGRESSION |

## Wins (promotion candidates)

ratio = alt/old ≤ 0.85 (≥ ~1.18x speedup), sorted best-first. Candidates for engine hot paths; check stability before promoting.

| ratio | class | old kernel | alt kernel | old | alt | stability | verdict |
|---:|---|---|---|---:|---:|---:|---|
| 0.004 | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 67.0 µs | 274.5 ns | 0.0% | WIN |
| 0.304 | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.2 ms | 1.9 ms | 0.0% | WIN |
| 0.645 | `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` | `newLazyValidateSummary` | 217.9 ns | 140.6 ns | 0.0% | WIN |
| 0.653 | `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` | `newLazyMissingSetSummary` | 218.3 ns | 142.5 ns | 0.0% | WIN |
| 0.822 | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs | 0.0% | WIN |
| 0.833 | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 483.9 µs | 403.2 µs | 0.0% | WIN |

## JNI floor groups

Groups with at least one kernel median below 200 ns sit on the JNI-transition floor (~115 ns measured in the scaling study): per-kernel micro-optimization is pointless there — batch more work per JNI call instead (engine-level batch-API candidates).

| group | class | fastest median | kernels below 200 ns |
|---|---|---:|---|
| g18 | `PaperNativeLegacyProvidedAliasRemoval` | 191.7 ns | `newReverseAliasRemoveSummary` (191.7 ns), `oldValuesRemoveIfSummary` (192.5 ns) |
| g28 | `PaperNativePluginClassLoaderGroup` | 146.7 ns | `oldLookupSummary` (146.7 ns), `skipRequesterSummary` (153.2 ns) |
| g30 | `PaperNativePluginLoadingAllocation` | 140.6 ns | `newLazyValidateSummary` (140.6 ns), `newPresizedSetupSummary` (142.2 ns), `newLazyMissingSetSummary` (142.5 ns), `oldDefaultCapacitySetupSummary` (154.0 ns) |
| g31 | `PaperNativePluginMetaDependency` | 192.1 ns | `oldStreamSummary` (192.1 ns), `newLoopSummary` (192.5 ns), `cachedSummary` (196.6 ns) |
| g32 | `PaperNativePluginNameLog` | 112.7 ns | `newArrayListSortSummary` (112.7 ns), `oldTreesetSummary` (117.8 ns) |
| g33 | `PaperNativePluginStartupRollup` | 143.1 ns | `newSummary` (143.1 ns), `oldSummary` (145.7 ns) |
| g35 | `PaperNativeRangeChoice` | 111.9 ns | `oldFillArraySummary` (111.9 ns), `optimizedFillArraySummary` (114.6 ns) |
| g36 | `PaperNativeRemapperIndexCleanup` | 196.7 ns | `oldEagerCleanupSummary` (196.7 ns) |
| g39 | `PaperNativeSpigotLoadOrderDependency` | 114.5 ns | `newLoadAfterBuildSummary` (114.5 ns), `oldLoadAfterBuildSummary` (114.8 ns) |
| g40 | `PaperNativeSpigotLoadOrderDependency` | 193.6 ns | `oldRemovedCountSummary` (193.6 ns), `newRemovedCountSummary` (195.9 ns) |
| g42 | `PaperNativeStaticCacheGet` | 138.2 ns | `oldBatchSummary` (138.2 ns), `newBatchSummary` (139.5 ns) |

## Baseline diff vs `baseline.tsv`

Per-pair ratio change: drift = (current ratio − baseline ratio) / baseline ratio; flagged when |drift| > 20%.

| class | pair (old / alt) | baseline ratio | current ratio | drift | flag |
|---|---|---:|---:|---:|---|
| `PaperNativeAquiferIndexStride` | `oldBatchSummary` / `newBatchSummary` | 0.932 | 0.932 | +0.0% | ok |
| `PaperNativeAquiferPositionalLocation` | `oldBatchSummary` / `directBatchSummary` | 1.001 | 1.001 | +0.0% | ok |
| `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` / `newBatchSummary` | 0.869 | 0.869 | -0.0% | ok |
| `PaperNativeBlendedNoise` | `oldBatchSummary` / `cachedBatchSummary` | 0.998 | 0.998 | -0.0% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `directHelperSummary` | 0.996 | 0.996 | +0.0% | ok |
| `PaperNativeCaveCarverSkip` | `oldLambdaSummary` / `reusedCheckerSummary` | 1.004 | 1.004 | -0.0% | ok |
| `PaperNativeChunkDependencies` | `oldImmutableListSummary` / `arraySummary` | 0.964 | 0.964 | +0.0% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `branchDistanceSum` | 1.018 | 1.018 | +0.0% | ok |
| `PaperNativeClimateParameterDistance` | `oldDistanceSum` / `subtractFirstDistanceSum` | 0.973 | 0.973 | -0.0% | ok |
| `PaperNativeCubicSplineCreate` | `oldIteratorSummary` / `indexSummary` | 1.005 | 1.005 | -0.0% | ok |
| `PaperNativeDensityAp2Fill` | `oldFlatSummary` / `scratchFlatSummary` | 0.881 | 0.881 | -0.0% | ok |
| `PaperNativeDensityAp2Fill` | `oldNestedSummary` / `scratchNestedSummary` | 0.935 | 0.935 | +0.0% | ok |
| `PaperNativeDensityAp2MinMaxFill` | `oldSummary` / `newSummary` | 0.995 | 0.995 | -0.0% | ok |
| `PaperNativeDensitySplineContext` | `oldWrapperSummary` / `newDirectSummary` | 0.935 | 0.935 | -0.0% | ok |
| `PaperNativeDensityVisitorHook` | `oldUnwrappingSummary` / `hookedUnwrappingSummary` | 1.009 | 1.009 | +0.0% | ok |
| `PaperNativeEntityBoundingBox` | `oldMakeThenSetSummary` / `directDimensionsSetSummary` | 1.022 | 1.022 | +0.0% | ok |
| `PaperNativeEntityChunkTransient` | `oldMixedSummary` / `newMixedSummary` | 1.016 | 1.016 | +0.0% | ok |
| `PaperNativeEntityLookupStatus` | `oldAccessibleSummary` / `directAccessibleSummary` | 1.001 | 1.001 | +0.0% | ok |
| `PaperNativeEntityLookupStatus` | `oldStatusSummary` / `directStatusSummary` | 1.011 | 1.011 | +0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `flatGradientDerivativeSummary` | 0.969 | 0.969 | +0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `inlineDerivativeSummary` | 0.978 | 0.978 | -0.0% | ok |
| `PaperNativeImprovedNoiseDerivative` | `oldDerivativeSummary` / `intTableDerivativeSummary` | 0.983 | 0.983 | +0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `arithmeticSummary` | 0.861 | 0.861 | +0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `flatGradientSummary` | 0.893 | 0.893 | -0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `inlineByteAccessSummary` | 1.026 | 1.026 | -0.0% | ok |
| `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` / `switchGradientSummary` | 0.822 | 0.822 | +0.0% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `optimizedBatchSummary` | 1.004 | 1.004 | +0.0% | ok |
| `PaperNativeJigsawCanAttach` | `oldBatchSummary` / `targetFirstBatchSummary` | 1.001 | 1.001 | -0.0% | ok |
| `PaperNativeLegacyProvidedAliasRemoval` | `oldValuesRemoveIfSummary` / `newReverseAliasRemoveSummary` | 0.996 | 0.996 | +0.0% | ok |
| `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` / `newCombinedUpdateSummary` | 5.550 | 5.550 | -0.0% | ok |
| `PaperNativeMarkerCache` | `oldSummary` / `cachedSummary` | 4.687 | 4.687 | -0.0% | ok |
| `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` / `newEmptyBlenderSummary` | 0.004 | 0.004 | +0.0% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` / `newFalseContextSummary` | 0.909 | 0.909 | -0.0% | ok |
| `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` / `newTrueContextSummary` | 1.000 | 1.000 | -0.0% | ok |
| `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` / `flatSummary` | 0.304 | 0.304 | +0.0% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `directMapsSummary` | 1.003 | 1.003 | -0.0% | ok |
| `PaperNativeObfHelperMaps` | `oldStreamDefaultSummary` / `presizedStringPoolSummary` | 1.013 | 1.013 | +0.0% | ok |
| `PaperNativeOreFeatureLoop` | `oldLoopSummary` / `optimizedLoopSummary` | 1.005 | 1.005 | +0.0% | ok |
| `PaperNativeOwnableRule` | `oldStreamSummary` / `newLoopSummary` | 0.891 | 0.891 | -0.0% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `directPackedSummary` | 2.297 | 2.297 | +0.0% | ok |
| `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` / `scratchThreadLocalSummary` | 0.833 | 0.833 | -0.0% | ok |
| `PaperNativePluginClassLoaderGroup` | `oldLookupSummary` / `skipRequesterSummary` | 1.044 | 1.044 | +0.0% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `directoryStreamSummary` | 0.920 | 0.920 | -0.0% | ok |
| `PaperNativePluginDirectoryScan` | `oldWalkDepth1Summary` / `newListSummary` | 0.918 | 0.918 | -0.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldDefaultCapacitySetupSummary` / `newPresizedSetupSummary` | 0.923 | 0.923 | -0.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` / `newLazyMissingSetSummary` | 0.653 | 0.653 | +0.0% | ok |
| `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` / `newLazyValidateSummary` | 0.645 | 0.645 | +0.0% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `cachedSummary` | 1.023 | 1.023 | +0.0% | ok |
| `PaperNativePluginMetaDependency` | `oldStreamSummary` / `newLoopSummary` | 1.002 | 1.002 | +0.0% | ok |
| `PaperNativePluginNameLog` | `oldTreesetSummary` / `newArrayListSortSummary` | 0.957 | 0.957 | +0.0% | ok |
| `PaperNativePluginStartupRollup` | `oldSummary` / `newSummary` | 0.982 | 0.982 | +0.0% | ok |
| `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` / `newCachedContainsSummary` | 1.752 | 1.752 | -0.0% | ok |
| `PaperNativeRangeChoice` | `oldFillArraySummary` / `optimizedFillArraySummary` | 1.024 | 1.024 | -0.0% | ok |
| `PaperNativeRemapperIndexCleanup` | `oldEagerCleanupSummary` / `newLazyCleanupSummary` | 1.047 | 1.047 | -0.0% | ok |
| `PaperNativeRemapperSkipHashes` | `oldStreamSummary` / `newLoopSummary` | 0.935 | 0.935 | +0.0% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldLoadAfterBuildSummary` / `newLoadAfterBuildSummary` | 0.997 | 0.997 | -0.0% | ok |
| `PaperNativeSpigotLoadOrderDependency` | `oldRemovedCountSummary` / `newRemovedCountSummary` | 1.012 | 1.012 | +0.0% | ok |
| `PaperNativeSpringFeatureMutablePos` | `oldBatchSummary` / `mutableBatchSummary` | 1.007 | 1.007 | -0.0% | ok |
| `PaperNativeStaticCacheGet` | `oldBatchSummary` / `newBatchSummary` | 1.009 | 1.009 | -0.0% | ok |
| `PaperNativeSurfaceRulesTestRuleState` | `oldStateRuleSummary` / `newStateRuleSummary` | 0.977 | 0.977 | +0.0% | ok |
| `PaperNativeTopographicGraphSortCapacity` | `oldDefaultCapacitySummary` / `newPresizedSummary` | 1.012 | 1.012 | +0.0% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldAtOrBeyondRangeSummary` / `guardedAtOrBeyondRangeSummary` | 0.989 | 0.989 | -0.0% | ok |
| `PaperNativeWaypointDistanceGuard` | `oldReallyFarSummary` / `guardedReallyFarSummary` | 0.986 | 0.986 | +0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldAtOrBeyondRangeValue` / `guardedAtOrBeyondRangeValue` | 0.972 | 0.972 | +0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldAzimuthValue` / `directAzimuthValue` | 0.969 | 0.969 | -0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldChunkVisibleValue` / `cachedChunkVisibleValue` | 0.894 | 0.894 | +0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldReallyFarValue` / `guardedReallyFarValue` | 0.979 | 0.979 | +0.0% | ok |
| `PaperNativeWaypointHotPath` | `oldWaypointManagerValue` / `optimizedWaypointManagerValue` | 0.926 | 0.926 | -0.0% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldDoubleBatchSummary` / `directDoubleBatchSummary` | 0.973 | 0.973 | -0.0% | ok |
| `PaperNativeXoroshiroPositionalDirect` | `oldFloatBatchSummary` / `directFloatBatchSummary` | 0.952 | 0.952 | +0.0% | ok |

* compared: 70, new pairs: 0, missing: 0

