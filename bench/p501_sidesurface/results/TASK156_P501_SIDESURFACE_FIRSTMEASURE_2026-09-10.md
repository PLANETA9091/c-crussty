# TASK-156 — P501 SIDESURFACE FIRST MEASUREMENT (agent-7625532f, 2026-09-10)

Status: **FIRST MEASUREMENT COMPLETE** — the 94 zero-evidence exports of the
TASK-155 census (§103 pre-registration) measured in a self-contained sidecar
rig. 49 groups / 37 classes attempted; 86 OK/SLOW rows, 0 ERR, 7 CRASH
groups, 42 live JVM forks. Canonical `bench/p500/` untouched.

## Provenance and pattern identity

`gen_p501_bench.py` imports `bench/p500/gen_p500_bench.py` and reuses
`load_rows/parse_params/stub_source/group_class/GROUP_IFACE/BENCH` verbatim —
the emitted driver is the canonical `Bench.java` **byte-for-byte** (same
`p500` package namespace in the sidecar tree: WARM=2, ROUNDS=5, ~120 ms
time-bounded batches, min-of-two-medians, DCE-proof sink, argument-strategy
ladder s0–s3, one JVM per group, 600 s timeout, crash-retry ladder N=16→1).
Only the group table differs: groups selected by the census zero-evidence set
(all-requirement; generator asserts selection == 94). Selection is
group-granular-safe: P500 coverage is (fqcn,sig)-granular and census evidence
is class-granular, so uncovered groups are all-or-nothing.

## Environment

```
javac: /home/z/bin/javac (ToolProvider shim — post-wipe #2 discipline, TASK-151)
jvm:   openjdk 21.0.12.1 (system Debian, same upstream as wiped Temurin)
N:     256 (default arg-array size; crash ladder retries N=16, N=1)
libs:  native/libpaper_native_jni.so + native/libpaper_native_chunk_encode_jni.so
```

Execution: smoke g0, then chunks 1–3 with `P501_APPEND=1` (chunked foreground
runs, canonical discipline). No server, no boots — INJECTS-ONLY.

## Headline results

### §103-registered in-group pair ratios (the only direction claims)

| group | pair | ratio (base/opt) | numbers |
|---|---|---:|---|
| g0 BeardifierBury | current/optimized | **1.00x** | 1.63us vs 1.62us |
| g1 BiomeGetBiome | current/optimized | **1.10x** | 18.0us vs 16.4us |
| g2 CarverIteration | foreach/indexed | **0.99x** | 212.9ns vs 214.4ns |
| g34 YClampedGradient | current/optimized | **1.00x** | 1.7us vs 1.7us |

No material wins on the registered conventions; BiomeGetBiome optimized is a
modest 1.10x. These are FIRST measurements — no baseline, no drift claims,
no wiring.

### Notable solo measurements (numbers only, no direction claims)

- g6 CompressionThresholdShape: **80.3 ms/op** — extreme outlier on n=256
  synthetic arrays; plausible super-linear scaling with the synthetic size
  (owner-review candidate, closed-source kernel).
- g5 ChunkTicketStage runBatch: 190.9us. g21 PalettedReencodeRemapCache:
  cachedPaletteIds 617.9us vs currentPreviousOnly 782.2us side-by-side
  (convention-free — no direction claim registered for this naming).
- g7 CraftPlayerCanSee: ALL 8 methods OK on synthetic args (2.2–8.0us) —
  the state-dependent family ran without rc-errors; values are semantically
  meaningless by construction (no player/world), reported as-is.
- g26 ShiftNoiseDirect: all 6 methods ~1.46us (tight wash on synthetic shapes).
- g12 ImprovedNoiseFloor: currentMth 9.6us vs mathFloor 9.5us (wash).
- g13/g14 Lz4: compress 53.8ns / decompress 159.3ns (synthetic small inputs).
- g38 FQ Climate nodeBestMatchUniqueBatch: 40.7ns; g44 NoiseChunkWrapCapacity:
  378.4ns. g43/g48 build-handle kernels: 40.4/40.8ns (wrapper-order cost).

