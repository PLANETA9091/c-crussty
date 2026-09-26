#!/usr/bin/env python3
"""dispatch_463_canary.py — TASK-463-88a canary: 1 run on the repaired master
(60dfd806: java pipe-literal repair, CP-EXACT gate). Bank-canon inputs,
NO cpu_band (canary-401 no-band protocol, band pool dead), empty lever_flag
(vanilla path — repair must be behavior-neutral: standalone gates arm only on
real flags). Usage: scripts/dispatch_463_canary.py [--dry-run]"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

CANARY_BRANCH = "round-463-pipe-repair-canary"
EXPECTED_MASTER_SHA = "60dfd806"  # repair x463 commit

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    # NO cpu_band_* — canary-401 no-band protocol (norm on absorb)
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


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


def ensure_branch(tok, branch, base_sha):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


def main():
    dry = "--dry-run" in sys.argv[1:]
    tok = token()
    master = sha_of(tok, "master")[:8]
    if not master.startswith(EXPECTED_MASTER_SHA):
        raise SystemExit(f"SHA MISMATCH: origin/master live={master} expected={EXPECTED_MASTER_SHA}")
    print(f"preflight OK: origin/master @ {master} (repair x463)", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    sha = ensure_branch(tok, CANARY_BRANCH, sha_of(tok, "master"))
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": CANARY_BRANCH, "inputs": INPUTS})
    print(f"dispatched canary: {CANARY_BRANCH} <- {sha[:8]} (bank-canon, no band, no lever)", flush=True)
    time.sleep(8)
    runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs"
                    f"?event=workflow_dispatch&head_branch={CANARY_BRANCH}&per_page=1")
    if runs.get("workflow_runs"):
        r = runs["workflow_runs"][0]
        print(f"RUN ID: {r['id']} status={r['status']} created={r['created_at']} head_sha={r['head_sha'][:8]}", flush=True)
    else:
        print("RUN ID: not yet visible — re-poll runs list", flush=True)


if __name__ == "__main__":
    main()
