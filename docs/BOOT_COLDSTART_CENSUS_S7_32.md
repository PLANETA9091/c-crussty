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
