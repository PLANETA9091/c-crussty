#!/usr/bin/env python3
"""dispatch_s7169.py — dispatch the X150K INSTRUMENTED RECON-13d leg (TASK-328 fix-verification (s7168 PG-D0 FAIL: armDumpTask silently missed kernel-loader class — arm moved INTO define-closure with loud ARMED/FAILED stdout markers),
instrument-gate for the TOP-1 alloc lane: chunk-parse 33.38% of ap-samples,
RECON-13b/13e) on top of BANKED CUMULATIVE v3.

The leg config is the EXACT v3 bank (inside_cache=1 + flush_diet=1 +
region_threads=4 + batch_collector=1, everything else 0, fp4/300s/150k/seed42/
r640/xmx10G) PLUS parse_diag=1 — pure observability, 0 behavior change:
  - ChunkParseDiagOps bridge (kernel loader) + ldc-"xPos"-anchored retarget of
    the SINGLE CompoundTag.getIntOr site inside SerializableChunkData.parse.
    Receiver-prepended static call: verifier-visible stack shape identical;
    delegate bit-exact; note = O(1) ConcurrentHashMap merge, catch(Throwable).
  - Census file world3-run/chunk-parse-diag.txt: per-chunk load counts +
    SUMMARY (total/unique/repeat_share). DECISION RULE (preregistered):
    repeat_share >= 30% over the soak window -> GO lever #12 (decoded-chunk
    cache with revision invalidation, parity-gated); repeat_share < 10% ->
    cache REFUTED, lane is first-loads (ticket churn) -> different lever.
    10-30% -> extended RECON (per-phase census).
  - This leg is NOT a gate leg for TPS banking: the diag bridge shifts no
    CPU measurably, but the leg exists to MEASURE — parity gates (TPS
    not-worse, remset, NCDFE=0, CRASH-FREE) still apply as health checks.

Workflow: world-bench.yml inputs parse_diag (TASK-327) + recon_diag=1 (remset/refine/JFR = s7165-класс базы для PG-D2/PG-D3) -> env
PARSE_DIAG -> CRUSSTY_PARSE_DIAG + CRUSSTY_PARSE_DIAG_FILE=$WORK/chunk-parse-diag.txt
(run_world3.sh); artifact upload includes the census file.
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
    """Extract the PAT from the origin remote URL (rule 1b setup)."""
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL — run rule (1b) remote set-url first")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def main():
    tok = token_from_remote()

    # remote-head == local-head law (S7-164 dispatch incident): push BEFORE dispatch
    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    # concurrency guard: any in-flight world-bench run? (law S7-108)
    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "4",
        "fluid_guard": "1",
        "paletted_demux": "0",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_free": "0",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "flat_traversal": "0",
        "zero_alloc": "0",
        "skip_store_bb": "0",
        "parse_diag": "1",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "1",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=1")
        runs = d.get("workflow_runs", [])
        if runs:
            r = runs[0]
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
            return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
