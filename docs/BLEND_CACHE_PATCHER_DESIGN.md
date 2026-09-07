# BLEND_CACHE_PATCHER_DESIGN — classfile patcher for the NoiseChunk blend path (TASK-29 / 6c)

* Author: subagent-6c (SESSION 006, wave-3 restart), 2026-09-08. **DESIGN ONLY — no product code
  changed.** Deliverable of CLAIMS TASK-29 row "subagent-6c: docs/BLEND_CACHE_PATCHER_DESIGN.md".
* Sibling doc: `docs/BLEND_CACHE_DESIGN.md` @ 4fb9d12 (TASK-29, subagent-3e — delivered late,
  after the SESSION-006 orphan-recovery had re-claimed the row; dup-delivery precedent TASK-03/11/12:
  both valid, both kept, cross-linked). **Convergent verdicts** (adopted here, not re-derived):
  the only defensible construct is an EMPTY-blender constant-fold guard splice; a literal per-column
  Java-side cache is rejected on measured evidence (g3 `BlendedNoise` cached-vs-old 0.980 [M],
  `PaperNativeMarkerCache.cachedSummary` 4.54x REGRESSION = canonical `DO_NOT_WIRE` [M]);
  probe-first build order. **This doc adds** (the 6c brief): exact byte-level transform (§2.2),
  the invalidation state machine (§3), retransform-chain interaction with foreign agents (§5),
  tiered memory budgets (§4), the full V1–V5 gameplay-parity ladder + rollback criteria (§6),
  the bench/acceptance matrix (§7), open questions (§8).
* Divergence from the sibling (refinement, not contradiction): its "~20 B" guard is here computed
  exactly (12 B / 9 B by variant, §2.2); its "≤~512 KB" budget is restated with explicit cap math (§4).
* Inputs: `docs/HOOK_BLEND_CACHE.md` (B10 proposal), `src/proto_blend_cache.rs` (observation-only
  prototype, gate `CRUSSTY_NATIVE_BLEND_CACHE` default OFF, `PATCH_ENABLED=false`), `src/area_map.rs`
  + `src/classfile.rs` (byte-patch machinery), `src/improved_noise.rs` (guards G1–G8 discipline),
  `src/kernel_policy.rs`, `bench/p500/results/P500_REPORT_v2.md` §21/§Wins, TASK-10 errata
  (floor 35–90 ns). Tags: **[M]** measured/cited, **[E]** estimate, **[A]** ASSUMPTION about
  Purpur 1.21.10 internals — each [A] has a verification plan and blocks implementation until resolved.

## 0. TL;DR

* Cache = **the folded constant answer for the EMPTY-blender equivalence class**, expressed as
  ~12 bytes of prepended guard bytecode, not a data structure. Hit criterion: `this.blender ==
  Blender.EMPTY` re-read per call (self-invalidating by construction). Non-EMPTY path = original
  instructions verbatim.
* Patch vehicle = the existing machinery: `cplug_sdk::hooks::register_bytes` pristine capture →
  quiet-worker classfile surgery in the `src/classfile.rs` style (append-only CP, hand-computed
  StackMapTable) → exactly **one** `retransform_class`. Same skeleton as `area_map.rs` [M].
* Invalidation = a three-fingerprint state machine (orig / ours / foreign). Foreign bytes ⇒
  **retire forever**, never fight another agent's patch. Doubt = invalidation = vanilla path.
* Per-instance cost **0 B**; the literal per-column memo cache is fully specified (C1, §1.3) so the
  brief's invalidation/budget questions have a concrete answer, but stays **NO-BUILD** unless the
  §5.4-style probe proves the vanilla machinery tax is real (current evidence says it is not [M]).
* Gameplay safety is the gate, not a property claim: observation-only → env-gated probe →
  javap-truth → stub-JVM parity → seed terrain-identity A/B → P500/ratio-gate → 24 h soak
  (§6). Any single failure = dormant forever this process; rollback = unset env + restart.

## 1. Caching surface — what is cached, where it is born, hit criterion

