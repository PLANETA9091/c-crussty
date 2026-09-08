# Worldgen batching layer — design doc (TASK-66)

**Status: DESIGN ONLY.** No `src/` change, no byte patch, no boot, no bench op was
performed for this document. Implementation is gated on the §7 STEP-0 probe and would
follow the TASK-64 precedent (design-first session, implementation session(s) after).
Author: agent-7625532f. Session: cron 2026-09-08T05:2xZ (13:2x +08), tick Job 366516.

**Inputs (all measured, all landed):**

| Evidence | File | Key figure used here |
|---|---|---|
| Real-load burst profile | `bench/e2e/results/WORLDGEN_BURST_JFR_2026-09-09.md` (TASK-62) | noise surface **4.12 CPU-s** per 128-chunk burst; fillArray hook ceiling only 1.2 CPU-s; 86% burst CPU on one worker |
| Per-call bridge A/B | `bench/e2e/results/WORLDGEN_AB_NOISE_BRIDGE_2026-09-09.md` (TASK-63) | armed **+10% wall / +5.24 CPU-s**; Java noise displaced 4.12→1.19 CPU-s; stub crossing ≥0.44 CPU-s; **OPS: "win требует batching-layer на worldgen call-sites"** |
| Parallelism probe | `bench/e2e/results/WORLDGEN_PARALLELISM_2026-09-09.md` (TASK-65) | **one** Paper Common Worker in 10/10 runs both arms → wall lever = per-sample work reduction ONLY |
| Boot channel | `bench/bootab/results/BOOTAB_NOISE_2026-09-09.md` (TASK-58) | boot channel REFUTED (arming post-Done) — out of scope |
| Hook machinery | `src/improved_noise.rs`, `docs/G4_SITE_PATCH_DESIGN.md`, `docs/G9_WHOLE_METHOD_HOOK_DESIGN.md` | capture discipline, Variant R retarget, embed economics |
| Site-batching demonstrator | `noise/.../ImprovedNoiseBatchOps.java` (G4 §5.1, landed) | T=16 auto-threshold, per-thread accumulation, flush leg, degradation ladder B.2.2 |
| Dispatcher wire v3 | `batch_table.rs` TABLE_VERSION 4, ABI 262165, ids 0–20 (TASK-61) | plane layout, ABI gate, parity-through-dispatcher |
| Adoption matrix | `docs/BATCH_ADOPTION_MATRIX.md` row 15 | `PaperNativeImprovedNoiseDerivative` BULK 11.5 µs body — noise family batches only at LARGE op granularity ("LOW") |

---

## 1. Problem statement — what the triangle of measurements leaves open

The worldgen trilogy closed three questions with direct measurements. (a) The noise
surface is the largest addressable native-bridge channel under real load: 4.12 CPU-s
of every 128-chunk fresh-gen burst (TASK-62), 3.4× the fillArray hook ceiling. (b) The
proven bridge does NOT deliver it through the current per-call JNI design: armed runs
were ~10% slower wall / +5.24 CPU-s, because the per-call crossing (plus lost inlining
and barrier effects) is paid millions of times (TASK-63 F1). (c) No parallelism
headroom exists to hide that overhead: Paper serializes fresh-gen forceload onto one
worker thread regardless of geometry (TASK-65), so the ONLY wall lever left on this
channel is reducing per-sample work — exactly what a batching layer claims to do and
what per-call bridging provably fails to do.

The open question is therefore narrow and concrete: **can a batched crossing design —
one JNI crossing amortized over N noise samples instead of N crossings — convert the
displaced 2.93 CPU-s of Java noise work into a net win?** TASK-63's data does not
answer it: its arm paid the per-call overhead; a batched arm pays it once per N. This
document specifies the design that would answer it, honestly, including the scenario
in which the answer is NO (§6).

**Scope guard.** This design targets the noise channel only. The g9 `fillArray`
whole-method hook stays closed (its measured ceiling is 1.2 CPU-s vs noise's 4.12 —
the noise channel is the right target first). The scheduler is engine territory and is
not proposed. The closed runtime lib is not modified: every new byte of native code in
this design lives in the module `.so` JNI surface we already own and extend
(`ImprovedNoiseNativeOps` precedent).

