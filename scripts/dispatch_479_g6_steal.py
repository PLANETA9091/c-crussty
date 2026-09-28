#!/usr/bin/env python3
"""dispatch_479_g6_steal.py — [479-G6] region_steal-ось: STEAL=1+bu_defer=1 при rt4 (w2-вариант).

CLAIM (слот G6 от A15/W3): barrier-доля RegionTickOps CyclicBarrier phase-sync
= 2.03% thread-wall (A15 wall-профиль x8 900s/640, run 36358155978, артефакт
10945018073) — единственный концентрированный крит-путь сайт; capture-потолок
балансировки ≤+0.60пп (канон x477-C29). W3: парк 33.4→39.4% wall при rt8,
100% парков = CyclicBarrier.await → рычаг = STEAL=1 shared-cursor + bu_defer=1
(env-only, 0 код-дельт). w1-вариант = x6-477-W4 run 36358114090
(rt8+STEAL1+bu_defer, band-open) — FAILURE (ре-ролл fail). w2 = ЭТА нога:
канон-режим rt4, in-band.

Диспатч (закон 14a/16 — этот docstring; LEDGER Л-479-G6):
  алиас round-479-g6-steal @origin/master FULL-sha (0 код-дельт к базе клейма
  1a0fb21d, дрейф docs/scripts-only верифицирован diff-статом), canon x466-C98
  ЯВНЫМ JSON 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G,
  band [6.0,9.5]M fast-fail, lever=''/'' (закон-5 запреты чисты: flat_traversal
  нет, fluid_bitmask=0, fluid_dirty=0, inside_bitmask=0, skip_store_bb=0),
  region_steal=1 + bu_defer=1 (1 диспатч = 1 ветка Л188b, FULL-sha+GET Л188a,
  run-id снапшот ДО + branch+sha-фильтр Л-470-S31.1).

Прегист-гейты (финишер = scripts/absorb_479_g6_steal.py):
  G1 delivery: run-env region_steal:1 + bu_defer:1 + region_threads:4 + canon;
  G2 NPE-FREE (урок s7176 navigatingMobs race — bu_defer-канализация): 0 NPE /
     0 «unexpected exception» / 0 threw, сервер жив в конце соака, AIOOBE=0,
     NCDFE=0, FIXTURE VALID;
  G3 M1 HOST-ценз: STW_total ≤23.0s ∧ young_avg ≤200ms (gc.log completion-строки);
  G4 barrier-share: доля CyclicBarrier/RegionTickOps в wall-collapsed vs
     A15-база 2.03% thread-wall → честные числа;
  G5 pair: norm_v5 ≥ +20 (маловероятно — honest числа) → пара; иначе REFUTED-класс.
  Вердикт (закон 18-iii): run >15 мин → DISPATCHED run-id.

Usage: dispatch_479_g6_steal.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-479-g6-steal"
PIN = "5d724fa0cbf6eaa0f3468be83ab9a2c6d40c4a50"   # origin/master live (board 479-A5)
BASE_CLAIM = "1a0fb21d"                             # база клейма командира

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72) + G6-ось: region_steal=1, bu_defer=1
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "1", "bu_defer": "1",
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
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def drift_check(tok, pin):
    """0-дельт-страж: origin/master == pin, дрейф BASE_CLAIM→pin docs/scripts-only."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    if live.startswith(PIN[:12]):
        pin = live
    stat = subprocess.run(
        ["git", "-C", "/home/z/c-crussty", "diff", "--stat", BASE_CLAIM, pin[:12]],
        capture_output=True, text=True).stdout
    hot = [ln for ln in stat.splitlines()
           if ("src/" in ln or ".github/" in ln or "Cargo" in ln or "pom" in ln
               or "native/" in ln or "bench/" in ln)]
    if hot:
        print("DRIFT-HOT files (bench surface touched!):", *hot, sep="\n", file=sys.stderr)
        sys.exit(3)
    n = stat.strip().splitlines()[-1] if stat.strip() else "0 files"
    print(f"drift {BASE_CLAIM}->{pin[:8]}: docs/scripts-only OK ({n})")
    return pin


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:8]}")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{name}", "sha": sha})
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha", "")
    ok = v == sha
    print(f"{name}: FULL-sha GET-verify {'OK' if ok else 'MISMATCH ' + v[:8]}")
    if not ok:
        sys.exit(4)


def snapshot_run_ids(tok):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=30")
    return {x["id"] for x in r.get("workflow_runs", [])}


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} -> {'204 OK' if r == {} else r}")
    return r == {}


def main():
    dry = "--dry-run" in sys.argv
    tok = token()
    pin = drift_check(tok, PIN)
    if dry:
        print(json.dumps({"branch": BRANCH, "pin": pin, "inputs": INPUTS}, indent=1))
        return
    ensure_alias(tok, BRANCH, pin)
    before = snapshot_run_ids(tok)
    print(f"run-id snapshot BEFORE: {len(before)} runs")
    if not dispatch(tok, BRANCH):
        sys.exit(2)
    print("waiting 25s for run creation...")
    time.sleep(25)
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=40")
    mine = [x for x in r.get("workflow_runs", [])
            if x["head_branch"] == BRANCH and x["head_sha"].startswith(pin[:12])
            and x["id"] not in before]
    for x in mine:
        print(f"RUN-ID {x['id']} status={x['status']} created={x['created_at']}")
    if not mine:
        print("RUN NOT-FOUND — перечитать runs API", file=sys.stderr)
        sys.exit(5)


if __name__ == "__main__":
    main()
