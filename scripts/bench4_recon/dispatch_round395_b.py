#!/usr/bin/env python3
"""dispatch_round395_b.py — TASK-395 mega-round, agent B (items_stagger).

Диспатч world-bench-parallel.yml на ref=round-395-b-stagger с РОВНО
каноническими инпутами банка v4 (fp=4, pop 150000 seed 42, radius 640,
seconds 300, xmx 10G/xms 4G, band 6.0M..9.5M) + lever_flag=items_stagger,
lever_arg=1. Concurrency-гвардов НЕТ (per-ref concurrency в самом
workflow, до 10 параллельных ранов). Токен: /tmp/gh_token.

ДИСПАТЧ: python3 scripts/bench4_recon/dispatch_round395_b.py
"""
import json, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
BRANCH = "round-395-b-stagger"
WORKFLOW = "world-bench-parallel.yml"
TOKEN_FILE = "/tmp/gh_token"


def token():
    with open(TOKEN_FILE) as f:
        return f.read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"token {tok}", "Accept": "application/vnd.github+json"})
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
        "lever_flag": "items_stagger",
        "lever_arg": "1",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/{WORKFLOW}/dispatches",
        method="POST", data={"ref": BRANCH, "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(24):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/{WORKFLOW}/runs?head_branch={BRANCH}&per_page=3")
        for r in d.get("workflow_runs", []):
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "event": r["event"], "head_sha": r["head_sha"][:7],
                              "created": r["created_at"]}))
            return 0
    print("no run appeared after 120s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