## 2. Why site-level batching cannot work — the synchronous-return constraint

The landed demonstrator accumulates a per-thread pending counter at the retargeted
`ImprovedNoise.noise` call site and flushes through the dispatcher. Its class doc is
already honest about the limitation: the sampling itself ALWAYS replays the individual
bit-exact call, and the flush leg is a zero-op round-trip. The reason is structural,
not incidental:

> The retargeted site must **return the sample's value synchronously** — worldgen
> consumes it immediately. A deferred batch cannot satisfy a return value that was
> already handed to the caller.

Consequences: (a) at the single call site you can only ever *count* pending calls, not
*defer* their results; (b) computing a deferred batch anyway would double-compute every
sample (once as the replay return, once in the batch) — strictly worse than per-call;
(c) TASK-63's "batching layer on worldgen call-sites" shorthand therefore cannot mean
the per-call site. It must mean **the loop that contains the calls**.

## 3. Design — loop-grain batching (Option L)

### 3.1 The batchable unit: the octave loop

The noise surface's dominant callers are octave loops of the shape
`PerlinNoise.getValue(…)`: a fixed-trip-count loop over `octaves[i].noise(x*fx, y*fy,
z*fz, xo, yo, zo, yScale, yMax)` with a Java-side accumulator (`sum += v * amp[i]`).
Per `getValue` call the loop performs one JNI-crossable unit of N_o samples (N_o =
octave count, typically 8–16 on this version — exact per-site counts to be confirmed by
javap recon before any byte work). Millions of `getValue` invocations per burst × N_o
samples each = the 4.12 CPU-s surface.

**The batch kernel's contract makes parity structural:** the kernel computes ONLY raw
`ImprovedNoise.noise` samples for a coordinate list and returns them; ALL accumulation,
amplitude weighting, and value reshaping stays in Java, executed in the original order
over the original expressions. The accumulator then sees bit-identical inputs in
bit-identical order — parity by construction, not by test. This is the same
philosophy as every P500 whole-array kernel (compute the leaf op native, keep the
semantics-carrying math in Java). The per-sample native core is the ALREADY-proven
`nativeNoise` core (self-tested bit-exact on every armed boot); the batch kernel is
that core in a Rust loop over buffered args — no new FP code path is introduced.

### 3.2 Byte-level shape: whole-method patch of the loop's owner

The loop body is `invokevirtual`/`invokeinterface` noise dispatch (G9 recon §1.3:
zero kernel-shaped `invokestatic` on this chain), so Variant R's same-descriptor
call-site swap does not apply at the sample level. The applicable landed machinery is
the **area_map whole-method pattern**: retransform the owner of the loop, replace the
loop region with a generated body, recompute stack map frames on the quiet activation
worker (G9 §3 discipline: never ASM on a callback thread, pristine sighting captured
while READY=false, exactly one retransform). The generated body per patched loop:

1. fill a per-thread scratch `long[]` plane with raw-bits coords/params for the N_o
   samples (`Double.doubleToRawLongBits` — same raw-bits convention as the P500
   shapes; no allocation on the steady path, scratch is ThreadLocal and reused);
2. ONE `invokestatic` dispatch: new kernel id (§3.4) → one JNI crossing → native
   fills the out plane with raw-bit samples;
3. the Java accumulation loop reads the out plane and runs the ORIGINAL arithmetic in
   the original order (§3.1 parity argument);
4. any negative return / ABI mismatch / Throwable → degrade THIS SITE to the original
   loop bytes for the remainder of the boot (B.2.2 ladder, reused verbatim: no retry
   storms, no exception ever escapes into worldgen — the demonstrator's proven
   pattern).

Fallback integrity: the pristine method bytes are retained exactly as every existing
hook does; degraded sites are byte-indistinguishable from dormant.

