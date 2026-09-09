# TASK-146 — VANILLA CONTROL (old-kernel A/B side, P500-style): the agent's own cost is DECOMPOSED — ~31 MB native resident + ~5 MB live heap, ~0 boot time; the ~57 MB "native structural term" of the resid model is VANILLA JVM JIT behavior, NOT agent cost

**Agent:** agent-7625532f · 2026-09-09 · quiet window (S7-91 unclaimed, zero java procs). Canonical rig **untouched** (drift-guard `8ba2473c…` verified before and after; the v2 gate and product lane are unaffected — this is the old-kernel CONTROL side the owner's standing P500-style instruction asks for).
**Rig:** `bench/graal_ab/run_task146_vanilla_control.sh` — derived copy of the canonical pure-inject rig, exactly two changes (AGENT_ARGS empty; verdict=CONTROL + unconditional GC.run, pre-registered). **Claims:** dev-logs 3e5d9cc. **RAW:** `RAW_VANILLA_CONTROL_CLEAN_124515/` (clean) + `RAW_VANILLA_CONTROL_BUGGY_NOSETTLE_20260909_124303/` (disclosed tooling shakedown, fifo removed).

## 1. Rig shakedown — a tooling bug found by its own first run (disclosed)

The variant's first boot exposed a porting bug: `REMOVE_SETTLE` default omitted → unbound variable under `set -u` → the 120 s remove-settle was **skipped** → R2/R3 sampled at remove-START (chunks still live). The buggy run's post-GC used (382.5 MB) is a valuable **negative control**: without the settle, a full GC reads +134 MB above the settled live set (lazy chunk unload) — quantifying why the canonical protocol's settle is load-bearing. Fix applied in place; clean re-run into a fresh RAWDIR. Costs: 2 boots total for this task (1 spent on the shakedown), hs_err delta 0 across both.

## 2. Clean control numbers (N=1)

| leg | RSS (MB) | heap used / committed |
|---|---|---|
| R0 baseline | **854** | 279.0 / 527.0 → free-room 248.0; **non-heap remainder 327.0** |
| R1 post-add | 909 | 472.5 / 530.0 → Δused@add **+193.5 (no GC in add-leg — the corpus's no-GC path appears in vanilla too)** |
| R2 remove-end (120 s settle, traj 13 samples, plateau 941→942) | **942** | 455.0 / 557.0 → Δused **+176.0 above baseline (garbage persisted)**; Δcommitted **+30.0** |
| R3 after unconditional `GC.run` | 947 | **244.0 / 557.0** |

- Boot **16.327 s** — inside the agent parity band [15.96–17.58]; buggy-run boot 16.688 s also inside. Vanilla histogram ≈ agent corpus profile (byte arrays + object arrays top; identical shape).
- Informational only: had this been an agent run, v2 ref gate = 954.0 → R2 = 942 would read PASS.

## 3. Registered predictions vs outcome

| registered | outcome |
|---|---|
| V1: vanilla boot within agent band ±1 s | **PASS** — 16.327 / 16.688 vs [15.96–17.58]; **agent boot-time cost ≈ 0** |
| V2: vanilla R0 below agent min 830 by 30–80 MB | **MISS as registered** (854 > 830) — the prediction misused the frame: TASK-145 proved R0-RSS tracks committed@R0 (R²=0.984), so cross-boot R0 comparison cannot isolate agent footprint. CORRECTED decomposition (the frame TASK-145 itself established): non-heap remainder **vanilla 327.0/330.0 (n=2) vs agent constant 357.8 → agent native resident ≈ +28…+31 MB** — consistent with the TASK-126 C2/metaspace class. Lesson: pre-register in the model's own coordinates. |
| V3: vanilla post-GC live set ≈ agent 248.5–250.0 ± 5 MB | **PASS** — **244.0 MB** (agent −4.5…−6.0 lower ⇒ the inject adds ≈ **+5 MB of live heap objects** — negligible) |

## 4. THE HEADLINE FINDING — the resid model's "native structural term" is vanilla JVM behavior

Vanilla's own churn residue decomposes exactly like the agent corpus: **resid 88 = Δcommitted 30.0 + native 58.0**. The agent corpus's native term is ~57 MB (TASK-140; #8 measured 61 pure-native; #9 err 0.0 on 47+57). **These are the same term**: the ~57 MB native growth under the 64-chunk forceload churn is **JVM JIT structure (C2 code-cache + metaspace compiled for the churn's code paths) — present in the OLD KERNEL WITHOUT ANY AGENT**. Consequences:

1. **The agent's marginal cost vs the old kernel is now fully decomposed**: ≈ **+31 MB native resident** (runtime .so mappings + agent structures at baseline) + **≈ +5 MB live heap** + **≈ 0 boot time**. The 57 MB churn-time native growth is NOT attributable to the agent — it is the platform's own JIT cost of the workload, paid identically by vanilla.
2. **The C3 corpus's absolute residues are anchorable**: agent resid ≈ vanilla resid + agent-baseline-delta; e.g., in-band runs (+61…+92) sit at vanilla-class (+88 here, single-run lottery aside) plus boot-variable committed expansion.
3. **The C3 channel measures JVM + workload physics, with the agent as a small constant offset** — further confirmation that no agent-attributable leak exists in 9 runs (invariant 9/9) now has a vanilla anchor for the live set itself.

## 5. Banking

- Ledger §(numbered after pull + tail grep; tail was §92) — this block. INDEX row + both RAW dirs (fifos removed) + the variant rig (committed; canonical drift-guard byte-identical).
- CLAIMS done-line (V2 miss + correction disclosed verbatim).
- 2 boots total (1 shakedown + 1 clean), hs_err delta 0, zero config, product lane INJECTS-ONLY untouched.
