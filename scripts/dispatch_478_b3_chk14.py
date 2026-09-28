#!/usr/bin/env python3
"""dispatch_478_b3_chk14.py — [478-B3] CHK-14 VERDICT-CLOSURE (тик ×478, v19.0 MEGA-SWARM).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча этим коммитом):
  chk-14 носитель cmp456_chunkmono (master-resident через МЕРЖ №6, chunkmono
  в блобах; armed ТОЛЬКО lever_flag=cmp456_chunkmono — без флага ваниль).
  leg_v5 = +23.22 FROZEN (BANK_V5_FREEZE §2), порог якоря ≤ +3.22,
  окно GLOB [6.0,9.5]M.

ВЕКТОР (банк-v5 канон x466-C98 бан-EXACT, Л240/Л-466-C41.1):
  640/300s/fp4/gc3(ic1/fd1/rt4/bc1/fd_bit0/ib0/ss0/rs0/bd0)/pop150k/seed42/
  10G/4G + band [6.0,9.5]M fast-fail (band-miss = free re-roll, Л188c).

A/B-ВЕКТОР (5 алиасов ОДНОГО sha, 0 код-дельт, 1 диспатч = 1 ветка Л188b):
  round-478-b3-a1..a3   — ваниль-якоря-фида  (lever_flag=""  → ваниль)
  round-478-b3-leg1..2  — свежие ноги-носители (lever_flag=cmp456_chunkmono)

ГАЙТЫ АБСОРБА (решает артефакт, не gate-эхо; cpu ТОЛЬКО из run-env Л195):
  G1 band PASS: runner_cpu_index ∈ [6.0,9.5]M (гейт-слой авто fast-fail;
     band-dead = free discard, не ран min-of-3, Л224-г).
  G2 vanilla-validity якоря: armed=null ∧ ncdfe_real=0 ∧ aioobe=0 ∧
     world afb3a0b3 ∧ fixture-VALIDITY VALID (Л209).
  G3 CLEAN M1 (gc.log-primary Л118/Л205): STW ALL ≤23.0s ∧ young avg ≤200ms;
     иначе HOST-ценз (BANK §3) → не-якорь, re-roll.
  G4 norm-математика A1-метод (канон C55 13/13, Л-477-C55): polls-median =
     медиана FIRST-числа строк "TPS from last 5s..." (<15.0 фильтр) окна
     300s; norm_v5 = 100*(med/tps_exp_v5(cpu) − 1); tps_exp_v5 interp узлов
     §2 FREEZE {6.5M→2.1252, 7.0M→2.2047, 7.5M→2.3271, 8.2M→2.4732,
     8.7M→2.5981, 9.0M→2.6280}, <6.5M/>9.0M — линейная экстраполяция (c42).
  G5 PAIR: Δcpu(якорь, нога-носитель) ≤50k pair-fresh ∧ anchor_norm ≤ +3.22
     → board PAIR {pair = 23.22 − anchor_norm ≥ +20.00}; на Distant-ноге
     допускается frozen-leg cpu 8687055 (окно [8637055,8737055]).
  G6 ноги-носители: lever_flag=cmp456_chunkmono armed-баннер обязателен;
     regime-зависимость fast-кластера: нога out-of-band → free re-roll (п.4
     задачи); нога in-band CLEAN → свежее re-мержение leg_norm для кросс-чека
     frozen +23.22 (MAE 5.91пп бюджет ±3.2-3.7, Л201).

Runs >15 мин → вердикт DISPATCHED run-id (финал ≤10 строк, абсорб 12e).

Usage: dispatch_478_b3_chk14.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

ANCHORS = ["round-478-b3-a1", "round-478-b3-a2", "round-478-b3-a3"]
LEGS = ["round-478-b3-leg1", "round-478-b3-leg2"]
BRANCHES = ANCHORS + LEGS
LEVER_LEG = "cmp456_chunkmono"

# вектор: банк-v5 канон x466-C98 бан-EXACT (gc3), единственная дельта = lever_flag
BASE_INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_arg": "",
}
INPUTS_BY_BRANCH = {br: dict(BASE_INPUTS, lever_flag=(LEVER_LEG if br in LEGS else ""))
                    for br in BRANCHES}


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


def master_sha(tok):
    return api(tok, "/repos/" + REPO + "/git/ref/heads/master")["object"]["sha"]


def ref_exists(tok, br):
    try:
        api(tok, f"/repos/{REPO}/git/ref/heads/{br}")
        return True
    except urllib.error.HTTPError:
        return False


def runs_on_branch(tok, br):
    """Атрибуция ТОЛЬКО по head_branch (S31-урок: head_sha ловит чужие алиасы)."""
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    return [(r["id"], r.get("status"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    sha = master_sha(tok)
    print(f"master @ {sha}", flush=True)
    for br in BRANCHES:
        if ref_exists(tok, br):
            raise SystemExit(f"REF DIRTY: {br} уже существует — атрибуция сломана")
        pre = runs_on_branch(tok, br)
        if pre:
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br} уже имеет runs {pre}")
        print(f"preflight OK: {br} ref_absent runs_before=0", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    # 1 диспатч = 1 ветка = 1 concurrency-группа (канон Л188b/Л224-в)
    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{br}", "sha": sha})
        live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
        if live != sha:
            raise SystemExit(f"REF VERIFY FAIL: {br} live={live}")
        print(f"ref created+verified: {br} @ {live[:8]}", flush=True)
    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        print(f"dispatched: {br} lever_flag='{INPUTS_BY_BRANCH[br]['lever_flag']}' (204)", flush=True)
    run_ids = {}
    deadline = time.time() + 300
    while time.time() < deadline and len(run_ids) < len(BRANCHES):
        time.sleep(15)
        for br in BRANCHES:
            if br in run_ids:
                continue
            hits = runs_on_branch(tok, br)
            if hits:
                rid, st, ca = hits[0]
                run_ids[br] = (rid, st, ca)
                print(f"run-id discovered: {br} -> {rid} status={st} created={ca}", flush=True)
    for br in BRANCHES:
        hit = run_ids.get(br)
        print(f"=== B3 leg: branch={br} sha={sha[:8]} lever='{INPUTS_BY_BRANCH[br]['lever_flag']}' "
              f"run={hit[0] if hit else 'POLL-NEEDED'} status={hit[1] if hit else '-'} ===", flush=True)


if __name__ == "__main__":
    main()