### 1.1 Where the cached state is born (vanilla Purpur 1.21.10)

During the NOISE chunk-status step the worldgen pipeline constructs a `NoiseChunk`
(`net/minecraft/world/level/levelgen/NoiseChunk`) per chunk **[A — Mojang-mapped name, Paper 1.21.x;
verify via `javap -p -c` + `log_candidate_sighting()` before any code]**. Its constructor receives a
`Blender` (`net/minecraft/world/level/levelgen/blending/Blender`) — for chunks with no
old-format neighbors this is the shared `Blender.EMPTY` singleton; otherwise it wraps the
`BlendingData` snapshots of the neighboring generated chunks **[A — same verification]**. The
density-function leaves `blend_offset()` / `blend_alpha()` then ask the chunk, **per column**
(blockX/blockZ), for an offset/alpha pair, dozens of times per column per chunk-gen
(`docs/HOOK_BLEND_CACHE.md` §1 [M-doc]). The measured model of this path is g21
`PaperNativeNoiseChunkBlendCache (II[J)I`: `oldEmptyBlenderSummary` **95.3 µs** vs
`newEmptyBlenderSummary` **301.1 ns** = **316.45x** [M, P500_REPORT_v2 §21 + §Wins; v1's 244x
superseded]. What is *not* known [A]: whether the live Paper path still pays the per-column
machinery for EMPTY (H1) or already folds it (H2) — §7 probe decides; see sibling doc §5.1 for the
hypothesis arithmetic.

### 1.2 Tier C0 (the shipped design): folded constant guard — "the cache is one compare"

| question | answer |
|---|---|
| cached object/state | the EMPTY-class answer: `blendOffset → 0.0`, `blendAlpha → 1.0` **[A — constants must be extracted from javap of the live class, never assumed; sign of zero included, §6]**, encoded as bytecode literals inside the guard |
| where it lives | inside the patched `Code` attribute of the target method (one `Arc<[u8]>` in `PATCH_CACHE`) |
| hit criterion | reference equality `this.blender == Blender.EMPTY` — evaluated **on every call** (the "cache lookup" is the check itself; nothing is stored per chunk/column) |
| why it is a cache | `newEmptyBlenderSummary` models exactly this O(1) early-out [M]; the guard is its bytecode embodiment — answer memoized at patch-compute time instead of per call |
| what is explicitly NOT cached | blender **instances** (no handle table), per-column entries (C1, rejected by evidence), `NoiseChunk` objects (no weak refs, no GC interaction) |

Self-invalidation property: because the guard re-reads `this.blender` per call, a mutated blender
field (if the field is not set-once [A]) is handled correctly with zero invalidation logic — the
equivalence class is re-derived every call, not trusted from memory.

### 1.3 Tier C1 (fully specified, default NO-BUILD): literal per-column memo

Specified so the brief's questions (invalidation, budget, eviction) have a concrete answer; built
only if the §7 probe proves H1 AND C0 alone leaves the profile still ≥0.5% [E].

* **Key**: (NoiseChunk identity via weak handle, packed column `((blockX & 15) << 4) | (blockZ & 15)`).
* **Value**: `{offset: f64, alpha: f64}` = 24 B/entry; direct-mapped 512-entry table (power-of-2
  mask, no chaining) = 12 KB/chunk.
* **Hit criterion**: same packed key within the chunk's current generation pass AND the chunk's
  `BlendingData` inputs provably immutable after construction **[A — unprovable today ⇒ part of why
  NO-BUILD]**.
* **Invalidation**: entries are identity-scoped and die with the `NoiseChunk` (phantom-reach
  reaper, the TASK-01/09 lifecycle pattern [M]); no cross-chunk invalidation exists because inputs
  are per-construction **[A]**. Fail-safe rule: any unprovable mutation path, any ctx that is not a
  pure function of the key ⇒ bypass to the original path for that call (doubt = invalidate).
