#!/usr/bin/env python3
"""dispatch_427a2.py — TASK-427-A2: legs x3 @round-427-a2-mobsoa lever=cmp424_mobfeed
+ vanilla anchors x3 @790dc2f lever='' (fresh same-window pairs).
Usage: python3 dispatch_427a2.py legs | anchors | all
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = {
    "a2-l1": ("round-427-a2-mobsoa", "cmp424_mobfeed"),
    "a2-l2": ("round-427-a2-mobsoa", "cmp424_mobfeed"),
    "a2-l3": ("round-427-a2-mobsoa", "cmp424_mobfeed"),
}
ANCHORS = {
    "a2-anchora": ("round-427-a2-anchora", ""),
    "a2-anchorb": ("round-427-a2-anchorb", ""),
    "a2-anchorc": ("round-427-a2-anchorc", ""),
}

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


def resolve_branch_sha(tok, branch):
    d = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return d["object"]["sha"]


def dispatch(tok, leg, branch, lever):
    sha = resolve_branch_sha(tok, branch)
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": branch, "inputs": inputs})
    print(f"dispatched {leg}: branch={branch} lever='{lever}' sha={sha[:8]}")
    return sha


def main():
    which = sys.argv[1] if len(sys.argv) > 1 else "all"
    tok = token_from_remote()
    if which in ("legs", "all"):
        for leg, (branch, lever) in LEGS.items():
            dispatch(tok, leg, branch, lever)
            time.sleep(4)
    if which in ("anchors", "all"):
        for leg, (branch, lever) in ANCHORS.items():
            dispatch(tok, leg, branch, lever)
            time.sleep(4)


if __name__ == "__main__":
    main()
