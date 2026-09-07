# BLEND_CACHE_DESIGN — blend-cache classfile patcher (TASK-29, DESIGN ONLY)

> Status: **design, no code** (TASK-29 wave-3, SESSION 005). Concretizes `docs/HOOK_BLEND_CACHE.md` (B10 proposal) against the actual patch
> machinery (`src/improved_noise.rs`, `src/classfile.rs`, `cplug-sdk`) and P500 v2 numbers. Gate `CRUSSTY_NATIVE_BLEND_CACHE` (default **OFF**,
> dormant by construction — improved_noise post-3a270ee discipline). Headline decision up front:
>
> **The blend cache is NOT a cache.** The only implementable, bit-exact, evidence-backed design is an EMPTY-blender constant-fold guard splice
> (~20 bytes of prepended bytecode, zero per-instance state). A literal per-column Java-side cache is **rejected with measured evidence** (§5.2).
> Build order: probe first (H1/H2 falsification, §5.4); the splice itself is gated on a GO verdict and probably does NOT get built (§7).

---

## 1. Problem — what exactly is recomputed per call

**(a) Vanilla `NoiseChunk` blend lookups (the real target).** `NoiseChunk` evaluates density functions per cell; the leaf nodes `blend_offset()` / `blend_alpha()` ask the chunk per column for offset/alpha derived from neighboring `BlendingData` (candidate methods, Mojang-mapped, **unverified until `javap -p -c` on the live kernel** — proto `src/proto_blend_cache.rs::CANDIDATE_PATCH_METHODS` probes `blendOffset`/`blendAlpha` `(LDensityFunction$FunctionContext;)D` on `net/minecraft/world/level/levelgen/NoiseChunk`, plus `Blender`/`BlendingData` existence). Per HOOK_BLEND_CACHE §1 the same column's values are requested **dozens of times** per chunk-gen, each going through vanilla's per-column cache machinery. For the **EMPTY blender** (overwhelmingly common: no old-chunk neighbors) the answer is a *constant* (offset `0.0`, alpha `1.0` — to be confirmed, never assumed), yet the machinery still runs. Measured model (P500_REPORT_v2 §21, g21 `PaperNativeNoiseChunkBlendCache` `(II[J)I`, 256-column batch):

| kernel | models | median |
|---|---|---:|
| `oldEmptyBlenderSummary` | machinery per column (hash probe + boxing/alloc) | **95.3 µs** ≈ 372 ns/col |
| `newEmptyBlenderSummary` | O(1) early-out on the constant answer | **301.1 ns** ≈ 1.2 ns/col |

