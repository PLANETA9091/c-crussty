# TASK-149 phase-1 — nativeEncodeSectionData/Sized identification: standalone context REFUTED (rc=-3 gate sits before wire-format logic)

* Agent: agent-7625532f, 2026-09-09, tick 21:40+08 (Job 366516). Claim: TASK-149 e51258c.
* Scope: the last two unmeasured chunk-encode exports (`nativeEncodeSectionData`,
  `nativeEncodeSectionDataSized` — BENCHFIRST §5 documented follow-up). Goal per the
  owner's standing «оптимизируй» directive: complete ABI identification → parity →
  P500-style old-vs-kernel timing (the only blind surface where the kernel has real
  win potential: per-voxel palette+bit-pack work, unlike light's pure 4.0x-losing memcpy).
* Rig: `ChunkEncodeSectionIdent.java` + `ChunkEncodeSectionProbe.java`/`Probe2.java` +
  `run_chunk_encode_section_ident.sh` / `run_chunk_encode_section_probe.sh` (headless,
  BENCH-MUTEX, kernel jar DIRECT classpath — see infra note). RAW: this dir
  `CHUNKENCODE_SECTION_IDENT_RAW.tsv` (35 rows, N=20..26 battery),
  `CHUNKENCODE_SECTION_PROBE_RAW.tsv` (differential probes ×2 contexts).

## 1. What is now PROVEN (assets banked)

**Reference generator = byte-exact vanilla wire semantics, headless.** Real
`PalettedContainer.write(FriendlyByteBuf)` + `LevelChunkSection.write` order
(javap-pinned: short nonEmpty → writeByte(bits) → palette.write → writeFixedSizeLongArray)
over REAL containers from `Strategy.createForBlockStates(Block.BLOCK_STATE_REGISTRY)` /
`Strategy.createForBiomes(...)` with the production scenario matrix (single air / single
stone / 1-bit / 4-bit linear / 6-bit hashmap / global-15-bit blocks; 1/3/8/12 biomes).
HEXDIFF rows verify vanilla format byte-level: BE short `10 00`=4096, bits `04`,
palette VarInt(2)+ids `02 00 01`, packed longs `11..` repeated with NO length prefix
(fixed-size) — exactly the production wire shape. refLen for N=20..26: 59,138–71,x00 B.

**Kernel jar direct = no renamed jar needed.** The TASK-78-era rig needed
`/tmp/expl/vanilla_mojang.jar` (ART-renamed; recipe lost in the sandbox wipe). Found:
`/home/z/server/versions/1.21.10/purpur-1.21.10.jar` (29.4MB inner jar) IS mojang-mapped
and compiles+serves the full vanilla surface (ClientboundLevelChunkPacketData,
PalettedContainer, registries). The vanilla_mojang.jar dependency is RETIRED for this rig.
(Note: `versions/purpur-1.21.10.jar` — the 57MB paperclip bootstrap — does NOT expose
classes; the javap-visible one is `versions/1.21.10/purpur-1.21.10.jar`.)

**rc semantics pinned (black-box).** `nativeEncodeSectionData`:
`-1` = null-argument layer (any null array → -1, even with others populated);
`-3` = second-layer gate — reached by EVERY well-formed input tried (16 variants:
N∈{0,1,20..26}; all-air/single/linear4/global; sizes+offsets layout; len-15 arrays;
0xF-header blobs; Sized variant with explicit capacities N∈{1,24,26}).
**light sanity in the same process: rc=38 (WORKS)** → .so load, stub binding, JNI
mechanics all fine; the -3 gate is section-kernel-specific.

## 2. Mechanism of -3 (machine-code evidence, closed lib)

Disassembly of `Java_..._nativeEncodeSectionData` (0x15450–0x157d0) +
`jni::wrapper::jnienv::JNIEnv::get_array_length` (0x188a0, symbol from .so strings):

* `-3` (0xfffffffd) is emitted at 15798 on TWO conditions: (a) the sign bit of FOUR
  `get_array_length` results OR-ed together (`js 15798`) — i.e. any length query
  reporting failure; (b) null pointer checks. Five `cmp $0xf,%al` checks on the wrapper's
  Result-discriminant scratch gate each length query (`jne` to per-array cleanup labels).
* The wrapper's `get_array_length` dereferences an internal global (0x57388 → bss
  0x59508) and requires `cmp $0x4` — an init/attach state machine value.
