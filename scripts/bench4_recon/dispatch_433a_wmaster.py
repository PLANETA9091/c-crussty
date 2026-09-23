#!/usr/bin/env python3
"""dispatch_433a_wmaster.py — TASK-433-A: wgen-leg on MEGA-COMPOSITE carrier
master(d282985)⊕wgen(898650c) @round-433-a-wmaster, lever_flag=cmp429_wgen
(wgen-plane rides the composite; measurement = gain OVER master composite).
inputs РОВНО банк (world-bench-parallel.yml); travel_diet / fluid_dirty_ledger
НЕ шлются; concurrency-гвардов нет."""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
REF = "round-433-a-wmaster"

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp429_wgen", "lever_arg": "1",
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-433/agent-a",
                          "remote", "get-url", "origin"],
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
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {}


def dispatch(tok):
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": REF, "inputs": INPUTS})
    print(f"dispatch sent ref={REF} lever={INPUTS['lever_flag']!r}")
    for _ in range(24):
        time.sleep(5)
        d = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=40")
        for r in d.get("workflow_runs", []):
            if r["head_branch"] == REF and \
               r["created_at"] > time.strftime("%Y-%m-%dT%H:%M", time.gmtime(time.time() - 600)):
                print(json.dumps({"run_id": r["id"], "ref": REF,
                                  "status": r["status"], "sha": r["head_sha"][:7]}))
                return r["id"]
    print("no run appeared in 120s", file=sys.stderr)
    return None


if __name__ == "__main__":
    tok = token_from_remote()
    rid = dispatch(tok)
    sys.exit(0 if rid else 1)
