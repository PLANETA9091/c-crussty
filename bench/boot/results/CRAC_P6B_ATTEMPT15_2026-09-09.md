# CRAC P6B ATTEMPT 15 — TRIPLE 13.1s BOOT + v3-JSA VERDICT + AR-LISTEN PLACEMENT BUG FIXED (2026-09-09, S7-77)

Rig v9. Pre-registered: CLAIM S7-77 (3e588c8-line). 1 boot.

## Results

- **Boot 13.1s — THIRD consecutive measurement** (a14/a15 flags-only; CDS-free; -24% vs 17.3s stands as 3/3).
- **R1 alive=yes 0.27s + R2 alive=yes 0.27s** — double-alive 3rd reproduction; sweep closed=14; NETTY-CLOSE rc=0 (P6B-17 respected); no new crash-reports.
- **v3-jsa reuse: REFUSED by JVM** — cds.log: `Opened archive crussty_boot_v3.jsa` + `The shared archive file was created by a different version or build of HotSpot` -> mapping rolled back (Unmapping region), CDS OFF. Verdict: v3 (Sep-8 era) incompatible with current Zulu CRaC build; classpath-match moot — version gate fires first. CDS path requires fresh training on THIS build (graceful-exit train = a16 candidate; kill-9 flow cannot dump).
- **AR-LISTEN placement bug (honest): acceptance (b) NOT met this boot.** Main-resource afterRestore (HOOK-AFTER-RESTORE) NEVER FIRES in any run (org Compat + raw proxy hooks are the live path — AR-ORG/AR-RAW x2). AR-LISTEN was added at the dead site. Fix landed same session: listenState() moved to AR-ORG hook (rig v9.1, committed) — verification = a16 boot (pre-registered >1-boot kill prevented re-run this session).
- Checkpoint clean again: zero CheckpointOpenSocketException; probes 4/4 DEAD (expected).

## Acceptance scorecard

(a) img>0 + R1+R2 alive + suppressions 0: PASS. (b) AR-LISTEN verdicts: FAIL this boot (placement bug) — fix landed, a16 measures. (c) CDS verdict honest: PASS (documented mismatch). (d) design doc: PASS — docs/CRAC_AFTERRESTORE_REBIND_DESIGN.md (R1 in-process re-bind at afterRestore, R2 rcon parity).

## Hygiene

1 boot 13.1s, hs_err 4/0, 0 config, BENCH flock held, v3.jsa untouched (probe read-only, AutoCreate dropped).
