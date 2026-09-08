# G9: Whole-method byte hook on `DensityFunctions$Ap2.fillArray` — DESIGN ONLY (dormant by default)

Session: S7-13 (Task 2-a). **Design doc only — no implementation landed, no `src/` changes, no boot/bench ops.** Author: g9-design subagent. Inputs: S7-12 javap recon (`reports/G4_JAVAP_RECON_g9_g42.md`), G4 conventions (`docs/G4_SITE_PATCH_DESIGN.md`), matrix g9 row + §2.1 (`docs/BATCH_ADOPTION_MATRIX.md:110,96,23-25,199,244`), wiring plan §B.4/§B.7 (`docs/BATCH_WIRING_PLAN.md:222,235-239,303-306`). File:line refs verified against master `b06dead` this session unless marked "per G4" (pre-rebase cite, §8.4).

## 1. Purpose and scope

G9 specifies — on paper only — the whole-method byte hook for the g9 `DensityAp2MinMaxFill` production site identified by the S7-12 recon: the **MIN arm (bc 154–226) + MAX arm (bc 229–298) of `net/minecraft/world/level/levelgen/DensityFunctions$Ap2.fillArray([Lnet/minecraft/world/level/levelgen/DensityFunction$ContextProvider;)V`**, reachable via `ChunkStatusTasks.generateNoise` → `NoiseChunk.fillSlice(ZI)V` (loop bc 17–132) → `NoiseChunk$NoiseInterpolator.fillArray` → `Ap2.fillArray` (recon §1.1–1.2). The recon's verdict rules out Variant R there — the chain is `invokevirtual`/`invokeinterface` with **zero kernel-shaped invokestatic** (recon §1.3) — leaving the area_map-pattern whole-method hook as the only byte-level path.

**Win-tier analysis first, honesty constraint.** The matrix g9 row (`BATCH_ADOPTION_MATRIX.md:110`): sig `(III[J)I`, **119.8–120.2 ns** (tier B), batch **MEDIUM**, R **22–87 ns/call**, S1/S2 **0/192 calls/tick** = **0.00/10.46 µs/tick**, E 0; adoption path quoted verbatim: *"Byte-hook on the section fill loop (area_map pattern: loader-local helper + 64-case selftest + env gate) ONLY after JFR confirms per-section call amplification."* The 10.46 µs/tick is a MODEL (192/tick × R_mid 54 ns, §5.1 `:199`), not a measurement. Batching is irrelevant at this size: direct 119.8 ns vs dispatcher ≈205 ns fixed + ≈40 ns/op marginal (G4 §1) — **the batch loses**, so the only conceivable HOOK win is a kernel swap, and whether any g9 swap-win exists inside the closed `.so` is UNKNOWN (P500 stubs carry no mirror notes, recon §0/§1.3). **Zero gameplay-value changes are claimed.** This doc lands nothing; implementation without the JFR proof is out of scope by definition.

## 2. What exists today (the pieces G9 would compose)

