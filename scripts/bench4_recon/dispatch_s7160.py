#!/usr/bin/env python3
"""dispatch_s7160.py — dispatch the X150K BATCH-COLLECTOR leg (S7-160).

Preregistered protocol: single A/B leg against the BANKED CUMULATIVE v2
(leg #5 = 35381522360: inside_cache=1 + flush_diet=1 + region_threads=4,
fp4/300s/150k/seed42/xmx10G, radius 640). The ONLY change vs v2 is
batch_collector=1 (S7-160 lever #8):
  - zero-map flat replacement of the vanilla StepBasedCollector
    (Entity.insideEffectCollector): flushStep = flat ORDER loop instead
    of 3 EnumMap ops x APPLY_ORDER per step transition (~60 map ops per
    entity per tick at 150k entities), zero RecordedEffect/BlockPos
    .immutable() allocations (flat long-packed positions);
  - lazy one-time per-entity swap in RegionTickOps.tickBucket (the
    single entity-tick entry point) BEFORE the vanilla consumer runs —
    one collector instance per tick episode, no parity seam;
  - DEFINE-ONLY rust wiring (batch_collector.rs): no kernel bytes are
    patched, no retransform, PG1 digest untouched.
Verification: cargo suite 147/0/1; BatchCollectorHarness OFFLINE PASS —
6000 random scenarios, flushStep playback queues BIT-IDENTICAL
(EFFECT type/pos + consumer order) vs the vanilla collector.

Preregistered absorb gates (next phase, same tick if possible):
  PG2  0 NoClassDefFoundError + ARMED (inside_cache + region_threads
       chain rc=0 + "batch_collector: defined ... kernel loader" +
       "batch_collector: ARMED first-swap") + population 150000 VALID;
  PG3  NON-REGRESSION: TPS median >= 1.60 (worst banked v2 leg);
       expectation +2..+8% from the collector axis;
  PG4'' young GC <= 180 (v2 leg #5 level) AND collector-family CPU
       (flushStep + advanceStep + applyAndClear-leaf samples)
       <= 55% of leg #5 (2143 -> <= ~1180; expectation -60..-80%);
  CRASH-FREE: 0 tracker-NPE, 0 uuid-dup, navigatingMobs watchlist.
Banking rule: full PASS -> CUMULATIVE v3 = v2 + batch_collector=1;
GATE FAIL -> REFUTED-BY-ECONOMICS verdict + rollback batch_collector=0
(fail-closed: the swap is gated by env, v2 stays banked).
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
    """Extract the PAT from the origin remote URL (rule 1b setup) —
    keeps this file secret-free (GitHub push protection, S7-153 lesson)."""
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

    # concurrency guard: any in-flight world-bench run? (law S7-108)
    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "summon_sweeps": "0",
        "fake_players": "4",
        "fluid_guard": "1",
        "paletted_demux": "0",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_free": "0",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
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
