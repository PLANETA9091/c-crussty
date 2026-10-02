#!/usr/bin/env python3
"""dispatch_434c.py — TASK-434-C ROUND-434 CHUNK-PIPELINE R5 BATCH-1:
4 anchors @master (lever="") + 3 chunkpl legs @round-434-c-chunkpl
(lever=cmp434_chunkpl: biomes-parse cache port + block_states deep cache on
the full composite union), ONE window, interleaved. INPUTS = банк тика-434
(НЕ ТРОГАТЬ)."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# (leg, branch, base_branch, lever) — interleaved anchor/leg
BATCH = [
    ("anchor-t4c",  "round-434-anchor-t4c",  "master",               ""),
    ("chk-1",       "round-434c-chk-1",      "round-434-c-chunkpl",  "cmp434_chunkpl"),
    ("anchor-u4c",  "round-434-anchor-u4c",  "master",               ""),
    ("chk-2",       "round-434c-chk-2",      "round-434-c-chunkpl",  "cmp434_chunkpl"),
    ("anchor-v4c",  "round-434-anchor-v4c",  "master",               ""),
    ("chk-3",       "round-434c-chk-3",      "round-434-c-chunkpl",  "cmp434_chunkpl"),
    ("anchor-w4c",  "round-434-anchor-w4c",  "master",               ""),
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
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": branch, "inputs": inputs})
    print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
    return sha


def main():
    tok = token_from_remote()
    for leg, branch, base, lever in BATCH:
        dispatch(tok, leg, branch, base, lever)
        time.sleep(4)


if __name__ == "__main__":
    main()
