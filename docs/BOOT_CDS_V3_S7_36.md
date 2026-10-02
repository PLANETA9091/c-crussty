# BOOT CDS v3 — explicit-cp topology: measured VERDICT (S7-36/TASK-95)

Status: **v3-vs-v2 = MEASURED NULL (within-session ABBA); e2e default stays v2.**
What IS banked from this session: (1) the archive-effect measurement,
(2) two fd-hygiene fixes in e2e, (3) three reusable rigs, (4) a
methodology correction (baseline drift) that invalidates cross-session
boot comparisons on this box.

## 1. What was attempted

TASK-93 (S7-35) quantified the native classload island: Inflater = 58% of
W1-native — decompression of ~1.3k library-jar classes the v2 archive
never captured (v2 maps 15,184/27,826 classes). Design: dump AND run both
use the app loader with an explicit static classpath
(`versions/1.21.10/purpur-1.21.10.jar` + every `libraries/**` jar, sorted)
+ `org.bukkit.craftbukkit.Main` — loader identity matches, the dump
captures everything loaded at boot (**29,680 classes**, 148MB archive),
the map serves it all back.

## 2. Measurements

### 2a. Archive effect within the explicit-cp topology (VALID, iron)

| Arm | Config | Times (s) | Mean |
|---|---|---|---|
| dump (vanilla, +ArchiveClassesAtExit) | no custom archive | 16.441 | — |
| control c1/c2 (explicit-cp + agent) | no custom archive | 17.764, 16.800 | 17.28 |
| v3 pool (a2,a3,v3c,v3d,d1,d2) | + v3 archive | 12.954, 13.044, 12.150, 12.800, 12.885, 12.650 | 12.75 |

Archive effect: **17.28 → 12.75 = −4.5s** inside the same topology.
Functional parity every boot: `Loaded 1461 recipes` / `Loaded 1574
advancements`, zero ERROR lines, hs_err 4/0 across all boots.

### 2b. v3 vs v2 (BOTH with archives) — the question that matters — NULL

Cross-session comparison (v3 pool n=6 mean 12.747 vs S7-32 default
13.351 n=3, MW p≈0.048) was **CONFOUNDED**: S7-38 (cron sibling)
discovered the 13.351s baseline was measured while the neighbor's
TASK-90 census run loaded the box; on an idle box the v2 default itself
boots **12.5-12.6s**. Cross-session boot arithmetic is invalid on this
machine — within-session ABBA is mandatory.

Within-session ABBA (D = v2 `-jar` topology + v2 archive; V = v3
`-cp` topology + v3 archive; anchor-restore per boot; only boots with
zero concurrent agent CPU accepted):

| Pair | D (v2) | V (v3) | ΔV−D |
|---|---|---|---|
| pair-1 (S7-38) | 12.559 | 12.866 | +0.307 |
| pair-2 (S7-38, idle-verified) | 12.810 | 12.109 | −0.701 |

Mean Δ = −0.20s with sign instability → **NULL** at the achievable
noise level (pair spread ±0.3-0.7s on 2 cores; two agents sharing the
box during any measurement injects ~+1-2s — one of my own D arms
measured 14.764 while the sibling agent compiled/analyzed on the other
core, discarded as contaminated).

Verdict: the ~1.3k extra mapped classes do NOT convert into wall-clock
boot time vs the v2 archive. The remaining boot time is object
construction (S7-32/33/35 censuses), not classloading — consistent with
the R1 gate verdict (4 islands, no ≥80% single mechanism).

**e2e default remains v2 (reverted).** `-Xverify:none` micro arm
(12.242/12.361, n=2) was measured inside the confounded series — NOT
banked; re-verify inside a dedicated idle-locked ABBA if pursued.

## 3. Bugs found and fixed (deliverables)

1. **"shared class paths mismatch" root cause** (v3a/v3b): rig inserted
   `$AGENT` unquoted into `CRUSSTY_BOOT_CMD`; e2e evals the command and
   `;` inside `-agentpath` terminated the `exec` line → java ran without
   `-cp`. Fix: single-quote agentpath inside boot commands.
2. **`-Xshare:on` hard-fails under the weaving agent** (a1 arm) — CDS
   downgrade; non-strict mapping verified via `-Xlog:cds` (8 map-lines
   per run). Never wire `-Xshare:on` with this agent.
3. **Flock leak class (4 incidents)**: e2e's detached stdin-holder AND
   the boot subshell (server JVM) inherit the rig's flock fd (200) →
   BENCH.lock stays locked after clean rig exits; live boots get killed
   by holder-purges (S7-37's live boot was killed by S7-36's purge —
   apology protocol in CLAIMS; no completed-arm data lost). **Fixed in
   e2e**: holder spawn and boot subshell close every inherited fd >9.
4. **direct-ABBA rig cwd bug** (`cds_v3_direct_abba.sh`): no `cd $SERVER`
   → boots ran from the agent's cwd and died on EULA (TASK-90 lesson
   re-learned; 3 boots burned before the poll-by-stdout-log change
   exposed it). Also `MANICMD`/`MAINCMD` typo killed all arms under
   `set -u`. Fixed in the rig.
5. **Poll-by-latest.log is unreliable** when log rotation interacts with
   truncation — poll the per-arm stdout log instead.

## 4. Reusable artifacts (committed)

- `bench/boot/cds_v3.sh` — v3 dump (idempotent, `RUN_DUMP=1`) + fixed
  quoting + persists archive to `$SERVER/crussty_boot_v3.jsa`.
- `bench/boot/cds_v3_matrix.sh` — resumable arm matrix (`ARMS="..."`).
- `bench/boot/cds_v3_ab.sh`, `cds_v3_abba.sh`, `cds_v3_direct_abba.sh`
  (S7-37/38) — sibling rigs, typo/cwd fixes folded in.
- `docs/BOOT_DF_U_SCHEMA_CACHE_DESIGN_R3.md` — R3 design line (DFU
  joins, 7-11% window CPU; cheapest real line = R3-c parallel schema
  `<clinit>` prewarm, blocked on R2 framework).
- v3 archive persists at `/home/z/server/crussty_boot_v3.jsa` (148MB) —
  available for future re-tests; NOT wired into any default.

## 5. Ranking after this session

1. collision-join island (18% W1) — attribute via TASK-84 collision probe.
2. R3 DFU design line (doc above).
3. `-Xverify:none` idle-locked re-verification (n=2 today, unbanked).
4. R2 serve-at-load (honest boot ceiling ~0.2s; value is post-Done).

Methodology law added to the boot ledger: **no cross-session boot
comparison on this box; only within-session ABBA with an idle-verified
co-tenant check (CLAIMS journal + pgrep) per arm.**