* **Why it is NO-BUILD today**: vanilla already keeps a per-column structure for the non-EMPTY case
  [A — HOOK §1]; a second parallel cache re-creates the `MarkerCache` 4.54x shape [M: per-call cache
  bookkeeping dominates]; for the EMPTY case (the only measured 316x surface) C0 is strictly better
  (0 B vs 12 KB/chunk, ~2 ns vs hash probe). The g3/g15 cachedBatch measurements (0.980/0.985 [M])
  show the blend-math layer gains nothing from caching.

## 2. Patch mechanics — the classfile transform

### 2.1 Pipeline (identical skeleton to `src/area_map.rs` [M])

1. `register()` (cplugin_init): gate OFF ⇒ log `blend_cache: dormant` and register **nothing**
   (post-3a270ee discipline; the proto already implements this half [M-code]). Gate ON ⇒
   `cplug_sdk::hooks::register_bytes(NOISE_CHUNK_CLASS, cb)` — on the class's own load
   (READY=false) the callback stashes pristine bytes once (`ORIG_BYTES`) and returns `None`;
   it never parses, never defines classes (COMPUTE_FRAMES deadlock rule, improved_noise header [M]).
2. `activate()` worker (quiet thread, post-`wait_for_boot()`): parity self-test V1 (exists in
   proto [M-code]) → candidate sighting probe (`log_candidate_sighting`, exists) → **no-op
   retransform capture** if the class predates the hook (improved_noise pattern [M-code]) →
   compute the splice from captured bytes (pure Rust, below) → guards → `READY=true` →
   **exactly one** `cplug_sdk::retransform_class(NOISE_CHUNK_CLASS)` → the callback serves the
   precomputed bytes. Nothing class-related happens on the JVMTI callback thread beyond an
   `Arc` clone (§5).
3. Kernel-selection policy: variant C0 routes to **no native kernel**, so `decide()` is not its
   gate; arming logs one audit-style line for `CRUSSTY_KERNEL_POLICY=audit` visibility. Variant B
   (bridge call to `newEmptyBlenderSummary`, which IS in `PROVEN_WINS` "P500 WIN (244x)" [M]) is
   **rejected**: non-EMPTY remainder is closed vanilla logic, and a JNI round-trip (≥35–90 ns floor
   [M errata]) is ~30x slower than the guard's ~2 ns [E]. The kernel stays the parity oracle.

### 2.2 The splice, byte-exact

Target: `NoiseChunk.blendOffset(Lnet/minecraft/world/level/levelgen/DensityFunction$FunctionContext;)D`
and `blendAlpha(...same...)D` **[A — candidates per proto `CANDIDATE_PATCH_METHODS`; a candidate
that does not resolve ⇒ redesign, never force (HOOK §3.3 rule)]**. Variant A assumes the blender is
reachable as a field of `this`; variant A2 (target = `Blender.blendOffset/Alpha`, `this` IS the
blender singleton) drops the `getfield`. Byte-exact guard, prepended before the **untouched**
original body:

```
variant A (target NoiseChunk.blend*, G = 12 bytes):
  0: aload_0                 0x2a
  1: getfield #blender       0xb4 cp2      ; this.blender
  4: getstatic #Blender.EMPTY 0xb2 cp2
  7: if_acmpne +5            0xa6 00 05    ; -> 12  (operand = target 12 − opcode addr 7)
 10: dconst_0 | dconst_1     0x0e | 0x0f   ; offset → +0.0, alpha → 1.0 [A: javap-extracted]
 11: dreturn                 0xaf
 12: <original instructions, byte-identical>

variant A2 (target Blender.blend*, G = 9 bytes):
  0: aload_0 / 1: getstatic / 4: if_acmpne +5 (-> 9) / 7: dconst_0 | dconst_1 / 8: dreturn
```

Code-attribute surgery (all bounded-checked, panic-free on hostile bytes — the A4 audit rules of
`classfile.rs` [M]):

* **Constant pool**: append-only `Pool` reuse (`classfile.rs::Pool::find/push`, saturation guard
  A4-F3 [M]); needed entries: UTF8s + `Fieldref NoiseChunk.blender` (if absent) + `Class Blender` +
  `Fieldref Blender.EMPTY`. No existing index moves.
