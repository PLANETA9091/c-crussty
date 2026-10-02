#!/usr/bin/env python3
"""dispatch_s7178.py - BANK RE-CALIBRATION leg (TASK-337, RECON-17 §5).

RECON-17 finding A: the TPS axis carries a ±20% runner-speed confounder
(runner_cpu_index probe already in run-env but never used in verdicts); the
bank 1.80 (leg #5) is not reproducible from s7169's own artifact (console
median5 = 1.60). All past n=1 TPS verdicts are contaminated (CLAIMS S7-163:
«шум n=1/1»).

s7178 = fresh bank-v3-config anchor leg on the CURRENT runner class:
region_steal=0, bu_defer=0 (rollback defaults), inside_cache=1, flush_diet=1,
region_threads=4, batch_collector=1. Next tick s7179 re-runs STEAL v2 on the
same runner class -> PAIRED A/B min-of-2, verdict on speed-normalized delta
>= +10% (GREEN banking v4) or lane stays open with lever #14 candidate
(TRAVEL-ALLOC-DIET, RECON-17 §4).

PREREGISTER GATES (absorb):
  PG-RC1 delivery: 0 NCDFE + pop 150k VALID + bank v3 markers
    (inside_cache=1, flush_diet=1, region_threads=4, batch_collector=1)
    + region_steal=0 + bu_defer=0 (rollback verified)
  PG-RC2 runner probe: runner_cpu_index recorded; leg valid for pairing only
    if a future steal leg lands within ±5% of it
  PG-RC3 TPS: console last-5 median recorded as NEW bank anchor (expectation
    band 1.4-1.8 given s7169=1.6 and leg#5=1.8); NOT a pass/fail gate — this
    leg measures, it does not lever.
  PG-RC4 GC sanity: young <= 174, 0 Full
Banking: the recorded median + runner index becomes CUMULATIVE v3-RECAL anchor.
INJECTS-ONLY, 1 sanctioned CI gate leg.
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL — run rule (1b) remote set-url first")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def main():
    tok = token_from_remote()

    # remote-head == local-head law (S7-164 dispatch incident): push BEFORE dispatch
    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    # concurrency guard: any in-flight world-bench run? (law S7-108)
    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "4",
        "fluid_guard": "1",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "flat_traversal": "0",
        "zero_alloc": "0",
        "skip_store_bb": "0",
        "region_steal": "0",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "0",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=1")
        runs = d.get("workflow_runs", [])
        if runs:
            r = runs[0]
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
            return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