* **JNI_OnLoad is ABSENT** from the .so exports (only the 3 Java_ symbols) — nothing
  initializes the global on `System.load` in a standalone JVM.
* Internal symbols (strings): `paper_native_chunk_encode_jni::encode_section_data_jni`,
  **`paper_native_chunk_encode_core::validate_section_input`** — the validator exists and
  runs BEFORE any wire-format logic.

**Runtime-agent experiments:** `-agentpath` bare (0 modules) and with the production
options (`modules=/home/z/server/modules;versions=...;kernel=purpur-1.21.10.jar` —
transform engine armed, 4 hook classes installed) → IDENTICAL probe results (still -3).
The state-4 global is NOT set by either standalone context.

## 3. Why the light kernel measured standalone but section does not

`nativeEncodeLightData` needs no array-length queries (fixed 26-section masks/2048B
arrays; shapes implied by the inputs) → it never touches the state-gated wrapper path.
`nativeEncodeSectionData` must query lengths of ≥4 variable-shape arrays → first call
into the gated wrapper → -3 before `validate_section_input`/encode ever sees content.
CONSEQUENCE: **standalone black-box identification is IMPOSSIBLE for this export**;
the earlier hypothesis ("blob semantics need n=1 identification rows") is refined: the
rows must be gathered where the runtime context genuinely exists — inside the production
server process (production Server thread + runtime transform machinery armed).

## 4. Pre-registered phase-2 (next tick, lane-permitting)

Carry the banked identification matrix IN-SERVER (TASK-108 in-server self-test pattern:
agent-active boot + in-server gather of probe rows via the selftest/shadow channel):
1. same 16-variant matrix + reference bytes shipped into the server JVM;
2. on first MATCH: full parity sweep (decode round-trip per BENCHFIRST discipline),
   then P500 timing old-vs-native per the pre-registered gates (WIN ≥1.10x with
   full separation, honest NULL otherwise — P500 law, no gate tuning);
3. any remaining -3 rows in-server → the export is dead-code-in-practice; honest
   closure candidate for the whole SectionData channel (no wiring ever occurred;
   production unchanged either way — INJECTS-ONLY discipline intact).

Promotion-relevance note (honest): the upside case survives — vanilla section encode
does real per-voxel palette+pack work where a native kernel could win; the physics case
(copy-dominated, as light lost 4.0x) is equally alive. Phase-1 refutes the MEASUREMENT
ROUTE, not the candidate.

## 5. Co-banked corrections this tick (see ledger §97)

* **TASK-108 stale-tail correction**: TASK-148's candidate-table row ("v3 array
  interpreter ... no warm A/B yet") and the §95/§96 open-tail ("TASK-108 v3 phase-2
  warm A/B = next gated optimization") are STALE — the warm+cold A/B ran at
  b10d7a2/§36 (01:12Z, ~12h BEFORE TASK-148): warm +12.2% / cold +14.8% vs gates
  ≤−3%/≤0% = honest NULL; noise-kernel channel CLOSED-NULL. Re-open criteria unchanged
  (§36: genuinely batchable root). TASK-148's perlin promotion itself is unaffected
  (its evidence = TASK-74 G-AB, independent of v3).
* **variant-C promotion-frontier closure note**: `SingleUserAreaMap` frames are ABSENT
  from the mobdense W4 census (0/1064 exec samples at 400 stable living entities —
  rg-verified against MOBDENSE_CENSUS_EXEC_SAMPLES_W4.txt). Available workloads:
  forceload-only = no area-map move traffic; mob-dense-400 = below measurement
  resolution → no available workload can demonstrate the ≥3% P500 bar for the
  area-map budget variant (TASK-64/C). Variant C stays opt-in by the repo's own
  promotion law (live G-AB required); no G-AB is achievable on this box's workloads.
  Recorded as decision-grade for the owner; the oracle-green per-call evidence
  (3.1–94.1x) stands unchanged.
