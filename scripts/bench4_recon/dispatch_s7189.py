#!/usr/bin/env python3
"""dispatch_s7189.py - lever #14 TRAVEL-ALLOC-DIET v2a ISOLATION leg (TASK-346),
PROTOCOL v8-REGRESSION (owner sanction 2026-09-20: "это твой проект — если что-то
блокировано, разбирайся с этим").

BLOCKER RESOLVED (GOAL x30-ADD): the runner pool is NOT bimodal — cpu_index is
continuous with a 1.9x daily span (6.49M..12.31M); the ±5% same-class pairing
band became a leg lottery (3 consecutive BAND-DISCARDs, the v2a leg never soaked).

PROTOCOL v8-REGRESSION (adopted on owner sanction):
  (1) WIDE BAND 6.0M..9.5M (cpu_band_min/max) — S7-96d fast-fail remains only for
      extreme landings (e.g. 12.31M);
  (2) verdict by DUAL BAR vs ANCHOR-SLOW (s7184: median5 1.60 @ 6680195):
      normalized = (med/runner)/(1.60/6680195) AND absolute = med/1.60,
      BOTH >= +10% -> CANDIDATE GREEN -> confirming min-of-2 leg -> banking v4
      = v3 + travel_diet. The two axes bracket the runner-index confound in both
      directions (index-independence evidence: bank v3 = 1.60 @ 6680195 (leg#3)
      AND 1.60 @ 8566450 (s7169); normalized is conservative above the anchor
      runner, absolute stays honest below it);
  (3) residual ±12% TPS noise (RECON-19) is covered by min-of-2 (sign AND
      magnitude). The owner DUAL BAR directive is preserved verbatim — nothing
      is banked on a single axis.

LEG INPUTS = v2a ISOLATION: bank v3 (inside_cache=1 + flush_diet=1 +
region_threads=4 + batch_collector=1) + travel_diet=1, zero_alloc=0,
skip_store_bb=0 (pure v2a effect, no v1 admixture). TravelDietOps verified
offline by TravelDietLockstepHarness (GOAL x29, 1.05M bit-exact cases).
Arm markers expected in stdout: "stage traveldiet composed (Retargeted { sites: 1 })".

PREREGISTER GATES (absorb_s7189.py): PG-T1 delivery (+ traveldiet arm marker),
PG-T2 crash-free (0 threw/unexpected/NPE incl. s7180 BlockUpdateOps incident
class + RECON-22 navigatingMobs catches), PG-T3 REGRESSION DUAL-BAR, PG-T4 GC
(young <= 174, 0 Full), PG-T5 DONE-park N/A tolerated (AP-PID defect s7178).
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

    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

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
        "travel_diet": "1",
        "inside_bitmask": "0",
        "skip_store_bb": "0",
        "region_steal": "0",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "0",
        "cpu_band_min": "6000000",
        "cpu_band_max": "9500000",
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
