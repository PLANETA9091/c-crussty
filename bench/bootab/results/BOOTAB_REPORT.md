# TASK-32 boot-latency A/B — harness + baseline (2026-09-07/08)

Agent: agent-7625532f (TASK-32-w4). Box: 2 vCPU / 4 GB RAM, LIVE Minecraft server
(PIDs 26544/26562, `/home/z/server`) resident throughout — all numbers are
directional, shared-CPU noise caveats in §6.

## 1. What was measured

Boot latency of a THROWAWAY Purpur 1.21.10 + CRUSSTY instance, A/B on the
**module** `.so` (`libcrussty.so`) while everything else is held constant:

| arm | module `.so` built from | md5 | size |
|---|---|---|---|
| A-base | c-crussty `4fb9d12` = parent of `106bb73` = pre-wave base (before TASK-22/23/26/27 perf commits) | `ea2eb1cc9fd084e00148487686c3fe75` | 948,368 B |
| B-master | c-crussty `217e3e8` (origin/master at build time; the only later commit `ed27eb0` touches bench aggregation only — zero `src/` delta) | `5e396db2018908a599d86d1cfce14e18` | 975,416 B |

Shared by both arms (identical bytes): deployed runtime agent
`libcrussty_runtime.so` md5 `b57b309280aff628b743cb27c11099da` (copy of
`/home/z/server/libcrussty_runtime.so`, engine v2.0.0), module.json, `native/*.so`
JNI kernels (copies of the deployed ones), `/home/z/jdk21` (Temurin 21), purpur
jar + mojang cache jar + libraries (per-run copies).

Base..master module-source delta (what B carries beyond A):
`cplug-sdk/src/{classes.rs +173, hooks.rs +262, main_thread.rs +143, log.rs +40, lib.rs +4}`,
`src/{area_map.rs +20, improved_noise.rs +73, batch_api.rs +182, kernel_policy.rs +66}` —
i.e. TASK-22 (find_class early-exit + sighting feed), TASK-23 (COW lock-free hook
readers), TASK-26 (serve-branch `Arc<[u8]>`), TASK-27 (method-ID cache + log.rs
lock release), TASK-24 (control-plane scratch), TASK-28 batch wiring
(`CRUSSTY_BATCH` off by default → inert here), plus test/docs commits.

**A/B B-side WAS possible now** — the boot-path perf wave lives in c-crussty
(module), which builds in-sandbox (`cargo build --release`, 5.3 s / 5.6 s). The
*engine* agent `.so` (`/home/z/CRUSSTY`, READ-ONLY) can NOT be rebuilt here —
but it did not need to be: both arms ran the same deployed runtime agent, so the
differential isolates the module-side wave. If TASK-32 blend-cache patcher
(TASK-22..27 sibling, in-progress) changes the *engine* runtime, re-run the same
harness with the new runtime copied into `/tmp/ab-boot/rt/`.

## 2. Ready-marker definition

Primary (parametrized as arg 1, used here):

```
[crussty-plugin] native surface live: 98 bridge classes, 283 natives registered (0 symbols unresolved)
```

ERE used: `native surface live`. This is the CRUSSTY injection-complete point —
the window most sensitive to the class-load-hook path (find_class, hook
dispatch, method-ID cache). NOTE: `[crussty-*]` markers go to **stderr**
(eprintln), NOT `logs/latest.log` — the harness captures stdout+stderr into one
`run.log`. Secondary stop/measure marker: `Done \([0-9.]+s\)` (Paper's own boot
completion, also self-reported as `paper_done_s`). The run is stopped (SIGTERM
to the exact PID) only after BOTH markers, or at the cap, whichever first.

## 3. Methodology

`bench/bootab/run_bootab.sh <ready-marker-ERE> <max-wait-s> <runs>` (env:
`BOOTAB_MODULES`, `BOOTAB_TAG`, `BOOTAB_PORT_BASE`, `BOOTAB_TSV`,
`BOOTAB_SECONDARY`, `BOOTAB_XMX`, ...). Per run:

1. `mktemp -d /tmp/ab-boot/run-<tag>-<i>-XXXXXX`; assemble a throwaway server
   root: **copies** of `versions/purpur-1.21.10.jar`, `cache/mojang_1.21.10.jar`,
   `libraries/` (never write through to `/home/z/server`); symlink `modules` →
   the arm's assembled modules dir; generated `eula.txt`, `server.properties`
   (unique port, `online-mode=false`, `level-type=flat`,
   `generate-structures=false`, `view-distance=4`, `simulation-distance=4`).
