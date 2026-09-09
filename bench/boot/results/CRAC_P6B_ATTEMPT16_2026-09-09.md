# CRAC P6B ATTEMPT 16 — CANONICAL REBASE + LAW P6B-18 CGROUP RACE + REBIND TARGET CORRECTED (2026-09-09, S7-78)

Rig v10. Pre-registered: CLAIM S7-78 (e604c06). 1 boot. **INJECTS-ONLY law applied** (owner 14:45, docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md): perf flags (CDS trio/TieredStopAtLevel/Xms/Xmx) removed — canonical launch + agent/CRaC operational flags only.

## Results

- **Boot 17.3s canonical** — honest regress from flags-era 13.1s (-24% was a flag lever, now owner-banned; future boot work = agent-side injects only).
- **Checkpoint REFUSED (1 suppression): /sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us** (CheckpointOpenFileException, layer B regular-file) => **LAW P6B-18: the cgroup late-open race is NON-DETERMINISTIC** — a14/a15 zero-suppression ×2 was timing luck of the flag-era boot shape (TieredStopAtLevel=1 + 1G heap alter thread/JIT cadence); canonical shape re-opens the hole (fd opened after the first sweep). Deterministic fix banked (a17 verify): layer-A policy `type: file action: close path: /sys/fs/cgroup/**` (open-time claim, sweep-timing-independent) + LATE second `anonInodeSweep()` at the end of beforeCheckpoint (race-window shrink).
- **AR-LISTEN plumbing LIVE — v9.1 placement fix VERIFIED**: AR-ORG fired with AR-LISTEN[PRE] 25565=absent 25575=absent and AR-LISTEN[POST] absent ×2 — on the **P6B-9 UNWIND path** (checkpoint refused => afterRestore = rollback), not a restore run. Kernel state consistent with C1/C4 (netty closed by surgery, kernel port free). LOADER-MS2=URLClassLoader: INSTR class discovery works at the hook site.
- **AR-REBIND-ERR no-bind candidates= (EMPTY)** => v10 target WRONG: no `*bind*` method anywhere on the ServerConnectionListener hierarchy. Offline javap (runtime jar versions/1.21.10/purpur-1.21.10.jar — 0 boots): real mojmap 1.21.10 API = `startTcpServerListener(InetAddress,int)` / `startTcpServerListener(SocketAddress)` (void, appends to the private `channels` list), `acceptConnections()` (running gate), `stop()`, `tick()`. **rebind v11 banked + compile-verified offline (javac OK)**: startTcpServerListener(new InetSocketAddress(25565)) + acceptConnections() re-arm + channels-size delta (N->N+1) + channel localAddress marker + bounded full-method-dump diagnostic on miss.

## Acceptance scorecard

(a) AR-LISTEN live verdicts: **PASS** (plumbing verified; verdicts delivered from unwind, not restore). (b) SLP DEAD->SERVING: **NOT REACHED** (no image — refused). (c) R1+R2 alive: **NOT REACHED**. (d) zero suppressions: **FAIL — cgroup race exposed (P6B-18, the valuable negative; deterministic fix banked)**.

## Hygiene

1 boot 17.3s, hs_err 4/0, 0 config, ports clean post-run, BENCH journal pairs clean, no perf flags (law compliant), v3.jsa untouched, 2 dev-logs push races union-resolved (twin TASK-129 done + TASK-130 claim).

## NEXT (a17, S7-79)

1 boot on rig v10.1: expect zero suppressions (policy + late sweep) => img>0 => restore x2 => AR-REBIND rc=0 (size N->N+1, local=EpollServerSocketChannel@/0.0.0.0:25565) => **SLP verdict upgrade attempt DEAD->SERVING (primary signal)**. Kills unchanged: >1 boot, config touch, hs_err increment, perf-flag reintroduction.
