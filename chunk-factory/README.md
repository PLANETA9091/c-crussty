# chunk-factory -- Native Chunk Factory (Phase 0 skeleton)

CRUSSTY platform module for Paper/Purpur 1.21.10 (Java 21). New workspace
crate (standalone, excluded from the root `crussty` workspace; std only, no
external dependencies).

## Mandate

Vanilla-identical Minecraft worldgen in Rust with a **>=100x pregeneration
speedup** vs vanilla Paper 1.21.10 at **100% semantic compatibility**.

A CHUNK is a pure function:

```
f(seed, datapack_hash, DataVersion, x, z) -> NBT
```

Same inputs -> semantically identical NBT. Everything that can change
generation is in the key; every cache keys on `(seed, world spec hash)`.

Implementation is **all Rust, no GPU, no approximate math**. Java
arithmetic is reproduced bit-exactly: wrapping int/long, exact floor/cast
semantics, Legacy `Random` (48-bit LCG) AND Xoroshiro128++ randoms with
their exact seeding derivations, MD5-seeded positional factories. No FMA,
no f32, no LUTs, no reordered float expressions.

## Delivery ladders

| Ladder | Shape |
|---|---|
| **A** | Offline `.mca` region writer: generate chunks out of process, hand vanilla a finished region file. |
| **B** | Virtual region files via the Moonrise chunk-IO hook: disk-missing chunk -> Rust NBT, ONE JNI call per chunk. |
| **C** | Optional stage bridges (finer-grained, only where the ladder B boundary is proven insufficient). |

## Phase plan

| Phase | Scope |
|---|---|
| **0** | Profiling + golden harness; THIS crate skeleton (policy gate + coverage + promotion ledger only). **Writing generation code in this crate is BANNED in Phase 0.** |
| **1** | IR extraction: datapack/preset -> declarative stage graph. |
| **2** | Bit-exact Tier1 kernels (noise / random / density) + the GATE. |
| **3** | Ladders A and B wired behind the gate. |
| **4** | Features, light, compression. |
| **5** | Structures (jigsaw, starts + references), spawns. |

**Phase 0 bans writing generation code in this crate.** No noise, no
randoms, no density math: Phase 0 ships only this skeleton (policy gate,
coverage reporting, promotion ledger) plus profiling/harness work outside
the crate.

## Key accelerations (K1..K6)

* **K1** worldgen-graph compiler (datapack -> IR -> optimized plan).
* **K2** exact interval culling on density-node bounds (proven-conservative
  only, exact values on sign-mixed cell corners).
* **K3** cross-chunk dedup (carver paths per source chunk, 2D fields,
  aquifer cells, structure scans).
* **K4** section-sparse model; light only in active sections.
* **K5** features/structures ported by tiers, each tier gated, Java
  fallback for everything unported.
* **K6** bit-exact discipline; the oracle is the real JVM, never a
  re-implementation of it.

## The GATE (precondition for ANY native enable)

```
>=10^4 chunks  x  >=5 seeds  x  {vanilla, Terralith, Tectonic}
  -> 0 divergences
AND noise stage >= 30x hot Java per core
```

If either leg fails, native stays off. No exceptions, no partial credit.
`src/gate.rs` enforces this organizationally: `NATIVE_READY` starts EMPTY
and a lane is only added with GATE evidence attached; `src/ledger.rs`
carries the evidence rows (`PROMOTED`, GATE constants, `gate_passes()`
encoding both legs) and is also EMPTY in Phase 0.

## Invariants (I1..I10; violation = revert the change)

Faithful summary of the owner NCF worklog, section 1:

* **I1 -- Equivalence.** Output equals what vanilla Paper produces for the
  same seed / datapack / DataVersion (4556). Criterion: semantic equality
  of the decoded NBT -- blocks, biomes, heightmaps, light, structure
  starts/references, block entities, PostProcessing, fluid ticks.
  Byte identity is desirable, not mandatory.
* **I2 -- Bit-exact math.** No FMA, no fast-math, no f32, no reordering of
  float operations, no LUT in place of an exact function.
