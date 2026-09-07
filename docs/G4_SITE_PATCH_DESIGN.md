# G4: First Batch Call-Site Consumer — Site-Patch Helper Design (dormant by default)

Session: S7-11 (2026-09-08). Status: DESIGN (implementation not landed). Author: main orchestrator, based on READ-ONLY recon (Task 2-c, S7-10) with file:line cites against master `704cba4`.

## 1. Purpose and scope

G4 is the remaining structural blocker between the batch rollout gate and actual execution: the batch kernel API is fully wired at the surface level (G1 gate `CRUSSTY_BATCH`, G2 do-not-wire hard guard, 15-kernel table, bridge class `crussty/batch/PaperNativeBatchDispatch`) yet **zero production call-sites consume it** — `rg 'batch_api::'` in `src/` matches only the init call (`src/lib.rs:210`) and doc comments. Nothing reads `rollout_mode()` except `batch_api::init` itself (`src/batch_api.rs:425`), so the gate is currently advisory: even with `CRUSSTY_BATCH=on`, no batch op ever executes. This document specifies the smallest honest consumer: a **call-site retarget helper** (Variant R) plus the **arming gate chain** it must pass through, shipped **dormant by default**.

Two honesty constraints shape everything below. First, measured dispatcher economics (worklog TASK-48): routing through `run()` costs a shape-independent fixed ≈205 ns/batch plus ≈40 ns/op marginal, so the floor-band wave-1 kernels (g42/g35/g39/g40, 34–90 ns direct) **can never beat direct calls** — batching is rational only for body-dominated kernels (≥ ~700 ns/op) at large K. Second, the runbook verdict on g42 (`docs/BATCH_ROLLOUT_RUNBOOK.md:97-103`): with the closed probe body, no measured threshold T exists and batching never wins on the probe. Therefore the first consumer is explicitly framed as a **gate-chain demonstrator**, not a performance claim: it proves the gate→arm→execute→degrade ladder end-to-end on the one call site we know byte-for-byte, and stays dormant until a real in-engine kernel body justifies arming.

## 2. What exists today (the pieces G4 composes)

**Batch surface.** Bridge class `crussty/batch/PaperNativeBatchDispatch` with `run` sig `([I[J[J[I[J[I)I` and `abiVersion()I = (TABLE_VERSION<<16)|KERNEL_COUNT` (131_087; `src/batch_api.rs:145-156,588-594`; `src/batch_table.rs:198,347`). Kernel table: 15 entries, ids 0–14, sig↔shape const-asserted (`src/batch_table.rs:201-341`); g42 `StaticCacheGet` is id 14, shape C, `(IIIII[I[J)I`. Wire contract: `args0` scalar plane packed by per-shape `scalar_width()` prefix sums, `args1` packed input arena for B/C shapes, `argCounts[i]` dual meaning (output capacity for A/A′, input length for B/C), shared `outs`/`outOffsets` windows, refused id → `-10` before any op executes (`src/batch_api.rs:36-70,711-1057`).

**Gate chain state.** G1 `RolloutMode{Off,Auto,On}` with fail-safe `parse_rollout` (only exact `auto`/`on` widen; `1`/`0`/garbage → Off) and OnceLock + boot marker (`src/batch_api.rs:308-363`). G2 `mask_bit(mode,class,method) = decide_in(mode,..).is_allowed() && do_not_wire_entry(..).is_none()` enforced per-op inside `run()` (`src/batch_api.rs:267-291,748-759`; tests `:1150-1180`). Missing: G4 arming (no consumer, no `batch: arm ...` line — the runbook `:133-138` expects that line to appear with the first consumer) and G5 threshold T (nothing in code; policy default T=16, g42→32, clamp {8,16,32,64} per `docs/BATCH_WIRING_PLAN.md:183-215`).

**Byte-patch machinery.** The module serves whole-class retransformed bytes through engine ClassFileLoadHook chaining (engine callback `CRUSSTY/runtime/src/lib.rs:330-460` → plugin dispatch hook `cplug-sdk/src/lib.rs:148-183`). Two rewrite engines exist module-side: (1) hand-rolled `src/classfile.rs` — bounds-checked CP parser with append-only pool and `method_ref` constructor (`:64-215`), `find_method` (`:243-275`), and `patch_update` = **whole-method Code replacement** with hand-built StackMapTable (`:341-487`); (2) embedded Java ASM helper `cplug-sdk/src/asm.rs` with ops **REPLACE_BODY (op 1)** and **MAKE_FIELDS_PUBLIC (op 2)** under COMPUTE_FRAMES (`asm-src/dev/dist/SdkAsmHelper.java:96-248`). **There is no call-site retarget capability anywhere** — no Code-attribute scanner, no CP-operand rewrite. On the engine side, `CRUSSTY/runtime/src/platform/transform.rs` has offset-fixup machinery but only as a `()V`-probe injector (`plan_before_call`, `:865-891`), engine-owned, and c-crussty registers no engine rules.

