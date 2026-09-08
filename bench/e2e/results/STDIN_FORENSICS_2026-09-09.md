# STDIN Forensics — launcher fifo → console chain (TASK-59)

Date: 2026-09-09 (cron tick 10:20+08) · Agent: agent-7625532f
Scope: TASK-57 §6 I1 finding ("launcher stdin forwarding does NOT deliver console
commands") — root cause, fix, validation. Server /home/z/server (Purpur 1.21.10 +
CRUSSTY runtime, launcher/launcher.jar dist-owned, NOT modified).

## 1. Verdict

ROOT CAUSE (mechanism + live proof): the launcher's stdin relay thread
`launcher-stdin` exits on FIFO EOF, and the only FIFO write end was held by the
**boot invocation's bash process** (`exec 9<> "$FIFO"` in `do_boot`), which exits
right after the `Done (` marker. From that moment the writer count is zero, the
relay's blocked `read()` returns 0 (EOF), the thread leaves its loop silently
(`catch IOException → return`; EOF is a clean loop exit, not an exception), and
nobody ever reads the launcher's fd 0 again. Commands written later (`echo stop >
$FIFO` from a *different* `shutdown` invocation) succeed as writes — a reader fd
is still open, only the reader is gone — and sit in the FIFO buffer forever. The
server never sees them; the `shutdown` mode's primary path was dead and every
stop fell through to SIGTERM (exit 143; S7-13 precedent explained).

FIX (script-level, launcher.jar untouched): `do_boot` now spawns a detached
holder (`setsid bash -c "exec 9<> '$FIFO'; while :; do sleep 3600; done"`) that
keeps a write end open for the launcher's lifetime; `do_shutdown` and the next
`do_boot` (stale cleanup) kill holders and scrub leftover FIFO files.
`scripts/e2e_orchestrate.sh` header rewritten from "KNOWN BROKEN" to
"FIXED" with the root cause.

VALIDATED END-TO-END on live boots (same day, dormant env, BENCH.lock held):
post-fix boot → relay thread **alive** past boot-script exit (pre-fix: dead at
that exact point) → `list` via canonical FIFO path executed by the server
(response in latest.log) → `stop` via the `shutdown` mode's primary FIFO path →
**`[launcher] server exited with code 0`** (graceful; was SIGTERM 143), holder
killed, 0 leftover FIFO files.

## 2. Method

Two-pass, static → live (evidence hierarchy: measurement > plausibility):

1. Static pass (no server): boot wiring in `e2e_orchestrate.sh` re-read line by
   line (subshell `eval "$CRUSSTY_BOOT_CMD" < "$FIFO" >> log 2>&1 &`, fd9
   `<>` hold, STATE_FILE round-trip to standalone `shutdown`); launcher.jar
   disassembled with `/home/z/jdk21/bin/javap -c -p` (Main.class from
   dev.dist.launcher). Findings: boot wiring correct; `forward()` relay code
   CORRECT (4096-byte `read` → `write(buf,0,n)` → **`flush()` per chunk**, loop
   `while n != -1`, `catch IOException → return`); `javaOpts()` = `DIST_JAVA_OPTS`
   passthrough (nothing console-disabling); tee threads explain why child stdout
   was always visible while stdin never worked — asymmetry hint.
2. Live forensics (one dormant boot, JFR excluded): `/proc/<pid>/fd` mapping of
   both JVMs; `jcmd Thread.print` on launcher and child; probe writes with
   per-leg observation.

## 3. Evidence chain (pre-fix boot, 02:24–02:31)

- Physical chain INTACT: launcher fd 0 → `logs/crussty_e2e_stdin.11016` (FIFO);
  launcher fd 6 → `pipe:[831123]` write end; child fd 0 → `pipe:[831123]` read
  end. Data *could* flow — the break had to be behavioral.
- Launcher thread dump: **no `launcher-stdin` thread** (full non-GC listing;
  only `launcher-tee` ×2, reaper, Attach Listener). The thread demonstrably
  starts (bytecode `start()` in `forward()`) — it was started and died.
- Child thread dump HEALTHY: `Server console handler` waiting on JLine
  `NonBlockingInputStreamImpl.read` (DumbTerminal path); `JLine terminal non
  blocking reader thread` RUNNABLE blocked in `FileInputStream.read0` on the
  stdin pipe — the console reader is alive and *starving*, consistent with
  "no bytes ever arrive".
- Direct-pipe injection probe: `echo list > /proc/<launcher>/fd/6` (re-opens the
  same pipe write end) → `[02:28:03] There are 0 of a max of 20 players online:`
  in latest.log within 2 s. **Child leg 100% functional; the fault is exactly
  the dead relay.**
- `echo stop > /proc/<launcher>/fd/6` → graceful stop: region-file I/O awaited,
  world saved, server exit, launcher `waitFor()` returned, both processes gone.
  (Also a valid emergency injection channel independent of the launcher relay.)

EOF mechanism (why the relay dies): POSIX FIFO read end sees EOF iff writer
count drops to 0. At boot time exactly one write end exists (boot bash fd9,
opened `<>`). Boot returns at `Done (` → process exit → writer count 0 → relay
`read()` → 0 → loop break → silent thread exit. Deterministic; explains why
*nothing* ever delivered, not even once.

## 4. Fix (scripts/e2e_orchestrate.sh, script-level only)

- `start_stdin_holder()`: `setsid bash -c "exec 9<> '$FIFO'; while :; do sleep
  3600; done" </dev/null >/dev/null 2>&1 &` — no trailing `exec`, so the cmdline
  keeps the FIFO path visible for discovery. Launched in `do_boot` right after
  `mkfifo`, before the launcher subshell.
- `kill_stdin_holders()`: `pgrep -f 'crussty_e2e_stdin.*sleep 3600'` → kill.
  Session is single-tenant (BENCH.lock discipline), so killing *all* holders is
  safe; called from `do_shutdown` (after server exit) and `do_boot` (stale
  cleanup).
- `scrub_stale_stdin()`: holders first, then `rm -f $LOGS_DIR/crussty_e2e_stdin.*`
  — 22 leftover FIFO files from previous boots accumulated (one per boot since
  the fifo name carries the boot script's `$$`); all scrubbed by the first
  post-fix boot.
- Header comment rewritten: KNOWN BROKEN → FIXED + root cause + validation
  pointer. SIGTERM fallback retained unchanged (still needed for hard failures).

Design note: keeping the fifo indirection (instead of always injecting via
/proc) preserves the canonical UX (STATE_FILE already round-trips the fifo path
to standalone `shutdown`) and keeps the launcher's own relay as the transport —
the /proc injection stays documented as an emergency fallback (§3).

## 5. Validation (post-fix boot, 02:31)

| Check | Result |
|---|---|
| `bash -n` | PASS |
| Stale cleanup on boot | 22 stale fifos scrubbed, log lines emitted |
| Holder alive past boot-script exit | PASS (holder etime 29 s while boot bash long gone) |
| `launcher-stdin` thread alive post-boot | PASS (RUNNABLE, blocked in FIFO read — pre-fix: dead) |
| `list` via canonical FIFO path | PASS — response in latest.log within 3 s |
| `shutdown` mode primary path (`stop` via FIFO) | PASS — `[launcher] server exited with code 0` |
| Holder cleanup on shutdown | PASS — holder killed by `do_shutdown` |
| Leftover FIFO files after shutdown | 0 |
| Server state hygiene | stopped as found, world saved (region I/O awaited), dormant env, BENCH.lock released |
| Gates (src untouched) | cargo test 51/51 PASS; clippy `crussty` lib 12 warnings = baseline Δ0; one PRE-EXISTING `unused import: crate::classfile::*` in the *lib test* target from landed HEAD (not this task's diff; observed, not touched) |

## 6. Implications unlocked

1. **Targeted load profiles are now reachable**: console `forceload add/remove`
   (or `tp` + worldgen via injected flows) can drive fresh-chunk worldgen bursts
   on demand — the g9 revisit trigger ("player-driven worldgen profile with
   fillArray frames", TASK-57 §2) is no longer blocked on infra.
2. **Graceful stop = exit 0** end-to-end via the script's primary path; JFR
   `dumponexit`, shutdown hooks and world saves ride the clean path (SIGTERM
   remains the fallback, not the norm).
3. `DIST_JAVA_OPTS` is the supported env channel for extra child JVM flags
   (e.g. future JFR sessions could use it instead of `JAVA_TOOL_OPTIONS`).
4. launcher.jar remains dist-owned and unmodified; if the dist ever ships its
   own holder, the script's holder is harmless (extra write end, killed on
   shutdown).

## 7. Honest limits

- One validation boot per mechanism claim (deterministic POSIX semantics back
  them; multiple boots ran incident-free across the session's e2e flows).
- Holder kill uses a cmdline pattern match — a same-named FIFO outside
  `$LOGS_DIR` would not be matched (none exists; single-tenant box).
- The relay thread's death was proven by absence in the live thread dump + the
  deterministic EOF walk through the bytecode; the exact instant of death was
  not directly timestamped (thread dumps don't show dead threads).
