#!/usr/bin/env python3
"""dispatch_436c.py — TASK-436-C: V3 (cmp435_chunk3) legs + own anchors.
Interleaved big-batch: anchor, leg, anchor, leg, leg (ONE window).
Legs  = @round-436-c-chunk3 (== round-435-c-chunkpl2 @b64b189 + R6 carrier),
        lever=cmp435_chunk3 (STRICT-OR over cmp434_chunkpl).
Anchors = @master lever="" (window supply for pair-hunt)."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BATCH = [
    ("anchor-1",  "round-436c-anchor-1", "master",              ""),
    ("chk3-1",    "round-436c-chk3-1",   "round-436-c-chunk3",  "cmp435_chunk3"),
    ("anchor-2",  "round-436c-anchor-2", "master",              ""),
    ("chk3-2",    "round-436c-chk3-2",   "round-436-c-chunk3",  "cmp435_chunk3"),
    ("chk3-3",    "round-436c-chk3-3",   "round-436-c-chunk3",  "cmp435_chunk3"),
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


if __name__ == "__main__":
    tok = token_from_remote()
    for leg, branch, base, lever in BATCH:
        sha = ensure_branch(tok, branch, base)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
