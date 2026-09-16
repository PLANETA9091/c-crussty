# RESEARCH — BENCHMARK 3.0 RESEARCH-BUCKETS: Rust-replacement ladder (huge round)

Agent-7625532f, 2026-09-16 (owner directive: «оптимизируй бесконечно, лимитов нет» +
«рисёрчи должны быть не просто маленькие а огромные» + autonomous-GitHub directive).
Status: PRE-IMPLEMENTATION RESEARCH. Every candidate below is a CLAIM template with
pre-registered gates — nothing here is a parity claim. Evidence source for prioritization
= BENCH 3.0 CI artifacts (BOTTLENECKS_3.md self-time by bucket). Order of attack is
DATA-DRIVEN: the table in §0 is re-ranked after every CI run.

Sources folded in: banked LLM→hot-path mappings (CLAIMS TASK-227 row: MLA latency-cache,
vLLM PagedAttention APC, speculative decoding, FlashAttention tiling/fusion) + fresh web
research 2026-09-16 (Folia/PaperMC regional threading; Leaf async mob-AI target search;
Pufferfish hopper/pathfind/memory fork lanes; JNI `GetPrimitiveArrayCritical` pinning
semantics — Shipilev/IBM/Oracle docs; broad-phase collision: spatial hash vs sweep-and-prune
vs uniform grid (CUDA broad-phase chapter); SoA/data-locality (gameprogrammingpatterns.com);
spark tick-loop docs; SIMD-noise discussions (gamedev.SE 2013, Jordan Peck 2017,
HN "SIMD Perlin Noise" 2014)).

---

## §0. Bucket ladder (re-ranked by every CI run)

| # | bucket (report_world3.py) | kernel lane | module lane already proven | next Rust candidate (§ refs) |
|---|---|---|---|---|
| 1 | worldgen/noise | levelgen/noise | PerlinNoise.getValue hotpatch G-AB wall −12.3% parity 0/20000 (TASK-74/148) | SIMD column-noise + improved_noise re-eval on CI profile (§1) |
| 2 | entities/mobs | world/entity | — (mobdense census found no ≥3% workload; CI fixes that) | async target-search + broadphase bridge (§2) |
| 3 | chunk system | chunk/ServerChunkCache | SingleUserAreaMap.update hotpatch 5075→3320 B (§6c) | zero-copy serialization + batch IO (§3) |
| 4 | block entities/hoppers | block/entity | — | batched hopper tick + dirty-set gate (§4) |
| 5 | redstone / tick scheduling | level/redstone, world/ticks | — | arena-allocated tick queue + speculative neighbor eval (§5) |
| 6 | JVM internals (GC/JIT) | libjvm | C3 corpus decomposition (§91–§96: vanilla pays the same ~57MB) | allocation-shape flags only; NO agent-side GC meddling (§7) |
| 7 | our Rust surfaces | libcrussty*.so | — | self-audit: module must stay <2% self-time (§8) |

---

## §1. worldgen/noise — kernel bucket #1 candidate

**Facts on the table.** G-AB (TASK-74, promoted default by TASK-148/§95): native whole-body
PerlinNoise.getValue = wall −12.3%, cpu_burst −11.1%, parity 0/20000 bit-exact. CI BENCH 3.0
will, for the first time, profile worldgen on a REAL pregenerated world under forceload —
where the noise share of self-time is expected to spike versus the C3 burst corpus.

**Research payload (web-researched):**
- Scalar noise is latency-bound, not throughput-bound: Jordan Peck (2017) measured SIMD
  *single-value* noise ~5% SLOWER — vectorizing one sample is a loss. The win is
  **column/batched evaluation**: worldgen always consumes noise in (x,z)-grids; evaluating
  8–16 samples per lane turns noise into a throughput problem (gamedev.SE 2013; HN 2014
  "beating the compiler with SSE").
- The kernel's octave loop re-derives the same lattice on overlapping coordinates. The
  FlashAttention mapping (banked) applies: **tile the (x,z) domain, fuse the whole octave
  stack per tile, never materialize intermediate octave planes** — one pass, one write.
- **LAW: parity gate is bit-exactness.** Noise is deterministic — any candidate ships with
  the same 0/20000 replica battery + corpus seed sweep (stages replica byte-identical).