* **StackMapTable**: the original first frame at absolute F0 gets its delta changed to `F0 − 1`
  and ONE `same_frame @G` is prepended (frame_type byte = G; valid since G ≤ 63); all later deltas
  unchanged (the shift propagates through `offset(i) = offset(i−1) + delta(i) + 1`). If the original
  method has **no** SMT (branchless getter — plausible for these leaf methods [A]), the patched
  method needs exactly one new frame: `same_frame @G`. Same hand-computation class as
  `classfile.rs::patch_update` (5075→3320 B) [M].
* **Exception table** (if present): every `start_pc/end_pc/handler_pc` += G; the handler's SMT
  frame shifts through the same delta rule.
* **LineNumberTable / LocalVariableTable**: **dropped** (debug-only attributes; legal to omit;
  avoids shifting every bci). Method access flags, name, descriptor, and the class's field/method
  tables are untouched — `SCHEMA_CHANGED` guard G8 [M].
* **max_stack** = max(original, 2) — the guard peaks at two refs before `if_acmpne`; `max_locals`
  unchanged.

### 2.3 Relation to the existing patchers

`area_map.rs` = whole-body replacement with a rebuilt 82-byte body + 2-frame SMT, computed at
patch-time from a captured fixture-shaped class [M]. `improved_noise.rs` = whole-body replacement
via `cplug_sdk::asm::replace_body` (COMPUTE_FRAMES, pre-defined helper) [M]. This design is a third,
smaller transform — **prepend-guard** — because the optimization only owns the EMPTY equivalence
class and must not re-express the non-EMPTY remainder. It deliberately reuses `classfile.rs`
(`Pool`, `find_method`, `parse_layout`, bounds-checked readers) and NOT the ASM helper: no bridge
call is emitted, so no helper class, no COMPUTE_FRAMES, no kernel-loader define. New code lands as
one function beside `patch_update` (e.g. `prepend_guard(bytes, method, desc, spec)`) with the same
roundtrip-test + byte-mirror-test pattern [M precedent: `tests/area_map_smoke`].

## 3. Invalidation — the state machine

Cache-key of the patch itself: (class internal name, `orig_len`, FNV-1a-64(orig bytes), splice-spec
version, JVM class-major). Single entry per process; nothing mutable ⇒ no coherency protocol.

| event (bytes delivered to the byte hook) | fingerprint vs orig / patched | action |
|---|---|---|
| pristine first load (READY=false) | — | stash once, return None |
| arming retransform | == orig | serve `PATCHED` (Arc clone) |
| any later retransform event (incl. our own re-fired events) | == patched | return **None** (idempotent; already installed) |
| foreign redefinition/retransform (spark, another agent, Paper update) | == neither | **retire forever**: serve None, log once `blend_cache: retire (foreign bytes)` — never overwrite someone else's patch |
| parity self-test failed (V1) | — | dormant **forever this process**, no retry (proto `PARITY_OK` semantics [M-code]) |
| unresolved candidate / class-major > JVM major / capture > 1 MB / boot timeout | — | dormant forever (self-arrest) |
| `CRUSSTY_NATIVE_BLEND_CACHE` unset | — | hook not even registered (gate read once per process; change = restart) |
| `CRUSSTY_KERNEL_PREF=old\|conservative` | — | dormant (conservative operators get vanilla + the two proven patches only) |

