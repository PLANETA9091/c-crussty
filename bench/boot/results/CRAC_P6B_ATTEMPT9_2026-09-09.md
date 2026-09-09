# CRAC P6C ATTEMPT 9 — S7-70 (2026-09-09) — agent v8 policies+anon_inode: LAYER B CONFIRMED, LAYER A SYNTAX DECODED

## Pre-registered (CLAIM + CRAC_P6C_REGISTRY_ANALYSIS.md §4)

## Result: PARTIAL — layer-B remedy DYNAMICALLY CONFIRMED; layer-A policy file rejected on syntax (parser decoded post-run); boot-cap consumed => attempt 9b = S7-71

### WINS
1. **ANON-SWEEP closed=12** (eventpoll/eventfd/timerfd) => suppressed list has ZERO anon_inode entries for the first time ever (was 6). LAW P6B-12 layer B DYNAMICALLY CONFIRMED: unclaimed native-scan fds are cleared by JNI close in agent beforeCheckpoint.
2. Netty close stable (netty=1, ms=44), PORT-CLEAR both false, dup-guard OK, AR-ORG/AR-RAW fired (refusal unwind, P6B-9 reconfirmed).
3. **Parser decoded** (javap loadPolicies): lines are `key: value` (COLON, not comma-k=v); `#` comments; `---` line = rule separator (blank lines do NOT flush); keys type/action required, rest => params map. My v8 file used comma format => ConfigurationException "cannot parse line 1" => NO policies applied => all layer-A entries kept default action=error.

### Suppressed set AFTER v8 surgery (new full view, tail no longer truncating)
- ./world*/session.lock x3, logs/latest.log, purpur jar — layer A (policy-owned; will clear with correct syntax)
- 25575 socket — layer A (policy-owned)
- NEW: `FD fd=154 type=regular path=/home/z/server/plugins/spark/tmp/spark-*.jfr.tmp` (spark profiler) — layer B native scan
- NEW: `FD fd=155 type=directory path=/proc/<pid>/task` — layer B native scan (spark/JFR thread walker)
- anon_inode — GONE (sweep)

## Attempt 9b design (S7-71, pre-registered here)
1. policies.txt (COLON format, --- separators):
   type: file / action: close / path: logs/latest.log --- (x absolute variant)
   type: file / action: close / path: world|world_nether|world_the_end/session.lock (+ ./ variant) ---
   type: file / action: reopen / path: /home/z/server/versions/** ---
   type: socket / action: close / localPort: 25575 ---
2. anonInodeSweep v2 = close targets: `anon_inode:*` OR contains `/spark/` OR starts `/proc/` (two-pass: snapshot+resolve first, close after iteration; /proc/self resolves to /proc/<pid> — closing own iteration dir fd after listing is harmless)
3. Same launch flag -Djdk.crac.resource-policies. Expected: ZERO suppressions => img>0 => restore x2 + prize vs 13.2s floor.

## Discipline
1 boot (17.3s), hs_err 4/0, ports clean, 0 config touch (policies = rig-local file + rig flag), BENCH journal clean.
