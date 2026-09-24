#!/usr/bin/env python3
"""dispatch_440c.py — TASK-440-C: chunk4 snapshot verdict legs ×2.
Legs = @round-438-c-chunk4b (chunk3 carrier b64b189a + cmp437_chunk4
chunk-send serialization snapshot), lever=cmp437_chunk4 (STRICT-OR over
cmp435_chunk3 — composite parse planes + send-snapshot plane).
Anchors = NOT dispatched here (main cron round-440-* batch supplies the
window; чужие имена не трогаем). Inputs = bank EXACT (21 keys)."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BATCH = [
    ("chunk4-1", "round-440c-chunk4-1", "round-438-c-chunk4b", "cmp437_chunk4"),
    ("chunk4-2", "round-440c-chunk4-2", "round-438-c-chunk4b", "cmp437_chunk4"),
]

EXPECTED_BASE_SHA_PREFIX = "e93bb8c2"  # step-2 tip (impl + gates green)

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


def token_from_gh_token_file():
    with open("/tmp/gh_token") as f:
        tok = f.read().strip()
    if not tok:
        raise SystemExit("/tmp/gh_token empty")
    return tok


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
    if "--dry-run" in sys.argv:
        for leg, branch, base, lever in BATCH:
            print(f"PLAN {leg}: branch={branch} base={base} lever='{lever}'")
        print("inputs:", json.dumps(INPUTS, sort_keys=True))
        sys.exit(0)
    tok = token_from_gh_token_file()
    base_sha = sha_of(tok, "round-438-c-chunk4b")
    if not base_sha.startswith(EXPECTED_BASE_SHA_PREFIX):
        raise SystemExit(f"SHA-DRIFT ABORT: round-438-c-chunk4b = {base_sha}, expected {EXPECTED_BASE_SHA_PREFIX}*")
    for leg, branch, base, lever in BATCH:
        sha = ensure_branch(tok, branch, base)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