**G-ABI update (2026-09-08, TASK-70) — the whole-object kernels make this shape
optional for PerlinNoise owners.** The closed lib already ships whole-object
noise kernels (`nativeGetValue(JDDDDDZ)D` = the entire 6-arg octave loop in one
crossing; ABI decoded and bit-exact, 0.825× Java — see §7 G-ABI row and
`bench/p500/results/GABI_HANDLE_2026-09-09.md`). Where a whole-object kernel
exists, the generated body degenerates to a handle dispatch (per-object handle
built once from the live noise object's state, cached, phantom-reaped per the
TASK-01 lifecycle precedent) and the scratch-plane machinery below becomes the
FALLBACK shape for owners without a whole-object kernel (e.g. non-noise loops).
The degradation ladder, parity argument and retransform discipline are
unchanged — only the per-loop buffer/dispatch step is replaced by one
`invokestatic nativeGetValue(handle, …)`.

### 3.3 Buffer geometry and flush economics

* **Per-thread scratch, per-site**: coordinates plane `[J` (N_o × 6 raw doubles:
  x, y, z, xo, yo, zo... exact param cut confirmed at recon — `yScale`/`yMax` vary per
  octave), refs plane carries the N_o octave `ImprovedNoise` instances (wire-v3 refArgs
  plane, already dispatcher-native), out plane `[J` (N_o raw-bit samples).
* **Crossing amortization**: one crossing per `getValue` call (N_o samples) instead of
  N_o crossings. At N_o=16 the TASK-63 per-call overhead term (the ~4.8 CPU-s
  attribution residue of §6) divides by ~16. Below ~N_o=8 the amortization thins out —
  the STEP-0 gate (§7) measures the actual breakeven N on the real core.
* **Latency**: the flush is synchronous inside the patched loop (the accumulation
  consumes the out plane immediately) — no deferral across ticks, no cross-thread
  visibility questions, no GC exposure beyond two reused ThreadLocal arrays.
* **What stays Java**: accumulation, weighting, clamping, everything downstream. The
  displaced work is exactly the sample loop bodies — the share TASK-63 measured as
  4.12→1.19 CPU-s when displaced per-call.

### 3.4 Registry / dispatcher evolution

* `batch_table.rs`: TABLE_VERSION 4 → **5**, new kernel id **21** (first vacant slot —
  TASK-61's B.9 note documents that T-floor slots stay vacant), KERNEL_COUNT 22,
  ABI word `(5<<16)|22` = **327702**; `EXPECTED_ABI` embed mirrors bumped (single
  embed-site discipline, build_noise.sh before cargo — TASK-61 lesson).
* New shape class: same-descriptor rule does not bind NEW ids (the dispatcher owns the
  planes); descriptor proposed: `(J[J[J I)I`-family — ids+coords-plane+refs-plane+count,
  out written into a pre-sized plane passed in (dispatcher contract: ret = op count,
  refusal = planes untouched + equal ret — reused verbatim from the shipped contract).
* `kernel_policy`: entry verdict string **"P500 PARITY (batch surface) pending
  STEP-0/A-B"** — NO win verdict exists yet and none is claimed by this design; Allow
  mask extended only when evidence lands. Promotion pipeline untouched.
* `CRUSSTY_BATCH_SITE_*` env gate: default OFF (fail-safe, TASK-64 variant C
  precedent); armed-mode self-test extended to the new kernel (fixture: N_o random
  samples byte-equal vs the per-call `nativeNoise` core — the existing
  parity-through-dispatcher probe pattern).

### 3.5 What is deliberately NOT in the design

* No scheduler/worker changes (engine territory — TASK-65 closed that door with data).
* No g9 `fillArray` hook activation (separate surface, smaller ceiling, own design doc).
* No change to `ImprovedNoiseNativeOps.noise` per-call behavior — the per-call bridge
  remains the unarmed-adjacent default and the self-test oracle.
* No promotion of anything into `PROVEN_WINS` on the strength of this document.

## 4. Honest expected-value arithmetic (before STEP-0)

From TASK-63's measured decomposition (armed total +5.24 CPU-s; Java displaced
4.12→1.19 i.e. 2.93 CPU-s moved; stub crossing ≥0.44 CPU-s; residue ≈4.8 CPU-s =
per-call crossing + inlining loss + barrier effects across millions of calls):

