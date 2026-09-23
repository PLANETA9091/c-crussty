# CRAC P6B — ATTEMPT 19 (S7-81, cron 370520, 2026-09-09)

**Pre-registered:** CLAIMS.md S7-81. Rig v11.3 = v11.2 + bounded checkpoint retry x3 @2s
(same boot, REFUSED-SURVIVED semantics, per-attempt cause capture jcmd_att*.out, hooks
re-entry safe via SURGERY-SKIP-DUP). Canonical injects-only launch.

## Verdict: PARTIAL — retry lever MECHANICALLY PROVEN; onion model surfaced layer 2 (P6B-21)

| Metric | Value |
|---|---|
| Boot | 17.3s canonical |
| CK-ATT1 | jcmd_rc=0, survived — refused on fd=134 cgroup (P6B-20 class) |
| CK-ATT2 | jcmd_rc=0, survived — refused on `Cannot close logs/latest.log` (NEW) |
| CK-ATT3 | jcmd_rc=0, survived — same as ATT2 |
| fd=134 cfs_quota in att2/att3 | **0 hits — cgroup fd is transient, P6B-20 µs-window model CONFIRMED** |
| Image | none |
| Boots | 1 (cap; retries are same-boot by pre-registration) |
| hs_err | 4/0, 0 crash-reports, 0 config |

## LAW P6B-21: POLICY-CLOSE IS NOT IDEMPONENT ACROSS CHECKPOINT ATTEMPTS
Attempt 1 executed the layer-A policy `close: logs/latest.log` (checkpoint-side close OK);
the refusal unwind did NOT reopen it (close action = stay closed). Attempt 2's engine pass
re-evaluated the same policy on a resource whose underlying stream is now closed →
`CheckpointOpenResourceException: Cannot close logs/latest.log` with
`Caused by: java.io.IOException: Stream Closed` → suppress → refuse. The policy layer
assumes a single-shot checkpoint lifecycle; a refused attempt permanently poisons
close-action resources for subsequent attempts on the same JVM.
(Distinct from P6B-15 restore-lane poisoning: this is checkpoint-side, between attempts,
same process — not cross-restore filesystem mutation.)

## Multi-attempt evidence table (honest union)
| Attempt | Suppressed cause |
|---|---|
| 1 | CheckpointOpenFileException fd=134 /sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us |
| 2 | CheckpointOpenResourceException Cannot close logs/latest.log (IOException Stream Closed) |
| 3 | same as 2 |

Sweeps: att1 main sweep closed=14 (12 anon_inode + spark JFR tmp + /proc/<pid>/task),
late sweep closed=0 — identical to a18. Hooks re-entry verified: BCP-RAW + BCP-ORG both
re-fired per attempt with SURGERY-SKIP-DUP x2 (no double-close, no deadlock);
AR-REBIND-SKIP no-image x1 (image-gate still correct); AR-LISTEN absent x4.

## Consequences for a20 (pre-registered design)
The retry lever itself is sound — att1's ONLY blocker was the transient cgroup tick, which
was GONE by att2. The blocker that migrated in is SELF-INFLICTED via P6B-21. Minimal fix:
**latest.log policy `close` → `ignore`** — fd stays open through checkpoint, engine restores
regular-file fds natively (proven class: versions/** jar ignore, a11+ restore-passed);
removes the P6B-21 surface entirely AND the close/reopen asymmetry. Log-file semantics:
restored process continues appending at preserved offset (same fd, O_APPEND).
Expected a20: att1 or att2 clears → img>0 → async AR-REBIND → SLP verdict. If a NEW layer
surfaces, onion continues (each fix = one policy line, zero boots banked separately).