2. Boot in background of the SAME foreground tool call (sandbox kills
   setsid/nohup between tool calls — each batch is one self-contained call):
   `java -agentpath:/tmp/ab-boot/rt/libcrussty_runtime.so=modules=<rundir>/modules;versions=<rundir>/versions;kernel=purpur-1.21.10.jar -Xms512M -Xmx1G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=<rundir> -jar <rundir>/versions/purpur-1.21.10.jar --nogui nogui`
   with `CRUSSTY_NATIVE_IMPROVED_NOISE=1` (replicates the live server env).
   Heap is 1G, not the live 2G — the box has 4 GB with the live server resident.
3. Poll `run.log` every 0.1 s; record t(primary), t(secondary); at primary also
   snapshot `/proc/<pid>/status` VmRSS; before kill snapshot VmHWM.
4. Kill by exact PID: TERM → 20 s grace → KILL; verify death (`kill -0` fails);
   `wait` for rc; scan for strays named after the unique run dir.
5. Save `run.log` to `/tmp/ab-boot/logs/<tag>-<i>.log` (regenerable, not
   committed), append TSV row, `rm -rf` the run dir.
6. `flock /home/z/BENCH.lock` held for each batch (sibling-agent courtesy).

## 4. Baseline numbers (2026-09-07 19:07–19:20Z)

Raw: `results/bootab_baseline.tsv` (n=3 per arm). Times are seconds from
`exec java` to marker detection.

**primary — `native surface live` (crussty injection-complete):**

| arm | r1 | r2 | r3 | median | spread |
|---|---|---|---|---|---|
| A-base | 3.11 | 3.03 | 3.10 | **3.10** | −0.07 / +0.01 |
| B-master | 3.07 | 3.02 | 3.02 | **3.02** | −0.00 / +0.05 |

**secondary — `Done (` (full boot incl. flat-world gen):**

| arm | r1 | r2 | r3 | median | spread |
|---|---|---|---|---|---|
| A-base | 31.29 | 30.79 | 31.03 | **31.03** | −0.24 / +0.26 |
| B-master | 31.21 | 31.85 | 30.03 | **31.21** | −1.18 / +0.64 |

Paper self-reported `Done (Xs)`: A 27.304 / 27.412 / 27.505 (median 27.412);
B 26.579 / 27.439 / 28.082 (median 27.439). RSS at primary: A 1.164–1.176 GB,
B 1.118–1.165 GB (VmHWM within ~60 kB of VmRSS snapshot).

### A/B verdict (honest)

Primary marker: B median 3.02 s vs A 3.10 s → **−0.08 s (−2.6 %), n=3,
ranges overlap — DIRECTIONAL ONLY, not a proven win.** Secondary/paper-done:
medians statistically indistinguishable (world-gen noise dominates).
Plausibly real but small: the pre-marker window is only ~3 s of early
class-loading, while much of the TASK-22 benefit (poller scans-avoided)
accrues over the 180 s post-marker sighting window that this harness stops
out of. No regression signal either.

**Baseline (A-arm) boot-latency methodology numbers: median 3.10 s ± 0.04 s
to `native surface live`, 31.03 s ± 0.25 s to `Done (`, RSS ≈ 1.17 GB.**

## 5. Boot anomalies observed

- `anom=4` in EVERY run, both arms: 4× `ERROR: No key layers in MapLike[{}]`
  during world-gen — deterministic artifact of the minimal flat
  `level-type` without `generator-settings` layers; identical across arms,
  zero differential meaning. (Baseline anomaly signature = exactly 4.)
- Zero unresolved symbols in all runs: `283 natives registered
  (0 symbols unresolved)` — the master module (B) loads cleanly under the
  deployed v2.0.0 runtime agent (cplug-abi compatible).
- Console-prompt `> > >` bleed in redirected stdout (jline + no TTY) —
  cosmetic, both arms.

## 6. Noise caveats

- LIVE server (PIDs 26544/26562) shares the 2 vCPUs — ps showed ~0 % instant
  CPU but it periodically ticks; sibling agents run builds/benches between
  batches. BENCH.lock was acquired per batch, but nothing serializes the live
  server. Treat sub-0.1 s deltas as noise; n=3 medians are methodology
  validation, not a claims-grade A/B. For a claims-grade verdict: n≥7
  interleaved A/B/A/B, or rerun when the live server is down.
- Quantization: 0.1 s poll + grep detection lag ≤ ~1 poll interval, symmetric
  across arms.
- First-run effects: none visible (smoke run preceded arm A; page cache warm
  for jars; per-run jar copies equalize).

## 7. Reproduce / run the B-side later

