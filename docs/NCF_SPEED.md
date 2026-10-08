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

## Current table (SPEED-LEDGER-introducing commit, 2026-10-09)

Measured: noise 38.8, surface 36.2, carvers 24.3, serialization 1.0 (averages
of two runs; singles 38.49-39.10 / 36.12-36.42 / 24.27-24.37 / 0.99-1.01).

| stage         | Rust ms/chunk | Java warm (P0.1) | speedup | share of Java budget |
|---------------|--------------:|-----------------:|--------:|---------------------:|
| noise+biomes  | 38.8          | 273.8            | 7.1x    | 75.4%                |
| surface       | 36.2          | 16.8             | **0.5x**| 4.6%                 |
| carvers       | 24.3          | 1.6              | **0.1x**| 0.4%                 |
| serialization | 1.0           | 2.6              | 2.6x    | 0.7%                 |
| PORTED TOTAL  | 100.4         | 294.8            | 2.9x    | 81.1%                |

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
sum = 75.4%/7.1 + 4.6%/0.5 + 0.4%/0.1 + 0.7%/2.6 + 18.48% = 0.4611
pregen speedup = 1 / 0.4611 = 2.17x  (vs Java warm 363.0 CPU-ms/chunk)
```

The projection makes the lever priorities brutally clear: surface (0.5x) and
carvers (0.1x) are NEGATIVE levers today — they subtract from the 7.1x noise
win. Bringing carvers alone to 1x raises the projection to ~2.55x; to the
noise-stage parity (~5x feasible per P2.13 path-caching plan) raises it to
~3.1x. Fixing surface to 1x adds ~+0.5x more. The noise cluster remains the
biggest single prize (75.4% share).

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
| (this commit: ledger introduced, baseline) | 2026-10-09 | 38.8 | 36.2 | 24.3 | 1.0 | 100.4 | 2.17x |