**The one known call site.** The improved_noise hook (env-gated by `CRUSSTY_NATIVE_IMPROVED_NOISE`, default off at `src/improved_noise.rs:62-69`) replaces the whole body of `net/minecraft/world/level/levelgen/synth/ImprovedNoise.noise(DDDDD)D` with an `invokestatic ImprovedNoiseNativeOps.noise(DDDDD+handle)D` bridge (`src/improved_noise.rs:513-539`). That invokestatic is a **fixed-offset, known-shape call site inside bytes we already produce** — the only call site in the codebase known byte-for-byte without any new class recon.

## 3. Variant R (recommended): same-length CP-operand retarget

**Mechanism.** In the target method's Code attribute, scan for `invokestatic` (opcode `0xb8`) instructions; for each, resolve the 2-byte CP index to its `CONSTANT_Methodref` and compare `class.method:descriptor` against the retarget spec. On match, append a new `CONSTANT_Methodref` entry for the batching helper to the pool (**append-only — the existing `Pool::method_ref`, `src/classfile.rs:188-196`, already builds this**), then rewrite only the 2-byte operand in place. The helper keeps the **original descriptor**, so the verifier-visible stack shape is unchanged: zero branch-offset fixups, zero exception-table changes, zero StackMapTable deltas, zero method-size growth.

**Cost.** One new capability: a bounded, panic-free Code-attribute scanner (walk `max_stack`/`max_locals`/`code_length`, step instructions, extract `0xb8` operands) — estimated 150–250 lines next to the existing `classfile.rs` parsers, following their bounds-checked style. Alternative implementation vehicle: a new **op 3 (RETARGET_INVOKESTATIC)** in the embedded `SdkAsmHelper` using ASM `visitMethodInsn` retargeting under COMPUTE_FRAMES — less code, but recompiles `asm-src/` and carries COMPUTE_FRAMES Class.forName deadlock risk if ever invoked on a hook thread (the discipline that forbids that is `src/improved_noise.rs:30-33,155-163`); the per-.so unique helper-name rule (`cplug-sdk/src/asm.rs:11-16`) and the CRUSSTY-side copy must not drift.