**Fail-safe rule (the brief's "сомнение = инвалидация")**: every branch not explicitly proven safe
falls to the right toward "do nothing". Serving a stale splice can never happen (fingerprint gate);
serving over foreign bytes can never happen (retire gate); wrong constants can never ship silently
(V3 javap-truth + V4 terrain A/B, §6). The world-state dimensions the brief asks about — world
switch, seed change, chunk-state change — require **no** invalidation logic in C0 because the cache
holds no data keyed by them: the constant is world-independent (0.0/1.0 are properties of the EMPTY
equivalence class, not of a world [A — V3/V4 verify]), and everything else is re-read per call.

## 4. Memory budget

| item | size | bound | lifetime |
|---|---|---|---|
| `ORIG_BYTES` pristine NoiseChunk capture | expected 50–200 KB [E — ImprovedNoise 5.7 KB / SingleUserAreaMap 5.1 KB are the measured small cases [M]; NoiseChunk is a far larger worldgen class [A]] | **hard cap 1 MB** — refuse patch if larger | process |
| `PATCH_CACHE` (`Arc<[u8]>`, TASK-26 end-state shape) | orig + ≤1.5 KB (12 B guard + 1 SMT frame + ≤6 CP entries + attribute headers; LNT/LVT drop may shrink) | ≤ orig + 1.5 KB | process |
| runtime per-instance | **0 B** (no side table, no weak map, no handle) | 0 | — |
| per-call allocations | 0 (two loads + acmp + const + return) | 0 | — |
| process-lifetime total | ≤ 2 × capture-cap + 1.5 KB ⇒ **≤ ~2.1 MB absolute worst**, expected ≤ ~0.6 MB | as stated | never freed (same policy as `KERNEL_LOADER` global ref [M]) |
| C1 (if ever built) | 12 KB/chunk × cap 64 live chunks = **768 KB hard cap**, off-heap, LRU by last-touch generation, phantom-reach free | 768 KB | tied to chunk lifetime |

## 5. Interaction with retransform (foreign instrumentation: spark & friends)

* **JVM TI chain semantics**: on any later `RetransformClasses`/instrument-agent retransform of
  `NoiseChunk`, the byte hook fires with the class's **current** bytes — i.e. our patched image.
  A java-agent transformer (spark) then transforms *our* bytes: our patch survives inside its
  input chain, and if spark emits a re-instrumented NoiseChunk, the hook fires again with
  spark's output ⇒ fingerprint = foreign ⇒ **retire** (log once). We never re-serve; the honest
  contract is "the last foreign redefinition wins, we stand down" — a profiler Instrumenting
  worldgen code is a legitimate owner of the class.
  Note for transparency: `improved_noise.rs`'s serve branch does **not** fingerprint today [M-code,
  C5] — this hook must not copy that weakness; if C5/TASK-26 lands first, adopt its `Arc` shape and
  extend it with the fingerprint triple (§3).
* **What the patch does NOT survive**: a foreign `RedefineClasses` (not retransform) replaces the
  class image wholesale; our retire branch detects it on the next hook event (the redefine itself
  fires the hook with the new bytes). Detection is guaranteed because every class-image change in
  the JVM flows through the registered hook (canary: the retire log line is the alert).
* **JIT**: a redefinition/retransform invalidates the class's compiled code (standard JVM TI
  redefinition behavior) — no stale-code window [A-JVM; verified implicitly by the soak + self-test
  after arming].
* **Retransform-count discipline**: exactly one arming retransform from us (area_map/improved_noise
  precedent [M]); repeated hook firings are handled by the fingerprint machine, not by more
  retransforms. `PATCHED.swap(true)` dedup of `area_map.rs` is insufficient here (it cannot
  distinguish foreign bytes) — the fingerprint triple replaces it.

## 6. GAMEPLAY SAFETY (the gate — most important section)

**Project rule**: no gameplay-value changes; a last-bit shift in blend math moves terrain
(HOOK §3.5 keeps bit-exact as default; ULP fallback only with explicit review sign-off — default:
never). Optimization ≠ fraud.

**Invariant**: for every (NoiseChunk, FunctionContext), the patched method returns the identical
64-bit double (`to_bits`, NaN compared by bits, **sign of zero compared by bits**) and performs the
same side effects (none) as vanilla. Non-EMPTY: original instructions verbatim ⇒ trivially
identical. EMPTY: the guard returns the constant — which is exactly the constant vanilla's EMPTY
path computes **if and only if V3 says so**.

**Verification stack (all mandatory, in order, same process):**

* **V1 — kernel-model parity** (exists [M-code]): 10k random samples through the real injected
  bridge (`oldEmptyBlenderSummary` vs `newEmptyBlenderSummary`), fresh `long[64]` per call, return
  int exact + written longs bit-exact, **0/10 000 budget**, determinism probe every 1000th,
  exceptions = failure. Confirms the oracles; not yet the splice.
