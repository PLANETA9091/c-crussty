#!/usr/bin/env python3
"""dispatch_432b.py — TASK-432-B: inside-plane DEEPENING leg(s) on
round-432-b-inside2 @5bdb4d2 (cmp432_inside2 STRICT-OR cmp430_inside).
Inputs bank = canon tick-432 (identical to dispatch_432.py; lever/arg differ)."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# (leg, ref, lever) — ref = the code branch itself (owner prompt: ref=round-432-b-inside2)
BATCH = [
    ("432b-inside2-l1", "round-432-b-inside2", "cmp432_inside2"),
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


def dispatch(tok, leg, ref, lever):
    sha = sha_of(tok, ref)
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": ref, "inputs": inputs})
    print(f"dispatched {leg}: ref={ref} lever='{lever}' sha={sha[:8]}", flush=True)
    return sha


def main():
    tok = token_from_remote()
    for leg, ref, lever in BATCH:
        dispatch(tok, leg, ref, lever)
        time.sleep(4)


if __name__ == "__main__":
    main()
