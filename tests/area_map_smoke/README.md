# area_map_smoke — headless unit-smoke for the PATCHED SingleUserAreaMap.update()

Task 4-a. Verifies, headless (no server, no CRUSSTY engine, no player
required), the logic that the live-server boot could NOT reach:
`SingleUserAreaMap` never loads without a player, so the area-map hook stayed
dormant there. This test drives the **actual patched bytes** the shipped Rust
patcher produces.

## What is under test

| component                              | source of bytes in this test                                        |
|----------------------------------------|---------------------------------------------------------------------|
| patched `SingleUserAreaMap.update()`   | `tests/fixtures/SingleUserAreaMap.class` (real Paper kernel bytes, major 65) patched by `patch_tool.py` — a byte-exact Python mirror of `src/classfile.rs::patch_update` (parity re-checked against the cargo gold whenever `/tmp/ccrussty_patched_SingleUserAreaMap.class` exists) |
| `SingleUserAreaMapOps{,$Scratch,$1}`   | the REAL shipped classes (`area-map/build/...`), rebuilt by `scripts/build_area_map.sh` (which also re-validates the Scratch newarray guard) |
| `PaperNativeAreaMap.nativeUpdateOpsBatch` | variant **A (stub)**: counting Java fake, same FQN/signature, increments a counter instead of entering JNI — the required core, fully headless. variant **B (real-.so)**: the real `native/libpaper_native_jni.so`, bound by exact JNI symbol name (`Java_ca_spottedleaf_moonrise_common_misc_PaperNativeAreaMap_nativeUpdateOpsBatch`, see `native/JNI_EXPORTS.manifest`) |
| environment                            | `Boot.java` defines the patched map class + everything else in one child loader (parent = platform loader) |

## Checks

- **CHK-0** provenance: the map class is defined from the patched bytes in the
  smoke loader; `NOT_SET == i32::MIN`.
- **CHK-1** *Scratch init*: first patched `update()` on 8 fresh threads does
  NOT NPE (`Scratch.ops/keys` field initializers, the A3-audit P0 regression),
  grow-only doubling past `INITIAL_CAP=578` (cap 650 → 1156), same-state after
  grow.
- **CHK-2** *same-state fast path*: 1001 identical-square updates → native
  invocation count delta **0**, no callbacks, state stable (variant A counts
  invocations; variant B checks the resulting state); a changed control call
  fires the native exactly once with the correct difference.
- **CHK-3** *changed state*: NOT_SET guard (returns false, no native, fields
  untouched), IllegalArgumentException guard (negative distance, fields
  untouched), 7 hand-verified spot cases + 512 xorshift64-seeded rects (same
  seed and draw order as `bridge_selftest`) — native invoked exactly once per
  changed update and applied callbacks **== naive_set_difference**
  (adds = new∖old op 0, removes = old∖new op≠0, key = z<<32 | x&0xFFFFFFFF).

## Usage

```bash
tests/area_map_smoke/run.sh          # one pass: variant A + variant B, fresh JVMs
tests/area_map_smoke/run.sh 2        # two full passes
```

Requirements: JDK 21 at `/home/z/jdk21` (or `JAVA_HOME`), `python3`, `bash`.
Runtime: ~4 s total on the 2-vCPU dev box. Exit codes: 0 all green; 1 variant A
(mandatory) failed; 2 patch/byte-parity failure; 3 variant A pass, variant B
failed.

Optional byte-parity gold (recommended before claiming patch drift):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo test classfile::tests::patch_roundtrip   # dumps /tmp/ccrussty_patched_SingleUserAreaMap.class
tests/area_map_smoke/run.sh                    # now byte-compares vs the Rust patcher
```

As of 2026-09-08 the Python mirror is **byte-identical** to the Rust
`patch_update` output: 3320 bytes,
sha256 `873c17cf5f9a85ef9f104bf11c818d6aa08560f6012fa98a38e03274daa3d45c`
(the patch shrinks the class: the per-cell enumeration body is ~1.8 KB, the
patched body is 82 bytes + append-only CP entries).

## Layout

```
run.sh                 entry point (build + 2 variants + verdict)
patch_tool.py          Python mirror of src/classfile.rs::patch_update
java/boot/Boot.java    app-classpath bootstrap; defines the patched bytes
java/harness/areamapsmoke/
  SmokeMain.java       CHK-0..CHK-3
  RecordingMap.java    concrete map recording add/remove callbacks
  NaiveDiff.java       Java mirror of area_map.rs::naive_set_difference
java/fake_native/...   variant A: counting PaperNativeAreaMap + SmokeProbe
java/real_so/...       variant B: native-declaring PaperNativeAreaMap + SmokeProbe (System.load)
build/                 scratch output (safe to delete)
```

Notes:
- Nothing here ships in the plugin; the fake native is a test-only class with
  the same name as the bridge — it never coexists with the real bridge in one
  process (variant B uses the real one).
- The real-.so variant doubles as evidence that the closed native enumerates
  the set difference correctly **standalone** (no CRUSSTY engine init), which
  the P500 harness also relies on.
- If CHK-1 ever fails with an NPE, `SingleUserAreaMapOps$Scratch` shipped
  without its field initializers again — fail the release (see
  scripts/build_area_map.sh guard and worklog session 004).

## Relationship to bench/areamap (TASK-11 duplicate delivery)

A parallel session delivered an independent stub-smoke for the same claim as
`bench/areamap/` (532597b): it drives `SingleUserAreaMapOps.run()` directly
through a same-FQCN classpath shadow and checks a wide native-parity grid
(S1-S5, incl. 4-thread scratch isolation). The two suites are complementary:

- `bench/areamap/` — native apply-loop parity + MIN_VALUE guard + threading.
- `tests/area_map_smoke/` (this dir) — the **actual patched bytes**: byte-exact
  Python mirror of the Rust patcher, full JVM verification of the patched
  classfile, and the patched `update()` body (fast path + Scratch init) with a
  counting stub / the real .so.

Keep both; run this one after any change to `classfile.rs::patch_update` or
`scripts/build_area_map.sh`.