```bash
# 0) worktree + build the two arms (module .so)
cd /home/z/c-crussty && git fetch origin master
git worktree add --detach /tmp/wT32base <pre-wave-sha> && git worktree add --detach /tmp/wT32boot origin/master
(cd /tmp/wT32base && cargo build --release) && (cd /tmp/wT32boot && cargo build --release)

# 1) assemble (COPIES only; never write /home/z/server)
mkdir -p /tmp/ab-boot/rt /tmp/ab-boot/modules-A/crussty /tmp/ab-boot/modules-B/crussty
cp /home/z/server/libcrussty_runtime.so /tmp/ab-boot/rt/
for m in A B; do cp -r /home/z/server/modules/crussty/native /tmp/ab-boot/modules-$m/crussty/; \
               cp /home/z/server/modules/crussty/module.json /tmp/ab-boot/modules-$m/crussty/; done
cp /tmp/wT32base/target/release/libcrussty.so /tmp/ab-boot/modules-A/crussty/
cp /tmp/wT32boot/target/release/libcrussty.so /tmp/ab-boot/modules-B/crussty/

# 2) measure (each batch = ONE foreground tool call, ≤600 s; sandbox kills bg procs between calls)
export CRUSSTY_NATIVE_IMPROVED_NOISE=1
BOOTAB_TAG=A-base  BOOTAB_PORT_BASE=26200 BOOTAB_TSV=/tmp/ab-boot/results-A.tsv \
  BOOTAB_MODULES=/tmp/ab-boot/modules-A bash bench/bootab/run_bootab.sh 'native surface live' 150 3
BOOTAB_TAG=B-master BOOTAB_PORT_BASE=26300 BOOTAB_TSV=/tmp/ab-boot/results-B.tsv \
  BOOTAB_MODULES=/tmp/ab-boot/modules-B bash bench/bootab/run_bootab.sh 'native surface live' 150 3

# 3) when a REBUILT ENGINE runtime .so lands: cp it over /tmp/ab-boot/rt/ and repeat —
#    same harness, now the agent side is A/B'd too.
```

Safety invariants honored: `/home/z/CRUSSTY` + `/home/z/server` untouched
(md5-verified before/after: runtime .so, module .so, both native kernels,
purpur jar — all OK; live PIDs alive 2h07m at finish), only self-spawned PIDs
killed (verified dead, no strays), unique ports 261xx–263xx (live = 25565),
no `.so` committed to any repo.

## Phase-2: scan-avoidance A/B (TASK-45-R, 2026-09-08)

Agent: agent-7625532f (TASK-45-R, rescue of TASK-45-w6 which died on deadline).
Question: does the TASK-22 find_class sighting-gate actually avoid full JVMTI
class-heap scans in a live-ish boot, measurable via the per-hook activation
counters (`<hook>: sighting feed: N full class-heap scans avoided`)?

### Counter semantics (read from CURRENT master source, incl. TASK-43 deltas)

`cplug-sdk/src/classes.rs` (origin/master 587fd1b): on a find_class cache miss,
`unsighted_scan_due()` gates the fallback scan — FIRST unsighted call scans,
then `UNSIGHTED_SKIP_BUDGET = 7` calls are answered from the sighting feed
(each bumps the per-name `avoided` counter), then every 8th scans again. The
counter prints ONCE per hook at activation (poller exit), so for each hook:

```
scans_performed   = 1 + floor(avoided / 7)          (exact, from gate cadence)
unsighted_calls   = avoided + scans_performed
pct_calls_avoided = 100 * avoided / unsighted_calls
```

Old-module (arm A @ 4fb9d12) poller cadence: FIXED 2 s (`oldmod/src/area_map.rs:108`,
`improved_noise.rs:234`), no gate — every one of those find_class misses IS a
full GetLoadedClasses scan. Master (arm B) poller: 10 s negative backoff while
unsighted (TASK-22), gated 1-in-8. So per hook per 180 s dormant poll:
A = 90 scans (model, source-derived) vs B = 18 calls → 1 + floor(17/7) = 3 scans
(model; the "~3-4" TASK-22 estimate). Neither side was measured over a dormant
180 s window here — see window findings below.

### Harness

`bench/bootab/run_bootab.sh <marker> <maxwait> <runs> --phase2 <s>` (salvaged
from w6 worktree /home/z/wT45, reviewed line-by-line, unchanged): after the
PRIMARY marker (`native surface live`) hold the server <s> s, SIGTERM the exact
PID, parse the log for armed/sighting/force-load-attempt lines, append phase2
TSV. Same throwaway-boot methodology as §3, same runtime agent
(md5 b57b309280aff628b743cb27c11099da both arms), `CRUSSTY_NATIVE_IMPROVED_NOISE=1`
(armed profiles legit per S7-6 closure), ports 262xx/263xx.