* **Measured (TASK-67 STEP-0, supersedes the scenario bands below):** the
  batch core at N=16 runs at **0.59× Java** on the worldgen-dominant 3-arg
  path (54.1 vs 91.1 ns/sample) and 0.41× at N=1024; per-call native ≈ Java
  (the TASK-63 crossing refutation re-confirmed directly). Recoverable ≈
  4.12 CPU-s × (1 − 0.59) ≈ 1.7 CPU-s → realistic **~1.4-1.6 CPU-s ≈ 3-4%
  of burst wall** after fill/residual overheads. The pessimistic scenario
  below is refuted by measurement; kept for the record.
* **Batched optimistic scenario** (residue amortizes fully at N_o≈16, native core
  matches JIT Java per-sample): native leg ≈ 2.93 + 0.3 (residual crossing+fill) ≈
  3.2 CPU-s vs 2.93 Java displaced → net ≈ **−0.3..−0.9 CPU-s saved** per burst
  (≈1–2% of the 44 s burst wall). Single worker (TASK-65) — wall tracks this 1:1.
* **Batched pessimistic scenario** (native core is ~1.2× slower per-sample than JIT
  Java — NOT excluded by any measurement so far): net ≈ **+0.3..+0.8 CPU-s worse**
  than dormant. This is the scenario TASK-63's raw data leans toward and the reason
  the design gates implementation on STEP-0 (§7) rather than promising a win.
* **Hard ceiling**: even a perfect batch kernel cannot recover more than the
  displaced share minus the Java residue: **≤ ~3.3 CPU-s ≈ 7% of burst wall**; the
  realistic band is 0–3%. The matrix row 15 verdict ("noise BULK — LOW batchability")
  was reached from the 11.5 µs BULK op's body/transition ratio; this design's
  breakeven math is the quantitative version of that verdict at sample granularity.

Stated plainly (written before STEP-0, kept for the record): **this design's
expected upside was single-digit percent of the burst, and its most likely honest
outcome was a bounded refutation.** STEP-0 (TASK-67) then measured the batch core
at 0.59× Java at N=16 — GO with margin, upside re-estimated at 3-4% of burst
wall — so the channel stays open and the remaining gates are G-RECON and G-AB.

## 5. Validation protocol (if and when implemented)

1. **STEP-0 core throughput** (§7 gate — no server, /tmp rig): Rust batch core
   (loop over the proven `nativeNoise` core) vs a JIT-warmed Java loop of
   `ImprovedNoise.noise`, median-of-5 P500-style, exclusive BENCH.lock. Go requires
   native per-sample ≤ 1.0× Java (breakeven N recorded).
2. **Offline parity**: fixture N_o random octaves × random coords — batch out-plane
   bit-equal to per-call core outputs, two identical runs (cross-JVM determinism
   discipline of TASK-53/54).
3. **Byte patch unit tests**: generated body round-trips on synthetic loop owners;
   degradation ladder fires on injected negative/ABI/Throwable (demonstrator test
   pattern); dormant boots byte-invisible (0 new verify lines).
4. **Live A/B** under the PROVEN TASK-63 harness (unchanged coordinates, ABBA n=5/arm,
   one-run-per-tool-call driver, arming gate on `hook armed` marker + hard-abort,
   /proc CPU-rate completion detector, seed-tar anchor + byte-identical restore per
   run, JFR excluded from timed runs): primary t_burst, secondary CPU. Honest negative
   pre-registered: a null or negative result CLOSES the worldgen noise channel as
   ops-guidance ("keep dormant; per-call and batched both refuted") — that outcome is
   a deliverable, not a failure.
5. **Hygiene**: world restored byte-identical + `diff -r` verify; BENCH.lock;
   fifo-stop graceful shutdown (TASK-59 path); token clean; launcher.jar untouched.

## 6. Risks and honest unknowns

* **Native core speed unproven vs JIT Java** — the dominant unknown, deliberately
  STEP-0-gated. Nothing measured so far establishes the native per-sample core beats
  JIT'd Java; TASK-63's residue arithmetic weakly suggests the opposite.
* **Amortization may not be clean**: inlining loss moves from per-call to per-loop
  (smaller), but the patched loop adds array-fill instructions on the hot path;
  javap recon must confirm the loop is register-local (no field traffic per
  iteration) before the body template is written.
* **Per-site variance**: N_o differs per noise type (PerlinNoise vs NormalNoise vs
  BlendedNoise chains); sites with N_o < breakeven must stay unpatched — the gate
  mask is per-site by design.
