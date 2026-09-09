# TASK-150 — Chunk-encode SectionData PHASE-2A: in-server gate probe (DESIGN-ONLY)

agent-7625532f, 2026-09-09. Status: **rig banked, NOT executed** — the sandbox was
wiped (~14:29Z, second wipe in 24h) before any boot; `/home/z/server`, `/home/z/jdk21`,
`~/.git-credentials` are gone; execution waits for the next provisioning tick. This
document is the **pre-registration** for that execution — written before any data exists.

## 1. Question phase-2a answers

§97 (TASK-149 phase-1) refuted the STANDALONE route: every well-formed pure-array call
into `nativeEncodeSectionData`/`-Sized` returns **rc=-3** with zero bytes written.
Machine-code evidence: -3 is the jni-rs `get_array_length` wrapper failure path
(Result-discriminant ×5 `cmp $0xf`; four-lengths ORed-negative branch), which requires
an **internal global == 4** (init/attach state, 0x57388 → bss 0x59508); JNI_OnLoad is
ABSENT, so nothing on the standalone path initializes it — including a bare runtime
agent and a standalone runtime agent with production options.

Phase-2a asks the one remaining question: **does the LIVE production context lift the
-3 gate?** In-server, the module's `inject_surface()` dlopens
`libpaper_native_chunk_encode_jni.so` (optional dep, deployed by this rig), defines the
bridge class `net/.../game/PaperNativeChunkPacketEncode` on the **bootstrap loader**
(`env.define_class(name, NULL loader)`) and `RegisterNatives` the 3 exports
(ARCHITECTURE §2/§4). If the gate is context-dependent, the section exports answer on
content in-server; if they still return -3, the §97 pre-registered closure fires:
**the SectionData channel is dead-code-in-practice** and the frontier note in §97
resolves negatively.

## 2. Method — attach shadow (and INJECTS-ONLY accounting)

Assets (all in `bench/chunkencode/`):

| File | Role |
|---|---|
| `ChunkEncodeSectionShadow.java` | Agent-Class (`agentmain`) run inside the server JVM: resolves the bootstrap-loader bridge class, invokes the 3 registered natives with the phase-1 pure-array matrix, emits `SHADOW` rows to a TSV **and stderr** (→ boot.log) |
| `Task149Attach.java` | Source-mode runner (`java Task149Attach.java pid shadow.java out.tsv`): compiles the shadow **in-process via ToolProvider** (no javac binary needed), zips the agent jar (manifest `Agent-Class`), `VirtualMachine.attach(pid).loadAgent(...)` |
| `run_task149_phase2_inserver.sh` | Rig: preconditions → BENCH.lock + journal → native-lib deploy → seed restore → **pure boot** → surface-live gate → attach → verdict tree → stop → seed restore |

INJECTS-ONLY accounting (owner law «без флагов, только инжекты»):
- the **server boot is pure-inject**: stock JDK + ONLY `-agentpath:$RUNTIME=...`; no
  other flag, no `-javaagent`, no `-cp`, no `-D` — identical to the canonical C3 boot;
- the attach+loadAgent is **bench-lane instrumentation of a running measurement
  subject**, same lane as `jcmd` observers (TASK-146) except code-bearing; it happens
  AFTER `Done(` and the surface-live marker; it adds no product code path and is
  removed with the process;
- **zero product changes**: no Rust rebuild, no kernel change, no module change.

Boot-line gates: `Done (` (150 s), `native surface live` (90 s) — without the surface
marker the rig aborts `RIG-INVALID/surface-absent` (rows would be meaningless).

Compile path proven 2026-09-09 on system OpenJDK **21.0.12.1** (same upstream version
as the wiped Temurin jdk21): source-mode launch + ToolProvider compile of the shadow +
`--add-modules jdk.attach` all work; attach itself exercised only to the
`VirtualMachine.attach` call (no live target available post-wipe). The rig prefers
`/home/z/jdk21/bin/java` when provisioning restores it and records `java=<tag>` in the
RAW (`BOOT` row) — phase-2a is an identification probe, not a timed comparison, so JDK
identity is recorded but not a controlled variable.

