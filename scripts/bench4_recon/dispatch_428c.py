#!/usr/bin/env python3
"""dispatch_428c.py — ROUND-428-C (law-8 chunk-ось): 3 legs cmp428_chunkunion
@round-428-c-chunkunion (408a94a) + 2 fresh vanilla anchors @master dc704c9."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
CU_SHA = None  # resolved from origin/round-428-c-chunkunion at runtime

RUNS = [
    # (branch_alias, kind, lever)  kind: cu=union carrier, anchor=vanilla master
    ("round-428-cu-l1", "cu", "cmp428_chunkunion"),
    ("round-428-cu-l2", "cu", "cmp428_chunkunion"),
    ("round-428-cu-l3", "cu", "cmp428_chunkunion"),
    ("round-428-anchorf", "anchor", ""),
    ("round-428-anchorg", "anchor", ""),
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
    global CU_SHA
    CU_SHA = api(tok, f"/repos/{REPO}/git/ref/heads/round-428-c-chunkunion")["object"]["sha"]
    master = api(tok, f"/repos/{REPO}/git/ref/heads/master")["object"]["sha"]
    print(f"chunkunion @ {CU_SHA[:8]}, master @ {master[:8]}")
    for branch, kind, lever in RUNS:
        sha = CU_SHA if kind == "cu" else master
        ensure_branch_at(tok, branch, sha)
        time.sleep(3)
        dispatch(tok, branch, lever)
        time.sleep(4)


if __name__ == "__main__":
    main()