Ratio 0.003 → **316x** (v1's "244x" superseded; worklog session 005 errata). Largest win in the P500 baseline — *between two native kernels*. Whether either kernel models the live Paper path is exactly what §5.4 must answer.

**(b) Our patched `ImprovedNoise.noise(DDDDD)D` path — NOT this task's target.** Already wired: patched body calls `ImprovedNoiseNativeOps.noise()` → striped handle lookup (~15–40 ns, C4/TASK-01/09) + one native call (35–90 ns floor). The per-`this` handle never changes, so the lookup is per-call recomputation, but it cannot be cached on-instance (`SCHEMA_CHANGED` forbids new fields) and C4's verdict stands: leave unless an A/B proves it visible. Out of scope.

**(c) Our patch machinery itself — already cached; formalize only.** `improved_noise.rs` computes the patched classfile **once** on the quiet activation worker into `PATCH_CACHE`; remaining waste is C5/TASK-26 (serve branch clones the full `Vec<u8>` ~3–6 KB + 2× `class_version` + `eprintln!` on the class-load thread). TASK-26 (another agent) moves it to `Arc<[u8]>`; this design consumes that end-state, not duplicating it.

## 2. Design — the EMPTY-guard splice (variant A)

### 2.1 What is cached, where, in what shape

| item | shape | location | lifetime |
|---|---|---|---|
| pristine `NoiseChunk` bytes | `Vec<u8>` (single capture) | `ORIG_BYTES` (proto pattern, `OnceLock<Mutex<Option<_>>>`) | boot → process end |
| patched bytes **+ orig fingerprint** | `struct PatchEntry { orig_len: usize, orig_fnv: u64, patched: Arc<[u8]> }` | `PATCH_CACHE` (one `Arc`, TASK-26 end-state) | boot → process end |
| EMPTY constants | bytecode literals `dconst_0` / `dconst_1` **inside the splice** | in the patched code attribute | = patched bytes |
| method references | constant-pool entries appended at patch-compute time (classfile.rs rule: append-only CP) | inside patched bytes | = patched bytes |
| per-NoiseChunk-instance cache | **none — zero bytes** | — | — |

Memory budget: orig capture assumed ≤256 KB (NoiseChunk is a large worldgen class; sanity cap: refuse patch if capture >1 MB); patched = orig + ≤1 KB (guard ~15 B + 1 StackMapTable frame + ~4–6 CP entries). Runtime per-instance cost = **0 B** — no `WeakHashMap` entry, no handle, no side table (a naive per-chunk cache would add one entry per live NoiseChunk — thousands on a busy world — plus the GC-timing risk flagged in HOOK §3.1). Process-lifetime total ≤ ~512 KB, one-shot at boot, never released (same policy as the `KERNEL_LOADER` global ref). No bridge/native-handle shape by design: variant A performs **no JNI call**, so the 35–90 ns floor is irrelevant to it (§5.3).

### 2.2 The splice itself

Prepend a guard to the resolved blend method body; the original bytecode is kept **byte-identical** after it (this is what makes non-EMPTY behavior trivially unchanged — same instructions, same order):

```
aload_0
getfield  <NoiseChunk.blender Lnet/minecraft/world/level/levelgen/blending/Blender;>
getstatic net/minecraft/world/level/levelgen/blending/Blender.EMPTY L...Blender;
if_acmpne L_ORIG
dconst_0            ; blendOffset -> 0.0   (blendAlpha -> dconst_1 = 1.0)
dreturn
L_ORIG: <original instructions untouched>
```

* If `javap` shows the delegate lives on `Blender` itself (`this` IS the blender singleton), the guard shrinks to `aload_0 / getstatic EMPTY / if_acmpne L_ORIG / dconst_0 / dreturn` and the target becomes `Blender.blendOffset` — patch-site resolution is a checklist item; **a candidate that does not resolve ⇒ redesign, never force** (HOOK §3.3 rule).
* `StackMapTable`: original entries stay valid except the FIRST frame's `offset_delta` grows by the guard's length, plus one new `same_frame` for `L_ORIG` (locals unchanged, empty stack). Same hand-computation class as `classfile.rs::patch_update` (area_map, 5075→3320 B) and `cplug_sdk::asm::replace_body` (improved_noise, 5691→5403 B).
* Field/modifier tables untouched (guard G8: `SCHEMA_CHANGED`).
* Computed **once** on the quiet activation worker, after `wait_for_boot()` (G4) and the class-major guard (G1); exactly **one** retransform; nothing defined inside the callback (G3 — no COMPUTE_FRAMES, no `define_class` mid-retransform; ASM helper pre-defined).

### 2.3 Cache key, invalidation, thread-safety

* **Key**: (target class internal name, `orig_len`, FNV-1a-64 of orig bytes, splice-spec version, JVM major). Single-entry cache — exactly one target class per process; no map, no eviction, nothing mutable ⇒ no coherency problem exists.
* **Invalidation events**:
  1. *Retransform with foreign bytes*: the hook receives CURRENT class bytes on every retransform. `len/fnv == patched` → return `None` (already ours; idempotent under repeated retransform events). `len/fnv != orig` → someone else redefined `NoiseChunk`: **retire** — serve `None` forever, log once (`blend_cache: retire (foreign bytes)`); serving a stale splice would clobber the foreign patch.
  2. *Parity failure* (§3): dormant **forever**, no in-process retry (existing `PARITY_OK=false` semantics).
  3. *Gate / kernel policy*: `CRUSSTY_KERNEL_PREF` / `CRUSSTY_KERNEL_POLICY` are read once per process (`OnceLock` in `kernel_policy.rs`) — change requires restart (same contract as every existing gate). `CRUSSTY_KERNEL_PREF=old|conservative` ⇒ blend hook **dormant** (conservative operators get vanilla + the two proven patches only).
  4. *Foreign agent redefinition*: covered by (1); *class unload*: n/a (kernel classes are never unloaded).
* **Thread-safety**: single writer (activation worker, spawned once; `Mutex` write of `PatchEntry` → `READY` Release-store, improved_noise discipline). Reader = JVMTI class-load/retransform thread: `Arc` clone = one refcount bump, zero copy, zero Java work — the serve branch is *designed as* the TASK-26 `Arc<[u8]>` end-state; if TASK-26 has not landed when this is built, implement the Arc shape locally, never clone `Vec`. Worldgen threads see no added shared state: the guard reads `this`'s own field per call — the same field vanilla reads — lock-free by the same argument as vanilla.

## 3. Bit-exactness invariant and how it is verified

**Invariant**: observationally identical — same doubles bit-for-bit (`to_bits`, NaN compared by bits; NO ULP fallback — HOOK §3.5 keeps bit-exact as default because a last-bit shift moves terrain), same call sequence to natives (variant A adds/removes **zero** native calls), same terrain. Formally the splice only changes behavior on the EMPTY input class, from "constant via machinery" to "the same constant, directly"; the non-EMPTY path is the original instructions verbatim (§2.2).

Verification stack (all mandatory, in order, same process):

1. **Kernel-model parity** (already implemented, `proto_blend_cache.rs` §3.5): 10k samples through the real injected bridge — `oldEmptyBlenderSummary` vs `newEmptyBlenderSummary`, fresh `long[64]` dst per call, return `int` exact + written longs bit-exact, **0/10 000** mismatch budget, determinism probe every 1 000th sample, exceptions count as failures. Confirms the *kernels* agree; NOT yet the splice.
2. **Splice parity on a stub** (new, mirrors `tests/area_map_smoke/`): the Python test mirror assembles a synthetic NoiseChunk-shaped class, applies the splice byte-in-byte, requires sha256 equality with a Rust-produced gold splice; the spliced stub loads into a JVM and is driven with EMPTY and non-EMPTY contexts, compared `to_bits` against the unpatched stub. VerifyError at retransform = loud failure — JVM keeps original bytes, patch self-arrests.
3. **`javap` truth extraction**: EMPTY constants (`0.0`/`1.0`) and the blender-field name come from `javap -p -c` of the live kernel class, not from this document. If vanilla's EMPTY path does NOT return the plain constants (any conditional, any extra math) ⇒ redesign or NO-GO.
4. **Terrain identity A/B** (final gate, HOOK §3.4.6): same seed, gate OFF vs ON, `forceload` a border region (non-EMPTY) AND a full-EMPTY region; resulting chunk data byte-identical.

## 4. Interaction with retransform, audit_wire, kernel gates

* **Serve branch** (TASK-26 end-state `Arc<[u8]>`): see §2.1/§2.3; the callback is `match (len,fnv) { orig => Some(patched.clone()), patched => None, _ => retire }` — no locks held across JVM work, no parse, no per-firing logging (one-line armed log).
* **`audit_wire` / `decide()`**: variant A routes to **no native kernel**, so `kernel_policy::decide()` is not its gate; on arming, log one audit-style line (`blend_cache: guard armed (pure Java fold, no native routing)`) so `CRUSSTY_KERNEL_POLICY=audit` shows the wiring decision. For completeness: the tempting variant B — patch body → bridge call to `newEmptyBlenderSummary` (which IS in `PROVEN_WINS`, "P500 WIN (244x)"; `decide()` would Allow) — is **rejected**: the non-EMPTY remainder is closed vanilla logic we must not reimplement, and even for the EMPTY case a JNI round-trip (≥35–90 ns floor) is ~30× slower than the guard's ~2 ns (§5.3). The native kernel stays what it already is: the parity oracle.
* **`DO_NOT_WIRE`**: untouched — the 4 confirmed regressions are unrelated. If anyone ever proposes the per-column Java cache, it belongs on that registry (§5.2 evidence) before any wiring.
* **Env gate**: `CRUSSTY_NATIVE_BLEND_CACHE` stays default-OFF; OFF ⇒ byte hook NOT registered, `activate()` no-op, log `blend_cache: dormant` (G2 discipline; proto already implements this half). `CRUSSTY_KERNEL_PREF=old` ⇒ dormant (§2.3.3).

## 5. Expected gain — honest ESTIMATE (nothing below is measured in-server)

### 5.1 The live-path hypotheses

* **H1 (old\* models live Paper)**: EMPTY lookups still pay machinery (~372 ns/col incl. boxing/alloc). Guard saves ~(372−2) ns × 256 cols × K lookups/column ⇒ ~95 µs × K per chunk-gen, K = "dozens" (HOOK §1) ∈ [10, 100] ⇒ **0.95–9.5 ms/chunk-gen**. At any realistic chunk rate that is tens of ms/tick — implausibly large, which is itself evidence *against* H1 (someone would have noticed a ~100 µs/column tax on every chunk).
* **H2 (new\* models live Paper — the likely truth)**: Paper already short-circuits EMPTY the way `newEmptyBlenderSummary` models. Win = **0 − patch risk ⇒ DO NOT BUILD**.
* **H1′ (middle)**: machinery is a bare map hit, no alloc (~20–60 ns) ⇒ saving ~5–60 µs × K per chunk-gen ⇒ 0.05–6 ms/tick at 5 chunks/tick — visible only at high K; marginal otherwise.

### 5.2 Why a literal per-column Java-side cache is rejected (negative result)

* Direct measurement, this codebase: `PaperNativeBlendedNoise` g3 — `cachedBatchSummary` 50.9 µs vs `oldBatchSummary` 51.9 µs = **0.980, parity** (P500_REPORT_v2: cached-vs-old wins *nothing* at the blend-math layer). Same story g15 `ImprovedNoiseDerivative` (0.985–1.000). The ONLY blend verdict that is a win is the O(1) empty early-out.
* Precedent: `PaperNativeMarkerCache.cachedSummary` — a per-call cache variant — measured **4.54x SLOWER**, scale-invariant, canonical `DO_NOT_WIRE` entry ("per-call cache bookkeeping dominates"). A Java-side blend cache would re-create that shape: on-instance impossible (`SCHEMA_CHANGED`), off-instance = `WeakHashMap` probes that cost what they save, on a path worldgen workers already run lock-free (HOOK §3.1 risks confirmed by these two measurements).
* Conclusion recorded as evidence: **do not build the map; fold the constant.**

### 5.3 Is a Java-side cache even visible when the kernel sits at the JNI floor?

The floor (35–90 ns) bounds *JNI transitions*; variant A makes zero transitions (~2–3 ns: two field loads + compare), so the floor is irrelevant to the guard — the real visibility question is whether vanilla's per-call cost it removes is ~370 ns (H1, visible) or already ~ns (H2, invisible). Any design that routes through a bridge (variant B) pays ≥ the floor per call and is dominated by the guard by construction.

### 5.4 GO/NO-GO probe (cheap, observation-only, run FIRST under BENCH lock)

1. `javap -p -c` the live `NoiseChunk`/`Blender` (Paper 1.21.x is Mojang-mapped): do candidates exist with expected descriptors, and does the EMPTY path already constant-fold in bytecode? Early constant-fold ⇒ H2 ⇒ **NO-GO**.
2. `log_candidate_sighting()` runtime probe (already in proto).
3. P500 N-scaling falsification (HOOK §1): re-run g21 with `-Dp500.n=1/16/256/4096`; `old*` linear + `new*` flat confirms the kernel models; paired with (1) it localizes which model is live.
4. async-profiler/JFR 60 s on a live worldgen loop (read-only, no patch): blend frames ≥0.5% of worldgen thread time ⇒ GO candidate; <0.1% ⇒ NO-GO.
5. 10k parity (§3.1) — mandatory regardless of verdict.

**GO** = (1) not-already-folded ∧ site resolves ∧ (4) ≥0.5% ∧ (5) pass. **Any single miss ⇒ NO-GO; record the negative result and close.**

## 6. Rollout gates + rollback

| Stage | What | Default |
|---|---|---|
| 0 (this doc) | design; no code | OFF |
| 1 | §5.4 probe only, via existing dormant proto (`PATCH_ENABLED=false` untouched); BENCH.lock; results appended here | OFF |
| 2 | splice implementation + stub byte-mirror tests (§3.2) + 10k parity; still no serve | OFF |
| 3 | seed A/B terrain identity (§3.4) on local server; only then flip serve | OFF |
| 4 | default-flip proposal — separate decision, live A/B + long-run parity | n/a |

**Rollback**: unset `CRUSSTY_NATIVE_BLEND_CACHE`, restart. Retransform patches are not runtime-reversible; the contract is "restart without the gate ⇒ vanilla path". Self-arrest to dormant: parity fail, unresolved site, class-major guard, boot timeout, foreign-bytes retire. Log greps: `blend_cache: dormant|parity|SELF-TEST FAIL|retire|guard armed`.

## 7. Verdict

**Conditional GO on the Stage-1 probe only; probably DO-NOT-BUILD on the patcher.** The evidence (BlendedNoise parity 0.980, MarkerCache 4.54x, the 316x being between native kernels, the implausibility of H1) points at H2 — Paper most likely already folds the EMPTY case, making the guard an invisible optimization, and an invisible optimization on worldgen-critical terrain code fails the project's risk/benefit bar outright. The probe costs ~half a day, touches nothing (proto exists), and either earns the splice a real GO with numbers or kills TASK-29's build with proof — both are valid outcomes. The per-column Java cache is rejected now, unconditionally.
