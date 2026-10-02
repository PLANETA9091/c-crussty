# BOOT COLD-START CENSUS — S7-32 (TASK-88)

Owner directive frame: **cold start only** («рестор это плохо — надо тестировать и оптимизировать холодный старт»).
The restore/checkpoint lane is CLOSED by owner + measured NO-GO on this kernel (see §4).
Engine repo changes authorized; no generation/config changes; everything tested.

## 1. e2e default wiring — DONE, verified

`scripts/e2e_orchestrate.sh` default boot replaced:
- **direct purpur jar** (bundler bypass) — byte-verified against hs_err "Command Line" (the
  launcher only resolved java.home and spawned exactly this cmdline);
- **AppCDS v2** mapped when `$SERVER_DIR/crussty_boot.jsa` present (safe degradation both
  directions proven S7-31: missing archive → baseline-speed boot, zero crash);
- flags replicate the launcher child's production flags 1:1 (agentpath, Xms512M/Xmx2G,
  G1, dist.root) — **zero server.properties/paper/gameplay changes, JVM flags CLI-only**.

Verification (anchor restore before each boot, hs_err 4/0 across all runs):

| run | boot (Done) | note |
|-----|-------------|------|
| v3a | 13.929s | CDS marker active |
| v3b | 13.111s | |
| v3c | 13.014s | |
| **mean** | **13.351s** | vs launcher-path baseline 16.668s → **−19.9%**, matches S7-31 A/B (13.597s) |

## 2. Cold-start composition at 13.35s (JFR 291 in-window samples + wall-gap census)

Wall gaps (>0.4s) from latest.log, census boot Done 13.130s @15:59:23:

| window | wall | content (JFR attribution) |
|--------|------|---------------------------|
| pre-log (JVM→first line) | ~1.0-1.5s | JVM start + agent .so + early classloading — CDS crushed this from 2-4s (S7-30) |
| **"Initialized 0 plugins" → "Environment:"** | **+5.0s** | vanilla registry/datapack bootstrap on ServerMain (single-threaded object construction: ConcurrentHashMap build, hashCode, `<init>`s, ZipFile.getEntryPos) |
| "Environment:" → recipes/advancements lines | **+3.0s** | DataFixerUpper Schema joins (isJoinNonEmpty/joinUnoptimized/merge leaves) + 1461 recipes + 1574 advancements parse |
| Server thread tail (keypair/level-prep/netty) | ~1-2s |Moonrise worker init, spawn-area prep |
| scattered 1.0s lines | ~1s | ping sample, game type, permissions |

Threads in-window: ServerMain 129 (44%), Server thread 85 (29%), Worker-Main-1 25,
**Paper Plugin Remapper ×2 = 28 samples (~1.3s — remap-cache verification pass, output
cached at cache/mojang_1.21.10.jar since Sep 7, not re-created)**, Yggdrasil Key Fetcher 11.

Analyzer: `scripts/boot/analyze_boot_jfr.py` (boot-window filter by Done wall-clock,
thread share / top leaves / top 3-frame paths).

## 3. Where the remaining seconds CANNOT be removed by classloading (CDS exhausted)

The 5s + 3s windows are **object construction** (registries, DFU schema joins, recipe/advancement
parse) — not classloading. CDS v2 already removed its share. Remaining legal engine levers,
by cost/benefit (all dormant-gated, tested):

- **R1 registry-persistence design** (the 5s): serialize the built registry object island once
  and restore per boot. Requires static-field relink weaving — design-level project, next
  census-first step = JFR proof that ≥80% of the 5s is registry-island construction vs
  removable I/O.
- **R2 parallel-boot weaving** (the 5s): move independent per-registry loads onto Paper's
  worker pool via bytecode transform. Semantic risk: registry dependency order — needs
  dependency census first.
- **R3 DFU-schema cache** (the 3s): DFU join graph is deterministic per jar version; schema
  objects hold lambdas → plain serialization unsafe; needs a purpose-built relinker.
- **R4 micro-A/B ready**: `-Xverify:none` (verification skip for the woven-remainder classes),
  remap-cache verification bypass — each is a 1-boot A/B, expected ≤0.3-0.5s each.

## 4. CRIU/userns lane — measured NO-GO on this kernel + CLOSED by owner

Unprivileged userns is available (unshare -Urmpf OK, full caps inside, CapEff 0x1ffffffffffffff),
criu 4.1.1 extracted and runnable (apt download + dpkg -x, no root). BUT:

1. private procfs mount inside userns: **denied by old mount(2) syscall** (EPERM);
2. via **new mount API**: `fsopen("proc")` OK → `fsconfig(CMD_CREATE)` OK → **`fsmount()` EPERM**
   (kernel 5.10.134-013.8.3.kangaroo.al8.x86_64 hardens procfs instantiation in userns);