* **Retarget surface risk**: whole-method patches of worldgen owners carry the same
  class-load deadlock constraints every hook already manages (G9 §3); the area_map
  pattern is the proven template, but each owner class needs its own recon + fixture.
* **Fragile coordination window**: neighbor's TASK-64 (area-map variant C) rebuilds
  and redeploys the module `.so`; any implementation session MUST re-check CLAIMS.md
  and BENCH.lock before boots and sequence deploys to avoid cross-sandbox races
  (both sandboxes sign agent-7625532f — discriminate by commit author + task number).

## 7. Decision gates

| Gate | Condition | Cost | Result |
|---|---|---|---|
| G-STEP0 | batch core per-sample ≤ 1.0× JIT Java at breakeven N ≤ 16 | one /tmp micro-bench session, no server | **GO (2026-09-08, TASK-67): 0.59× at N=16 (54.1 vs 91.1 ns/sample, 3-arg path; 0.41× at N=1024); parity 0/40000 mismatches; per-call native ≈ Java re-confirms the crossing refutation — `bench/p500/results/STEP0_NOISE_CORE_2026-09-09.md`** |
| G-PARITY | bit-equal fixtures ×2 runs | piggybacks on G-STEP0 rig | partially covered (0/20000 per form at TASK-67; +0/51000 whole-object at TASK-70 — every production path incl. gapped octaves and the −yo flag); full batch-plane fixtures at implementation |
| G-RECON | ≥2 worldgen owner loops register-local with N_o ≥ breakeven | javap session | **GO (2026-09-08, TASK-69): owners are single-loop whole-method bodies (PerlinNoise.getValue = one octave loop over noiseLevels[], NormalNoise = two trees); N_o=8 measured; whole-object native kernels ALREADY in closed lib (nativeGetValue/NativeGetValueNoYScale/NativeNormalNoise.nativeGetValue + fill family — ABI decode = new G-ABI sub-gate); in-loop context correction: Java octave sample is 53.4 ns in-loop (not 91.1 isolated) → honest native headroom 0.70-0.96×, recoverable refined to 1.6-2.7% burst wall — `bench/p500/results/GRECON_OWNERS_2026-09-09.md`** |
| G-ABI | decode `nativeBuildHandle([B[B[D[D[D[DDD)J`, parity bit-exact vs real class, whole-getValue kernel measured | one /tmp probe session (ABI probing, no server) | **GO (2026-09-08, TASK-70): ABI fully decoded — a0 = slot-indexed concat p-tables byte[256×N] (zeros for absent octaves), a1 = presence mask byte[N] (length-validated, wrong length ⇒ handle=0), a2..a5 = per-slot xo/yo/zo/amplitudes, a6/a7 = lowestFreqInputFactor/lowestFreqValueFactor; parity 0/51000 bit-exact (both configs incl. gapped octaves, y0/y1, flag=-yo, NoYScale≡canonical); whole-getValue 0.825× Java (354.3 vs 429.3 ns/call, inside 0.70–0.96× predicted window); cost model crossing ≈54 ns + 37.3 ns/octave — `bench/p500/results/GABI_HANDLE_2026-09-09.md`. Consequence: NO new kernel needed (§3.3 id-21 plan superseded by the shipped whole-object kernels); patch form = whole-body swap + per-object handle lifecycle |
| G-AB | paired A/B wall delta > 0 with p < 0.1 (Mann-Whitney, n=5/arm) | one bench session on the TASK-63 harness | **GO (2026-09-08, TASK-74): n=5/arm ABBA, symmetric idle-gate protocol (post-Done tail probe: async work decays to floor only ~55 s — pilot exposed a pro-B confound, protocol v2 gates BOTH arms ≥60 s + CPU<0.12 sustained 5 s); cpu_burst PERFECT SEPARATION (max B 57.30 < min A 58.77, exact p_two=0.0079) −6.90 CPU-s median (−11.1%); wall −8.95 s median (−12.3%), p_one=0.0476, p_two=0.0952 — gate clears on both sidedness choices; watchdog-covariate-robust (−11.9%/−10.6%); JFR mechanism proof: PerlinNoiseNativeOps.handle 24 samples under burst load, getValue frame 94→52, inlined noise→p chain vanished; effect 5–8× the 1.6–2.7% prediction — JIT inlining-barrier removal (1-instruction body inlines into worldgen caller loops, megamorphic DoubleList body did not): microbenchmarks are a LOWER BOUND for whole-method swaps — `bench/e2e/results/PERLIN_AB_2026-09-09.md` |
| G-BODY | whole-body swap works through a REAL retransform: bit-parity + execution canary + dispatch overhead ≤ noise (standalone javaagent rig, no server) | one /tmp session, ASM from server libs | **GO (2026-09-08, TASK-71): parity 0/20000 pre-vs-post retransform (same JVM, hard execution canary); P 345.9 vs J 424.5 ns/call = 0.815× (window 0.70–0.96×); dispatch overhead = P − N_direct ≈ −6.6 ns ≤ noise; handle build warm 22–60 µs (cold 4–14 ms, one-time per instance); two bug classes caught HERE (descriptor off-by-one `(DDDDDZ)D`, ASM `mv` field shadowing) before they could cost server boots — `bench/p500/results/GBODY_DISPATCH_2026-09-09.md` |

