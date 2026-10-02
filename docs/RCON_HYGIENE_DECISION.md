# RCON HYGIENE — owner decision (TASK-75, S7-21)

Session: S7-21 main orchestrator (cron 15:43+08 Job 366450). Carried as NEXT-6 from
S7-14 through S7-15/19/20; unblocked by the TASK-59 verdict (stdin-chain root-caused
and fixed, which made this a real decision instead of a forced dependency).

## 0. Verdict (TL;DR)

1. **RCON stays ENABLED** (`enable-rcon=true`, port 25575) — it remains the robust
   programmatic command channel for live probes.
2. **Secret ROTATED.** The S7-14 one-time password is considered **burned**: it was
   committed verbatim into the dev-logs git history (SESSION 014 worklog text) and
   pushed to GitHub. The replacement is a fresh 48-hex random value.
3. The new secret lives **only** in `/home/z/.rcon_password` (chmod 600, outside any
   repo — same convention as `~/.git-credentials`). It must never appear in any
   committed file, log, report, or CLAIMS text.
4. `scripts/rcon.py` is hardened **fail-closed**: the password is NEVER a positional
   argument (ps-exposure); resolution order `--password-file` → env
   `CRUSSTY_RCON_PASSWORD` → default file.
5. **Git history is NOT rewritten.** Rotation renders the burned string inert; the
   audit trail stays intact. §2 lists the burn scope for the record.

## 1. Context

RCON was enabled in S7-14 to drive the g9 JFR probe because the launcher fifo did not
deliver mid-run console commands. TASK-59 later root-caused and fixed the stdin chain,
so RCON is no longer the *only* channel — but it remains the *robust* one: zero-touch,
round-trip responses, independent of fifo/lifecycle fragility. The TASK-59 verdict
explicitly deferred `rcon`/`server.properties` handling as an "owner decision"; this
document is that decision.

## 2. Audit findings (S7-21-A forensic pass, read-only)

- **Consumers:** `scripts/rcon.py` is the only code consumer. Doc mentions only:
  `docs/G9_WHOLE_METHOD_HOOK_DESIGN.md` (infra note), `scripts/README.md` §3
  (mcrcon-as-workload example), `bench/e2e/results/JFR_PROFILE_2026-09-09.md`
  (report, historical). The e2e / bootab / p500 harnesses have **no RCON dependency**
  (they use fifo/stdin + /proc-based detectors).
- **Burn scope of the old secret:**
  - dev-logs repo: 2 commits / 10 blobs, **pushed to GitHub** (worklog SESSION 014
    text + the TASK-75 claim text that names the string for forensic identification).
  - dev-logs working tree: `c-crussty/worklog.md:1376` and `CLAIMS.md:225`.
  - live `/home/z/server/server.properties:51` — **rotated in place this session**.
  - local-only copies (no remote): `/home/z/my-project` (4 cron commits / 7
    tool-results blobs, unpushed), `/tmp` (9 ephemeral files).
- **c-crussty repo: CLEAN** — exhaustively verified (full ODB scan including stash
  and unreachable objects; `git log -S` empty; working tree and HEAD clean).

## 3. Threat model

The box is a single-user sandbox and the vanilla RCON listener is only reachable from
localhost context — the *real* exposure surface is the dev-logs GitHub remote: anyone
with read access to that repo can extract the old secret from history. Residual risk
after rotation: the old string authenticates nothing (proved by the negative probe,
§5). The new secret carries ~192 bits of entropy, exists in exactly one 600-mode file
outside every repo, and is never passed on a command line.

## 4. Decision and rationale

1. **Keep RCON on.** The stdin fifo is fixed (TASK-59) but remains lifecycle-fragile;
   RCON is the only channel with synchronous round-trip responses, which the
   worldgen-probe class (g9 revisit) depends on. Disabling it would re-fragile-ize
   live probing for zero security gain on a localhost-only surface.
2. **Rotate, don't rewrite.** History rewrites would break the audit trail this
   project relies on (claims, cross-session evidence chains). Rotation is the correct
   mitigation for a leaked credential and was applied to the live config in place;
   the running-lane impact is zero because RCON config is read at boot and no
   protocol in flight uses RCON.
3. **`rcon.py` fail-closed (breaking change).** Positional password removed
   deliberately: the only CLI consumer ever was the one-shot S7-14 g9 probe (already
   run and archived). Keeping a positional fallback would preserve the ps-exposure
   class of bug this task exists to close.
4. **Secret discipline going forward.** The TASK-75 claim text names the *burned*
   string — acceptable only because it was already public in the same repo's history.
   The NEW secret must never be repeated in any committed text, unlike its
   predecessor; the verification script extracts the burned string at runtime from
   the committed worklog instead of hardcoding it.

## 5. Verification evidence (throwaway, TASK-58/67 precedent)

`scripts/rcon_verify_throwaway.sh`, run S7-21 (this session):

- Pre-flight: 0 foreign java processes; `/home/z/BENCH.lock` last lock entry closed.
- Dormant flat-world purpur throwaway in `/tmp/rcon_hygiene_verify` (ports
  25586/25585, `nice -n 19` — polite to the concurrent TASK-74 timing lane on this
  2-core box; no `/home/z/BENCH.lock` taken, server lane untouched).
- `Done (26.135s)` → **PROBE_NEW_OK**: hardened `rcon.py` (secret read from
  `/home/z/.rcon_password`) executed `list` with round-trip response
  `There are 0 of a max of 20 players online`.
- **PROBE_OLD_REJECTED**: the burned string (extracted at runtime from the committed
  dev-logs worklog text) gets the connection closed at auth — no access.
- **CLEAN_STOP** via RCON `stop` (dogfood of the new flow), process exit verified.
- `VERDICT: PASS`.

`/home/z/server` itself was **not booted** this session (no-cross with the concurrent
TASK-74 server lane); its rotated config takes effect automatically at the next boot.
Any future RCON user must read the secret from the file — the old one is proven dead.

## 6. Consequences / coordination

- The S7-14 one-time password is dead everywhere; any script still carrying it will
  be rejected (negative probe proves this is enforced by the server, not just policy).
- The concurrent lane (TASK-74, Session-2 armed boot + G-AB) uses fifo + /proc
  detectors, not RCON — zero expected impact; notified via the TASK-75 done-line in
  CLAIMS and the SESSION 021 worklog section.
- `server.properties` is not in any git repo; the rotation is therefore not itself a
  commit-sized event in the code repo — only the doc/tool changes land in git.

## 7. Revisit triggers

- The box gains any external network exposure → re-evaluate (bind `server-ip`,
  firewall, or disable RCON entirely).
- A dev-logs history rewrite ever becomes an owner priority → §2 is the burn-scope
  checklist (GitHub remote included).
- RCON consumer count drops to zero (stdin-fifo fully replaces it in practice) →
  consider `enable-rcon=false` in a future hygiene wave.
