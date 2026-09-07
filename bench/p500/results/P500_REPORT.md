# P500 benchmark report — Crussty CE native kernels (old vs optimized)

* Generated: 2026-09-07T15:28:14+00:00
* Raw data: `p500_raw.tsv` — one JVM fork per group, time-bounded batches (~120 ms), median of 5, identical synthesized args per group.
* Hardware note: shared 2-CPU sandbox; treat <±15% deltas as parity.

* Groups measured: 49, skipped: 0, crashed: 0, kernels measured: 129

## Per-group results

### 0. `PaperNativeAquiferIndexStride` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newBatchSummary` | alt | 9.9 µs | 9.9 µs | 9.9 µs |
| `oldBatchSummary` | old | 10.6 µs | 10.6 µs | 10.6 µs |

### 1. `PaperNativeAquiferPositionalLocation` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldBatchSummary` | old | 2.2 µs | 2.2 µs | 2.2 µs |
| `directBatchSummary` | alt | 2.2 µs | 2.2 µs | 2.2 µs |

### 2. `PaperNativeAquiferSurfaceSampling` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newBatchSummary` | alt | 5.5 µs | 5.5 µs | 5.5 µs |
| `oldBatchSummary` | old | 6.3 µs | 6.3 µs | 6.3 µs |

### 3. `PaperNativeBlendedNoise` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `cachedBatchSummary` | alt | 50.6 µs | 50.6 µs | 50.6 µs |
| `oldBatchSummary` | old | 50.7 µs | 50.7 µs | 50.7 µs |

### 4. `PaperNativeCaveCarverSkip` `(I[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `directHelperSummary` | alt | 34.3 ms | 34.3 ms | 34.3 ms |
| `oldLambdaSummary` | old | 34.4 ms | 34.4 ms | 34.4 ms |
| `reusedCheckerSummary` | alt | 34.5 ms | 34.5 ms | 34.5 ms |

