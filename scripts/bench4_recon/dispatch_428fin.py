#!/usr/bin/env python3
"""dispatch_428fin.py — ROUND-428-FIN: re-roll пачка (2 ноги A2 @fe4ee57 + 2 якоря @master dc704c9)."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
A2_SHA = "fe4ee5774180c4b909a6936257d3ab5b0b958e15"  # winner carrier (round-427-a2-mobsoa code)
MASTER_SHA = None       # resolved at runtime

RUNS = [
    # (branch_alias, sha_ref, lever)
    ("round-428-a2-l6", "a2", "cmp424_mobfeed"),
    ("round-428-a2-l7", "a2", "cmp424_mobfeed"),
    ("round-428-anchord", "master", ""),
    ("round-428-anchore", "master", ""),
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
        print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def ensure_branch_at(tok, branch, sha):
    try:
        d = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
        cur = d["object"]["sha"]
        print(f"branch {branch} exists @ {cur[:8]}")
        return cur
    except urllib.error.HTTPError:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": sha})
        print(f"branch {branch} created @ {sha[:8]}")
        return sha


def dispatch(tok, branch, lever):
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": branch, "inputs": inputs})
    print(f"dispatched branch={branch} lever='{lever}'")


def main():
    tok = token_from_remote()
    global MASTER_SHA
    MASTER_SHA = api(tok, "/repos/PLANETA9091/c-crussty/git/ref/heads/master")["object"]["sha"]
    print(f"master @ {MASTER_SHA[:8]}")
    for branch, kind, lever in RUNS:
        sha = A2_SHA if kind == "a2" else MASTER_SHA
        ensure_branch_at(tok, branch, sha)
        time.sleep(3)
        dispatch(tok, branch, lever)
        time.sleep(4)


if __name__ == "__main__":
    main()
