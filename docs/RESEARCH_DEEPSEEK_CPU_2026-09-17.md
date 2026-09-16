# RESEARCH — DeepSeek-architecture lens on CPU-only Minecraft mega-speedup (task162 / S7-93)

Owner directives driving this round (2026-09-17):
- «рисерч делай от дипсика например архитектуры» — the research round must mine
  the DeepSeek-family engineering literature (architectures) for transferable
  mechanisms.
- «и без гпу тоже надо мега ускорение» — every lever must be CPU-only: no GPU
  assumed anywhere (GitHub runners have no GPU; the module's proven machinery is
  JNI/Rust, which is CPU-native anyway).

Measured basis: Benchmark 3.0 run#10 (real MineShield-3 Min world, 9,216 chunks
force-loaded, zero-player soak, 224,660 CPU samples, full-bridge mode). The
chunk-state READ lane is the top kernel super-lane:

| leaf frame (exact) | self-time | share |
|---|---|---|
| `PalettedContainer.get` | 8357 | 3.7% |
| `LevelChunk.getBlockStateFinal` | 3775 | 1.7% |
| `SimpleBitStorage.get` | 3618 | 1.6% |
| `PalettedContainer.readPalette` | 3378 | 1.5% |
| `BlockBehaviour$BlockStateBase.getBlock` | 2473 | 1.1% |
| **lane total** | **~21,600** | **~9.6%** |

Tick-phase split (report v2 ancestry): entity tick 43.3% / block entities 8.1% /
random tick 5.6% / chunk tick 3.7% — the chunk-state read lane feeds ALL of
these phases, which is why it outranks every single-phase target.

## 1. Literature mined (8 web-search rounds, 2026-09-17)

1. **MLA (Multi-head Latent Attention, DeepSeek-V2/V3)** — the KV cache is
   compressed into a LOW-RANK LATENT form; consumers apply an "absorb" so the
   expensive up-projection never runs per token. Core lesson: *compress what
   gets stored, and fold the decode into the consumer* — not "cache less".
2. **DeepSeek-V3 FP8 with fine-grained scaling** — 1x128 tile-wise for
   activations, 128x128 block-wise for weights; the fine grain keeps outlier
   ranges local so a generic fallback path is never needed. Core lesson:
   *specialize per tile, not per model* — data-local specialization beats one
   generic path.
3. **vLLM PagedAttention** — logical-to-physical block table; non-contiguous
   physical blocks; ZERO allocation on the serving fast path. Core lesson: *the
   fast path never allocates and never searches; it only translates through a
   prebuilt table*.
4. **FlashAttention** — tiling + fusion; the N×N intermediate is NEVER
   materialized in HBM; tiles are sized to SRAM. Core lesson: *process in
   cache-resident tiles and fuse the consumer into the pass over the tile*.
5. **DualPipe (DeepSeek)** — full overlap of compute and communication phases by
   bidirectional scheduling. CPU-only mapping: overlap main-thread tick phases
   with off-main chunk workers (module already owns batch/async encode
   machinery). PARKED (same verdict as ENT-BP async: escalation only — write
   paths here are not safely separable without a shadow-diff farm of evidence).
6. **MTP / speculative decoding (DeepSeek-V3 1.8x at high acceptance)** — cheap
   guess + exact verify. Core lesson: *a guess is only free if verification is
   exact and cheaper than the general path*.
7. **AVX2 gather reality check** (nickb.dev 2025; kernel-probe 2021) — vgather
   measured 0.95x-1.2x on small working sets; "3x" only in gather-dominated
   app benchmarks. REFUTATION BANKED: SIMD gather is NOT the mechanism here.
8. **oxidized-mc/chunks + protocol docs** — community Rust reimplementations of
   `PalettedContainer`/`BitStorage` confirm the packing model; no production
   fused-bulk native surface exists upstream — the lane is open.

## 2. Mapping matrix (DeepSeek mechanism -> Minecraft hot path -> module machinery)

| DeepSeek mechanism | Minecraft analogue (measured) | Module machinery (proven) | Verdict |
|---|---|---|---|
| MLA latent + absorb | palette = latent codebook; per-read cost = decode+deref; consumers mostly COMPARE states | bridge-class native over a section (latent-space scan, `indices_of`) | **SELECTED (core landed)** |
| FP8 fine-grained tiles | section = 4096-entry tile; single-value palettes dominate many real sections | specialized fast path per section shape (single-value shortcut) | **SELECTED (core landed)** |
| PagedAttention zero-alloc fast path | `readPalette` + per-call iterator garbage on hot reads | caller-owned output buffers; G3 zero-alloc discipline (entity_mirror precedent) | **SELECTED (core landed)** |
| FlashAttention tiling+fusion | one JNI crossing per BATCH of positions instead of per position; section fits L1 (≤32KB) | whole-body bridge native `bulk_states`/`indices_of` | **SELECTED (core landed)** |
| MTP guess+verify | single-value palette: EVERY index is 0 — "guess" proven by construction, no verify needed | single-value fast path | **SELECTED (core landed)** |
| AVX2 gather | bit-unpack = data-dependent shifts; q8 says gather ≈ scalar here | — | **REFUTED for v1 (banked with numbers; revisit only on G2 evidence)** |
| DualPipe overlap | main-tick vs chunk-worker phases | batch/async machinery | PARKED (escalation only) |
| MoE dispatch | per-type block-entity tick routing | hopper/BE lane 1.5% — below 3% gate | PARKED (below gate) |