### 5. `PaperNativeChunkDependencies` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `arraySummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs |
| `oldImmutableListSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs |

### 6. `PaperNativeClimateParameterDistance` `([J[J[J[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `subtractFirstDistanceSum` | alt | 651.8 ns | 651.8 ns | 651.8 ns |
| `oldDistanceSum` | old | 670.1 ns | 670.1 ns | 670.1 ns |
| `branchDistanceSum` | alt | 682.3 ns | 682.3 ns | 682.3 ns |

### 7. `PaperNativeCubicSplineCreate` `(I[F[F[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldIteratorSummary` | old | 153.1 µs | 153.1 µs | 153.1 µs |
| `indexSummary` | alt | 153.8 µs | 153.8 µs | 153.8 µs |

### 8. `PaperNativeDensityAp2Fill` `(II[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `scratchFlatSummary` | alt | 11.4 µs | 11.4 µs | 11.4 µs |
| `oldFlatSummary` | old | 12.9 µs | 12.9 µs | 12.9 µs |
| `scratchNestedSummary` | alt | 19.1 µs | 19.1 µs | 19.1 µs |
| `oldNestedSummary` | old | 20.4 µs | 20.4 µs | 20.4 µs |

### 9. `PaperNativeDensityAp2MinMaxFill` `(III[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newSummary` | alt | 220.8 ns | 220.8 ns | 220.8 ns |
| `oldSummary` | old | 221.9 ns | 221.9 ns | 221.9 ns |

### 10. `PaperNativeDensitySplineContext` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newDirectSummary` | alt | 1.9 µs | 1.9 µs | 1.9 µs |
| `oldWrapperSummary` | old | 2.1 µs | 2.1 µs | 2.1 µs |

### 11. `PaperNativeDensityVisitorHook` `(III[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldUnwrappingSummary` | old | 775.0 ns | 775.0 ns | 775.0 ns |
| `hookedUnwrappingSummary` | alt | 781.9 ns | 781.9 ns | 781.9 ns |

### 12. `PaperNativeEntityBoundingBox` `(I[F[F[D[D[D[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldMakeThenSetSummary` | old | 5.7 µs | 5.7 µs | 5.7 µs |
| `directDimensionsSetSummary` | alt | 5.9 µs | 5.9 µs | 5.9 µs |

### 13. `PaperNativeEntityChunkTransient` `(IIJ[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldMixedSummary` | old | 394.8 ns | 394.8 ns | 394.8 ns |
| `newMixedSummary` | alt | 401.3 ns | 401.3 ns | 401.3 ns |

### 14. `PaperNativeEntityLookupStatus` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldAccessibleSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs |
| `directAccessibleSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs |
| `oldStatusSummary` | old | 1.1 µs | 1.1 µs | 1.1 µs |
| `directStatusSummary` | alt | 1.1 µs | 1.1 µs | 1.1 µs |

### 15. `PaperNativeImprovedNoiseDerivative` `([B[I[I[I[D[D[DI[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `flatGradientDerivativeSummary` | alt | 11.5 µs | 11.5 µs | 11.5 µs |
| `inlineDerivativeSummary` | alt | 11.6 µs | 11.6 µs | 11.6 µs |
| `intTableDerivativeSummary` | alt | 11.7 µs | 11.7 µs | 11.7 µs |
| `oldDerivativeSummary` | old | 11.9 µs | 11.9 µs | 11.9 µs |

### 16. `PaperNativeImprovedNoiseInline` `([BI[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `switchGradientSummary` | alt | 7.6 µs | 7.6 µs | 7.6 µs |
| `arithmeticSummary` | alt | 8.0 µs | 8.0 µs | 8.0 µs |
| `flatGradientSummary` | alt | 8.3 µs | 8.3 µs | 8.3 µs |
| `oldPMethodSummary` | old | 9.3 µs | 9.3 µs | 9.3 µs |
| `inlineByteAccessSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs |

### 17. `PaperNativeJigsawCanAttach` `(I[I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldBatchSummary` | old | 655.3 ns | 655.3 ns | 655.3 ns |
| `targetFirstBatchSummary` | alt | 656.1 ns | 656.1 ns | 656.1 ns |
| `optimizedBatchSummary` | alt | 658.0 ns | 658.0 ns | 658.0 ns |

### 18. `PaperNativeLegacyProvidedAliasRemoval` `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newReverseAliasRemoveSummary` | alt | 191.7 ns | 191.7 ns | 191.7 ns |
| `oldValuesRemoveIfSummary` | old | 192.5 ns | 192.5 ns | 192.5 ns |

### 19. `PaperNativeLevelChunkHeightmap` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldFourUpdateSummary` | old | 2.2 ms | 2.2 ms | 2.2 ms |
| `newCombinedUpdateSummary` | alt | 12.4 ms | 12.4 ms | 12.4 ms |

### 20. `PaperNativeMarkerCache` `(III[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldSummary` | old | 119.2 µs | 119.2 µs | 119.2 µs |
| `cachedSummary` | alt | 558.6 µs | 558.6 µs | 558.6 µs |

### 21. `PaperNativeNoiseChunkBlendCache` `(II[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newEmptyBlenderSummary` | alt | 274.5 ns | 274.5 ns | 274.5 ns |
| `oldEmptyBlenderSummary` | old | 67.0 µs | 67.0 µs | 67.0 µs |

### 22. `PaperNativeNoiseChunkFlatCacheContext` `(II[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newFalseContextSummary` | alt | 18.7 µs | 18.7 µs | 18.7 µs |
| `newTrueContextSummary` | alt | 19.0 µs | 19.0 µs | 19.0 µs |
| `oldTrueContextSummary` | old | 19.0 µs | 19.0 µs | 19.0 µs |
| `oldFalseContextSummary` | old | 20.5 µs | 20.5 µs | 20.5 µs |

### 23. `PaperNativeNoiseInterpolatorSlice` `(IIII[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `flatSummary` | alt | 1.9 ms | 1.9 ms | 1.9 ms |
| `oldJaggedSummary` | old | 6.2 ms | 6.2 ms | 6.2 ms |

### 24. `PaperNativeObfHelperMaps` `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldStreamDefaultSummary` | old | 214.0 ns | 214.0 ns | 214.0 ns |
| `directMapsSummary` | alt | 214.6 ns | 214.6 ns | 214.6 ns |
| `presizedStringPoolSummary` | alt | 216.8 ns | 216.8 ns | 216.8 ns |

### 25. `PaperNativeOreFeatureLoop` `([D[D[D[D[I[I[I[I[I[IIIIII[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldLoopSummary` | old | 2.0 µs | 2.0 µs | 2.0 µs |
| `optimizedLoopSummary` | alt | 2.0 µs | 2.0 µs | 2.0 µs |

### 26. `PaperNativeOwnableRule` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newLoopSummary` | alt | 3.8 µs | 3.8 µs | 3.8 µs |
| `oldStreamSummary` | old | 4.2 µs | 4.2 µs | 4.2 µs |

### 27. `PaperNativePalettedReencodeScratch` `(I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `scratchThreadLocalSummary` | alt | 403.2 µs | 403.2 µs | 403.2 µs |
| `oldNewArraySummary` | old | 483.9 µs | 483.9 µs | 483.9 µs |
| `directPackedSummary` | alt | 1.1 ms | 1.1 ms | 1.1 ms |

### 28. `PaperNativePluginClassLoaderGroup` `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldLookupSummary` | old | 146.7 ns | 146.7 ns | 146.7 ns |
| `skipRequesterSummary` | alt | 153.2 ns | 153.2 ns | 153.2 ns |

### 29. `PaperNativePluginDirectoryScan` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newListSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms |
| `directoryStreamSummary` | alt | 1.3 ms | 1.3 ms | 1.3 ms |
| `oldWalkDepth1Summary` | old | 1.5 ms | 1.5 ms | 1.5 ms |

### 30. `PaperNativePluginLoadingAllocation` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newLazyValidateSummary` | alt | 140.6 ns | 140.6 ns | 140.6 ns |
| `newPresizedSetupSummary` | alt | 142.2 ns | 142.2 ns | 142.2 ns |
| `newLazyMissingSetSummary` | alt | 142.5 ns | 142.5 ns | 142.5 ns |
| `oldDefaultCapacitySetupSummary` | old | 154.0 ns | 154.0 ns | 154.0 ns |
| `oldEagerValidateSummary` | old | 217.9 ns | 217.9 ns | 217.9 ns |
| `oldEagerMissingSetSummary` | old | 218.3 ns | 218.3 ns | 218.3 ns |

### 31. `PaperNativePluginMetaDependency` `(I[Ljava/lang/Object;[Z[Z[I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldStreamSummary` | old | 192.1 ns | 192.1 ns | 192.1 ns |
| `newLoopSummary` | alt | 192.5 ns | 192.5 ns | 192.5 ns |
| `cachedSummary` | alt | 196.6 ns | 196.6 ns | 196.6 ns |

### 32. `PaperNativePluginNameLog` `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newArrayListSortSummary` | alt | 112.7 ns | 112.7 ns | 112.7 ns |
| `oldTreesetSummary` | old | 117.8 ns | 117.8 ns | 117.8 ns |

### 33. `PaperNativePluginStartupRollup` `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newSummary` | alt | 143.1 ns | 143.1 ns | 143.1 ns |
| `oldSummary` | old | 145.7 ns | 145.7 ns | 145.7 ns |

### 34. `PaperNativeProtoChunkHeightmap` `(I)J`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldEnumSetForeachSummary` | old | 1.2 µs | 1.2 µs | 1.2 µs |
| `newCachedContainsSummary` | alt | 2.1 µs | 2.1 µs | 2.1 µs |

### 35. `PaperNativeRangeChoice` `([D[I[I[II[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldFillArraySummary` | old | 111.9 ns | 111.9 ns | 111.9 ns |
| `optimizedFillArraySummary` | alt | 114.6 ns | 114.6 ns | 114.6 ns |

### 36. `PaperNativeRemapperIndexCleanup` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldEagerCleanupSummary` | old | 196.7 ns | 196.7 ns | 196.7 ns |
| `newLazyCleanupSummary` | alt | 205.9 ns | 205.9 ns | 205.9 ns |

### 37. `PaperNativeRemapperSkipHashes` `(ILjava/lang/String;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newLoopSummary` | alt | 33.3 µs | 33.3 µs | 33.3 µs |
| `oldStreamSummary` | old | 35.6 µs | 35.6 µs | 35.6 µs |

### 38. `PaperNativeServerEntityDeltaIdentity` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldDistanceSummary` | old | 2.4 µs | 2.4 µs | 2.4 µs |

### 39. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newLoadAfterBuildSummary` | alt | 114.5 ns | 114.5 ns | 114.5 ns |
| `oldLoadAfterBuildSummary` | old | 114.8 ns | 114.8 ns | 114.8 ns |

### 40. `PaperNativeSpigotLoadOrderDependency` `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldRemovedCountSummary` | old | 193.6 ns | 193.6 ns | 193.6 ns |
| `newRemovedCountSummary` | alt | 195.9 ns | 195.9 ns | 195.9 ns |

### 41. `PaperNativeSpringFeatureMutablePos` `(I[I[I[I[Z[I[I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldBatchSummary` | old | 2.9 µs | 2.9 µs | 2.9 µs |
| `mutableBatchSummary` | alt | 3.0 µs | 3.0 µs | 3.0 µs |

### 42. `PaperNativeStaticCacheGet` `(IIIII[I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldBatchSummary` | old | 138.2 ns | 138.2 ns | 138.2 ns |
| `newBatchSummary` | alt | 139.5 ns | 139.5 ns | 139.5 ns |

### 43. `PaperNativeSurfaceRulesTestRuleState` `(II[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `newStateRuleSummary` | alt | 1.2 µs | 1.2 µs | 1.2 µs |
| `oldStateRuleSummary` | old | 1.3 µs | 1.3 µs | 1.3 µs |

### 44. `PaperNativeTopographicGraphSortCapacity` `(I[I[I[I[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `oldDefaultCapacitySummary` | old | 375.3 ns | 375.3 ns | 375.3 ns |
| `newPresizedSummary` | alt | 379.8 ns | 379.8 ns | 379.8 ns |

### 45. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `guardedAtOrBeyondRangeSummary` | alt | 9.5 µs | 9.5 µs | 9.5 µs |
| `oldAtOrBeyondRangeSummary` | old | 9.6 µs | 9.6 µs | 9.6 µs |

### 46. `PaperNativeWaypointDistanceGuard` `(I[D[D[D[D[D[D[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `guardedReallyFarSummary` | alt | 8.4 µs | 8.4 µs | 8.4 µs |
| `oldReallyFarSummary` | old | 8.5 µs | 8.5 µs | 8.5 µs |

### 47. `PaperNativeWaypointHotPath` `(I)D`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `guardedReallyFarValue` | alt | 1.0 µs | 1.0 µs | 1.0 µs |
| `oldReallyFarValue` | old | 1.0 µs | 1.0 µs | 1.0 µs |
| `guardedAtOrBeyondRangeValue` | alt | 1.2 µs | 1.2 µs | 1.2 µs |
| `oldAtOrBeyondRangeValue` | old | 1.3 µs | 1.3 µs | 1.3 µs |
| `directAzimuthValue` | alt | 2.1 µs | 2.1 µs | 2.1 µs |
| `oldAzimuthValue` | old | 2.2 µs | 2.2 µs | 2.2 µs |
| `cachedChunkVisibleValue` | alt | 3.2 µs | 3.2 µs | 3.2 µs |
| `oldChunkVisibleValue` | old | 3.6 µs | 3.6 µs | 3.6 µs |
| `optimizedWaypointManagerValue` | alt | 92.6 µs | 92.6 µs | 92.6 µs |
| `oldWaypointManagerValue` | old | 100.0 µs | 100.0 µs | 100.0 µs |

### 48. `PaperNativeXoroshiroPositionalDirect` `(I[I[I[IJJ[J)I`

| kernel | kind | median ns/op | min | max |
|---|---|---:|---:|---:|
| `directFloatBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs |
| `directDoubleBatchSummary` | alt | 1.6 µs | 1.6 µs | 1.6 µs |
| `oldFloatBatchSummary` | old | 1.7 µs | 1.7 µs | 1.7 µs |
| `oldDoubleBatchSummary` | old | 1.7 µs | 1.7 µs | 1.7 µs |

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

