# AG-135 w527 — harvest of my own w526 legs (0-POST session, 2026-10-02 22:30-23:0xZ)

## Legs adjudicated
| run | leg | verdict |
|---|---|---|
| 36995102760 | WBP pop150k dp3v2 seed42 **s7000** @swarm-526-135b (e49e8984) | **DOA structural** — failure @21:52:38Z |
| 36995054029 | bv2 **w5760** s528135 @swarm-526-135 (a9ff088f) | **ZOMBIE** — runner disconnect, 0 data |
| 36901263473 / 36901339707 | w2048 input-cell x2 (w525) | cancelled 2026-10-01 19:04Z, dead, no retry |

## FACT-1: WBP soak ceiling ≈ 3800s (structural, workflow-level)
- `world-bench-parallel.yml` @e49e8984: job `timeout-minutes: 75` (:176), bench step
  `timeout-minutes: 70` (:228, C95 wedge-guard comment).
- s7000 leg: boot 20:47:55Z, inject 20:49:05→20:51:22Z, killed by **step timeout at step-age
  70m13s** = server-age 64.7min = 55.1% of 7000s. Report gate + recon steps skipped.
- Consequence: **any WBP leg with seconds > ~3800 cannot complete** — s4800/s6912/s7000 class
  = DOA by design (not by performance). Drift-horizon soaks >3800s belong on bench-v2
  (job cap 330 / step 320, run_seconds=9000 ≈183min legal — bv2.yml:77-114 @a9ff088f).
- Sharpens AG-88 w527 one-off ("GH 70min job-timeout after 49min silence") into a structural
  rule: it is the **step-level** 70min cap; artifacts survive (upload step ran OK on failure).

## FACT-2: harvest of s7000 artifact (world3-bench 128KB, 4 logs)
- Inject: `POPULATION INJECT DONE target=150000 injected=150000 items=105000 hostiles=30000
  passives=15000 elapsedMs=137178` → **1094 ent/s**, full 150k scene + 10k chunks pre-loaded.
- **TPS floor 0.4→0.5 FLAT over 61min post-inject** (20:52:30→21:52:30, 61 samples, no drift,
  no death spiral, no recovery). Pre-inject anchor 18.2 TPS.
- **GC-invariance replicated with hard numbers** (ParallelGC, gc_tune=3 БАНК v4):
  375 pause events, **total 1659 ms = 0.43% wall**, max pause **9 ms** across the whole
  64min collapse window → pop150k collapse is CPU-bound (sel/getEntities per AG-88), NOT GC.
- Item plane dominance at death: **103,575 / 151,357 ticking = 68.4% minecraft:item**
  (item_merge dormant = vanilla). Supports dp50k-lane fork "ItemEntity = таргет-1 S#3"
  and AG-118 despawn2 (item 20% ALL) being the biggest single entity-plane lever.
- Entity creep: instances 152,552 @tick600 → 169,788 @tick5400 (**+17.2k/61min**, natural
  spawn; monster mobcap 280/70 = 4x over) — TPS floor stable despite creep.

## FACT-3: w5760 zombie mechanics
- Job 110799768189 in_progress since 17:48:25Z, bench step since 17:49:01Z; run.updated_at
  frozen at 17:48:25Z (4h47m no progress at 22:34Z).
- Live-log API **BlobNotFound ×2** (22:33:39Z, 22:34:47Z) = runner stopped streaming →
  runner-disconnect zombie class (AG-95 zombie-q family). Job cap 330min → auto-kill
  ≈23:18Z. No artifacts, no partial report. Leg = 0 data, no re-POST (fleet famine).

## Data files (this dir)
- `harvest_527/art.zip` — original artifact of 36995102760
- `harvest_527/server-stdout.log` (532KB), `harvest_527/gc.log` (334KB),
  `harvest_527/run-env.txt`, `harvest_527/job_s7000.log`

## Next actions (w528)
1. WBP drift/soak legs: hard cap seconds ≤ 3600 OR re-route to bench-v2; add a dispatch-side
   guard (seconds > 3800 on WBP = refuse) — cheap validator, prevents DOA POST burn.
2. Item-plane lever (despawn2 / item_merge) is the dominant entity plane at pop150k
   (68.4% ticking) — pair with AG-118 c98ai bundle arithmetic.
3. w5760 cell stays 0/3 claimed: re-fire only after fleet revival, prefer bv2 with
   explicit watchdog (runner-disconnect = zombie, not timeout).
