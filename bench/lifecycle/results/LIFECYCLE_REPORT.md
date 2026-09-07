# LIFECYCLE REPORT — ImprovedNoiseNativeOps old vs new (P500-style)

- OLD: Handle.finalize() + ONE global synchronized WeakHashMap (pre-TASK-01 shipped code)
- NEW: PhantomReference reaper thread + 16 identity-striped WeakHashMaps (TASK-01/09)
- Same native kernels (nativeBuildHandle/nativeNoise/nativeFreeHandle, libpaper_native_jni.so)
- -Xms512m -Xmx512m, one JVM per (impl, scenario), best-of-5 batches (hotpath)

## M1 hot path: ns per noise() under contention — `hotpath`

| metric | old | new | new/old |
|--------|-----|-----|---------|
| ns_per_noise_t1 | 43.5 | 49.1 | 1.13x |
| ns_per_noise_t2 | 156.3 | 74.2 | 0.47x |
| ns_per_noise_t4 | 145.8 | 65.7 | 0.45x |
| ns_per_noise_t8 | 135.4 | 73.2 | 0.54x |
| map_entries | 256 | 256 | 1.00x |
| SINK | 4682631347506906941 | 4681813568048679600 | 1.00x |

## M2/M4 churn: 20k handles built then dropped — `churn`

| metric | old | new | new/old |
|--------|-----|-----|---------|
| ns_per_build_and_sample | 1409 | 1293 | 0.92x |
| handles_built | 20000 | 20000 | 1.00x |
| ns_per_drop_wave | 261576 | 255780 | 0.98x |
| quiet_reclaim_ms | TIMEOUT>12000 | 15 | n/a |
| tickled_reclaim_ms | 542 | 0 | 0.00x |
| freed_total | 20000 | 20000 | 1.00x |
| native_unfreed | 0 | 0 | n/a |
| map_entries_after_settle | 0 | 0 | n/a |
| gc_collections | 4 | 3 | 0.75x |
| gc_time_ms | 40 | 37 | 0.93x |
| SINK | -4606918185924497542 | -4606918185924497542 | 1.00x |

## M3 churn + parallel junk allocator (GC pressure) — `gcchurn`

| metric | old | new | new/old |
|--------|-----|-----|---------|
| ns_per_build_and_sample | 4621 | 3239 | 0.70x |
| handles_built | 20000 | 20000 | 1.00x |
| ns_per_drop_wave | 279768 | 254970 | 0.91x |
| quiet_reclaim_ms | TIMEOUT>12000 | 20 | n/a |
| tickled_reclaim_ms | 561 | 0 | 0.00x |
| freed_total | 20000 | 20000 | 1.00x |
| native_unfreed | 0 | 0 | n/a |
| map_entries_after_settle | 0 | 0 | n/a |
| gc_collections | 481 | 20 | 0.04x |
| gc_time_ms | 405 | 50 | 0.12x |
| SINK | 4720286705479802107 | 4699113942200065876 | 1.00x |
