#!/usr/bin/env python3
"""dispatch_470_s59_eindexfix.py — S59 / КЛИМБ-eindex (round-470): java rowBase
fix smoke leg (cmp405_eindex solo, ID_CAP 1<<14 reserve aboard).

Канон диспатчера тик-470: argv-guard, sha-pin, canonical inputs, band
6.0-9.5M fast-fail, POST + run-id discovery (Л188a/b).

Вектор: EXACT re-run of the crashed S18 smoke 36264754336 env
(gc6/fp4/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G; Л486 header) on the FIXED
branch — единственная дельта = java rowBase fix (gz=(rz<<5)|z) + Throwable
belt + ID_CAP 1<<14 cherry 6b0e2d0f. AIOOBE -32 @EntityIndexOps.query:411
(rz>=+1 zone) убит:DetectorRail-rect @chunk-z 32 (world z>=512) больше не
крэшит inject-start.

Гейты ДО диспатча (все зелёные, Л230-канон): javac-21 rebuild major 65,
blobgate eindex-subset (markers cmp405_eindex/ARMED/ERR_STRUCT/native + javap
load + src-mirror flag), ncdfe_guard ok=2 fail=0 (throwable-handlers 4),
javap query(): ishl/ior gz @503-510, belt putstatic broken @569, selftest
EntityIndexWalkSelfTest PASS (-32 repro + rz∈{-2..+2} sweep 6125 rects ==
rust dst[(cz-minCZ)*w+(cx-minCX)], bijective).

Прогноз: CRASH→CLEAN (AIOOBE=0, ARM+count-skip живы); norm — справочная
(потолок лейна +0.9-2.7пп@150k, Л232 REFUTED_CENS — ≥+20 недостижим, leg =
fix-пруф + 300-400k лестницу разблокирует).

Usage: dispatch_470_s59_eindexfix.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-470-s59-eindexfix"
PIN_SHA = ""  # filled at commit time (S59 fix sha)

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp405_eindex", "lever_arg": "",
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
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


def find_run_id(tok, full_sha, since_epoch):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&head_sha={full_sha}&per_page=10")
    for r in runs.get("workflow_runs", []):
        if r.get("created_at") and r.get("head_sha") == full_sha:
            return r["id"], r["html_url"], r.get("status")
    return None


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    live = sha_of(tok, BRANCH)
    if PIN_SHA and not live.startswith(PIN_SHA[:7]):
        raise SystemExit(f"SHA MISMATCH: {BRANCH} live={live[:8]} pin={PIN_SHA}")
    print(f"preflight OK: {BRANCH} @ {live[:8]} (eindex rowBase fix + ID_CAP reserve)", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": BRANCH, "inputs": INPUTS})
    print(f"dispatched: {BRANCH} gc6 pop150k cmp405_eindex (HTTP 204)", flush=True)
    run_id = None
    for _ in range(30):
        time.sleep(5)
        hit = find_run_id(tok, live, time.time())
        if hit:
            run_id = hit[0]
            print(f"run-id discovered: {hit[0]} status={hit[2]} {hit[1]}", flush=True)
            break
    if run_id is None:
        print("run-id NOT discovered within 150s — poll actions/runs by head_sha", flush=True)
    print(f"=== S59 eindex fix smoke DISPATCHED: branch={BRANCH} sha={live[:8]} run={run_id} ===", flush=True)


if __name__ == "__main__":
    main()