- **area_map whole-method pattern** (`src/area_map.rs`): `register()` byte hook with READY/PATCHED one-shot flags (`:50-74`); helper classes defined into the TARGET class's loader, never the bootstrap (loader-shadowing rule `:12-15`); post-activation self-test driving the REAL bridge over random rects (`:17-22`). Body-swap engine #1: `src/classfile.rs` `patch_update` = whole-method Code replacement with hand-built StackMapTable (per G4 §2, `:341-487`).
- **cplug-sdk ASM** (`cplug-sdk/src/asm.rs`): op 1 **REPLACE_BODY** / op 2 MAKE_FIELDS_PUBLIC under COMPUTE_FRAMES via the embedded `SdkAsmHelper` (helper java `:96-248` per G4 §2); `ReplaceBody{method_name, method_desc, bridge_owner, bridge_name, bridge_desc, args}` with `ArgSpec::Local{slot,ty}` / `ArgSpec::ThisField{name,desc}`; per-.so unique helper name (`:11-16`); warmup pre-resolves ASM classes at define time, never lazily. This is the mechanism improved_noise uses (`src/improved_noise.rs:544-571` ReplaceBody spec).
- **Capture discipline** (improved_noise): pristine sighting captured in the hook callback while READY=false — the callback performs NO JNI/ASM work (current master `:192-206`; G4 §2's `:155-163` cite is pre-rebase, §8.4); patch computed on the quiet activation worker; exactly one retransform. Never ASM on a callback thread: COMPUTE_FRAMES resolves frame types via `Class.forName` and deadlocks against live class loads (`improved_noise.rs:24-33`).
- **Class-major guard**: embedded bytes must not exceed the JVM's class-file major, fail-early with actionable numbers (`improved_noise.rs:303-336`; stale-v69 incident).
- **Kernel-policy gating**: `debug_assert!` + `audit_wire` before the READY flip (`improved_noise.rs:605-609`), PROVEN_WINS registry as single source of truth (`src/kernel_policy.rs`; verdict classes per wiring plan B.2.3).
- **Why Variant R is out for this site** (`reports/G4_JAVAP_RECON_g9_g42.md` §1.3): the fill loop is `invokevirtual`/`invokeinterface` (`fillArray`, `compute`, `minValue/maxValue`) plus `Math.min/max` statics — a same-length CP-operand retarget has nothing to match on.

## 3. Mechanism sketch (design-only) — two candidates, one shared content problem

The target body carries **all four Ap2 arms in one tableswitch** on `TwoArgumentSimpleFunction$Type.ordinal()` (enum order ADD, MUL, MIN, MAX; `Type` ordinal order verified by the recon §1.2). A whole-body replacement must therefore reproduce *everything*, not just the g9 MIN/MAX arms.

**Candidate A — cplug-sdk ASM REPLACE_BODY** (same vehicle as improved_noise; recommended). Swap `fillArray`'s body for one `invokestatic` bridge. Args are expressible with existing ArgSpec: `Local{slot 0, 'L'}` (this), `Local{slot 1, 'L'}` (the `ContextProvider[]` param), `ThisField{argument1/argument2, 'L...'}` (private-field reads inside the class's own method are legal; no access-flag changes). What the replacement must preserve bit-exactly (recon §1.2 bytecode evidence): (1) the ordinal dispatch incl. the **ADD/MUL arms — NOT transcribed by the recon** (only MIN/MAX are fenced); a fresh `javap -p -c` pass over them is a hard prerequisite; (2) MIN arm: `argument2.minValue()` **hoisted before the loop** (bc 154–158), loop bound = `arraylength` (bc 172), per element `cur = dst[i]`, `if (cur >= arg2min)` via **`dcmpg`/`ifge`** → compute else keep cur (the short-circuit), `Math.min` @216, `dastore`/`iinc`; (3) MAX arm: same shape against `maxValue()` with **`dcmpl`/`ifle`**, `Math.max` @291; (4) exact NaN comparison polarity and the per-element `argument2.compute(ctx.forIndex(i))` sequence. Risks: the body re-verifies under COMPUTE_FRAMES (fine off-thread), but any divergence in any arm changes terrain generation — this is the highest-blast-radius method c-crussty has ever hooked (every density chain; recon §3.1).

**Candidate B — `classfile.rs` `patch_update` hand-rolled** (precedent: area_map). Hand-building a StackMapTable for a ~300-byte body containing a tableswitch plus two double-compare branches is high-risk; the S7-12 `Pool::parse` latent bug (G4 §7) shows how long parser bugs can hide when fixtures lack the hard shapes. **Not recommended.**

**PRIMARY BLOCKER (alongside JFR).** Any new body must call a bridge with EXACTLY the original semantics. The closed `.so` exposes `PaperNativeDensityAp2MinMaxFill.oldSummary/newSummary` `(III[J)I` whose **3-scalar production mapping is UNDOCUMENTED** (recon §1.3 "honest unknown"; §3 open question 2), and the `(III[J)I` shape has **no slots** for the two `DensityFunction` refs or the `ContextProvider[]` (the wave-1 shape-coverage gap, wiring plan `:235-239`). So the bridge-callable semantics are **UNPROVEN**: the recon's own framing is that the kernel is "the shape-compatible *calibration* of the hook body, not a drop-in retarget" (§1.3). No bridge can be written or parity-tested until the scalar mapping is documented or a new native entry point is proven — regardless of JFR.

