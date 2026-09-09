# TASK-123 — Bench Rig Inventory & Hygiene (census 2026-09-09)

**Agent:** agent-7625532f · claim 43e28e3 · zero boots, repo-static · commit of record: see ledger §55

## Scope

Post-queue consolidation: the bench tree accumulated 14 graal_ab rigs + 14 subdirs over
TASK-31…TASK-122, and no single document maps them. This inventory fixes that, verifies
hygiene invariants (flock/journal/reentrancy/fifo/gitignore), and executes the only zero-risk
repair found. Twin's `bench/boot` lane is censused at directory level only (their keep-out).

## 1. Tree census (tracked files / on-disk size)

| subdir | tracked | size | notes |
|---|---:|---:|---|
| graal_ab | 254 | 48M | JIT channel rigs TASK-96…121; 14 rigs, 11 results docs, 19 RAW dirs — RAW git-tracked |
| p500 | 219 | 3.1M | the canonical 49-group/70-pair/129-kernel rerun source of truth |
| e2e | 442 | 2.5M | end-to-end wiring checks |
| batch | 52 | — | adoption matrix waves |
| step0_noise / areamap / boot | 54/35/51 | 576K/452K | early waves + twin's CRaC boot lane (census only) |
| lifecycle / bootab / chunkencode / betick / blendprobe / noise_ab / jitflags | 31/9/8/4/5/6/6 | — | point campaigns, all closed |
| dirtyrate | 10 | 8.8M | see §3 gitignore law |
| **total** | | **65M** | |

## 2. graal_ab rig hygiene matrix (14 rigs)

Every rig carries `flock` on `/home/z/BENCH.lock` + a start/done journal pair in
`BENCH.lock.journal` (14/14 ✓). Explicit `RAWDIR`-reentrancy exists only on the two newest
rigs (`run_task118_cicount_ab.sh`, `run_task119_c3_probe.sh` — the two with cadence/follow-up
futures). The older 12 are single-shot campaign rigs whose RAW dirs and results docs are
committed; they are **documented as-is, deliberately not rewritten** (proven rigs don't get
touched; none of them has a scheduled re-run). printf-`\t` TSV emission appears where the
TSV law postdates the rig (TASK-118+); earlier rigs either emit no TSV or use whitespace-
tolerant parsers already banked in their results docs.

| rig | task | flock | journal | reentrant | status |
|---|---|---|---|---|---|
| run_graal_ab.sh | 96 | ✓ | ✓ | — | closed |
| run_graal_loaded_ab.sh | 97/100 | ✓ | ✓ | — | closed |
| run_graal_soak.sh | 117 | ✓ | ✓ | — | closed (SOAK-PASS) |
| run_task100_bundle.sh | 100 | ✓ | ✓ | — | closed |
| run_task105_jfr.sh | 105 | ✓ | ✓ | — | closed |
| run_task106_cold.sh | 106 | ✓ | ✓ | — | closed |
| run_task108_{smoke,v3_ab,v3_census,v3_smoke}.sh | 108 | ✓ | ✓ | — | closed (v3 census + NULL) |
| run_task116_jit_isolation_ab.sh | 116 | ✓ | ✓ | — | closed (+23% win) |
| run_task118_dedup_ab.sh | 118 | ✓ | ✓ | — | closed (NULL) |
| run_task118_cicount_ab.sh | 118 | ✓ | ✓ | ✓ | closed (NULL), re-runnable |
| run_task119_c3_probe.sh | 119/121 | ✓ | ✓ | ✓ | **live — cadence C3 probe** |

## 3. Discoveries & the one fix

- **gitignore law (dirtyrate):** `.gitignore:23` = `bench/dirtyrate/RAW_*/` — campaign RAW
  dirs deliberately ignored, with the canonical analysis dir
  (`RAW_DIRTYRATE_20260908_175619/`, 8 files) force-added + 2 results docs. This is a prior
  curation decision, **not drift**. Recorded so nobody "fixes" it in either direction without
  checking the ledger first.
- **THE FIX (executed):** 106 stale `console.fifo` transport stubs (0-byte named pipes from
  completed runs) sat in ignored/untracked space under `bench/dirtyrate/` and older graal_ab
  dirs. Deleted via `find bench -name '*.fifo' -type p -delete`; post-check: 0 fifos remain,
  `git status` clean (proves zero git impact). The recurring hygiene step (fifo removal
  pre-commit) now has this sweep as its backstop; live rigs still delete their own fifo
  before committing their RAW.
- **git-vs-disk trap banked:** `grep -c "Java_"` on `JNI_EXPORTS.manifest` counted 284 vs the
  true 283 exports because a header comment contains the substring — the same class of naive-
  count error as the fifos: counts must exclude comments/blank lines before comparison.
- **RAW tracking:** graal_ab RAW dirs are fully git-tracked (spot-check: TASK-119 dir = 5/5
  files), matching the evidence-permanence policy that survived the TASK-86 reset lesson.
- boot/ rigs (twin): 51 tracked files incl. per-attempt results docs — censused only; their
  hygiene is owned by the TASK-115 lane.

## 4. Verdict

**HYGIENE-PASS.** Tree is consistent: every closed campaign has rig + results doc + RAW
(inside or deliberately outside git per recorded law), every rig takes the mutex and journals
itself, the only live rig (C3 cadence probe) is reentrant, and the one piece of physical
clutter (106 fifo stubs) is gone. No dead rigs found — every script maps to a closed or live
ledger entry. Standing: re-run this census after any new campaign lands in bench/.
