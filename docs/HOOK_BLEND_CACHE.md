# HOOK_BLEND_CACHE — NoiseChunk per-column blend cache (B10 prototype)

> Status: **PROTOTYPE, gate `CRUSSTY_NATIVE_BLEND_CACHE` default OFF.**
> Even with the gate ON, this prototype is **observation-only**: it captures
> pristine `NoiseChunk` bytes, runs a 10k-sample parity harness through the
> already-injected P500 bridge, and probes candidate patch points. It never
> serves patched bytecode (the classfile patcher does not exist yet — see
> §3.4). Default behavior of any server is unchanged by construction.
>
> Owner: src/proto_blend_cache.rs + this doc. Wired in src/lib.rs
> (`mod proto_blend_cache;` + `register()` in cplugin_init + `activate()` at
> the end of inject_surface, mirroring area_map/improved_noise); the gate
> stays OFF by default, so the dormant path is a no-op on every server.

---

## 1. Kernel background — what the blend cache does in vanilla Paper

**Measured facts** (from this repo, not inferred):

* Bridge class `PaperNativeNoiseChunkBlendCache`, two natives
  (src/jni_table.rs rows 159–160, JNI_EXPORTS.manifest rows 161–162):

  | method | sig | P500 median (G21: args `p0=256, p1=31, dst=long[64]`) |
  |---|---|---:|
  | `oldEmptyBlenderSummary` | `(II[J)I` | **67.0 µs** / call |
  | `newEmptyBlenderSummary` | `(II[J)I` | **274.5 ns** / call |

  → **244x** (bench/p500/results/P500_REPORT.md §21; raw
  results/p500_raw.tsv lines 76–77, run "OK s0" = no retries, no crashes).

**Vanilla semantics (inferred — the kernel sources are closed; the names
below are the Mojang-mapping names to CONFIRM via `javap` against the
running kernel before any patch work, see §3.4 checklist):**

