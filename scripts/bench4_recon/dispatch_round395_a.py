#!/usr/bin/env python3
"""dispatch_round395_a.py — TASK-395 mega-round, agent-A lever `items_index`.

Dispatches world-bench-parallel.yml on ref=round-395-a-merge-index with the
bank-v4 input set + lever_flag=items_index (lever_arg=1). Inputs are the
round-mandated EXACT dict (world_url/natives_url defaults omitted; the
removed travel_diet/fluid_dirty_ledger slots are NOT sent — they no longer
exist in the workflow). No concurrency guard by round policy (per-ref
concurrency is built into world-bench-parallel.yml).
"""
import json, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
BRANCH = "round-395-a-merge-index"


def token():
    with open("/tmp/gh_token") as f:
        return f.read().strip()


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
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {}


def main():
    tok = token()

    local = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-395/agent-A",
                            "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/{BRANCH}").get("sha", "")
    if not remote or remote[:12] != local[:12]:
        print(f"branch head mismatch: local {local[:12]} != remote {remote[:12] or 'MISSING'} — push first")
        return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "4",
        "fluid_guard": "1",
        "gc_tune": "3",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "fluid_bitmask": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "inside_bitmask": "0",
        "skip_store_bb": "0",
        "region_steal": "0",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "cpu_band_min": "6000000",
        "cpu_band_max": "9500000",
        "lever_flag": "items_index",
        "lever_arg": "1",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench-parallel.yml/dispatches",
        method="POST", data={"ref": BRANCH, "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(24):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench-parallel.yml"
                     f"/runs?head_branch={BRANCH}&per_page=1")
        runs = d.get("workflow_runs", [])
        if runs:
            r = runs[0]
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
            return 0
    print("no run appeared after 120s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