* **I3 -- Java semantics.** Wrapping int; exact floor and double->int
  casts; Legacy AND Xoroshiro randoms; polar gaussian with the cached next
  value; MD5 seeding of positional factories.
* **I4 -- No approximations.** Never skip neighbor-chunk influence; cull
  work only along proven conservative bounds.
* **I5 -- Keyed caches.** Every cache keys on `(seed, world spec hash)`;
  no global state without a world key.
* **I6 -- Owner law.** No JVM flags, ever -- only `-agentpath` + module.
* **I7 -- New code in the new workspace crate.** The shipped closed `.so`
  gains no kernels; recovered sources under `native/` (317 selftests) are
  reference/oracle only.
* **I8 -- Java fallback.** A chunk with unsupported content (a structure,
  a feature, a foreign BlockPopulator, blending against an old world) is
  generated ENTIRELY by Java; the coverage % metric is mandatory in every
  report (`src/coverage.rs`).
* **I9 -- Gate before enable.** A native stage/feature is enabled only
  after its zero-diff gate (kernel_policy.rs pattern: evidence ledger,
  default-off -- `src/gate.rs` + `src/ledger.rs`).
* **I10 -- Honest statistics.** A/B with byte-identical world restore,
  exact Mann-Whitney U, cold and warm modes reported separately.

## Banned list (permanent)

* GPU.
* Approximate noise (any ULP of drift).
* Fewer octaves / cheaper curve substitutions.
* f32 anywhere vanilla uses f64/int math paths.
* "Small JNI kernel x 100 call sites" hopes -- the ladder B boundary is ONE
  JNI call per chunk, not per-stage crossings.
* Ungated default enable -- the gate defaults to dormant, `Off`.

## The enable gate

`src/gate.rs` mirrors `../src/kernel_policy.rs` (the do-not-wire doctrine)
at chunk-status-lane granularity:

```rust
use chunk_factory::gate::{decide, Decision};

match decide("structures/jigsaw") {
    Decision::Allow => { /* generate natively */ }
    Decision::KeepJava { reason } => { /* vanilla path; reason says why */ }
}
```

* `NATIVE_READY` -- lanes proven zero-diff (evidence tag per row). EMPTY in
  Phase 0, by design: nothing is proven, so nothing may claim to be.
* `DO_NOT_ENABLE` -- lanes with known divergence / unreplicated semantics;
  evaluated FIRST, so a listed lane can never be enabled (seeded with
  `structures/jigsaw` and `legacy_blending` as forward-looking docs).
* Modes via `CRUSSTY_CHUNK_FACTORY`: unset/empty/`off`/unknown -> `Off`
  (default-dormant, everything Java; unknown values never widen the gate),
  `strict` (only `NATIVE_READY` lanes go native), `audit` (strict + logs
  every decision). There is no bypass value: an unproven lane cannot be
  forced native by any env setting.

## Metrics

Placeholder in Phase 0 -- no NCF numbers exist yet. All measured numbers
(per-stage per-chunk budgets, chunks/s/core cold vs warm, core scaling,
coverage %, total pregen speedup vs the >=100x target) are recorded in the
OWNER NCF worklog, section 6 ("МЕТРИКИ") and referenced from there; this
section will cite concrete reports as phases close.

Pre-NCF baseline (from the repo, 2 vCPU, Purpur 1.21.10): pregen 3721
chunks (Chunky, radius 30) vanilla 5:39 vs c-crussty 5:29; burst 128
chunks vanilla 41 s / 37.9 CPU-s vs 38 s / 35.7 CPU-s.

## Build / test

```
cargo test --manifest-path chunk-factory/Cargo.toml
```

(Standalone crate: build it by its own manifest; it is excluded from the
root workspace on purpose -- the shipped closed `.so` cannot gain kernels,
and this crate must not depend on the agent's dependency tree.)

## Phase 0 statement

This crate currently contains NO generation code and must not gain any in
Phase 0: only the crate skeleton, the enable gate, coverage reporting, and
documentation. Worldgen math (noise, randoms, density) arrives in Phase 2,
behind the GATE, after the Phase 0 profiling + golden harness work.
