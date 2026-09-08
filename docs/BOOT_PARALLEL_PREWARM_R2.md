# Boot Parallel Prewarm — R2-Framework First Brick (TASK-97, S7-40)

Owner directive context: «не делай такие мелочи — делай много» → micro-flags only as
combined batches; primary effort on the big levers. After TASK-95 closed the
classloading channel (CDS v3 = measured NULL vs v2 within-session ABBA), the entire
remaining cold-boot cost is **object construction**, single-threaded:
the +5.0s silent registry/datapack island (ServerMain 44% of samples) plus the +3.0s
DFU-schema/recipes/advancements window. This document specifies and measures the
cheapest genuine parallel-weaving brick: a prewarm agent that moves **library-family
`<clinit>` work off the serial main thread and onto the idle second core during the
silent window**.

## 1. Lever theory (2-core Amdahl)

The boot window is dominated by main-thread construction (JFR: ServerMain 44%).
The box has 2 cores; during the silent island the second core is largely idle
(remapper threads are transient, ~1.3s). Any construction work that (a) is
guaranteed to happen later anyway and (b) has no bootstrap-ordering dependency can
be executed early on worker threads with zero semantic change — the main thread
later observes initialized classes instead of paying lazy-init inside its serial
window. Candidate families (tier-S, pure library, zero `net.minecraft` /
bootstrap coupling): `com.mojang.datafixers.*`, `com.mojang.serialization.*`,
`com.mojang.brigadier.*`, `com.google.gson.*`, `org.apache.commons.*`,
`it.unimi.dsi.*`, `joptsimple.*` → **1,305 classes** from the 29,680-class boot
dump (`/tmp/cds_v3/cl_dump.txt`).

Counter-hypothesis (honest): on 2 cores, prewarm workers compete with the main
thread for CPU, memory bandwidth and JVM init locks; if lazy-init cost is smaller
than the contention it causes, the paired delta is negative. The measurement is
the arbiter, not the theory.

## 2. Design

`bench/boot/prewarm/PrewarmAgent.java` — minimal `-javaagent`:

- `premain(options)`: `options` = path to class list. Absent/empty → **dormant
  no-op (arm A)**. Otherwise spawns `prewarm.workers` (default 2) MIN_PRIORITY+1
  daemon threads doing `Class.forName(name, true, systemLoader)` over the list
  (round-robin split). Every per-class Throwable is caught and counted
  (`ok=/fail=` stderr line) — tier-S contamination can never fail the boot.
- **Variable isolation**: both ABBA arms run the IDENTICAL flag set, classpath,
  agent count and CDS state; only the agent's options string differs. The paired
  delta is therefore the pure prewarm effect — immune to the S7-38 baseline-drift
  confound by construction (within-session ABBA, anchor restore per boot).
- Topology preserved: default direct-purpur-jar boot + `SharedArchiveFile=v2`.
  Wrapper-main was REJECTED at design time: `org.bukkit.craftbukkit.Main` direct
  requires the 126-jar explicit cp, measured +4.5s topology cost (TASK-95 control
  17.28s) — it would drown any prewarm signal.
- CDS interplay: `-Xlog:cds=info` on every boot documents mapped-regions per arm;
  `-javaagent` appends the agent jar to the system class path, which *may* flip
  v2-archive validation (silent downgrade without `-Xshare:on`). Both arms carry
  the agent jar → downgrade, if any, is arm-symmetric and cancels in the paired
  delta; regions count is reported per boot for the ledger.

## 3. Protocol

`bench/boot/prewarm/run_prewarm_abba.sh`: ABBA (pair-1 A,B; pair-2 B,A), anchor
restore before every boot (`world_census_seed.tar.gz`, 0 overworld regions),
cwd=/home/z/server, RCON stop, BENCH-MUTEX flock + marker, hs_err passive count,
functional parity greps (recipes/advancements counts) per boot.

GO/NO-GO interpretation: 2 pairs; if Δ(B−A) signs agree and |mean Δ| ≥ 0.3s →
directional verdict (extend to 4 pairs / Mann-Whitney next tick); mixed signs →
INCONCLUSIVE (effect below machine variance ~0.5s, honest ceiling statement,
channel joins the ≤0.5s "batch-micro" class unless a larger family set is proven
safe).

## 4. Follow-on (if GO)

1. Widen tier-S: pull the full 1.9k-class non-shared library tail + JDK-internal
   immutable infra; measure again with 4 pairs.
2. Tier-R (risky, separate env flag): `DataConverterRegistry`/schema-join families
   — the +3.0s DFU window; requires bootstrap-ordering proof per class (javap
   static-reads audit) before any arm runs.
3. R3-c unblocked: the prewarm pool is the R2-framework primitive that the
   parallel schema-`<clinit>` prewarm line rides on (docs/BOOT_DFU_SCHEMA_CACHE.md).
4. Engine-native variant (only if the javaagent lever proves GO): move the pool
   into the JVMTI agent (src/, P500 duty applies), eliminating the agent-jar
   classpath delta and enabling ClassFileLoadHook-integrated prewarm accounting.

## 5. MEASURED VERDICT (S7-40, within-session ABBA, this machine)

- **v1 design (system-loader forName) = mechanically NULL**: `ok=0 fail=653/652`
  per worker — library classes live on paperclip's CHILD URLClassLoader in the
  default direct-jar topology, invisible to the system loader. Rig correctly
  captured the failure (no false claim).
- **v2 design (ClassFileTransformer loader-capture) = mechanism PROVEN,
  lever NULL**: first target-family load captures `java.net.URLClassLoader`
  (paperclip child loader, as theorized); workers initialize ~1,012/1,305
  classes early (ok=506+~506, fail≈292, ~1.05s wall on 2 workers); functional
  parity every boot (1461 recipes / 1574 advancements); hs_err 4/0 through
  8 boots (v1 + v2 runs); v2 archive mapped every boot (`Opened archive
  crussty_boot.jsa`; `-Xlog:cds=info` docs; `-javaagent`'s
  `jdk.module.addmods=java.instrument` disables optimized module handling —
  arm-symmetric, documented).
- **Timing** (ABBA 2 pairs, identical flags/classpath/agent-count, only agent
  options differ): A 13.872/13.549 (mean 13.711), B 13.798/13.909 (mean
  13.854), paired Δ(B−A) = {−0.074, +0.360} — mixed signs, |effect| < machine
  variance ~0.5s → **NULL, same class as CDS v3 (TASK-95)**.
- **Falsified hypothesis**: "library-family lazy `<clinit>` interleaves on the
  boot critical path and can be stolen by parallel prewarm." The ~1s of init
  work moved off-thread produced no wall-clock change: the +5s silent island is
  **mojang main-thread construction** (ServerMain 44%), not library `<clinit>`.
  Consistent with the S7-32 census (no family ≥14%).
- Consequence for the R2 line: generic class-prewarm is DEAD. The only
  remaining parallel-weaving variant that targets the island itself is Tier-R
  (weave/offload mojang construction: DataConverterRegistry / registry-builder
  families) — requires per-class bootstrap-ordering proof BEFORE any arm;
  pre-registered suspicion: ordering coupling will make it a design NO-GO;
  measured tier-S NULL is the cheap evidence that construction work is
  serial-coupled, not conveniently deferrable.
