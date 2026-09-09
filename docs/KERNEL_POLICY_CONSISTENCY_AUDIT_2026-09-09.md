# TASK-122 — KERNEL_POLICY Consistency Audit (post-TASK-86-recovery drift check)

**Agent:** agent-7625532f · 2026-09-09 · claim 428a725 · zero boots, repo-static + cargo

## Purpose

TASK-86's whole-body promotion gates were re-done after the sandbox rootfs wipe (recovery
session), and the adoption waves kept growing the registries afterwards. This audit checks the
five layers of the promotion ledger for drift so the next promotion decision starts from a
verified-consistent state. No server, no engine, no gate logic touched.

## Layers audited

### 1. Registries in `src/kernel_policy.rs` — extractable, structurally sound ✓

Exact array bounds located (`DO_NOT_WIRE` 126–159, `PROVEN_WINS` 182–457, `PROMOTE_PAIRS`
657–698; `PROMOTE_PAIRS` is a separate `PromotablePair` type and does not inflate counts):

| registry | entries | state |
|---|---:|---|
| `DO_NOT_WIRE` (RegressedKernel) | **4** | LevelChunkHeightmap/newCombinedUpdateSummary 5.70×, MarkerCache/cachedSummary 4.54×, PalettedReencodeScratch/directPackedSummary 2.35×, ProtoChunkHeightmap/newCachedContainsSummary 1.78× — all with 2026-09-08 rerun source strings |
| `PROVEN_WINS` (ProvenKernel) | **36** | includes the two TASK-86 whole-body entries (`PerlinNoise/getValueWholeBody` line 216, `ImprovedNoise/noiseWholeBody` line 222) + adoption-wave live-verified entries |
| `PROMOTE_PAIRS` | **5** | env-gated WIN-direction binding slice (TASK-53/54) |

### 2. TASK-86 two-key whole-body gates — test-enforced ✓

`perlin_noise.rs:143` and `improved_noise.rs` arm through `decide(PERLIN_CLASS, POLICY_KERNEL)`
(= `net/minecraft/world/level/levelgen/synth/PerlinNoise` + `getValueWholeBody`, and the
ImprovedNoise equivalent) in addition to the env key. The exact drift class feared from the
reset (gate key renamed on one side, silently dormant despite env) is closed by construction:
`kernel_policy::tests::whole_body_bridge_wirings_are_policy_gated` asserts all four
(short/full class × kernel) forms are `Allow` in strict mode **and** that an unproven
whole-body swap (`Entity/updateFluidHeightAndDoFluidPushingWholeBody`) stays `KeepJava`.
`cargo test --locked`: **65 passed, 0 failed** — suite includes the registry↔jni_table sig
test (`fallback_symbols_exist_in_jni_table_with_matching_sigs`).

### 3. `docs/KERNEL_POLICY.md` (living prose) vs code — ONE stale item, FIXED ✓

- `WIN_MAX` 0.85 / `REG_MIN` 1.18 in the doc == aggregator constants in
  `bench/p500/aggregate_p500.py` and `kernel_policy.rs` ✓
- `DO_NOT_WIRE` table (4 rows: class/kernel/paired_old/ratio/source) == code entries
  **verbatim** ✓
- "registered 283-native surface" == `native/JNI_EXPORTS.manifest`: 298 lines = 3 header
  comments + 2 blank + 283 export lines, zero duplicates ⇒ **283 exact** ✓ (an earlier naive
  grep counting 284 "Java_" occurrences was picking up the word `Java_` inside a header comment)
- **STALE (fixed in this task):** line 42 said "4 + 25 entries" for the scanned registry
  slices. Actual: 4 + 36 (PROVEN_WINS grew 25 → 36 through the post-wave-2 adoption entries).
  Updated in place with a dated qualifier (count re-audited 2026-09-09 TASK-122).

### 4. `docs/KERNEL_POLICY_COVERAGE.md` vs registries ✓ (with a scope note)

This doc is the **DO_NOT_WIRE remap coverage** evidence (TASK-13, `verify_kernel_pref.sh`),
not a registered-module list: 4/4 remap candidates verified live, and its 4 class.kernel
pairs + registry ratios match the current code exactly. Its "10 `PROVEN_WINS` entries" line
is **commit-pinned historical evidence** (`commit tested: d87067e`, 2026-09-07) — drift from
10 → 36 since is expected for a pinned doc and is NOT edited; this note is the pointer.
Its drift guards are tests already counted in the 65.

### 5. Hygiene ✓

`cargo clippy --all-targets`: 22 warnings, all style-class (doc-list overindent ×9,
`is_multiple_of` ×3, `repeat().take()` ×2, plus singletons), **none in kernel_policy.rs /
perlin_noise.rs / improved_noise.rs** (the two nearest — unused import in classfile.rs's own
test module, unused var `d` in batch_desc.rs — predate TASK-86 and are outside all five
layers). No new correctness lint.

## Verdict

**CONSISTENT after one fix.** The promotion ledger's five layers agree: registries extractable
and complete, TASK-86 gates test-locked in both directions, living doc constants and the
283-native surface count exact, DO_NOT_WIRE remap coverage 4/4, suite green. The single stale
count (doc line 42, 25 → 36) was the only real drift and is fixed with a dated qualifier.
Standing protocol: re-run this audit after any registry-touching task or kernel jar bump.
