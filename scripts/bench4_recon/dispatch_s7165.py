#!/usr/bin/env python3
"""dispatch_s7165.py — dispatch the X150K INSTRUMENTED RECON leg (TASK-317,
instrument-gate for lever #13 SKIP-STORE-DIET) on top of BANKED CUMULATIVE v3.

The leg config is the EXACT v3 bank (run 35399980345: inside_cache=1 +
flush_diet=1 + region_threads=4 + batch_collector=1, everything else 0,
fp4/300s/150k/seed42/xmx10G, radius 640) PLUS recon_diag=1 — pure
observability, 0 behavior change:
  - -Xlog:gc+remset=debug / gc+refine=debug: aggregate old->young card
    intensity per GC cycle (refines the TASK-316 estimate 4-6M/s).
    Logging is NOT a config-win: GC policy/heap are untouched (TASK-316
    NEXT verdict).
  - -XX:StartFlightRecording settings=profile (Temurin 21, smoke-tested):
    jdk.OldObjectSample IS ACTIVE in profile.jfc (memory-leaks default =
    stack-traces) -> objects reaching the OLD GEN with their ALLOCATION
    STACK = direct producer attribution of the old->young store-firehose
    (setDeltaMovement/setBoundingBox/sync chains; C2-inlined setters make
    async-profiler alloc stacks unreliable — mirrored lesson #8).
    jdk.ObjectAllocationSample (300/s) = control alloc profile.
  - This leg is NOT a gate leg: JFR overhead shifts CPU/TPS — its numbers
    are NEVER used for PG2/PG3/PG4-style gates or banking. Sole purpose:
    measure the share of entity-field producers among old->young stores.
    Decision rule (preregistered in CLAIMS TASK-316 NEXT): share >= 40%
    AND model skip ceiling >= 2-3% wall -> GO #13 (preregister + lockstep
    oracle >= 1M + identity-use grep); otherwise -> paper-REFUTED #13.

Workflow: world-bench.yml input recon_diag (added TASK-317) -> env
RECON_DIAG -> run_world3.sh EXTRA_JVM_DIAG array (bash array, not string:
word-splitting-safe; lessons #5-class delivery hygiene: smoke-tested flags
locally on Temurin 21.0.12.1 — JFR starts, remset debug lines present,
jfr metadata shows both event types).
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
        "summon_sweeps": "0",
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
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
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