* **V2 — splice parity on a stub JVM** (new; `tests/area_map_smoke` pattern [M]): Python test
  mirror assembles NoiseChunk-shaped stub classes (EMPTY and non-EMPTY shapes), applies the splice
  byte-in-byte, requires sha256 equality with the Rust-produced gold splice; the spliced stub loads
  in a JVM; EMPTY shape returns the constant (checked via `Double.doubleToRawLongBits`), non-EMPTY
  shape returns the unpatched result; `VerifyError` = loud fail, JVM keeps original bytes, patch
  self-arrests.
* **V3 — javap truth extraction**: the EMPTY constants (+0.0 / 1.0, signs included), the blender
  field name/descriptor, and the method descriptors are read from `javap -p -c` of the **live**
  kernel class. If the vanilla EMPTY path is not a plain constant (any conditional, any extra math,
  `-0.0`, a NaN sentinel) ⇒ redesign or NO-GO. Never patched from this document's assumptions.
* **V4 — terrain identity A/B** (final gate before any live serve; HOOK §3.4.6): same seed, gate
  OFF vs ON, `forceload` (a) a border region — exercises non-EMPTY, (b) a fully-EMPTY region —
  exercises the guard; resulting chunk data compared **byte-identically** (region files or chunk
  PPOI hash). One differing byte = rollback criterion.
* **V5 — collateral regression gate**: full P500 rerun (49 groups) + TASK-14 ratio-gate
  (baseline.json, paired 1.2x [M]) green — the patch adds no native calls, so g21 medians must be
  unchanged [M expectation]; a moved median elsewhere = unexplained side effect = rollback.

**Phase ladder**: Phase 0 this doc (proto dormant, gate OFF) → Phase 1 probe (observation-only,
§7, BENCH.lock, GO/NO-GO recorded as an addendum to this file — a negative result is a valid
delivery, sibling doc §7 precedent) → Phase 2 implementation + V1+V2, still `PATCH_ENABLED=false` →
Phase 3 V3+V4 on a local staging server; only then env-gated serve (`CRUSSTY_NATIVE_BLEND_CACHE=1`)
→ Phase 4 P500 before/after + **24 h live soak** → Phase 5 default-flip proposal (separate
decision, out of scope). Live server /home/z/server is never touched during bench windows.

**Rollback criteria — any ONE triggers immediate disable (env unset + restart) and permanent
process-dormant**: ≥1/10k parity mismatch; any `VerifyError`/`NoClassDefFoundError`/
`ExceptionInInitializerError` naming NoiseChunk/Blender in the log; V4 1-byte terrain diff;
unexplained worldgen exception during soak; live chunk-gen wall-time regression > 5% vs gate-OFF
[E threshold, recorded per run]; an unexplained `retire` event. **Rollback mechanics**:
retransform-based patches are not runtime-reversible — the contract is "restart without the gate ⇒
vanilla path" [M, HOOK §5]; every failure mode self-arrests to dormant (no in-process retry);
alerting greps: `blend_cache: dormant|parity|SELF-TEST FAIL|retire|guard armed|retire (foreign)`.

## 7. Required benches & acceptance

All runs under `flock /tmp/crussty_bench.lock`; live-server interference noted per report
convention [M convention].

**Phase 1 probe (GO/NO-GO, observation-only, ~half a day [E]):**

1. `javap -p -c` live `NoiseChunk`/`Blender`: candidates exist with expected descriptors? EMPTY
   path already constant-folded in bytecode? (already-folded ⇒ H2 ⇒ **NO-GO**, close TASK-29-build).
2. `log_candidate_sighting()` runtime probe (proto, exists [M-code]).
3. g21 N-scaling falsification: `-Dp500.n=1/16/256/4096`; old* linear + new* flat ⇒ kernel models
   confirmed [M-method, HOOK §1].
