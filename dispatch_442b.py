#!/usr/bin/env python3
"""dispatch_442b.py — TASK-442-B: 2 ноги ins4-диеты (cmp440_ins4d) поверх
носителя round-436-b-ins6 @c57d2aa8 (диета c2ed1090 + STRICT-OR гейты + dispatcher).
INPUTS РОВНО по брифу (travel_diet/fluid_dirty_ledger НЕ слать;
concurrency-гвардов НЕ ставить). sleep 4 между диспатчами."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BATCH = [
    ("b442b-1", "round-442b-ins4d-1", "round-442-b-ins4d", "cmp440_ins4d"),
    ("b442b-2", "round-442b-ins4d-2", "round-442-b-ins4d", "cmp440_ins4d"),
]

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}
# DIET_SHA = the diet-code commit every leg MUST contain (ancestor check —
# stable against further bookkeeping commits on the branch).
DIET_SHA = "c2ed10909cf76f91d9105b8bbbdfafbed6b96ad2"


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


def dispatch(tok, leg, branch, base, lever):
    sha = ensure_branch(tok, branch, base)
    if subprocess.run(["git", "-C", "/home/z/rounds/ROUND-442/agent-b", "merge-base", "--is-ancestor",
                       DIET_SHA, sha]).returncode != 0:
        raise SystemExit(f"SHA MISMATCH preflight: {branch} @ {sha[:8]} lacks diet sha {DIET_SHA[:8]}")
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": branch, "inputs": inputs})
    print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
    return sha


if __name__ == "__main__":
    tok = token_from_remote()
    live = sha_of(tok, "round-442-b-ins4d")
    if subprocess.run(["git", "-C", "/home/z/rounds/ROUND-442/agent-b", "merge-base", "--is-ancestor",
                       DIET_SHA, live]).returncode != 0:
        raise SystemExit(f"diet code sha {DIET_SHA[:8]} is NOT an ancestor of live base — abort")
    print(f"preflight OK: origin/round-442-b-ins4d @ {live[:8]} contains diet sha {DIET_SHA[:8]}")
    print(f"preflight OK: origin/round-442-b-ins4d @ {live[:8]}")
    for leg, branch, base, lever in BATCH:
        dispatch(tok, leg, branch, base, lever)
        time.sleep(4)