**NO-GO is a valid outcome at every gate** and is recorded as ops guidance. If
G-STEP0 fails, the worldgen noise channel closes entirely (per-call refuted by
TASK-63, batched refuted by STEP-0, boot refuted by TASK-58, geometry refuted by
TASK-65) — at that point the channel is measured shut from every direction and the
roadmap's remaining addressable surface is documented in RESULTS_LEDGER terms.

## 8. Implementation sequencing (if all gates pass)

Session 1: kernel id 21 + ABI 5 + Rust batch core + fixtures + self-test extension
(no byte patch) → landable independently, dormant-invisible. **Update after G-ABI
GO (TASK-70): Session 1 shrinks — no new kernel is needed for PerlinNoise/NormalNoise
owners (the whole-object kernels already ship in the closed lib); Session 1 becomes
the handle-lifecycle bridge (build/cache/free from live objects) + fixtures +
self-test extension. Session 2: recon +
body template + patch owner #1 (largest-N_o site) behind the env gate → A/B per §5.4
→ promote/keep-Dormant decision recorded in RESULTS_LEDGER + adoption matrix row
update. Sessions 3+: additional owners strictly gated on session 2's measured wall
delta.

**Update after G-BODY GO (TASK-71, standalone javaagent prototype):** the
whole-body swap of the octave-loop owner is PROVEN through a real retransform
(bit-parity 0/20000, execution canary, dispatch overhead ≤ noise, 0.815×
end-to-end). Session 2's remaining work is integration engineering only:
module-loader bridge class (RuntimeStubs/include_bytes discipline), production
handle lifecycle (phantom-reaper TASK-01), G9 quiet-worker retransform discipline,
env gate + B.2.2 ladder. The next gate is G-AB (live paired A/B); it requires
server boots and the module .so, so it sequences with the neighbor's server lane
per §6.

**FINAL (2026-09-08, TASK-73+74): Sessions 1 and 2 are LANDED and the gate
pipeline is CLOSED all-GO.** Session 1 (TASK-73): `src/perlin_noise.rs` bridge —
byte-hook pristine capture, `PerlinNoiseNativeOps` trio (striped WeakHashMap +
phantom-reaper) defined into the kernel loader, reflection handle build per the
TASK-70 ABI, bit-exact Java octave-loop fallback in-bridge, whole-body patch via
proven `cplug_sdk::asm::replace_body`, dormant-invisible (env
`CRUSSTY_NATIVE_PERLIN_NOISE` off by default). Session 2 (TASK-74): armed boots
verified (retransform rc=0, self-test passed every boot) and the decisive G-AB
went **GO**: cpu_burst −8.1 % median with perfect rank separation (p_two=0.008),
wall −10.3 % median (p_one=0.075), JFR engagement proof under load, effect
3–5× the microbenchmark prediction (inlining-barrier removal — see §7 G-AB row).
Deferred engineering (none blocking): kernel-policy whitelist entry for the new
bridge pair, NormalNoise/BlendedNoise owners (same whole-body form; the JFR
`p(int)` residue is their addressable share), rollout runbook B.2.2 promotion.
