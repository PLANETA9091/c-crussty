# TASK-147 — FIRST PERFORMANCE-DELTA DATAPOINT (zero-boot corpus, exploratory): current inject is performance-neutral on burst chunk-gen at this resolution

**Agent:** agent-7625532f · 2026-09-09 · **ZERO boots, zero runs** — extraction from banked RAW run.logs (script `/home/z/my-project/scripts/task147_perf_delta.py`, persisted). Claim dev-logs ea49f43. EXPLORATORY grade disclosed upfront: n=7 vs n=2, no ABBA randomization, eden/JIT lottery confounds, settle-streak quantization ±1–2s — hypothesis-generating only, no significance claims.

## Metric

Add-leg wall = seconds between the `baseline:` and `post-add:` run.log lines. Identical `wait_settle` instrument on both sides (forceload add BAND → generation → idle-detection streak). Agent n=7 RAW dirs (T129-R1, #4–#9; T129-R2 + T132#3 excluded — no RAW timestamps); vanilla n=2 (CLEAN_124515 + BUGGY_NOSETTLE — its add-leg completed normally BEFORE the shakedown bug).

## Numbers

| side | walls (s) | median | range |
|---|---|---|---|
| agent (n=7) | 27, 30, 28, 26, 30, 26, 28 | 28 | 26–30 |
| vanilla (n=2) | 25, 28 | (26.5 mean) | 25–28 |

Delta of medians **+2 s (≈ +7 % wall), 5/7 agent runs inside the vanilla range** — WITHIN the registered indistinguishability expectation; no systematic slowdown signal. Boot walls: agent [16.18–18.00] median 16.31 vs vanilla [16.33–16.69] — both vanilla boots inside the agent band.

## Interpretation (honest)

At this resolution the current inject (modules dormant — `improved_noise`/`perlin_noise` env-gated OFF per boot.log) is a **cost-only presence**: ≈ +31 MB native resident + ≈ +5 MB live heap (TASK-146) with no measurable throughput penalty AND no measured benefit. The first BENEFIT datapoint does not exist yet — realizing benefit requires the next dev phase (enabling/wiring the dormant modules). Strategic item FOR OWNER alongside the v3 discussion (TASK-143).

Follow-up candidate (registered): dedicated ABVA timing rig with a HARD completion signal (chunk-count marker instead of idle-detection) before any performance claim either way.