4. async-profiler/JFR 60 s on a live worldgen loop (read-only): blend frames ≥ 0.5% of worldgen
   thread time ⇒ GO candidate; < 0.1% ⇒ NO-GO [E thresholds].
5. V1 parity — mandatory regardless of verdict.

**GO** = (1) not-already-folded ∧ site resolves ∧ (4) ≥ 0.5% ∧ V1 pass. **Any single miss ⇒ NO-GO;
record the negative result here.**

**Phases 2–4 (only after GO):** V2 stub-JVM pass + gold-sha match; V3 truth file committed as a
test fixture; V4 byte-identical chunks on staging; P500 before/after (the patch's own before/after
is the **in-server** A/B — identical seed, forceload N chunks, median-of-5 wall-clock gate OFF vs
ON; P500 g21 itself is expected flat [M expectation] and serves as the no-collateral canary);
TASK-14 ratio-gate green; 24 h live soak with grep-marker audit: only expected
`blend_cache:` lines, zero unexplained errors, phase-4 chunk-gen improvement ≥ 3% on an
EMPTY-dominated world **[E target — if the measured win is below the run's noise floor, verdict =
NOT-VISIBLE ⇒ keep dormant; an invisible optimization on worldgen-critical code fails the
project's risk/benefit bar, sibling doc §7]**.

## 8. Open questions (each blocks its phase; verification plan attached)

1. Exact method surface on Paper 1.21.10 (`blendOffset/blendAlpha` vs Blender-side variants) —
   javap + probe (Phase 1). A non-resolving candidate ⇒ redesign, never force.
2. Is `NoiseChunk.blender` set-once/final? Only C1 depends on it (C0 re-reads per call) — javap
   field flags + constructor scan (Phase 1).
3. H1 vs H2 (does live Paper pay the EMPTY machinery?) — §7 probe items 1+3+4.
4. Vanilla EMPTY constants incl. sign of zero (+0.0 vs −0.0) — V3 javap extraction.
5. Live K (blend lookups per column per chunk-gen) — async-profiler Phase 1 item 4.
6. Paper-specific worldgen patches in 1.21.10 touching the blend path (moonrise/blending rewrite?)
   — diff Paper sources during Phase 1.
7. Retransform ordering with spark/other agents — staging drill: force a foreign redefinition of
   NoiseChunk and observe the retire line (Phase 3).
8. Hook registration vs class-load race (NoiseChunk may load before plugin init) — improved_noise
   no-op-retransform capture pattern [M-code]; confirm in Phase 2 boot log.
9. p0/p1 semantics of the g21 kernels (batch-count hypothesis) — affects only the harness sweep;
   N-scaling probe.
10. Whether a branchless target method (no original SMT) exists on the live class — javap; the
    §2.2 single-frame case covers it.

## 9. Sources

`docs/HOOK_BLEND_CACHE.md` (B10: semantics, §3.4 checklist, §3.5 parity rules, §5 rollout);
`docs/BLEND_CACHE_DESIGN.md` @ 4fb9d12 (sibling: guard-splice verdict, per-column-cache rejection
evidence, H1/H2 arithmetic); `src/proto_blend_cache.rs` (Phase-1 vehicle: gate, capture, V1,
sighting probe); `src/area_map.rs` + `src/classfile.rs` (register_bytes → capture → quiet-worker →
one-retransform pipeline; Pool/find_method/SMT machinery, A4 bounds guards); `src/improved_noise.rs`
(dormant-gate discipline 3a270ee, predates-hook capture, G-guards); `src/kernel_policy.rs`
(PROVEN_WINS g21 "P500 WIN (244x)", DO_NOT_WIRE MarkerCache 4.54x, audit_wire);
`bench/p500/results/P500_REPORT_v2.md` §21 (95.3 µs / 301.1 ns) + §Wins (316.45x, floor anchors);
TASK-10 errata (floor 35–90 ns); TASK-01/09 (phantom-reaper lifecycle pattern for C1);
`tests/area_map_smoke/` (byte-mirror + JVM stub verification precedent); TASK-14 ratio-gate
(baseline.json, 1.2x paired gate).
