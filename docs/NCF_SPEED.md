# NCF SPEED LEDGER

End-to-end speed accounting for the native chunk factory, per the owner
directive of 2026-10-09 (Job 441690, WORK LIST item 1). Every speed-lever
commit (P2.15, P2.16, P2.13, ...) MUST append a history row here with its own
commit hash and before/after ms/chunk. A lever that gains less than 5% is
reverted and recorded with the numbers (rule: no unfalsifiable claims).

## Method

`cargo build --release` then, from `chunk-factory/`:

```
NCF_WG=<worldgen extract> NCF_DATA_ROOT=<same> ./target/release/bench 3053459 169 ledger
```

- fixed corpus: 169 chunks = 13x13 (chunks 0..13 x 0..13), seed 3053459;
- warm: 3 throwaway full-chain chunks first (allocator/caches paged);
- single core, release profile, no FMA (C3/T7 holds by CI check);
- stages timed IN the chain: `generate_noise_chunk_with_beardifier` (noise),
  `apply_surface_pass` (surface), `apply_carvers_pass` (carvers),
  `full_chunk_nbt` (serialization, NBT build only — no zlib, no disk IO);
- features are NOT ported into the chain yet (P4 tier, default-off) — they do
  not appear in the Rust column.

Repeatability: two back-to-back runs agree within 2% on every stage.

## Current table (shared-biome-tree lever, 2026-10-09)

Measured: noise 23.58, surface 21.06, carvers 5.43, serialization 1.02 (averages
of two runs; singles 23.48-23.67 / 21.04-21.07 / 5.42-5.44 / 1.01-1.02).

| stage         | Rust ms/chunk | Java warm (P0.1) | speedup | share of Java budget |
|---------------|--------------:|-----------------:|--------:|---------------------:|
| noise+biomes  | 23.6          | 273.8            | 11.6x   | 75.4%                |
| surface       | 21.1          | 16.8             | **0.8x**| 4.6%                 |
| carvers       | 5.4           | 1.6              | **0.3x**| 0.4%                 |
| serialization | 1.0           | 2.6              | 2.5x    | 0.7%                 |
| PORTED TOTAL  | 51.1          | 294.8            | 5.8x    | 81.1%                |

Stage-to-stage mapping honesty:
- Rust "noise+biomes" = filler (density + aquifer + ore veins + biome fill +
  section content) == Java "noise+аквиферы+ore veins" (248.1) + "биомы"
  (25.7); the filler does both, so the cluster is the honest unit.
- Rust "serialization" = in-memory `full_chunk_nbt` build (DataVersion 4556);
  Java's 2.6 CPU-ms includes compression. Our number excludes zlib/disk, so
  2.6x is an upper bound for that stage.
- Rust "surface"/"carvers" include no Java-side scheduling overhead, matching
  the P0.1 stage buckets directly.

## Amdahl projection

Formula (owner directive): `pregen speedup = 1 / sum(share_i / speedup_i)`,
unported stages at 1x. Shares are the P0.1 warm CPU-ms shares of the 363.0
CPU-ms/chunk total; unported = features 4.8 + light 4.3 + scheduler 9.1 +
jvm_other 48.9 = 67.1 CPU-ms = 18.48% at 1x.

Current state:

```
sum = 75.4%/11.6 + 4.6%/0.8 + 0.4%/0.3 + 0.7%/2.5 + 18.48% = 0.3275
pregen speedup = 1 / 0.3275 = 3.07x  (vs Java warm 363.0 CPU-ms/chunk)
```

Lever priorities after the shared-biome-tree win: the noise cluster is still
the biggest prize (75.4% share, 11.6x). Surface (0.8x) is now the biggest
NEGATIVE lever — reaching 1x adds ~+0.15x; the P2.13 SIMD path toward ~5x on
noise adds ~+0.9x; carvers to 1x adds ~+0.03x (P2.15 path cache /aquifer share
diminished after this commit: the remaining 5.4 ms is ~2.8 aquifer build +
~2.2 walk+carve).

## Vanilla Java warm reference (P0.1, measured)

jcmd+JFR shares x cpu_burst; Purpur 1.21.10-2535, seed 3053459, burst 128
chunks, 2 vCPU, -Xmx1536m; warm = 2nd burst of the same JVM:

| Java stage                  | warm CPU-ms/chunk | share |
|-----------------------------|------------------:|------:|
| noise+aquifers+ore veins    | 248.1             | 68.3% |
| biomes                      | 25.7              | 7.1%  |
| surface                     | 16.8              | 4.6%  |
| carvers                     | 1.6               | 0.4%  |
| features                    | 4.8               | 1.3%  |
| light                       | 4.3               | 1.2%  |
| NBT+compression             | 2.6               | 0.7%  |
| scheduler/ChunkStatus       | 9.1               | 2.5%  |
| jvm_other (GC/JIT+unmatched)| 48.9              | 13.5% |
| TOTAL                       | 363.0             | 100%  |

## History (one row per commit that moves a number)

| commit | date | noise+biomes | surface | carvers | serialization | ported total | Amdahl |
|--------|------|-------------:|--------:|--------:|--------------:|-------------:|-------:|
| 98197f6 (ledger introduced, baseline) | 2026-10-09 | 38.8 | 36.2 | 24.3 | 1.0 | 100.4 | 2.17x |
| this commit (shared biome tree + carver refs cache; parent 98197f6) | 2026-10-09 | 23.6 | 21.1 | 5.4 | 1.0 | 51.1 | 3.07x |