## 3. The lever: PALETTE-GATHER v1 (task162)

**Thesis.** The chunk-state read lane pays, PER POSITION: a Java call, bounds
checks, bit extraction (incl. the word-straddle branch), a palette dereference,
and — for comparisons — a `BlockState` identity check. A single bridge native
that resolves WHOLE POSITION BATCHES (or scans WHOLE sections) in one pass:
amortizes the crossing, keeps the tile L1-resident, never allocates, and can
compare in LATENT space (palette indices) instead of dereferenced states —
the MLA absorb, applied to blockstate reads.

**Core (landed this round, production Rust, 0 new deps):** `src/palette_gather.rs`
- `SectionPacked` — bit-exact `SimpleBitStorage` replica INCLUDING the
  straddle case (`v |= words[w+1] << (64-shift)`, Java `>>>`/`<<` semantics).
- `get_index` (fast single-word path + straddle fallback), `get_state` (fused
  deref), `bulk_states` (batch resolve, zero alloc, fail-fast on corrupt
  section WITHOUT partial writes), `indices_of` (latent-space whole-section
  scan with SWAR-style word-chunk extraction).
- Single-value fast path (MTP-by-construction) on every query path.

**Tests:** 7 new, suite 79/79 PASS:
- hand straddle vectors (incl. maximal 1-bit straddle, all-ones words),
- property: scalar == naive oracle over bpe 1..=16, all positions,
- property: bulk + whole-section scan == oracle walk (same order, same count)
  for bpe {1,3,4,5,6,7,8,9,12,15} over every palette index,
- corrupt inputs report (None, no partial writes), shape validation,
- zero-alloc fast path (per-thread counting allocator, exact 0),
- single-value fast path, occupancy helpers.

**S7-93 lesson (allocator):** the shared global counting allocator raced with
sibling tests' threads (entity_mirror zero-alloc test flapped at 185 counts
once heavy property tests landed). Fixed to PER-THREAD const-init thread_local
counting — exact-zero assert is now schedule-deterministic for BOTH cores.

## 4. Pre-registered gates (one lever; committed BEFORE any A/B)

- **G1 — bit-exactness.** Every returned state must be the IDENTICAL
  `BlockState` reference the kernel path returns (identity, not equals).
  Core-level: property parity vs oracle (landed). Kernel-level: shadow-diff leg
  over live sections during a CI soak (same ladder as task161).
- **G2 — performance.** CI A/B x2 (min-of-2 both arms), env-gated
  `CRUSSTY_NATIVE_PALETTE_GATHER=0/1`, whole-soak wall-clock delta ≥ **3%**
  (pre-registered BEFORE wiring; lane basis 9.6% self-time, conservative).
  NOISE-COLS precedent: if the gate never fires (lane absent in the profiled
  world), the lever is demoted, not force-landed.
- **G3 — allocation.** Query paths allocate NOTHING (landed, exact 0).
- **G4 — promotion.** TASK-148 flow: prefix-strip → prune → retrain → JFR →
  commit gate, then production promotion.

## 5. Verification ladder (next ticks)

1. **Recon leg (CI, from the booted jar — not from memory):** javap dump of
   `PalettedContainer`/`PalettedContainer$Data`/`SimpleBitStorage`/palette
   variants (entity-recon job pattern from this tick extends to a
   `state-recon` artifact). Establishes field layout + method bodies for the
   whole-body patch decision (bridge vs hot-patch).
2. **Shadow-diff leg:** native reads vs kernel reads over a full soak —
   zero mismatches required to advance.
3. **A/B x2 (G2):** min-of-2, both arms, same world, same forceload.
4. **G4 promotion** per TASK-148 flow.

## 6. Follow-up levers (pre-ranked, same discipline)

1. **RAND-LANE** (`ServerLevel.optimiseRandomTick` 2.2% +
   `SimpleThreadUnsafeRandom.advanceSeed` 1.7% + `SpreadingSnowyDirtBlock` 0.6%
   ≈ 4.5%): native seed pipeline + precomputed per-state random-tick dispatch
   table (FP8 tile lesson: specialize the common state set). Gate ≥3%.
2. **ENT-BP v2** (task161): ladder in flight — CI recon artifact lands THIS
   tick (entity-recon job added to world-bench workflow).
3. **PUSH-MEMO** (scoreboard 0.99%), **ENT-DATA** (0.79%) — below-gate alone,
   banked as bundle candidates.
4. **DualPipe-style phase overlap** — parked; requires shadow-diff farm first.

## 7. Honesty ledger

- All numbers above are run#10 CI artifacts (diagnostic boot — they RANK
  hotspots; they are NOT parity evidence).
- The 9.6% lane is an UPPER bound on the lever's reachable win; A/B decides.
- AVX2-gather refutation is banked with cited measurements (0.95x-1.2x); the
  SWAR scalar path is the v1 production path.
- No engine (CRUSSTY) files touched this round; module-only, INJECTS-ONLY,
  0 sandbox boots.
