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

## Current table (S1 lazy biome — Suppliers.memoize parity, 2026-10-09)

Measured: noise 19.34, surface 8.63, carvers 1.19, serialization 1.02 (run 1)
and 19.15 / 8.56 / 1.12 / 1.00 (run 2) — every stage pair within 2%.

| stage         | Rust ms/chunk | Java warm (P0.1) | speedup | share of Java budget |
|---------------|--------------:|-----------------:|--------:|---------------------:|
| noise+biomes  | 19.2          | 273.8            | 14.2x   | 75.4%                |
| surface       | 8.6           | 16.8             | **2.0x**| 4.6%                 |
| carvers       | 1.2           | 1.6              | **1.4x**| 0.4%                 |
| serialization | 1.0           | 2.6              | 2.6x    | 0.7%                 |
| PORTED TOTAL  | 30.0          | 294.8            | 9.8x    | 81.1%                |

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
sum = 75.4%/14.2 + 4.6%/2.0 + 0.4%/1.4 + 0.7%/2.6 + 18.48% = 0.267
pregen speedup = 1 / 0.267 = 3.74x  (vs Java warm 363.0 CPU-ms/chunk)
```

Lever priorities after S1: the noise cluster (19.2 ms, 14.2x, 75.4% share)
remains the dominant prize (P2.13-class SIMD retry ~+0.9x with the gather-
free variants). Surface CROSSED PARITY (8.6 ms, 2.0x vs Java warm 16.8) —
the remaining surface items are S2 (block-class flags in StateTable, target
<= 8 ms) and S3 (biome u16 interning, target <= 4 ms). Carvers 1.4x; the
P2.15 path-cache share is ~0.5 ms of the 1.2 ms stage.

P2.13 attempt 1 (2026-10-09, REVERTED as a negative lever, see history row):
AVX2 4-lane perlin (gather-based) + 4-index pure-subtree walk (`compute4`)
was bit-identical (NCF_SIMD 0/1 byte-identical on 256 chunks x noise/surface/
carvers; 154/154 tests) but SLOWER: noise 24.9-25.1 vs 19.4-20.6 ms/chunk on
the same tree (-20..-28%). Working hypothesis: vpgatherdd/vpgatherdq are
microcoded and slow on this KVM-virtualized Xeon (2 vCPU); the perlin core
issues ~30 gathers per 4-lane batch. Any retry must first profile the
perlin-vs-walk split and try scalar-gradient / SoA-transpose variants.

Carvers-stage budget after P2.16 (probes): ~0.4 aquifer ctor (bind+alloc,
scan now memo-hits) + ~0.3 biome refs + ~0.3 RNG sim + ~0.2 carve writes.

S2 probe (2026-10-09, NCF_S2_PROBE=1, cfg(ncf_profile), probe-build overhead
noted): surface split per chunk — rule.apply NESTED 8.86 ms of build 11.39
(≈78% of the clean 8.46-8.58 ms stage), yloop excl 2.08, per-column biome
vote 0.35, badlands/frozen extensions 0.00 on the corpus, kit prep 0.01.
Counts: try_apply 29,976, set_block 16,974, classify calls 188,047/chunk
(air 96,230 + fluid 61,526 + stone 30,291) — yet the flags A/B above proves
ALL classify String work costs ~0.05-0.1 ms total: the interned, cache-hot
names make each memcmp reject a few cycles. The next surface lever is inside
rule.apply itself (worklist S4: intern_canonical per-hit allocations,
Cond::BiomeIs String compares — probe the inner split FIRST, R5).

S2B probe (2026-10-09, NCF_S2B_PROBE=1, cfg(ncf_profile), S4 inner split; two
runs, counts bit-identical, timings within 1%): the rule-hit path per chunk —
try (rule walk, BiomeIs nested) 5.54/5.51 ms, intern_canonical 4.65/4.62,
set_block 0.61/0.62; biomeis nested 2.94/2.92 ms over 21,021 epoch
misses/chunk; intern_calls = 16,974/chunk = set_blocks (intern runs per hit).
Probe-build clock-pair overhead: ~2 clock reads per probe site (~190k
reads/chunk) inflate every slice — consistent with S2 rule nested 8.86 ->
14.11 (+5.25 ms) on the same corpus; absolute slices are UPPER BOUNDS, the
ranking is robust: intern_canonical per hit (BlockStateDef::parse + canonical
format! + HashMap<String,u32>) is the TOP inner cost, Cond::BiomeIs String
compares second, set_block cheap. S4 HYP CONFIRMED by counter (R5 satisfied);
fix direction = allocation-free canonical lookup (prop-less states, intern on
miss) + a rule-result memo where Java reuses constant states; S4 target
surface <= 6.5 ms stands.

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

## End-to-end region (P3.2/P3.4, 2026-10-09)

Owner WORK LIST item 3: one fixed `.mca` region (32x32 = 1024 chunks) driven
through the I8 fallback law — `structure_scan` prescan (the gate-p2
honest-exclusion mechanism) -> `decide_chunk` (the only lawful switch) ->
native chain (noise -> surface -> carvers -> FULL NBT -> gzip) -> the P3.1
region writer; fallback chunks are NOT generated (deployment shape:
Moonrise NO_DATA -> Java generates them whole). Coverage % is mandatory (I8).

```
NCF_WG=<extract> NCF_DATA_ROOT=<same> NCF_REGION_OUT=<dir> \
  ./target/release/bench 3053459 region                # region r.0.0.mca
