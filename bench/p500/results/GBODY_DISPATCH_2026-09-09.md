# G-BODY — whole-body dispatch prototype: retransform proven, parity bit-exact, 0.815× end-to-end (TASK-71)

**G-BODY gate (standalone precursor of design doc §8 Session 2) — VERDICT: GO.**
Date: 2026-09-08+08 (cron tick 14:40). Agent: agent-7625532f. CPU-only session
(no server boot, no deploy, no module .so changes — zero overlap with the
neighbor's TASK-68 server lane). Rig: `bench/step0_noise/bodyagent/`
(`BodyAgent.java` javaagent + `BodyDispatch.java` + `Step0BodyBench.java` +
`run_body.sh`); raw: `GBODY_DISPATCH_RAW_2026-09-09.log` (runs 1–6 incl. all
negative iterations).

## 1. What was proven

The full patched shape of design doc §3.2 (whole-body swap of
`PerlinNoise.getValue(DDDDDZ)D` → one `invokestatic BodyDispatch.getValue`
→ lazy per-instance native handle → `nativeGetValue(JDDDDDZ)D`), executed
through a REAL JVM retransform:

1. **Retransform machinery**: javaagent (`Premain-Class` +
   `Can-Retransform-Classes: true`), trigger-based flow — pristine bytes
   captured at class load (11030 bytes = exactly the jar's PerlinNoise.class
   size), baseline computed on the pristine body, then
   `retransformClasses` swaps the whole method body mid-JVM. Retransform
   applies cleanly to this owner (no VerifyError, no frame problems: the
   replacement body is straight-line — no StackMapTable needed, ASM
   `COMPUTE_MAXS` only, no class-resolution pitfalls).
2. **Parity**: 0/20000 mismatches pre-vs-post retransform on the same
   deterministic coordinate stream, in the SAME JVM — the whole patched
   path (ASM-rewritten bytes + dispatch + WeakHashMap + JNI + native
   kernel) is bit-exact against the pristine production body. Hard
   execution canary: the patched body MUST build the cont handle during the
   parity phase (guards against the "patch applied but never executed"
   failure mode — see §2).
3. **End-to-end cost** (P500: 120 ms batches, median-of-5, fwd+rev,
   min-of-medians, SINK, two separate JVMs per TASK-58 A/B discipline):

| Arm | ns/call | note |
|---|---|---|
| J_pristine (no agent, pristine JVM) | 424.5 | production body |
| **P_patched (agent, whole-body swap)** | **345.9** | lazy handle + dispatch + JNI |
| N_direct_native (same JVM as P) | 352.5 | bare nativeGetValue, pre-warmed handle |
| **P/J ratio** | **0.815×** | inside the TASK-70 predicted 0.70–0.96× window |

**Dispatch overhead = P − N ≈ −6.6 ns → ≤ measurement noise (±5 ns).** The
WeakHashMap lazy-handle lookup adds nothing measurable to the bare native
crossing. Cross-run consistency: J = 424.5 here vs 429.3/402.4/403.2 in the
TASK-70 runs; P/J = 0.815× vs the isolated-kernel 0.825× (TASK-70) — the
full patched shape does not degrade the kernel economics.

4. **Handle build cost (one-time per instance)**: warm 22–60 µs
   (5 measurements: 60032/58932/40341/21884 ns + earlier runs 24–43 µs);
   first-ever build in a cold JVM 4–14 ms (reflection + JIT cold path).
   Production impact: noise objects are created at world load, thousands of
   getValue calls follow — negligible; a world with ~200 noise objects pays
   ≈ 5–12 ms one-time at load.

## 2. Negative iterations (kept honest — this is WHY the prototype exists)

* **Runs 1–4 — false GO caught by the canary**: the first agent version
  used the method descriptor `"(DDDDDDZ)D"` (SIX `D`s) — the real 6-arg
  getValue is `(DDDDDZ)D` (FIVE doubles + boolean). The patch "applied"
  (transformer returned rewritten bytes, no exception) but matched NO
  method → installed bytes were functionally identical to the original →
  parity trivially passed while the patched body never executed. Detected
  only because the fresh-instance handle built COLD (4 ms) instead of warm
  and the handle map was empty for `cont`. The execution canary is now a
  HARD gate (`patch-not-executed` verdict).
* **Run 5 — ASM field shadowing**: inside the anonymous `MethodVisitor`
  the unqualified `mv` resolves to the INHERITED `this.mv` field (the null
  delegate), not the outer local — NPE at `mv.visitCode()`. Fixed by
  capturing the writer in a differently-named final (`body`).
* Both bug classes are exactly what Session 2 would have hit inside the
  module .so, where debugging costs server boots. The prototype paid for
  itself twice.

## 3. Consequences for the design / roadmap

1. **Session 2's body-template work is de-risked**: the whole-method swap
   of the octave-loop owner works through a real retransform with bit-exact
   parity and the dispatch machinery is free. What remains for the module
   implementation is engineering integration, not unknowns: bridge class
   defined in the module loader (RuntimeStubs/include_bytes discipline),
   production handle lifecycle (phantom-reaper TASK-01 pattern), G9 quiet-
   worker retransform discipline, env gate + B.2.2 degradation ladder.
2. **Remaining gate: G-AB** (live paired A/B on the TASK-63 harness,
   Mann-Whitney n=5/arm, p<0.1) — the only decision-grade production
   number. All CPU-side evidence is now GO: G-STEP0 → G-RECON → G-ABI →
   G-BODY.
3. Coordination note: G-AB requires many server boots and the module .so —
   it MUST wait for / sequence with the neighbor's TASK-68 server lane
   (design doc §6 risk).

## 4. Rig provenance

`bench/step0_noise/bodyagent/`: `BodyAgent.java` (transformer + whole-body
ASM rewrite, trigger-based), `BodyDispatch.java` (lazy per-instance handle,
decoded TASK-70 ABI P3/D1/F1; prototype WeakHashMap — production uses the
phantom-reaper lifecycle), `AgentProbe.java` (dormancy canary via system
property set by premain), `Step0BodyBench.java` (two-mode driver:
`java` pristine arm / `patched` arm with parity + canary + build cost +
P/N timing), `run_body.sh` (flock BENCH.lock, one run per tool call).
ASM 9.8 from the server's bundled libraries; classpath = patched jar +
all server libs (same as TASK-67/69/70 rigs).
