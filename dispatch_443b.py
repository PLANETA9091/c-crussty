#!/usr/bin/env python3
"""dispatch_443b.py — TASK-443-B: 3 ноги mega-композиции (cmp443_mega) на
носителе round-443-mega (ins4d ⊕ chunk4 юнион-мердж bca9499a).
INPUTS РОВНО по брифу (travel_diet/fluid_dirty_ledger НЕ слать;
concurrency-гвардов НЕ ставить). sleep 4 между диспатчами."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BATCH = [
    ("b443b-1", "round-443-mega-1", "round-443-mega", "cmp443_mega"),
    ("b443b-2", "round-443-mega-2", "round-443-mega", "cmp443_mega"),
    ("b443b-3", "round-443-mega-3", "round-443-mega", "cmp443_mega"),
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
MEGA_SHA = "bca9499ad54cae59b957466d9447b7ae8953d951"


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
    if subprocess.run(["git", "-C", "/home/z/rounds/ROUND-443/agent-b", "merge-base", "--is-ancestor",
                       MEGA_SHA, sha]).returncode != 0:
        raise SystemExit(f"SHA MISMATCH preflight: {branch} @ {sha[:8]} lacks mega merge {MEGA_SHA[:8]}")
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": branch, "inputs": inputs})
    print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
    return sha


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "dry":
        print("DRY-RUN plan:", BATCH, "inputs:", len(INPUTS) + 2, "keys"); sys.exit(0)
    tok = token_from_remote()
    live = sha_of(tok, "round-443-mega")
    if live != MEGA_SHA:
        raise SystemExit(f"live origin/round-443-mega {live[:8]} != expected mega sha {MEGA_SHA[:8]} — abort")
    print(f"preflight OK: origin/round-443-mega @ {live[:8]} == mega merge commit")
    for leg, branch, base, lever in BATCH:
        dispatch(tok, leg, branch, base, lever)
        time.sleep(4)
