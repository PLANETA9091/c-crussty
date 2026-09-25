#!/usr/bin/env python3
"""dispatch_456b_fix.py — TASK-456-B step-5: re-rolls (≤2) of the cmp456_poi
carrier AFTER the NCDFE fix (5ecd841a: EntityGoalQueryOps define gate carries
cmp456_poi; fa9054d9 arm-AFTER-define canon).

2 legs from the FIXED base round-456b-poi:
  poi456-3 -> round-456b-poi-3
  poi456-4 -> round-456b-poi-4
Canonical inputs identical to dispatch_456b.py (canon tick-456).
Usage: dispatch_456b_fix.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BATCH = [
    ("poi456-3", "round-456b-poi-3", "round-456b-poi", "cmp456_poi"),
    ("poi456-4", "round-456b-poi-4", "round-456b-poi", "cmp456_poi"),
]

EXPECTED_SHA = {
    "round-456b-poi": "5ecd841a",  # step-4 NCDFE fix (define gate carries cmp456_poi)
}
ANCESTRY_BASES = set()

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
    url = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-456/agent-b2",
                          "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    return re.match(r"^https://[^:]+:([^@]+)@github.com/", url).group(1)


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
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ancestry_ok(tok, pin, ref):
    try:
        cmp = api(tok, f"/repos/{REPO}/compare/{pin}...{ref}")
        return cmp.get("status") in ("ahead", "identical")
    except Exception:
        return False


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


def main():
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print("usage: dispatch_456b_fix.py [--dry-run]")
        raise SystemExit(0)
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token_from_remote()
    print("=== TASK-456-B POI CARRIER RE-ROLL DISPATCH (post NCDFE fix 5ecd841a) ===", flush=True)
    bases = {b for _, _, b, _ in BATCH}
    for base in sorted(bases):
        exp = EXPECTED_SHA.get(base)
        if exp is None:
            raise SystemExit(f"NO PIN for base {base}")
        live = sha_of(tok, base)[:8]
        if base in ANCESTRY_BASES:
            if not ancestry_ok(tok, exp, base):
                raise SystemExit(f"ANCESTRY MISMATCH: {base} live={live} pin={exp}")
            print(f"preflight OK (ancestry): {base} @ {live}", flush=True)
        elif not live.startswith(exp):
            raise SystemExit(f"SHA MISMATCH: {base} live={live} expected={exp}")
        else:
            print(f"preflight OK: {base} @ {live}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for leg, branch, base, lever in BATCH:
        sha = ensure_branch(tok, branch, base)
        if not sha.startswith(EXPECTED_SHA[base]):
            raise SystemExit(f"LEG BRANCH {branch} not at pinned base: {sha[:8]}")
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: {branch} <- {base} lever='{lever}' sha={sha[:8]}",
              flush=True)
        time.sleep(4)
    print(f"=== TASK-456-B RE-ROLL BATCH COMPLETE: {len(BATCH)} dispatches ===", flush=True)


if __name__ == "__main__":
    main()
