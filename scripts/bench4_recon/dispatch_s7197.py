#!/usr/bin/env python3
"""dispatch_s7197.py - S7-172 P2-OFFLOAD v1 leg (TASK-371), RECON-37 fork:
OFFLOAD-READY branch (I = 1.01 <= 1.15).

EVIDENCE CHAIN (all digest-verified, 5-tick token blocker broken 2026-09-20):
  s7196 re-roll run 35488526730 artifact 10598539161
  sha256 98ae00f6e748f19e346e5e7cfe9e0ee00b43e9aeeaa0f16b904e8e3bea3e498d
  -> RECON-37 threaded wall (parser fix: '[name tid=N]' canonicalization):
     worker duties 67.4/66.3/68.2%, I = max/avg = 1.01 (prereg RECON-36 §4:
     I<=1.15 -> OFFLOAD-READY, ceiling +25..33% wall-clock > ДВОЙНОЙ БАР);
     main: active-entity 62.0%, park 12.6% (slot-0 bucket + serial phases).

LEVER (S7-172): CRUSSTY_REGION_STEAL="2" = MAIN-OFFLOAD static mode —
region_threads=4 helpers tick ALL 4 buckets (helper i -> slot i-1), main
orchestrates only (snapshot fill -> GO release -> DONE join -> phase-4/4b).
Critical path re-targets from main-total (19.7 units) to the slowest worker
(14.9 units at I=1.0). Parity: bucketOf unchanged, per-bucket order =
snapshot order, GO/DONE discipline unchanged (barriers w+1 parties);
slot-0 mid-tick pump suppression = the same S7-157b class already accepted
for slots 1..3. Rides the REGION_STEAL input enum (workflow at the 25-input
GitHub cap). Config = bank v3 otherwise: inside_cache=1, flush_diet=1,
region_threads=4, batch_collector=1, travel_diet=0, region_steal=2(!),
bu_defer=0.

PROTOCOL v8-REGRESSION: WIDE BAND 6.0M..9.5M (cpu_band_min/max); verdict by
DUAL BAR vs ANCHOR-SLOW (s7184: median5 1.60 @ 6680195): normalized
(med/runner)/(1.60/6680195) AND absolute med/1.60, BOTH >= +10% ->
CANDIDATE GREEN -> confirming min-of-2 leg -> banking v4 = v3 + main_offload.
BAND-DISCARD = not a verdict (re-roll). Crash -> rollback default 0
(bit-identical legacy path) + verdict doc.

ABSORB: adapt absorb_s7196.py -> absorb_s7197.py (expect region_steal: 2 in
PG-A run-env; RECON-37 fork-confirmation gate: re-run worker-balance on the
new threaded wall if present).
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
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

    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

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
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "travel_diet": "0",
        "inside_bitmask": "0",
        "skip_store_bb": "0",
        "region_steal": "2",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "0",
        "cpu_band_min": "6000000",
        "cpu_band_max": "9500000",
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
