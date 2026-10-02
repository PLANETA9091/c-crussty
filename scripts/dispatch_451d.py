#!/usr/bin/env python3
"""TASK-451-D dispatch (argv-guarded, golden_443.py NOT used — subagent canon).
Usage:
  scripts/dispatch_451d.py <branch> <leg-tag> [--dry-run]
  branch must start with round-451d- (self-guard: only my branches).
Inputs pinned to the tick canon (TASK-451 prompt):
  radius 640, seconds 300, fake_players 4, fluid_guard 1, gc_tune 3,
  inside_cache 1, flush_diet 1, fluid_dirty 0, fluid_bitmask 0,
  region_threads 4, batch_collector 1, inside_bitmask 0, skip_store_bb 0,
  region_steal 0, bu_defer 0, population_target 150000, population_seed 42,
  server_xmx 10G, server_xms 4G, cpu_band_min 6000000, cpu_band_max 9500000,
  lever_flag=cmp451_senseins, lever_arg=1
"""
import json, sys, urllib.request

REPO = "PLANETA9091/c-crussty"

def token():
    return open("/tmp/gh_token").read().strip()

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1",
    "flush_diet": "1", "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4", "batch_collector": "1", "inside_bitmask": "0",
    "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp451_senseins", "lever_arg": "1",
}

def api(tok, path, payload=None):
    url = f"https://api.github.com/repos/{REPO}/{path}"
    data = json.dumps(payload).encode() if payload is not None else None
    req = urllib.request.Request(url, data=data, method="POST" if data else "GET",
        headers={"Authorization": f"token {tok}", "Accept": "application/vnd.github+json"})
    return json.load(urllib.request.urlopen(req))

def main():
    args = sys.argv[1:]
    dry = "--dry-run" in args
    args = [a for a in args if a != "--dry-run"]
    if len(args) != 2:
        print(__doc__); sys.exit(2)
    branch, leg = args
    if not branch.startswith("round-451d-"):
        print(f"ARGV-GUARD: branch '{branch}' is not mine (round-451d-*) — refusing"); sys.exit(2)
    tok = token()
    ref = api(tok, f"git/ref/heads/{branch}")
    sha = ref["object"]["sha"]
    print(f"branch {branch} @ {sha[:7]} leg {leg}")
    if dry:
        print("DRY-RUN payload:", json.dumps(INPUTS, indent=1)); return
    r = api(tok, "actions/workflows/world-bench-parallel.yml/dispatches",
            {"ref": branch, "inputs": INPUTS})
    print("dispatched (204 expected). payload:", json.dumps(INPUTS))

if __name__ == "__main__":
    main()