- `improved_noise` (default-OFF by measurement: TASK-63 +10% worldgen regression, TASK-79
  COMBO NO-GO) must NOT be re-armed on vibes: only if the CI profile shows a
  blender/octave-dominant shape that the improved evaluator specifically removes, AND a
  fresh CI A/B on the bench-3 harness (which the sandbox could not provide) shows ≥3%.

**Pre-registered candidate NOISE-COLS (Rust, native module):**
- Claim: batched column evaluator for the exact sampler chain the hotpatch serves.
- Gates: (G1) parity bit-exact 0/N on replica + 3 worldgen stages byte-identical;
  (G2) CI no-player window wall −3% on worldgen bucket self-time, min-of-2 CI legs;
  (G3) sandbox canonical p500 noise pair reproduces (never regress canonical);
  (G4) INJECTS-ONLY: env-gated default-OFF until both gates green, then TASK-148-style
  promotion flow with kernel-policy two-key contract.

## §2. entities/mobs — the bucket CI un-earths

**Facts.** In-sandbox the module never booted bots (INJECTS-ONLY); mobdense census found no
≥3% workload to measure (§95-era variant-C note). A forceloaded real world with farms,
villagers and item entities is the FIRST time the entity bucket is honestly profiled.

**Research payload:**
- Leaf (Paper fork) ships **async mob-AI target search**: the expensive part (nearby-entity
  scan for targeting) moved off the main thread. This is the speculative-decoding mapping
  from CLAIMS: **cheaply *propose* (background scan, possibly stale), *verify* on the
  tick thread** — correctness is preserved because verification is the same predicate the
  kernel runs; only the search is amortized.
- Broad-phase: kernel entity iteration is an O(n·neighbors) scan per section. Game-dev
  literature: uniform grid / spatial hash beats sweep-and-prune when objects are
  chunk-stationary (most Minecraft entities are), S&P wins when velocity is high (arrows).
  The CUDA broad-phase chapter's bucketing maps directly: **bucket by section, sort by
  bucket, prune pairs inside buckets only**.
- SoA/data-locality (gameprogrammingpatterns): entity positions are consumed as hot arrays
  by collisions/AI; an SoA mirror (f64 x/y/z arrays updated via JNI at block boundaries)
  removes pointer-chasing per pair. This is the MLA mapping: keep the *hot working set*
  in a dense compact form, keep the *full object graph* in Java untouched.
- JNI contract per banked PagedAttention/APC mapping: **block-granular pooling** of the
  entity mirror — fixed 4096-entity blocks, free-list reuse, ZERO allocation on the fast
  path; `GetPrimitiveArrayCritical` only for short pinned copies (Shipilev: no blocking,
  no other JNI calls inside the critical region — enforce by construction: copy out, then
  compute).

**Pre-registered candidate ENT-BP (broadphase bridge):**
- Claim: section-bucketed AABB broadphase mirror in Rust, verification-on-tick contract.
- Gates: (G1) zero behavioral delta = kernel still makes every final collision decision
  (bridge only prunes candidate SETS that the kernel would then scan — must be a superset
  of kernel results, proven by a shadow-diff harness on CI run artifacts);
  (G2) entity-bucket self-time −5% min-of-2 CI legs; (G3) no allocation on fast path
  (alloc profile: zero new types); (G4) INJECTS-ONLY, default-OFF env gate.

## §3. chunk system — I/O and serialization

**Facts.** SingleUserAreaMap.update hotpatch is banked (5075→3320 B). The CI world is
PREGENERATED: first forceload sweep = saved-region I/O storm, steady state = scheduler.
Two distinct sub-buckets will show in the flamegraph — do NOT conflate them.

**Research payload:**
- PagedAttention mapping (banked): chunk data is already block-tabled by the kernel;
  the Rust side should mirror **the block table, not the chunks** — a level of indirection
  that lets the bridge work on chunks WITHOUT copying palettes.
- Zero-copy parse identity (banked R57 engine design, TASK-227 row): the same
  `&[u8]`-lens discipline applies to NBT/region frames the module touches — parse stages
  hand out byte lenses, ownership stays with the caller's buffer.
- Speculative decoding mapping: **chunk-load prefetch heuristic** — forceload sweeps issue
  tiles in scan order; a tiny predictor (previous tile = strong hint of next) lets the
  bridge issue readahead. Verification = kernel still validates each chunk; worst case
  = wasted IO, never corruption. Gate on CI wall of the FIRST sweep leg only.

