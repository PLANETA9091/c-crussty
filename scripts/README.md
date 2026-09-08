# scripts/ — build & profiling tooling

| script | purpose |
|---|---|
| `profile_server.sh` | one-shot JFR profiling harness for the live Purpur+CRUSSTY server |
| `flame_diff.py` | self-time delta (regressions / improvements) between two JFR runs |
| `build_noise.sh` | rebuild the noise bridge classes (see header of that file) |
| `rcon.py` | minimal vanilla-protocol RCON client (TASK-75: password is NEVER positional — `--password-file` / env `CRUSSTY_RCON_PASSWORD` / default `/home/z/.rcon_password`, chmod 600, вне репо; see docs/RCON_HYGIENE_DECISION.md) |
| `rcon_verify_throwaway.sh` | end-to-end rotation verification on a dormant flat-world throwaway (new-secret round-trip + burned-secret rejection + clean stop; polite to a concurrent timing lane, takes no BENCH.lock) |

No sudo is required anywhere: JFR attach works for same-user JVMs.

## 0. Prerequisites

A **full JDK 21** (a plain JRE has no `jcmd` / `jfr`). The harness looks for it at
`/home/z/jdk21`, then `$JAVA_HOME`, then `PATH` — override with `--jdk PATH`
(or `PROFILE_JDK`). Bootstrap it fresh (e.g. after a sandbox reset):

```bash
mkdir -p /home/z/jdk21
curl -sSL "https://api.adoptium.net/v3/binary/latest/21/ga/linux/x64/jdk/hotspot/normal/eclipse" \
  | tar xz --strip-components=1 -C /home/z/jdk21
/home/z/jdk21/bin/jcmd --version   # sanity
```

## 1. Boot the server (recap)

The server setup lives in `/home/z/server` (recreate with `crussty init --dir /home/z/server`
if the sandbox was reset — needs Purpur 1.21.10 + CRUSSTY `launcher.jar` + eula + `online-mode=false`).

```bash
cd /home/z/server
setsid /home/z/jdk21/bin/java -Xms1G -Xmx2G -jar launcher.jar --nogui \
  > server_console.log 2>&1 &
tail -f server_console.log          # wait for "Done"
```

`profile_server.sh` auto-detects that JVM via `pgrep -f 'launcher.jar|purpur|paper'`
(filtered to real java executables). If several java processes match, it lists
them and exits — pick one with `--pid PID` (or `SERVER_PID=...`).

## 2. Profile

```bash
scripts/profile_server.sh --duration 120 --output profiles/idle
```

What happens: PID detect → `jcmd <pid> JFR.start settings=profile duration=120s filename=...`
→ wait → quick triage (`jfr summary`, top hot methods, top allocation sites).

Artifacts written to `--output DIR`, prefix `profile_<ts>`:

| file | contents |
|---|---|
| `.jfr` | the recording (open in JMC / re-print any time) |
| `.summary.txt` | `jfr summary` dump (event counts) |
| `.hot_methods.csv` | `method,samples` — **self time** (leaf frame of every ExecutionSample); input for `flame_diff.py` |
| `.alloc.csv` | `class,~bytes` — sampled allocation weight (`jdk.ObjectAllocationSample`) |
| `.meta.txt` | pid, cmdline, duration, workload, jdk |
| `.jcmd.log` | raw jcmd output (diagnostics) |
| `.execution_samples.txt` | raw `jfr print` dump, only with `--keep-raw` (large) |
| `.workload.log` | stdout/stderr of `--workload CMD`, if used |

Options that matter:

```bash
--mode jcmd          # open-ended recording, stopped explicitly (vs duration-bounded 'jfr')
--workload 'CMD'     # run CMD during the recording window (chunk-load burst below)
--pid PID            # explicit target JVM
--keep-raw           # also emit the full jfr-print text dump
--no-triage          # just record, no summary/hot/alloc printing
```

Exit codes: `0` ok, `1` usage, `2` no jcmd/jfr (install the JDK above),
`3` server PID not found / ambiguous, `4` jcmd control failed,
`5` recording file missing.

## 3. Workloads

Anything a shell can run. It executes **while the recording is open**, gets its
output captured to `.workload.log`, and is TERM'd right after the window.

Chunk-load burst (if rcon is enabled on the server — TASK-75 hygiene: the secret is
NOT passed on the command line; keep it in `/home/z/.rcon_password` (chmod 600) or
pass `--password-file`, see `docs/RCON_HYGIENE_DECISION.md`; the example uses the
hardened `rcon.py` instead of `mcrcon -w <pw>` to avoid ps-exposure):

```bash
scripts/profile_server.sh --duration 60 --output profiles/chunkburst \
  --workload 'scripts/rcon.py 127.0.0.1 25575 "forceload add -512 -512 512 512"'
```

No-rcon fallbacks: a headless client/bot walking new terrain, `forceload` via a
command block + scheduled function, or simply re-starting the server (boot = the
heaviest chunk-load burst) and profiling it with `--pid`.

## 4. Diff two runs

```bash
# baseline (idle) vs profile (burst):
python3 scripts/flame_diff.py profiles/idle/profile_*.jfr profiles/chunkburst/profile_*.jfr
# or compare the CSVs directly (no jfr binary needed):
python3 scripts/flame_diff.py profiles/idle/profile_*.hot_methods.csv \
                              profiles/chunkburst/profile_*.hot_methods.csv --out CHUNK_DIFF.md
```

`flame_diff.py` accepts `*.hot_methods.csv`, raw `jfr print --events jdk.ExecutionSample`
dumps, or `.jfr` files directly (auto-detects per input; needs a `jfr` binary for
`.jfr`). A `jfr summary` dump alone has no per-method data and is rejected.
It prints the top-30 REGRESSIONS and top-30 IMPROVEMENTS of self time, ranked by
**share of total samples** (pp = percentage points) so runs of different lengths
stay comparable; raw sample deltas and `new`/`gone` markers are shown too.
`--top N` changes table size.

Method names are normalized (signatures and `(Native Method)` suffixes stripped,
`$$Lambda+42/0x7f...` collapsed) so overloads and lambdas aggregate across runs.

Self-test (must always pass, stdlib only):

```bash
python3 scripts/flame_diff.py --self-test
```

## 5. Full workflow, end to end

```bash
# 1. boot
cd /home/z/server && setsid /home/z/jdk21/bin/java -Xms1G -Xmx2G -jar launcher.jar --nogui > console.log 2>&1 &

# 2. profile idle
scripts/profile_server.sh --duration 120 --output profiles/idle

# 3. profile chunk-load burst
scripts/profile_server.sh --duration 60 --output profiles/chunkburst \
  --workload '<rcon forceload / bot walk / client join>'

# 4. diff
python3 scripts/flame_diff.py profiles/idle/profile_*.hot_methods.csv \
                              profiles/chunkburst/profile_*.hot_methods.csv --out results/IDLE_VS_BURST.md

# 5. eyeball a single recording without diffing
jfr summary profiles/idle/profile_*.jfr
jfr print --events jdk.ExecutionSample profiles/idle/profile_*.jfr | less
```

Interpretation tips for this project: hot frames under `net.minecraft.*` are
kernel cost; frames referencing `PaperNative*` / crussty bridge classes show the
injected JNI surface; a wall of `jdk.internal.misc.Unsafe.park` = idle threads,
not a real regression — filter by share, not raw counts.
