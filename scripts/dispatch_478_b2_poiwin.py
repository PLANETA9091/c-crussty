#!/usr/bin/env python3
"""dispatch_478_b2_poiwin.py — [478-B2] Commander: poi456-4 окно 2/3 → 3/3 (тик ×478).

CLAIM (прегистер: BLACKBOARD.md [478-B2] START @6946f45f, закон 14a/16):
  POI=[8907260,9007260], anchor_norm ≤ −1.99 (v5-конверсия leg 18.01−20;
  pair = 18.01 − norm ≥ +20), страж a86@8978124.
  4 ваниль-зонда @master 6946f45f (0 код-дельт, алиасы одного sha — прецедент
  Л-470-S52.1 / Л188b: алиасы = независимые concurrency-группы).
  A17-рецепт зондов: без window-гейта (фиксированные оффсеты устаревают —
  low-mode дрейф ~650k/тик); band 6.0–9.5M fast-fail = free re-roll.
  hit = cpu ∈ POI ∧ norm_v5 ≤ −1.99 ∧ CLEAN M1 (STW≤23.0s, young-avg≤200ms).
  Метод (A1, канон C55): polls-median (первое значение 5s-полла, валид <15.0),
  tps_exp interp BANK_V5_FREEZE §2 (8.7M→2.5981 / 9.0M→2.6280), pair = 18.01−norm.
  Runs >15 мин → DISPATCHED run-id.

Usage: dispatch_478_b2_poiwin.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = [f"round-478-b2-f{i}" for i in range(1, 5)]
PIN_SHA = "6946f45f5ccc25a564594a2e5597aa7b26a69318"  # master: board-START [478-B2] поверх 60ee45cc

# x466-C98 канон EXACT (как dispatch_470_s35_window.py; окно НЕ в band-гейте)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


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
            print(f"HTTP {e.code}: {e.read()[:300]}", flush=True)
        raise
    return json.loads(body) if body else {}


def ref_sha(tok, branch):
    try:
        return api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")["object"]["sha"]
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return None
        raise


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status")) for r in runs.get("workflow_runs", [])
            if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    master = ref_sha(tok, "master")
    print(f"origin/master={master[:8]} pin={PIN_SHA[:8]} match={master == PIN_SHA}", flush=True)
    if master != PIN_SHA:
        print("WARN: master сдвинулся (конкуренция тик-агентов) — пин остаётся прегистерованным", flush=True)

    for br in BRANCHES:
        live = ref_sha(tok, br)
        pre = runs_on_branch(tok, br)
        if live is None:
            if dry:
                print(f"DRY: would create ref {br} @ {PIN_SHA[:8]}", flush=True)
                continue
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{br}", "sha": PIN_SHA})  # Л188a FULL-sha
            live = ref_sha(tok, br)
        if live != PIN_SHA:
            raise SystemExit(f"SHA MISMATCH: {br} live={live[:8]} pin={PIN_SHA[:8]}")
        if pre:
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br} уже имеет runs {pre}")
        print(f"ref OK: {br} @ {live[:8]} runs_before=0", flush=True)
    print(f"inputs: {json.dumps(INPUTS)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return

    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} (HTTP 204)", flush=True)
        time.sleep(2)

    seen, deadline = {}, time.time() + 300
    while time.time() < deadline and len(seen) < len(BRANCHES):
        time.sleep(12)
        for br in BRANCHES:
            if br in seen:
                continue
            for rid, st in runs_on_branch(tok, br):
                if st in ("queued", "in_progress", "completed"):
                    seen[br] = (rid, st)
                    print(f"run-id discovered: {br} -> {rid} status={st}", flush=True)
                    break
    print("=== B2 DISPATCHED: " + " ".join(
        f"f{i}={seen.get(br, ('POLL-NEEDED',))[0]}" for i, br in enumerate(BRANCHES, 1)) + " ===", flush=True)


if __name__ == "__main__":
    main()