## §4. block entities/hoppers

**Facts.** Pufferfish/Paper forks prove the surface is optimizable (hopper cooldowns,
batched transfers, reduced block-entity ticking classes). Vanilla hoppers re-poll
inventory state every tick.

**Research payload:**
- MLA mapping: hoppers hold a tiny hot state (cooldown, direction, buffer) — the bridge
  can keep a **latent/compact mirror** of hopper grids per chunk and only re-sync on
  dirty marks (vanilla already emits block updates → free dirty signal).
- Batch model (banked batch_* lanes in the module src): one JNI call per 64-hopper
  group with a packed state array (u8 flags + u16 cooldowns), returns transfer decisions;
  single critical section per batch, never per hopper.
- Gate: (G1) item-count invariant byte-exact on a CI summon/observed farm window;
  (G2) block-entity bucket self-time −5% min-of-2; (G3) default-OFF.

## §5. redstone / tick scheduling

**Facts.** spark docs: ticks cannot execute in parallel in Paper (single-threaded tick
loop) — Folia's answer is REGION-LEVEL threading, which is a kernel-fork property, NOT
module-reachable. The module lane is therefore: make the SERIALIZED work cheaper, not
parallel (do not fight the law).

**Research payload:**
- Arena tick queue (PagedAttention mapping again): scheduled ticks are consumed in
  time order; a bounded arena + binary heap mirror in Rust with zero per-tick allocation
  replaces per-object queue nodes. Kernel keeps ownership; bridge mirrors + hands back
  the same ordering (parity = ordering byte-identical over full CI window).
- Speculative neighbor eval (speculative-decoding mapping): redstone wire updates
  re-evaluate power levels repeatedly within one clock; propose-verify via a 1-tick
  memo keyed on (pos, input-mask) — verification = recompute on any dirty signal.
  Undo cost is bounded because evaluation is pure.
- Honest ceiling note: redstone in the CI world is whatever MineShield farms contain;
  if the bucket lands <3% self-time, this lane stays RESEARCH-ONLY (owner's ≥3% bar).

## §6. JVM internals (GC/JIT) — what NOT to touch

C3 corpus §91–§96 proved the churn native term (~57MB) is VANILLA JVM physics — the
old kernel pays it identically. The module's marginal cost = +31MB native resident +
~5MB live heap + 0 boot. Research verdict: **no GC-tuning agent work**; the only lane is
keeping OUR surfaces allocation-free (APC mapping: no allocation on fast paths), so the
GC never learns the module exists. Continues to hold for CI boots: G1 on 6GB heap with
a 43GB world is a kernel/flag decision for the owner, not an inject decision.

## §7. network — expected noise in CI

Zero players ⇒ network bucket ≈ keepalive/latency noise. If it shows >2% self-time,
suspect the console fifo loop first (rig artifact), not the kernel. Pre-registered:
network bucket is EXCLUDED from the ladder until a playered CI leg exists (owner decision).

## §8. our Rust self-audit (standing law)

BOTTLENECKS_3.md rows `c-crussty module (Rust)` + `engine runtime (Rust)` + `CE natives
(JNI)` must sum <2% of self-time in every CI run. Violation = the round's FIRST fix
(before any kernel bucket): our own frames are contamination, not opportunity.

---

## §9. Execution protocol for the next rounds (pre-registered, per module ledger law)

1. CI run lands → BOTTLENECKS_3.md committed as artifact → re-rank §0 table with REAL
   shares → pick TOP bucket with share ≥3% (owner bar) AND a viable Rust lane above.
2. Round = claim (CLAIMS.md + ledger §N) → implement behind env gate → CI A/B legs ×2
   (world-bench-3 dispatch twice: arm ON/OFF) → report diffs min-of-2 → parity replica
   per §-gates → bank results + verdict verbatim (no reclassification).
3. Escalation: two consecutive no-win legs on the same bucket ⇒ bucket parked with the
   measured refutation in this doc's addenda (append-only) and the ladder re-ranks.
4. Full-world (43.4GB) runs and summon_sweeps=1 legs are owner-dispatch choices; both
   documented as deviations when used; no-player semantics never silently widened.

## Addenda (append-only; fill after each CI run)

- (empty — first BENCH 3.0 run #1 in flight at write time; §0 re-rank lands in ledger §108)