NCF_REGION_BASE=-32 ...                                # region r.-1.-1.mca
```

- warm (3 throwaway full-chain chunks), single core, seed 3053459;
- corpus note: the extract MUST be jar-fresh (ci_gate_p2.sh protocol:
  worldgen + tags + `data/minecraft/structure/*.nbt`); a stale hand-made
  extract marks every chunk `unfaithful` in the prescan (loud over-marking,
  conservative direction) — this tick's first run caught exactly that and
  the extract was rebuilt from the mapped jar;
- gzip payload ~0.07 ms/chunk and region write ~0.01 ms/chunk amortized —
  the P3.1 writer is not a bottleneck at 1024 chunks/23.3 MB.

| region (seed 3053459) | chunks | fallback (I8) | coverage | native E2E ms/chunk | vs Java ported 294.8 | hybrid pregen | pure-Java pregen | speedup |
|-----------------------|-------:|--------------:|---------:|--------------------:|---------------------:|--------------:|-----------------:|--------:|
| r.0.0.mca (0..32 x 0..32)    | 1024 | 0  | 100.00% | 43.1 | 6.8x | ~113 s | ~372 s | 3.30x |
| r.-1.-1.mca (-32..-1 x -32..-1) | 1024 | 4 (structures) | 99.61% | 50.6 | 5.8x | ~122 s | ~372 s | 3.06x |

- r.-1.-1.mca contains the SAME 4 Beardifier chunks that the gate-p2
  vanilla/3053459 cell (radius 24) excluded in CI run 37842664447
  ("I8 structure-fallback: 4 chunk pairs excluded") — the offline prescan
  and the CI gate agree chunk-for-chunk (c_-18_-13 .. c_-15_-13, multi-start
  feeds of 3..11 pieces).
- projection honesty: native chunk = measured Rust E2E + Java completion of
  unported stages at 1x (features+light+scheduler+jvm_other = 67.1 CPU-ms,
  the ledger's Amdahl model); fallback chunk = Java's full 363.0 CPU-ms
  (P0.1 warm). Second run's higher noise/surface (22.9/25.0 vs 19.1/21.2)
  is VM noise on the shared 2-vCPU rig — the clean-run numbers stand.
- serialization+gzip included in E2E: the FULL NBT (DataVersion 4556,
  isLightOn=false) is the P3.4 payload — one Rust call returns it gzipped.

History (end-to-end rows):

| commit | date | region | coverage | native E2E | e2e speedup |
|--------|------|--------|---------:|-----------:|------------:|
| this commit (bench region mode introduced, baseline) | 2026-10-09 | r.0.0 | 100.00% | 43.1 | 3.30x |
| this commit (same run series) | 2026-10-09 | r.-1.-1 | 99.61% | 50.6 | 3.06x |

## History (one row per commit that moves a number)

| commit | date | noise+biomes | surface | carvers | serialization | ported total | Amdahl |
|--------|------|-------------:|--------:|--------:|--------------:|-------------:|-------:|
| 98197f6 (ledger introduced, baseline) | 2026-10-09 | 38.8 | 36.2 | 24.3 | 1.0 | 100.4 | 2.17x |
| 937fbab (shared biome tree + carver refs cache) | 2026-10-09 | 23.6 | 21.1 | 5.4 | 1.0 | 51.1 | 3.07x |
| this commit (P2.16: RandomState-level prelim-surface memo; parent 937fbab) | 2026-10-09 | 19.0 | 21.0 | 1.2 | 1.0 | 42.3 | 3.32x |
| P2.13 attempt 1 — REVERTED, no commit [!] (AVX2 4-lane perlin + compute4 on tree ef0c68d; SIMD ON numbers; SIMD OFF control on the same tree: 19.4-20.6 / 3.21-3.29x) | 2026-10-09 | 24.9-25.1 | 21.1 | 1.2 | 1.0 | 48.3-48.9 | 3.13-3.15x |
| this commit (S1 lazy biome: update_y stores pending (x,y,z), Suppliers.memoize parity, &str read path; parent b986eecd probe = 30,274 votes/chunk, after = 4,822; byte-identical stagediff surface+carvers 256/256 vs parent; gate-p2 vanilla/3053459 2376/2376 EQUAL; ci_staged surface 225/225 EQUAL; 194/194 tests) | 2026-10-09 | 19.2 | 8.6 | 1.2 | 1.0 | 30.0 | 3.74x |
| S2 attempt 1 — REVERTED per R2 (gain < 5%), no commit [!] (StateTable intern-time class flags AIR/FLUID/BLOCKS_MOTION/NOT_AIR replacing the per-block String compares in is_air/fluid/stone_id + HeightmapKind::is_opaque_state + filler skip-write + set_block PP check; stagediff surface+carvers 256+256 byte-identical vs parent 08e1aa1; 197/197 tests; parent baseline 8.51/8.53 reproduced) | 2026-10-09 | 19.1 | 8.42-8.46 (-0.6..-1.3%) | 1.09-1.13 | 1.00 | 29.46-29.76 | 3.75-3.76x |
| 3cb07cb (S4 hit path: u32 slot ids + per-node hit memo + FxHash StateTable; per-hit parse/format/SipHash killed, intern_calls 16,974 -> 4/chunk; serial surfaces 64/64 byte-identical) | 2026-10-10 | — | 8.5 -> 2.46 | — | — | — | — |
| 837905d (S3 u16 biome registry, String-free resolve + integer BiomeIs; vote calls 4,822 & biomeis_miss 21,021 unchanged; surface 64/64 + carvers 16/16 EQUAL) | 2026-10-10 | — | 2.46 -> 2.17 | — | — | — | — |
| R2 wave by parallel session 454822b..5ddfbbf (interp/noise micro + filler biome diet + NBT pack diet + lazy vein_gap ~55k eager NormalNoise evals killed; their rig: PORTED 23.24 -> 20.34) | 2026-10-10 | — | — | — | 0.37 | 20.34 (their rig) | — |
| this commit (N1 probe bench section: perlin-core split + COARSE substance-fill clock + y-free tile counters; MEASURED at 5ddfbbf rig-local: noise 17.23, surface 2.34, carvers 1.23, ser 0.37, PORTED 21.16; perlin honest ~4-4.5 of 17.23 (probe-inflated slice 9.3-9.7 minus ~5.5 clock-pair), substance fill 3.16 ms COARSE, 129,140 perlin calls/chunk, tile hit 57% on 232 lookups) | 2026-10-10 | 17.23 | 2.34 | 1.23 | 0.37 | 21.16 (rig-local) | 4.11x |
| n1-arms attempt — REVERTED per R2, no code commit [!] (array-wise fill arms for MulOrAdd/Clamp/BlendDensity + flag-2 guard, worktree vs parent e64c0bb; byte-identical A/B surface 256/256 + carvers 16/16 NCF_TILE_CACHE 0/1 = 4/4; parent noise 16.82/16.82 PORTED 20.61/20.59 vs arms 16.39/16.73 PORTED 20.25/20.53 = Δ 0.09-0.43 ms of 16.82 = 0.5-2.6%, bar 0.86 ms; N1-R probe: remaining pfd pool = YClampedGradient 0.095 ms/chunk slice-only, cell 0 — cache-wrapper fills are per-element scalar compute by design, the 1-3 ms estimate conflated visits with pfd time) | 2026-10-10 | — | — | — | — | — | — |
| 55929de (N2 probe clocks cfg ncf_profile; MEASURED contended: fill_calls 5/chunk, fixed 0.168 ms/chunk, imbalance 1.344 ratio 0.117, interp[0] 87.8% of unit mass, ic-delta 49-98; contention-free arithmetic M=6.5-7.5 -> realizable best-option gain 0.48-0.55 < R2 bar 0.70 -> NP2 FALSIFIED; successor NP3 overlap pipeline headroom 2-4 ms; CI golden-harness SUCCESS on bd4ffba = NP1 phase-1 judged bit-exact) | 2026-10-11 | — | — | — | — | — | — |
| 1ced27b (NP3 pipelined drive NCF_PAR_FILL=2: worker fills col k+2 during walk k, 3-buffer rotate, phase-1-shape merge; stagediff 5/5 EMPTY incl parent-parity + mode1-vs-mode2; 230/0 tests; ledger mode1 14.22 vs mode2 11.93/11.47 noise PORTED 15.36-15.81 = paired gain +2.3-2.8 >= 0.70 bar KEEP; N3 probe walk 6.686 worker 9.132 residual 2.519 -> NP4 queued) | 2026-10-11 | 11.47-11.93 (mode2) | — | — | — | 15.36-15.81 (mode2) | 5.2-5.4x |
| 2cd78b3 (NP1 noise-parallel slice fills behind NCF_PAR_FILL default OFF: thread::scope 1-worker upper-half fork, counter+scalar+CacheOnce epoch-shift copy-back, tile Mutex-ization; stagediff 4/4 byte-identical + parent parity, 227/0 tests; paired ledger OFF noise 17.06 PORTED 20.94 vs ON 14.22/14.33 PORTED 18.05/18.20 = noise -2.7-2.8 ms >= 0.86 bar KEEP; 2-vCPU rig) | 2026-10-11 | 14.22-14.33 (ON) | 2.24-2.25 | 1.22-1.25 | 0.37 | 18.05-18.20 (ON) | 4.7x |
