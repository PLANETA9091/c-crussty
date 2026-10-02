# LIFECYCLE REPORT — ImprovedNoiseNativeOps old vs new (P500-style)

- OLD: Handle.finalize() + ONE global synchronized WeakHashMap (pre-TASK-01 shipped code)
- NEW: PhantomReference reaper thread + 16 identity-striped WeakHashMaps (TASK-01/09)
- Same native kernels (nativeBuildHandle/nativeNoise/nativeFreeHandle, libpaper_native_jni.so)
- -Xms512m -Xmx512m, one JVM per (impl, scenario), best-of-5 batches (hotpath)

## M1 hot path: ns per noise() under contention — `hotpath`

| metric | old | new | new/old |
|--------|-----|-----|---------|
| ns_per_noise_t1 | 43.4 | 49.3 | 1.14x |
| ns_per_noise_t2 | 150.2 | 79.6 | 0.53x |
| ns_per_noise_t4 | 136.8 | 70.4 | 0.51x |
| ns_per_noise_t8 | 124.7 | 79.3 | 0.64x |
| map_entries | 256 | 256 | 1.00x |
| SINK | 4682456950456656592 | 4681631389343908192 | 1.00x |

## M2/M4 churn: 20k handles built then dropped — `churn`

| metric | old | new | new/old |
|--------|-----|-----|---------|
| ns_per_build_and_sample | 1505 | 1799 | 1.20x |
| handles_built | 20000 | 20000 | 1.00x |
| ns_per_drop_wave | 257973 | 262717 | 1.02x |
| quiet_reclaim_ms | TIMEOUT>12000 | 21 | n/a |
| tickled_reclaim_ms | 540 | 0 | 0.00x |
| freed_total | 20000 | 20000 | 1.00x |
| native_unfreed | 0 | 0 | n/a |
| map_entries_after_settle | 0 | 0 | n/a |
| gc_collections | 4 | 3 | 0.75x |
| gc_time_ms | 36 | 57 | 1.58x |
| SINK | -4606918185924497542 | -4606918185924497542 | 1.00x |

## M3 churn + parallel junk allocator (GC pressure) — `gcchurn`

| metric | old | new | new/old |
|--------|-----|-----|---------|
| ns_per_build_and_sample | 3334 | 2476 | 0.74x |
| handles_built | 20000 | 20000 | 1.00x |
| ns_per_drop_wave | 289018 | 255166 | 0.88x |
| quiet_reclaim_ms | TIMEOUT>12000 | 35 | n/a |
| tickled_reclaim_ms | 554 | 0 | 0.00x |
| freed_total | 20000 | 20000 | 1.00x |
| native_unfreed | 0 | 0 | n/a |
| map_entries_after_settle | 0 | 0 | n/a |
| gc_collections | 512 | 21 | 0.04x |
| gc_time_ms | 523 | 51 | 0.10x |
| SINK | 4720591072632200443 | 4699756881624407892 | 1.00x |