**Recommendation:** Candidate A (ASM REPLACE_BODY), exactly because COMPUTE_FRAMES removes the StackMapTable risk that makes B dangerous; B only if ASM ever proves unavailable in the target loader.

## 4. Gate chain (proposed; nothing built)

1. **G-env:** `CRUSSTY_NATIVE_AP2_FILL`, **off by default**; fail-safe parse — only exact on-values (`1/true/on/yes`) widen, anything else = off (improved_noise `enabled()` pattern, `improved_noise.rs:76-84`).
2. **JFR precondition (what constitutes proof):** matrix §2.1 scaling identity (`:23-25`): R ns/call ≡ R ms per 10⁶ calls; **≥1 ms/tick requires ≥12k–40k amplified single-op calls/tick** (§5.3 `:244` repeats the bar for g9 explicitly). For g9 at R_mid 54 ns that is ≈**18.5k calls/tick** (10⁶/54) — ≈96× the modeled S2 192/tick. Proof artifact = an archived JFR/async-profiler method profile from a live boot (reports/), counting ≥ that many `Ap2.fillArray` MIN/MAX-arm invocations/tick **on the actual loop** — "never arm on the model alone" (B.7 Stage 3, `:303-306`; the model's own ceiling if proven is S2 ≈ 0.047 ms/tick total envelope, ≤0.13% of a 50 ms tick). If the measured rate stays model-consistent, the honest terminal state is *hook-never*.
3. **Kernel-policy:** any new wiring gets a **PROVEN_WINS entry with parity evidence** per the TASK-54 lifecycle standard — byte-exact old≡new parity over full inputs incl. dst, two identical runs = cross-JVM determinism, armed-boot self-test, unarmed boots show 0 marker lines (`docs/RESULTS_LEDGER.md:62`; `docs/PROVEN_WINS_SYNC.md` §4 item 5) — plus `debug_assert!`/`audit_wire` at activation (`improved_noise.rs:605-609`) and the matrix-row "64-case selftest" (`:110`).

## 5. Risks

- **Verifier/StackMapTable complexity:** the replacement body re-verifies from scratch; COMPUTE_FRAMES does frame types but the whole four-arm re-derivation must compile clean — any frame/type mismatch = VerifyError at retransform (fail-safe: serve proven original bytes, area_map PATCHED-swap convention).
- **Short-circuit branch semantics:** the `minValue()`/`maxValue()` early-outs are **gameplay-value-adjacent** — they decide which cells keep the incoming `dst[i]` — and MUST be bit-exact, including `dcmpg` vs `dcmpl` NaN ordering and the hoist of `minValue()`/`maxValue()` out of the loop (recon §1.2).
- **Undocumented scalar mapping** (§3 PRIMARY BLOCKER): parity cannot even be formulated against the closed `.so` kernels today.
- **JFR overhead:** profiling distorts the very loop being measured; profile with the hook dormant, on a otherwise-stock boot (same convention as all live boots on this 2-vCPU box, B.7 preamble).
- **Retransform schema rules:** no field/access-flag changes — JVMTI rejects schema changes (`improved_noise.rs:1-15` header discipline); a body-only swap is schema-clean by construction.
- **Panic-across-JNI:** hook/worker paths keep the poison-recovery + `catch_unwind` discipline (improved_noise TASK-46 comments `:130-136`; hook-callback catch_unwind in `cplug-sdk/src/lib.rs:136-146`, per G4 §6).
- **ADD/MUL collateral:** g8 `DensityAp2Fill` (ADD/MUL arms, recon §1.1) shares the same method — hooking `Ap2.fillArray` rewrites arms outside g9's declared scope; their parity burden is identical.

## 6. Validation plan (when/if implemented)

1. **Offline parity gate FIRST** — the TASK-54 WinPairParity pattern on the REAL `.so` (byte-exact result + full dst over exhaustive-shaped inputs, two identical runs), incl. synthetic `ContextProvider`/`DensityFunction` fixtures reproducing the short-circuit branches; **no armed boot before parity is green**.
2. **Dormant boot PASS-by-absence:** env unset → 0 hook marker lines, served bytes bit-identical (G4 §7 dormant-boot pattern; e2e row modeled on `batch site arm`).
3. **P500 drift tripwire** after any `src/` landing (S7-12 gate rule: full 49-group rerun, ratio-gate rc=0).
4. Armed-boot marker lines in the improved_noise shape (`ap2_fill: dormant|pristine sighting|computed patch|hook serve|self-test ...`) + verifier acceptance = retransform rc=0.

## 7. Verdict

**NO-GO until BOTH blockers clear:** (a) **JFR amplification proof** — measured ≥ ~18.5k calls/tick on the live `Ap2.fillArray` loop (≈2 orders above the 192/tick model; matrix §2.1 band 12k–40k); (b) **documented scalar semantics / parity gate** — the `(III[J)I` mapping written down or a new native entry point proven, with byte-exact parity green. Either alone is fatal. Today the modeled value is 10.46 µs/tick (≤0.13% of a tick) and the bridge semantics are unproven: **this hook is a paper design and should stay one.**

## 8. Open questions

1. **Hook grain:** `Ap2.fillArray` (finest, both arms, every density chain) vs `NoiseChunk.fillSlice(ZI)V` (chunk-grain driver, larger helper) — recon §3.1; only JFR on a live boot can decide.
2. **Scalar-mapping documentation** (recon §3.2): record the intended `(III[J)I` mapping (and g8 flat/nested variants) in a bench README so parity work never re-derives it from `.so` strings.
3. **Does any g9 kernel-swap win exist in the closed `.so` at all?** No mirror notes anywhere (recon §0 sources list); without one, the hook has no measurable payoff even if JFR fires.
4. **Doc-drift backlog (doc-only):** G4 §2/§4 cite `improved_noise.rs:155-163`/`:570-574` — current master has the pristine-sighting branch at `:192-206` and `audit_wire` at `:605-609` (post TASK-53/54 rebase); refresh on the next docs wave.

## 9. Measured update (TASK-57, 2026-09-09, agent-7625532f) — both blockers now MEASURED

First live JFR profile of this box (544 s: boot + ~8 min idle-tick + graceful stop; dormant env;
`bench/e2e/results/JFR_PROFILE_2026-09-09.md`): **blocker (a) resolved NEGATIVELY — stronger than
"amplification unmet": 0 of 927 ExecutionSamples contain `fillArray` anywhere in the stack.** All
`Ap2` traffic flows through single-value `compute()` (400+ frame-hits; top caller `Climate$Sampler.sample`,
117) inside one 5-second boot structure-ring burst (287 noise-stack samples 01:27:21–26, then zero per
minute-bucket). The hook target method does not run on this server/version/workload, so the ≥18.5k/tick
bar is unsatisfiable for `fillArray` — §7's "either alone is fatal" applies, and the §8 grain question
(§8.1) is answered empirically: the live surface is object-context `compute()`, not expressible by any
existing batch shape. **g9 remains closed as a paper design; revisit trigger = a player-driven worldgen
profile actually showing `fillArray` frames.** Boot-window noise cost (2.87 CPU-s per boot) was probed by
the TASK-58 boot A/B and REFUTED as a lever: the noise arm chain is server-boot-gated (arming lands
after `Done (`), so the pre-Done burst cannot route native (`bench/bootab/results/BOOTAB_NOISE_2026-09-09.md`;
paired n=5/arm medians 16.738 vs 16.542 s, ranges overlap) — orthogonal to this hook, and now measured closed.