3. without private procfs criu cannot even initialize kerndat ("Can't open N/clear_refs on
   procfs"); without pidns criu demands init-ns CAP_SYS_ADMIN/CAP_CHECKPOINT_RESTORE (absent).

Plus the owner decision: **restore-based boot is unwanted** — cold start only. Lane closed
twice over; re-open requires a kernel where fsmount(proc) works inside userns.

## 5. Trajectory

| stage | boot (mean) | mechanism |
|-------|-------------|-----------|
| S7-30 baseline (launcher path) | 16.668s | — |
| S7-31 AppCDS v2 (rig) | 13.597s | classloading −18.4% |
| **S7-32 e2e default (banked)** | **13.351s** | direct jar + CDS v2 default |
| next | R1-R4 queue | construction cores (5s+3s) |

## ADDENDUM-3 (S7-33, TASK-91): 1ms-sampling deep census — in-boot noise-sampling discovered; engine-kernel arming queued

Deep census (1ms ExecutionSample, boot 14.435s incl. ~1.1s JFR overhead, 2793 in-window
samples, analyzer unchanged — boot-window filter by Done wall-clock):

- **In-boot noise-sampling is REAL and significant** (10ms S7-32 census undersampled it):
  `SimplexNoise.dot` 131 + `ImprovedNoise.gradDot/sampleAndLerp` 95 = **226 samples (~8.1%)**
  + Climate RTree `search` 64 + `buildParameterSpace` 35 ≈ **11.6% of window CPU** —
  spawn-area generation runs during boot ("Preparing level" phase), not only post-Done.
- DFU Schema joins ~207 samples (~7.4%), hashCode leaf 99 (3.5%, intern/hash storms),
  SHA2 44 (1.6%), zip getEntryPos 39.
- Threads: ServerMain 1469 / Worker-Main-1 546 / Server thread 517 — the 5s registry
  window remains single-thread-dominated.

**Lever connection (engine, zero new code):** CRUSSTY ships proven native noise kernels —
`improved_noise.rs` (dormant, env `CRUSSTY_NATIVE_IMPROVED_NOISE=1`) with measured G-AB
cpu −11.1% p=0.0079, wall −12.3%, parity 0/20000 — and the PerlinNoise whole-body bridge
(kernel-policy whitelisted "allowed (proven)", two-key arming). Deep census says these
same kernels cut the BOOT window too (gradDot path is 95 in-window samples). Gap: the
131-sample `SimplexNoise.dot` path has NO kernel — future engine-work candidate (census-first
per pre-registered discipline).

Armed-boot A/B (IMPROVED_NOISE+PERLIN_NOISE vs default 13.351s mean) was queued but NOT
measured this session: server lane handed to neighbor's TASK-90 hopper census (their claim
5a1d7b6 = 4×300s profiles). Coordination: armed A/B runs after their lane frees.

**Harness bug found + fixed-in-notes (do not repeat):** reap-by-fd-holder helpers must
NEVER run inside the flock'd section — the helper's kill-by-fd matches the script's OWN
fd 200 (and any neighbor process legitimately booting). Out-of-lock reap only, self
excluded; server-lane processes never killed by pattern heuristics.

## ADDENDUM-3 (S7-34, TASK-91 cont.): armed-boot A/B — measured NULL with mechanism; boot-window native kernels structurally deferred

### Deployed-artifact pair corrected (infra)
The -agentpath runtime is built from **CRUSSTY/runtime** (exports Agent_OnLoad/OnAttach/OnUnload,
1,847,800 bytes) while the plugin module is built from **c-crussty** (`modules/crussty/libcrussty.so`).
An S7-34 deploy mistakenly copied the plugin .so into the agent path → instant
"Could not find Agent_OnLoad" death on every boot (caught in the first A/B run, no silent state).
Correct pair deployed + verified: agent rc=0 (transform engine armed, 4 hook classes), plugin
module rc=0 (98 bridge classes, 283 natives, 0 unresolved). Backups: `libcrussty_runtime.so.backup-s7-34`,
`modules/crussty/libcrussty.so.bak-0635`.

### Armed A/B (ABBA n=3/arm, anchor restore each boot, hs_err 4/0)
- dormant a: 14.068 / 12.848 / 14.016 (mean **13.644s**)
- armed b: 13.863 / 13.727 / 13.319 (mean **13.636s**) → **NULL, full overlap**
- Mechanism (this is why, measured): the whole-body arming chain is **deliberately deferred
  past Done** — `activate()` waits for find_class (JVMTI scan cadence) → `wait_for_boot()`
  (boot marker) → define bridges into the kernel loader → retransform → serve. This deferral
  exists to avoid the S7-25 defineClass1 SIGSEGV race with the boot-time class-loading storm.
  With an immediate post-Done shutdown the chain never completes (b-runs: force-load attempts
  1-3, no patch). With a 45s post-Done settle the chain completes GREEN on the direct-jar
  topology: "server booted, defining bridge" → "hook armed, retransform rc=0" → "self-test
  passed" (improved_noise full chain + batch helper round-trip abi 262165).
- PerlinNoise additionally requires the class to actually load — idle post-Done never loads
  it (attempts 1-5 no sighting); it arms under real worldgen load (G-AB evidence −11.1% CPU
  p=0.0079 stands).

### Consequence for the cold-start program (S7-33 NEXT-2 closed)
The in-boot noise-family CPU (11.6% of the boot window, S7-33 1ms census) **cannot be
accelerated by the existing kernels**: arming is post-Done by design. Boot-window native
noise would require serve-at-load (patched bytes returned by the ClassFileLoadHook at the
FIRST load of ImprovedNoise/PerlinNoise), which re-opens the defineClass race for the bridge
helpers (S7-25 constraint) — recorded as a design line, not attempted silently. The armed
kernels' value remains post-Done worldgen (P500-relevant, proven).