During world generation, `NoiseChunk`
(`net/minecraft/world/level/levelgen/NoiseChunk`) evaluates density
functions over every cell of a chunk. Two density-function leaf nodes —
`blend_offset()` and `blend_alpha()` (vanilla
`DensityFunctions.BlendOffset` / `BlendAlpha`, delegating to the
`NoiseChunk`'s `DensityFunction.ContextProvider` implementation) — ask the
chunk, per **column** (blockX/blockZ), for:

* the **blend offset**: vertical shift toward the "old" terrain surface, and
* the **blend alpha**: 0..1 interpolation weight,

derived from `BlendingData`
(`net/minecraft/world/level/levelgen/blending/BlendingData`) of the
neighboring already-generated chunks (world-upgrade / generator-transition
borders; the vanilla `Blender` collects the 4 nearest quarter-resolution
height columns per neighbor and interpolates them). The same column's
offset/alpha is requested **dozens of times** (once per density sample per
cell), so the chunk keeps a per-column cache keyed by the packed column
coordinates. The `EMPTY` blender (chunk far from any old chunk — the
overwhelmingly common case) must yield a constant offset `0` / alpha `1`
through the same machinery.

The P500 kernel pair models exactly this per-column cache/lookup path:

* `oldEmptyBlenderSummary(n, p, dst)` — the old code path: per-column work
  goes through the full cache machinery (packed-key hash lookups,
  object-boxed entries, Blender/`Optional` plumbing) even when the blender
  is empty. Cost **scales with `n`** (256 columns → 67 µs ≈ 262 ns/column,
  consistent with a `Long2Object`-style lookup + allocation per column).
* `newEmptyBlenderSummary(n, p, dst)` — the optimized path: **empty-blender
  short-circuit / native hash-bitmap fast path**. 274.5 ns **total** for the
  same 256 columns ≈ 1.07 ns/column — below the cost of even one
  Java-side hash probe, and only ~2.4× the measured ~115 ns JNI transition
  floor (P500_REPORT.md "JNI-floor insight"). An O(1) per-batch early-out
  is the only mechanism consistent with that flat number.

**Falsification probe** (cheap, do it first): re-run G21 with
`-Dp500.n=1 / 16 / 256 / 4096`. If `old*` scales linearly with `n` while
`new*` stays flat ≈ 275 ns, the short-circuit hypothesis is confirmed. This
mirrors the N-scaling methodology of bench/p500/results/P500_SCALING.md.

## 2. Why this matters (win analysis)

* 244x is the **largest win in the P500 baseline** and it sits on the
  *worldgen* hot path: with blending enabled (old-chunk borders, world
  upgrades) every chunk does `16×16 × cells` blend lookups; even the
  empty-blender case pays the cache-machinery tax per column today.
* The optimized kernel replaces a Java recompute/lookup chain with a native
  O(1) check + primitive summary — same class of win as `area_map`
  (branch-minimal native batching) but inside the noise chunk.
* Caveat: 244x is measured on the *bench model* of the path, on a 2-CPU
  sandbox, with synthesized args. The in-server win depends on how often
  blending is actually exercised (full-world chunks = always EMPTY blender
  → the fast path dominates; upgraded-world borders → per-column blending
  work remains, just cheaper). A live A/B (§5) is mandatory before any
  default flip.

## 3. Hook design

### 3.1 Risk analysis — why this is NOT a `noise()`-style immediate patch

`ImprovedNoise.noise` and `SingleUserAreaMap.update` were patchable because
their semantics are (a) pure per-call math / per-structure bookkeeping and
(b) trivially testable against a naive reference. `NoiseChunk` is
**worldgen-critical**: a wrong blend offset or alpha shifts actual terrain
heights at chunk borders. That is a **gameplay/terrain change = FORBIDDEN**
by the project rules (worklog PROJECT CONTEXT: "не менять геймплейные
значения"). Additional constraints:

* A retransformed class cannot gain new instance fields (JVMTI
  `SCHEMA_CHANGED`), so a native per-chunk cache handle cannot be stored on
  the chunk itself; it needs an identity-keyed side table
  (`WeakHashMap<NoiseChunk, handle>` behind the bridge) — extra moving
  parts, GC-timing-sensitive.
* Worldgen runs multi-threaded (worker pool); any patched entry point must
  be thread-safe per chunk, and the side table adds contention risk on a
  path vanilla already keeps lock-free.
* The exact vanilla method surface (`blendOffset`/`blendAlpha`/Blender
  internals) is **unverified** in this repo — see checklist below.

### 3.2 Decision: bridge-first adoption, optional patch behind a NEW env gate

1. **Expose the bridge for direct caller adoption (the safe integration).**
   The optimized kernel already lives behind
   `PaperNativeNoiseChunkBlendCache.newEmptyBlenderSummary` (injected on
   every server by `inject_surface`, 0 unresolved). Kernel-side/Paper-side
   adopters can call it directly; we publish the parity evidence and a
   reference harness instead of patching anything.
2. **Optional byte-hook patch, behind `CRUSSTY_NATIVE_BLEND_CACHE=1`
   (default OFF)** — same discipline as `improved_noise` (post
   dormant-gate-leak fix 3a270ee): gate OFF ⇒ byte hook is NOT registered,
   activate() is a no-op, the log says "dormant". No server behavior can
   change without an explicit operator action.
3. **Parity gate before any patch**: the 10k-sample self-test (§4) must pass
   bit-exact, or the hook stays dormant **forever** (no retry, log states
   why). A patch is served only if the parity harness AND the patch-point
   checklist (§3.4) both pass in the same process.

### 3.3 Candidate patch points (to verify — DO NOT implement until confirmed)

| # | candidate (Mojang-mapped, internal name) | idea | risk |
|---|---|---|---|
| P1 | `NoiseChunk.blendOffset(DensityFunction$FunctionContext)D` | replace body with bridge call returning cached offset | medium: hot, but per-column state must live off-instance |
| P2 | `NoiseChunk.blendAlpha(DensityFunction$FunctionContext)D` | same for alpha | same as P1 |
| P3 | `NoiseChunk` per-column cache fill (method name TBD via javap) | native hash/bitset cache keyed by packed column | higher: touches cache coherency with vanilla |
| P4 | `Blender`/`BlendingData` empty fast path | early-out when `Blender == EMPTY` | lowest surface, but may already be what `new*` models |

Exact signatures and bytecode layout must be confirmed at runtime:
`javap -p -c` the kernel's `NoiseChunk` (Paper 1.21.x is Mojang-mapped, so
the names above are plausible), plus the module's
`log_candidate_sighting()` probe (already in the prototype — it resolves
candidate method IDs against the live class and logs which exist). A
candidate that does not resolve ⇒ redesign, do not force it.

### 3.4 Patch-point checklist (ALL must be true before a patch is served)

1. [ ] Candidate method exists with the expected descriptor on the live
       kernel class (`log_candidate_sighting`).
2. [ ] Class-file major of the kernel class ≤ JVM major (improved_noise's
       v69 lesson; same guard pattern).
3. [ ] Bridge/ops classes compiled `--release 8` and define_class into the
       **kernel loader** (area_map pattern: helper must resolve the kernel
       classes directly, bootstrap copy would shadow/fail).
4. [ ] No class definitions inside the byte-hook callback (ASM
       COMPUTE_FRAMES deadlock — improved_noise.rs rationale); patch
       computed on the quiet activation worker, single retransform.
5. [ ] Parity self-test (§4) passed bit-exact in THIS process.
6. [ ] Deterministic-seed A/B on a local server: identical seed, gate OFF
       vs ON, `forceload` a border region, compare resulting chunk data
       byte-identically (terrain must not move — project rule).
7. [ ] P500 N-scaling probe (§1) documented in this file.

### 3.5 Self-test design (10k-sample parity through the bridge)

Driver: `blend_parity_selftest()` in src/proto_blend_cache.rs. Read-only
against the JVM: it calls the two already-registered natives and compares.

* **Inputs**: fixed-seed xorshift64 (`0x9E3779B97F4A7C15`, area_map's LCG
  style) → 10,000 samples; per sample
  `n ∈ 0..=256`, `p ∈ 0..=63` (matching G21's argument space:
  `p0=N=256`, `p1=SMALL[1]=31`), and a **fresh** `long[64]` dst for each
  call (P500 lesson: kernels may mutate their inputs).
* **Procedure**: call `oldEmptyBlenderSummary(n, p, dstOld)` then
  `newEmptyBlenderSummary(n, p, dstNew)`; compare.
* **Comparison rules / tolerance**:
  * Return value: exact `int` equality.
  * Written payload: `r = min(nOld, nNew)` longs compared **bit-exact**
    (the kernels are deterministic integer/long summaries — any bit
    difference is a semantic divergence, not float noise).
  * Doubles (future real-patch parity on blend offset/alpha, P1/P2):
    primary rule **bit-exact** (`to_bits` equality, NaN by bits). A
    ULP≤1 / rel-1e-12 fallback may only be enabled by an explicit code
    review sign-off, because a last-bit shift in blend math can move
    terrain — default stays bit-exact.
  * **Mismatch budget: 0 of 10,000.** Any mismatch ⇒ log the first ≤5
    offenders with full inputs (reproducible seeds) and keep the hook
    dormant forever (`PARITY_OK=false`, no retry in-process).
* **Determinism probe**: every 1,000th sample, call the native path twice
  on identical inputs and require identical output (catches
  input-mutating kernels even when old/new happen to agree).
* **Robustness**: every JNI call is wrapped: pending exceptions cleared and
  counted per iteration (a throwing kernel = parity failure, dormant);
  batched 500 samples + small sleep so a boot-time worker never starves a
  core; total budget ~1s (old kernel ≈ 67 µs × 10k).
* **Known limitation**: a kernel that crashes the JVM (SIGSEGV) cannot be
  caught in-process. The bench already survived s0..s3 for G21, but this is
  exactly why the whole module is behind the default-OFF gate.

## 4. Prototype mapping (src/proto_blend_cache.rs)

Mirrors src/area_map.rs / src/improved_noise.rs structure with the same
crate APIs (`cplug_sdk::hooks::register_bytes`, `cplug_sdk::classes::find_class`,
`cplug_sdk::jni_util::with_attached`, `jvmti_bindings` prelude):

* `register()` — gate check (dormant ⇒ log + no-op, byte hook NOT
  registered); else registers the byte hook on
  `net/minecraft/world/level/levelgen/NoiseChunk` that (READY=false)
  captures pristine bytes + class-file major for later phases and always
  returns None (observation-only by construction, `PATCH_ENABLED=false`).
* `activate()` — gate check; background thread: `wait_for_boot()`
  (Bukkit.getServer() + settle, improved_noise pattern) → parity self-test
  through `PaperNativeNoiseChunkBlendCache` (found by name; it is a
  bootstrap-loader bridge injected by `inject_surface`) →
  `log_candidate_sighting()` probe of §3.3 → verdict. Phase-2 define-bridge
  and Phase-3 patch/retransform steps are explicit `TODO(B10 Phase N)`
  blocks with the exact pattern references (area_map.rs / improved_noise.rs).
* Compilable **in isolation**: the module references only `cplug_sdk::*`
  and `jvmti_bindings::*` (no `crate::` items), so it can be type-checked
  standalone (scratch crate with `#[path]` include) even while other
  agents' edits leave src/lib.rs mid-flight.

## 5. Rollout plan + rollback

| Stage | What | Risk | Default |
|---|---|---|---|
| 0 (this PR) | prototype merged, gate OFF, dormant everywhere | none — no code path runs | OFF |
| 1 | gate ON in staging: parity harness + candidate probe logs; N-scaling probe of §1; P500 regression run | read-only vs kernel | OFF (explicit env in staging only) |
| 2 | ops bridge compiled (--release 8) + defined into kernel loader when gate ON; publish adoption notes for kernel-side callers; still NO patch | low (additive classes, no retransform) | OFF |
| 3 | patch behind §3.4 checklist incl. seed A/B terrain identity; single retransform; watchdog counters in log (`[crussty-plugin] blend_cache:` prefix) | medium — needs sign-off | OFF |
| 4 (far) | proposal to flip default — separate decision, requires live A/B perf + long-run parity | — | n/a |

**Rollback at any stage**: unset `CRUSSTY_NATIVE_BLEND_CACHE` and restart
the server. Retransform-based patches are not runtime-reversible; the
rollback contract is "restart without the gate ⇒ vanilla code path". All
failure modes self-arrest to dormant: parity mismatch, unresolved patch
point, class-version guard, boot-marker timeout. Log greps for alerting:
`blend_cache: dormant`, `blend_cache: parity`, `blend_cache: SELF-TEST FAIL`.

## 6. Open questions

1. Confirm `p0`/`p1` semantics of the summary kernels (batch-count
   hypothesis) via the N-scaling probe — affects how the parity harness
   sweeps inputs (harness already randomizes both).
2. Exact vanilla identifiers for §3.3 candidates (`javap` the kernel class;
   `log_candidate_sighting` does the runtime half).
3. Is there a Paper-side caller to adopt the bridge (C2ME-style blend
   handling / chunk upgrade flows)? Adoption is the intended Stage-2 path.
4. Where does the per-chunk native handle live at Stage 3 (identity-keyed
   weak side table behind the bridge vs. vanilla cache-interior patch)?