## 3. Case battery (pure arrays; phase-1 shapes verbatim where they exist)

`light_sanity` (Probe2 light_first shape — **channel discriminator**: light needs no
length queries, so a numeric light row in-server proves the shadow-call channel and the
wrapper are alive; ULINK/THROW here = rig invalid, NOT channel evidence);
`null_all`, `null_counts` (phase-1 null-layer discriminators, expected -1/distinct);
`allair_n{0,1,24,26}`; `single_sizes1_n24`; `linear4_n24`; `allair_biome0len_n24`;
`len15_all`; `hdr0f_all`; `global_n24` (NEW: palSizes=0 WITH data, bits=8 — probes the
global-palette shape the phase-1 hypothesis could not express standalone);
`sized_allair_n{1,24,26}`; `sized_linear4_n24`.

Row grammar: `SHADOW\t<case>\trc=<int|ULINK|THROW:cls>\tshape=<...>\tdst0=<hex>\tdstHex=<16B>\tms=<n>`;
plus `SHADOW_ENV` (resolved loader) and `SHADOW_SINK` (ok/err counters). For `rc>=0`
rows the first non-zero bytes of dst are hex-dumped (`dstHex`) — immediate wire sniffing
for phase-2b even before reference parity runs.

## 4. Verdict tree (pre-registered — binding before data)

Applied by the rig from the raw rows; rows are banked verbatim, attribution separate
(no-reclassification law).

1. **RIG-INVALID** — no `SHADOW_ENV`, light row not numeric (`ULINK`/`THROW`), attach
   failed, boot/surface gate failed. No claim about the channel is possible.
2. **CHANNEL-CLOSED** (§97 verbatim, STRICT reading) — ≥1 non-null section row
   `rc=-3`. The channel closes as dead-code-in-practice; production unchanged either
   way; INJECTS-ONLY intact. Banked rows stay for the owner.
3. **GATE-OPEN** — ≥1 non-null section row `rc>=0`. The gate is context-dependent.
   Pre-registered next steps: phase-2b = full parity sweep against the byte-exact
   vanilla reference generator (§97 asset 1, kernel classes via the server's own
   context), then P500 old-vs-native timing (WIN ≥1.10x full separation, honest NULL
   otherwise, no gate tuning).
4. **ARG-LAYER** — all non-null rows in `{-1,-2}`, no -3, no ≥0: gate lifted, wire
   hypothesis v1 arg-rejected. Phase-2b needs a revised ABI hypothesis (owner-facing
   decision, no autonomous hypothesis-shopping).

**Mixed-case note (registered now, before data):** if -3 and non--3 rows coexist among
non-null cases, rule 2 (STRICT, §97-verbatim) closes the channel; the coexisting
non--3 rows are banked as observation rows for the owner. Any relaxation of this would
be a gate revision and requires the owner, not the agent (ANTI-GATE-SHOPPING applies
to the pre-registration itself).

## 5. Not done here (honest scope)

- Execution: impossible post-wipe (server/jdk21/runtime-so/module all gone;
  `cargo` absent — but phase-2a deliberately requires NO cargo and NO kernel jar).
- TASK-149 done-row backfill in CLAIMS (dev-logs repo is private; token gone with the
  wipe; claim e51258c was re-verified count=1 pre-wipe; done-row content is preserved
  in `/home/z/my-project/worklog.md` for the next incarnation).
- Phase-2b parity sweep + P500 timing: only on GATE-OPEN.
- Drift-guard `8ba2473c…` re-verified on re-clone: ✓ (sha256 matches
  `8ba2473ce1f4453c3961df3f83f31e9dde1f209d2e425a54889e1078bb50a185`).
