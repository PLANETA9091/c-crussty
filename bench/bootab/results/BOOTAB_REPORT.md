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