Arms: A = module @4fb9d12, md5 ea2eb1cc9fd084e00148487686c3fe75 (byte-verified
against §1 table; salvaged /tmp/ab-boot/modules-A, not rebuilt — build reproducible
from /tmp/oldmod worktree whose target .so had the identical md5). B = module
rebuilt fresh from origin/master 587fd1b (md5 0c1291c11e05992bf6d4f44611c742f7;
includes TASK-43 + P0-fix chain 28cdc09→4da13af→b002d9c).

### Results

Raw: `results/bootab_phase2.tsv`; per-run boot rows in /tmp/ab-boot/results-*.tsv
(regenerable, not committed); logs /tmp/ab-boot/logs/ (not committed).

| arm | commit | window | runs | hooks armed | sighting lines | avoided/hook | scans/hook (derived) | pct calls avoided |
|---|---|---|---|---|---|---|---|---|
| A-p2 | 4fb9d12 | 60 s | 2 | 2/2 | **0** (counter does not exist pre-TASK-22) | — | — | — |
| A-p2w | 4fb9d12 | 120 s | 2 | 2/2 | **0** | — | — | — |
| B-p2R | 587fd1b | 60 s | 2 | 0/2 (!) | 0/4 | — | — | — |
| B-p2wR | 587fd1b | 120 s | 2 | 2/2 | 4/4 | **7** (deterministic) | **2** | **77.8 %** |

- A-side evidence type: **NO COUNTERS (expected)** — the sighting feed / avoided
  counter landed with TASK-22, after 4fb9d12. Salvaged w6 runs (module md5
  verified) show both hooks ARM in 60 s and 120 s windows with zero counter
  lines; A-side scan counts are therefore **MODELED from source cadence, not
  measured**: fixed 2 s poll → every miss scans → ≈1 scan per 2 s of polling
  (≈90 scans/hook over a dormant 180 s poll).
- B-side (MEASURED, n=2 runs × 2 hooks, deterministic): activation log prints
  `sighting feed: 7 full class-heap scans avoided` on every hook → per hook
  9 unsighted calls = 2 scans + 7 feed-answered = 77.8 % of calls avoided,
  scans reduced to 2 in the measured activation window.
- Window finding (honest negative, measured): with the 60 s post-marker window
  (w6's original plan) B hooks NEVER arm — the gate's own cadence pushes the
  post-load scan-due call (#9) to ~70–90 s, past the window (observed: 6
  force-load attempts, no pristine sighting, no activation, 0 counters). 120 s
  windows arm 2/2. The gate trades activation latency (≤~80 s here) for the
  scan reduction; dormant-poll savings (the 90→3 model) accrue only when the
  class never loads within 180 s.

### Verdict

- MEASURED (B, master 587fd1b): **7 scans avoided per hook, 77.8 % of unsighted
  find_class calls answered scan-free, scans/hook 2 vs modeled-A ~90 per dormant
  180 s poll** — deterministic across 4/4 hook activations.
- MODELED (A, 4fb9d12): no counters exist at that commit; ~90 scans/hook/180 s
  from source-verified fixed 2 s cadence. A/B differential is therefore
  measured-counter vs source-model, explicitly NOT a measured A-side count.
- TASK-22's claim estimate (~90 → ~3-4 scans/hook/180 s) is CONSISTENT with the
  measured per-8 gate cadence (180 s/10 s = 18 calls → 3 scans) and with the
  S7-6 live-server armed boot (`sighting feed: 7 scans avoided`, area_map).
- Discarded: w6's phase2-B-p2.tsv rows (module 5e396db2, commit stamp wrong,
  hooks never armed — same 60 s-window artifact) and this rescue's B-p2R 60 s
  rows (kept in TSV as the window finding, not used for the pct).

### Caveats

- n=2 per arm-window, sequential not interleaved, throwaway flat-world boots on
  the shared 2 vCPU box (live server resident) — directional, same §6 caveats.
- A-arm runs are salvaged from the dead w6 agent (2026-09-08 ~20:07–20:12Z);
  provenance = md5-verified module + runtime + logs in /tmp/ab-boot/logs/A-p2*.
- 60 s post-marker (the task's nominal window) is structurally too short to
  observe B-arm activation counters; 120 s is the minimal honest window found.
  A 180 s dormant-poll measurement (the full TASK-22 model window) was NOT run
  (deadline) — the ~3 scans/180 s number remains model, not measurement.
- No product code, no .so, no live-server artifacts touched; only
  bench/bootab/* + these scratch dirs.
