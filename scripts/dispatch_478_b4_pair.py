#!/usr/bin/env python3
"""dispatch_478_b4_pair.py — [478-B4] swarx-1 (MERGE №8 85aea399) fresh-pair verification.

CLAIM (пregel 14a/16, закон 16 — зафиксирован ДО диспатча):
  swarx-1 leg_v5 = +19.53, порог якоря ≤ −0.47 (BANK_V5_FREEZE §2 v5-FROZEN),
  окно GLOB 6.0–9.5M. Пары свежие: 2 ваниль-якоря @master (0 код-дельт) +
  1 leg swarx-семантики (lever cmp458_swar arg=1, STRICT-OR носитель MERGE №8;
  swarx-union master-resident x463 → нога @master = 1 дельта = env-lever).
  pair = leg_norm − anchor_norm ≥ +20 → PAIR; иначе банк-фид §3 / DISPATCHED.

Norm-математика A1-метод (Л-478-A1.1/.2): медиана TPS poll-first-of-window
(≤5 сэмплов, C55 13/13) / tps_exp_v5 interp (узлы 6.5M→2.1252 … 9.0M→2.6280,
Л201-узел [6.9,7.2]M=2.1293) ×100−100; HOST-ценз M1: STW ≤23.0s ∧ young avg
≤200ms (gc.log-primary); vanilla-validity: ncdfe=0 / aioobe=0 / world afb3a0b3.
Пар-легальность: Δcpu(run-env) ≤50k pair-fresh; band-miss = free re-roll.

C66-C72 УРОК: полный canon-вектор явным JSON (старые yml дефолты неканон) —
все inputs передаются явно, 0 надежды на workflow-defaults.
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "33b42bbd4e90c368543d56087666c2f3f5775a01"  # master x478 (A5 HB2 хвост)

BRANCHES = ["round-478-b4-a1", "round-478-b4-a2", "round-478-b4-leg"]

# полный canon x466-C98 явным JSON (C66-C72): 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/
# pop150k/seed42/10G/xms4G, band [6.0,9.5]M fast-fail; единственная дельта =
# lever (ваниль "" vs cmp458_swar+arg1)
BASE_INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "gc_tune": "3",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}
INPUTS_BY_BRANCH = {
    "round-478-b4-a1": dict(BASE_INPUTS),                      # ваниль-якорь 1
    "round-478-b4-a2": dict(BASE_INPUTS),                      # ваниль-якорь 2
    "round-478-b4-leg": dict(BASE_INPUTS, lever_flag="cmp458_swar",
                             lever_arg="1"),                   # swarx-семантика
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


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    return [(r["id"], r.get("status"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    if not all(a in ("--dry-run",) for a in sys.argv[1:]):
        raise SystemExit(f"argv-guard: {sys.argv[1:]}")
    dry = "--dry-run" in sys.argv[1:]
    tok = token()
    for br in BRANCHES:
        live = sha_of(tok, br)
        if not live.startswith(PIN_SHA[:8]):
            raise SystemExit(f"SHA MISMATCH {br}: live={live[:8]} pin={PIN_SHA[:8]}")
        if runs_on_branch(tok, br):
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br}")
        print(f"preflight OK: {br} @ {live[:8]} runs_before=0", flush=True)
    if dry:
        print("DRY-RUN OK", flush=True)
        return
    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        print(f"dispatched: {br} lever={INPUTS_BY_BRANCH[br]['lever_flag'] or '(vanilla)'}", flush=True)
    found = {}
    deadline = time.time() + 300
    while time.time() < deadline and len(found) < len(BRANCHES):
        time.sleep(20)
        for br in BRANCHES:
            if br in found:
                continue
            for rid, st, ca in runs_on_branch(tok, br):
                found[br] = (rid, st, ca)
        print(f"poll: {found}", flush=True)
    print(json.dumps(found, indent=1))


if __name__ == "__main__":
    main()