**Idempotency.** Retargeting must be a no-op on re-sighting the already-patched class: the scanner treats an invokestatic whose operand already resolves to the helper as PATCHED (the engine's `is_invokestatic_to` idempotency check is the pattern, `CRUSSTY/runtime/src/platform/transform.rs:842-849`), matching the module's PATCHED-swap convention (`src/area_map.rs:47,55-57`).

**Rejected alternative (Variant G, guard-prepend at MethodEntry):** inserting a batch-accumulating guard at method entry changes code length and therefore requires the full offset-fixup machinery (branches, exception table, StackMapTable deltas, LineNumber/LV) — that machinery is engine-owned, `()V`-only, and porting it module-side multiplies risk for zero benefit over Variant R. Rejected as primary path.

## 4. Gate chain: from env var to executed batch op

Arming is a one-time, boot-phase decision made on the quiet activation worker (never on a hook callback thread — deadlock discipline `src/improved_noise.rs:30-33`):

1. **G1:** `rollout_mode() != Off` (`src/batch_api.rs:338-363`).
2. **G2:** `mask_bit(mode, class, method)` — do-not-wire refusals hold even in `on` mode (`src/batch_api.rs:284-291`).
3. **G4-new:** proposed `batch_api::site_arm(site: &SiteSpec) -> bool`: checks 1+2, `debug_assert!(decide(..).is_allowed())` and `audit_wire(class, kernel, "batch site <hook>")` exactly mirroring the improved_noise kernel wiring (`src/improved_noise.rs:570-574`), then emits the runbook-expected marker line `batch: arm <Class>.<method> id=<n> T=<t> site=<hook>`.
4. **G5 plug-in:** T comes from the B.3 policy (default 16, g42→32, clamp 8–64; `docs/BATCH_WIRING_PLAN.md:183-215`) and lives at the **flush check**, not at arm time: the helper accumulates ops in a ThreadLocal buffer and at flush does `pending >= T ? run(...) : replay individual static calls`.
5. **Degradation ladder (B.2.2):** a negative `run()` return for kernel id *i* sets a per-site flag degrading that site to single-call for the remainder of the boot — no retry storms, no partial-batch ambiguity. Refused ids (`-10`) can never reach execution (guard runs before any op, `src/batch_api.rs:748-759`).

**Dormant by default:** `CRUSSTY_BATCH` unset parses to `Off`; arming then never fires, the retarget never applies, and the served class bytes are bit-identical to today's. The only new behavior visible with the gate off is the absence of the `batch: arm` line (verifiable in e2e).

## 5. Candidate call sites, ranked

1. **`ImprovedNoise.noise` bridge retarget — effort S (design reference implementation).** Retarget the invokestatic we ourselves emit (`src/improved_noise.rs:513-539`) to a same-descriptor batching helper. Zero class recon (bytes are cached in `PatchCache`), zero gameplay risk (hook is env-gated off by default), and the retargeted path is body-dominated (noise ≈ 11.5 µs; LOW win-tier per `docs/BATCH_ADOPTION_MATRIX.md:168`), so it is a demonstrator, honestly framed with no perf claim. This is the recommended first implementation.
2. **g9 `DensityAp2MinMaxFill` section-fill loop — effort M, dormant until recon.** Only wave-1 kernel with a shape already in the table (A′ ids 12/13) and a named adoption path (byte-hook on the section fill loop, `docs/BATCH_ADOPTION_MATRIX.md:110`); modeled S2 = 192 calls/tick ≈ 10.46 µs/tick (`:96,199`). Blockers: enclosing class/method unnamed anywhere (requires a `javap -p -c` pass around `NoiseChunk`, the TODO(B10) discipline `src/proto_blend_cache.rs:45-48`) and JFR amplification proof; at 119.8 ns direct the batch does not win today, so it would ship dormant.
3. **g42 `StaticCacheGet` — effort M-L, deferred.** Canonical stage-1 kernel (shape C, id 14 ready) but the runbook rules: no measured T exists on the closed probe body (`docs/BATCH_ROLLOUT_RUNBOOK.md:97-103`) and the production call site is unidentified (`docs/BATCH_ADOPTION_MATRIX.md:242` says only "hook the enclosing loop method"). Arm only after a real in-engine body or keep single-call.

## 6. Risks and mitigations

- **Class-file version:** helper/bridge embedded bytes must not exceed the target class major — existing guard pattern (`src/improved_noise.rs:303-336`, stale-v69 incident documented) applies to the new helper.
- **Retransform schema rules:** field/access-flag changes are rejected by JVMTI (`ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED`); Variant R touches only CP + a 2-byte operand — no schema change.
- **Hook-thread reentrancy:** retargeting happens on the activation worker (patch computed off-thread, served from cache on hook thread), preserving the never-ASM-on-callback-thread discipline.
- **CP 64K overflow:** saturating guard already in the pool (`src/classfile.rs:143-147,366-368`); retarget adds ≤1 entry per retarget spec per class.
- **Panic-across-JNI:** all new scanning code must be panic-free/bounds-checked; hook-callback paths keep the poison-recovery + `catch_unwind` discipline (`src/improved_noise.rs:109-114`, `cplug-sdk/src/lib.rs:136-146`, engine 4f9998b).
- **Verification cost:** the retargeted method re-verifies with unchanged frames; still, the e2e self-test pattern (functional round-trip through the real bridge, as improved_noise does) must extend to the batching helper before any `on`-mode boot.

## 7. Validation plan (when implemented)

1. Unit: scanner round-trips on existing fixtures (`tests/fixtures/SingleUserAreaMap.class`), retarget idempotency, CP-append safety, negative cases (truncated code, unknown CP tag).
2. Offline: serve patched bytes to `javap -v` and assert the retargeted call site + unchanged StackMapTable.
3. e2e: dormant boot → `batch: arm` line ABSENT (PASS-by-absence row); `CRUSSTY_BATCH=on` armed boot → `batch: arm ImprovedNoise.noise id=… T=… site=improved_noise` present, verify row captures it (new `ck_cap` row alongside `batch kernels resolved`), self-test PASS, graceful shutdown.
4. P500 re-run after any `src/` landing (gate rule; even though dormant-by-default makes impact nil, the run is the drift tripwire).

## 8. Doc-vs-code divergences found during recon (fix backlog, doc-only)

1. `bench/batch/java/BatchRolloutBench.java:12-13`, `bench/batch/run_batch_rollout.sh:8`, `bench/batch/results/BATCH_ROLLOUT_AB.md:9` claim "product code has NO CRUSSTY_BATCH env" — stale since G1 landed.
2. `run_batch_rollout.sh:64` + matrix §5.3.3 use `CRUSSTY_BATCH=1` as the "on" arm — superseded: `parse_rollout("1") → Off` (test-pinned, `src/batch_api.rs:1198`).
3. `src/batch_api.rs:1-9,105-107` header still carries the obsolete ~115 ns floor figure (canon 35–90 ns per TASK-33 errata).
4. `docs/BATCH_WIRING_PLAN.md:167` cites `improved_noise.rs:427-437` — pattern is now at `:570-574`.
5. Runbook `:22` cites `batch_api.rs:401` for `CRUSSTY_BATCH_NATIVE_LIB` — actual read at `:558`.
6. `docs/KERNEL_POLICY.md:101` says "ids 0-13" — table now has 15 ids (G3 added id 14).

## 9. Open questions

- Exact bytecode offset stability of the emitted `invokestatic` across ASM COMPUTE_FRAMES runs (CP append shifts operand indexes — the scanner resolves by name lookup, not fixed offset, precisely because of this).
- Whether op-3-in-SdkAsmHelper should shadow the Rust scanner long-term (one mechanism, two consumers) or stay a fallback — decide after the Rust scanner lands and is exercised on real bytes.
- g9/g42 enclosing method identification (`javap -p -c` live recon) — prerequisite for candidates 2/3, not for the demonstrator.
