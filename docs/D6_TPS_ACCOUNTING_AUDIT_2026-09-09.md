# D6 — TPS-accounting per-tick cost: static anatomy + arithmetic close (TASK-94)

Date: 2026-09-09 (cron tick 00:40+08) · Agent: agent-7625532f
Origin: HOTSPOT_CANDIDATES_V2 status addendum 2 **D6** (P3, idle-CPU hygiene, "design-first, zero
gameplay-value surface") from the first live JFR profile (`bench/e2e/results/JFR_PROFILE_2026-09-09.md` §4).
Evidence class: **javap-fidelity static audit of the deployment runtime classpath** + arithmetic on the
already-measured JFR sample set (TASK-92 discipline — cheapest close: 0 boots, 0 src/).

## 1. Pre-registered gate (from the TASK-94 claim)

GO only if the per-tick accounting cost is ≥3% of the tick budget under a realistic load profile.
Everything else closes OUT/NO-GO. Re-open criteria recorded in §6.

## 2. Static anatomy (javap, remapped classpath jar)

Runtime classes (`ca.spottedleaf.moonrise.common.time.*`, from
`plugins/.paper-remapped/remap-classpath/299BEE…881.jar` — NOT the on-disk purpur jar; Paper 1.21+ executes
the remapped copy):

| Class | Role |
|---|---|
| `TickData` | one rolling window: `long interval` + `ArrayDeque<TickTime> timeData`; `addDataFrom` trims entries older than `interval` from the head (amortized O(1)) |
| `getTPSAverage(TickTime, long)` | **full iterator walk of the whole deque**, calling `TickTime.differenceFromLastTick(now)` per element — O(window length), allocates the iterator + boxes the result |
| `TickTime.differenceFromLastTick(now)` | `tickStart − previousTickStart` when a previous tick exists (**now-independent**); `max(now, tickLength())` only for the boundary entry without a predecessor |
| `MinecraftServer` | holds **7 windows**: `tickTimes1s/5s/10s/15s/1m/5m/15m` |

Per-tick path (MinecraftServer bytecode):

1. `addTickTime(TickTime)` — under `statsLock`: `addDataFrom` on **all 7 windows**, then
   `clearTickTimeStatistics()` which nulls the `tps:[D` and `msptData5s` caches. Net effect: the TPS cache
   is **invalidated every single tick**.
2. Later in the same tick, `C()` (tick-scheduling method, references `PurpurConfig.tpsCatchup`) calls
   `getTPS()` → cache miss (guaranteed — just invalidated) → `computeTPS()` → `getTPS(TickData, long)` for
   the **4 report windows: 5s, 1m, 5m, 15m** → each a full deque walk.

## 3. Arithmetic — mechanism matches the measured profile

Element counts at 20 TPS (deque length = interval × tick rate): 5s→100, 1m→1,200, 5m→6,000, 15m→18,000.
Total per tick ≈ **25,300** `differenceFromLastTick` visits (plus iterator overhead), of which the 15m
window is ~71%.

Measured (JFR_PROFILE_2026-09-09 §4, idle, 927 samples / 10 ms period / 544 s):
`TickData.getTPSAverage → ArrayDeque` leaf cluster 142+17+23 samples ≈ 1.42 s CPU over 544 s =
**~0.13–0.17 ms/tick**. Model check: 130 µs / 25,300 visits ≈ **5.1 ns/visit** — exactly the plausible
cost of a JIT'd `iterator.next()` + record-field loads + long arithmetic. The measured number is fully
explained by the mechanism; no hidden contributor.

Load-independence note (honesty correction to the F3 framing): the deque sizes are **time-bounded, not
load-bounded** — the ~0.13–0.17 ms/tick cost is a **constant at 20 TPS whether idle or fully loaded**.
Only its *relative* share dilutes under player load; the absolute cost does not. The verdict therefore
does not hinge on idle-vs-load at all.

## 4. Verdict

**OUT / DO-NOT-BUILD (12th closed x1000 branch, second-cheapest close after TASK-92).**

| Gate | Value | Result |
|---|---|---|
| Pre-registered GO gate | ≥3% of tick budget | measured ceiling **0.26–0.34%** (0.13–0.17 ms / 50 ms) → **fails by 9–23×** |
| x1000 bar | >100× same-state win | perfect removal = 1.002–1.003× of a tick → **2+ orders below** |
| Config-side alternative | paper/purpur knob to shrink windows | **none exists** in this build (window intervals hardcoded; `PurpurConfig.tpsCatchup` affects catch-up scheduling only — bytecode-verified) |

Even a *perfect* zero-cost patch removes ≤0.34% of the tick budget. This is the same economics shape that
killed TASK-80 (fluid guard) and TASK-89 (BE-tick guard), one order smaller than TASK-89's ceiling.

## 5. Design sketch (retained per the P3 "design-first" ask — NOT implemented)

The structure is append-only + head-trim with now-independent per-entry deltas (§2), so a **rolling-sum
O(1) aggregate is strictly correct**: maintain `count` and `sum of (tickStart − previousTickStart)` in
`addDataFrom` — add the new tail's delta, subtract each trimmed head's delta — and compute
`getTPSAverage` as `sum / count`, handling the single now-dependent boundary entry
(`max(now, tickLength())`) exactly as today. No correctness drift, no staleness window, no extra
allocation. Alternatively a config-side window shrink would need an upstream moonrise/purpur change.
Patch vehicle if ever needed: the existing class-retransform machinery (same class as the area_map
patch). None of this is wired: the ceiling (§4) makes it un-justifiable on this campaign's gates.

## 6. Re-open criteria

* JFR under a realistic player-load profile shows the cluster ≥3% of the tick budget (would require a
  ~10× drop in per-visit cost or tick budget, not a workload change — window sizes are fixed).
* Engine change to `TickData`/`computeTPS` (e.g., upstream adding the rolling sum upstream — then the
  lever moves upstream for free anyway).
* Hosting-density deployment profile where idle-main-thread hygiene is explicitly prioritized over the
  standard gates (owner decision, not census).