### CRASH inventory (7 groups, all as-is per pre-registration)

| group | kernel | exit | reason (log) |
|---|---|---|---|
| g4 | ChunkExpireCount coldSummary (III[J)I | 134 | `memory allocation of 17179869184 bytes failed` — 16 GiB alloc from synthetic scaling; survived no ladder attempt |
| g39–g42 | FQ ClimateRTree checksum/free/search ×4 | 134 | Rust panic (SIGABRT) on synthetic constant handles — handle consumers need an in-JVM build→use chain |
| g45–g47 | FQ PerlinNoise free/getValue ×3 | 134 | same handle-consumer panic class |

The handle **build** kernels (g43 ClimateRTree nativeBuildTreeHandle,
g48 PerlinNoise nativeBuildHandle) run OK at ~40ns; consumers cannot receive
their handles because each (fqcn,sig) group is a separate JVM by rig design.
A chained build→use probe is a possible future pre-registration (new rig mode,
NOT attempted here — no post-hoc rig changes).

### §97 trio (chunk-encode surface) — measured, semantics disclosed

g35 nativeEncodeSectionDataSized 207.0ns, g36 nativeEncodeLightData 115.9ns,
g37 nativeEncodeSectionData 276.8ns — all OK s0 on synthetic sidecar shapes.
§97's rc=-3 closure was established for WIRE-SHAPED variants (byte-exact
vanilla reference); the sidecar's synthetic shapes took a non-error-looking
path whose return semantics are unknown standalone. ns/op of an unknown-semantics
path is not a perf number — banked as-is for completeness. The in-server gate
probe remains TASK-150 phase-2a (pending /home/z/server + jdk21).

## Census bucket arithmetic correction (append-only, TASK-155 prose)

The banked census TSV is the source of truth: 283 exports = 129 P500 +
153 uncovered = 94 zero-evidence (col5 empty) + 59 with-evidence. The §103
prose table listed the chunk-encode trio BOTH in its own bucket and inside
the 94 (table sums to 286) and said "60 with evidence" (TSV: 59). The 94-set
measured here therefore INCLUDES the §97 trio (groups 35–37) — handled above.
No census verdict changes; bucket bookkeeping normalized by this doc.

## Honesty and scope

- FIRST MEASUREMENT: every number is exploratory; no baseline exists for this
  surface; no drift claims; results never feed adoption gates.
- Ratios only for the three §103-registered conventions; everything else flat
  tables + spread.
- State-dependent families (CraftPlayerCanSee, Waypoint*) and handle families
  (ClimateRTree, PerlinNoise) run with synthetic args; errors/garbage/crashes
  reported as-is; no wiring, no gate tuning.
- INJECTS-ONLY: 0 server boots, 0 product changes, canonical bench/p500/
  untouched (no groups.tsv/G*.java regeneration; `old*` rule intact).

## Artifacts

- `bench/p501_sidesurface/gen_p501_bench.py` — generator (imports canonical)
- `bench/p501_sidesurface/run_p501.sh` — runner (canonical pattern + shim ladder)
- `bench/p501_sidesurface/aggregate_p501.py` — FIRST-MEASUREMENT aggregator
- `bench/p501_sidesurface/README.md` — scope + honesty rules
- `bench/p501_sidesurface/results/P501_SIDESURFACE_RAW_2026-09-10.tsv` — raw
- `bench/p501_sidesurface/results/P501_SIDESURFACE_REPORT_2026-09-10.md` — report
- `bench/p501_sidesurface/results/P501_ENV.txt` — env provenance
- `bench/p501_sidesurface/logs/g<gid>.{out,log}` — per-group logs (on disk,
  gitignored — same practice as canonical `bench/p500/{classes,logs}/`; the
  commit accidentally carried them once and a follow-up cleanup commit
  untracked them, append-only)
