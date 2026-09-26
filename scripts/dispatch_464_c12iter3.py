#!/usr/bin/env python3
"""dispatch_464_c12iter3.py — TASK-464-33 chkclimb-12 iter-3 (P32-плечо) re-dispatch.

MARKER + CANON DISPATCHER. This branch is the P32-ARM leg of the chkclimb-12
multi-iteration wave on the cmp456_chunkmono_p31quant carrier (round-462-
chkclimb-12 @ c37e4d9d, P34 cert-fix tip).

ROUND CONTEXT (tick x464):
  - chkclimb-12 v5 = +17.64pp on the quant carrier (cmp456_chunkmono_p31quant,
    BANK-v5 threshold chkclimb-12 <= -2.57 @K12 + x464 re-measure +17.64).
  - iter-2 (round-464-c12-iter2, TASK-464-32) = same carrier, lever_arg=2
    (stagger N=2), IN FLIGHT at dispatch time of this leg — NOT pushed, so
    this leg bases on the canonical chkclimb-12 branch c37e4d9d per
    dispatch directive ("если iter-2 ещё не запушена").

HYPOTHESIS (iter-3 = P32-СОЛО на quant-носителе):
  lever_flag=cmp459_snapreg (P32 SNAP sidecar registry) alone, lever_arg=1.
  P32 does NOT require P31 (law-11 scaffold pairing is not a runtime
  dependency). Hypothesis: the quant carrier (cmp456_chunkmono_p31quant)
  preserves CHM structures such that the P32 snapreg sidecar achieves an
  INDEPENDENT capture of +0.5..+1.5pp on top of the +17.64pp baseline —
  i.e. P32-solo is additive on the quant carrier, unlike the P31-coupled
  legs (lever cmp456_chunkmono_p31snap) where snapreg capture is mediated
  by the INSIDE-BATCH bridge.

DISPATCH CANON (world-bench-parallel.yml):
  radius=640, seconds=300, fake_players=4, fluid_guard=1, gc_tune=3,
  inside_cache=1, flush_diet=1, region_threads=4, batch_collector=1,
  population_target=150000, population_seed=42, server_xmx=10G,
  server_xms=4G, cpu_band_min=6000000, cpu_band_max=9500000.
  Freed slots (travel_diet / fluid_dirty_ledger) NOT sent — pinned '0' in
  the workflow. fluid_dirty / fluid_bitmask / inside_bitmask /
  skip_store_bb / region_steal / bu_defer sent as '0' (canon INPUTS shape,
  dispatch_456c_reroll.py ancestry).

Usage: scripts/dispatch_464_c12iter3.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LABEL = "c12-iter3-p32solo"
BRANCH = "round-464-c12-iter3"
BASE_BRANCH = "round-462-chkclimb-12"
LEVER_FLAG = "cmp459_snapreg"
LEVER_ARG = "1"

# Ancestry pin: iter-3 = P32-solo leg off the P34 cert-fix tip of chkclimb-12.
EXPECTED_SHA = {
    BRANCH: "c37e4d9d",
}

INPUTS = {
    "lever_flag": LEVER_FLAG, "lever_arg": LEVER_ARG,
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
    return re.match(r"^https://[^:]+:([^@]+)@github.com/", url).group(1)


def api(method, path, token, payload=None):
    req = urllib.request.Request(
        f"{API}{path}", method=method,
        data=json.dumps(payload).encode() if payload is not None else None,
        headers={
            "Authorization": f"token {token}",
            "Accept": "application/vnd.github+json",
            "User-Agent": "c12iter3-dispatch",
        })
    with urllib.request.urlopen(req) as r:
        body = r.read().decode()
    return json.loads(body) if body else {}


def head_sha(token, ref):
    try:
        return api("GET", f"/repos/{REPO}/git/ref/heads/{ref}", token)["object"]["sha"]
    except urllib.error.HTTPError:
        return None


def main():
    if "--dry-run" not in sys.argv:
        pass  # argv-guard kept minimal: single-leg canon, no batch switch.
    token = token_from_remote()
    sha = head_sha(token, BRANCH)
    if sha is None:
        print(f"FAIL: branch {BRANCH} absent on origin — push it first")
        return 2
    pin = EXPECTED_SHA[BRANCH]
    if not sha.startswith(pin):
        print(f"FAIL: ancestry drift — origin/{BRANCH} = {sha[:8]}, expected {pin}.*")
        return 2
    print(f"ancestry OK: origin/{BRANCH} @ {sha[:8]} (base {BASE_BRANCH} @ {pin})")
    print(f"lever: {LEVER_FLAG} arg={LEVER_ARG} (P32-solo on quant carrier)")
    payload = {"ref": BRANCH, "inputs": INPUTS}
    if "--dry-run" in sys.argv:
        print(json.dumps(payload, indent=2))
        return 0
    api("POST", f"/repos/{REPO}/actions/workflows/{WF}/dispatches", token, payload)
    print("dispatched, polling run id (40s)...")
    time.sleep(40)
    runs = api("GET",
               f"/repos/{REPO}/actions/workflows/{WF}/runs?event=workflow_dispatch&per_page=5",
               token)
    for r in runs.get("workflow_runs", []):
        if r["head_branch"] == BRANCH:
            print(f"RUN_ID={r['id']} status={r['status']} created={r['created_at']}")
            return 0
    print("RUN_ID=<not found in last 5 runs — re-check actions list>")
    return 1


if __name__ == "__main__":
    sys.exit(main())
