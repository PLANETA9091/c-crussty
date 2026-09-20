#!/usr/bin/env python3
"""dispatch_round395_j.py — agent J (TASK-395, vector items_manager) bench dispatch.

Workflow: world-bench-parallel.yml (per-ref concurrency, 10 параллельных
ранов), ref=round-395-j-manager, lever_flag=items_manager, lever_arg=1.
Банк v4 без изменений: inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1 + gc_tune=3 (ParallelGC); радиус 640,
seconds 300, fake_players 4, pop 150000 seed 42, xmx 10G/xms 4G,
band 6.0M..9.5M. world_url/natives_url/travel_diet/fluid_dirty_ledger
НЕ слаются (freed slots).

Запуск: python3 scripts/bench4_recon/dispatch_round395_j.py
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
BRANCH = "round-395-j-manager"
WORKFLOW = "world-bench-parallel.yml"
WORKTREE = "/home/z/rounds/ROUND-395/agent-J/repo"
TOKEN_FILE = "/tmp/gh_token"


def token():
    try:
        tok = open(TOKEN_FILE).read().strip()
        if tok:
            return tok
    except OSError:
        pass
    url = subprocess.run(["git", "-C", WORKTREE, "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token")
    return m.group(1)


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

    local = subprocess.run(["git", "-C", WORKTREE, "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/{BRANCH}").get("sha", "")
    if not remote or remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/{WORKFLOW}/runs"
                 f"?head_branch={BRANCH}&per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
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
        "lever_flag": "items_manager",
        "lever_arg": "1",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/{WORKFLOW}/dispatches",
        method="POST", data={"ref": BRANCH, "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/{WORKFLOW}/runs"
                     f"?head_branch={BRANCH}&per_page=1")
        runs = d.get("workflow_runs", [])
        if runs:
            r = runs[0]
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "head_branch": r["head_branch"],
                              "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
            return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
