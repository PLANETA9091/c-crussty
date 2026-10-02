#!/usr/bin/env python3
"""dispatch_474_c12_poire.py — C12 / POI-окно-стюард (round-474): POI-окно ре-роллы,
окно [8907260,9007260], порог ≤−1.99 (poi456-4 leg_v5 +18.01, 2/3 {a24-456w2 +25.8,
a20 +26.0} — нужен 3-й якорь, Л98/BANK_V5_FREEZE). Скан тика-473 (C12-абсорб):
POI-именованные раны s89-poiA/B/C/D (gc6, cpu 6.69-7.08M) + s76-poia (gc3, 7.20M)
все ВНЕ окна; s100-guardfix 8982373 in-window но norm −1.00 > −1.99 + код-дельта
(guard-fix) → не якорь. Хитов 0 → диспатч своих ре-роллов (≤2 канон).

Вектор = ваниль-якорь: банк-канон v5 (640/300s/fp4/ic1/fd1/rt4/bc1/pop150k/
seed42/10G/xms4G) + gc_tune=3 (ParallelGC, BANK v4 канон; POI gc3-deep волна —
gc3-носитель обязателен для чистых пар, §3.4/BOTTLENECK-474C) + lever_flag=""/
lever_arg="" + band [6.0,9.5]M fast-fail (окно НЕ в band-гейт — Л201/S31 канон:
банк-фид полным band + пост-хок idx-фильтр).

Канон диспатчера (Л188a/b + S20-урок FULL-sha): прямой POST /git/refs (алиас-
ветка на пин origin/master d791550d — docs-only, 0 код-дельт) + GET-верификация
object.sha ДО dispatch; 1 диспатч = 1 ветка = 1 concurrency-группа.

ГЕЙТЫ (preregister, закон 14a/16; абсорбер следующего тика):
  (1) band PASS по echo шага-3; band-dead = free discard (Л188c) → ре-ролл;
  (2) cpu ТОЛЬКО из run-env.txt (Л195); hit = cpu ∈ [8907260,9007260] ∧
      norm_v5 ≤ −1.99 → poi456-4 3-й якорь → 3/3 → МЕРЖ-хвост (min-of-3, Δcpu≤50k
      pair-fresh против leg@8957260);
  (3) CLEAN-ценз M1 gc.log-primary: STW ≤23.0s ∧ young avg ≤200ms (gc3-класс);
  (4) vanilla-validity Л209: armed=null ∧ n=null ∧ ncdfe_real=0 ∧ aioobe=0;
  (5) norm_v5 = tail(<20) median / tps_exp_v5(cpu) − 1; interp 6.5M→2.1252,
      7.0M→2.2047, 7.5M→2.3271, 8.2M→2.4732, 8.7M→2.5981, 9.0M→2.6280;
  (6) вне окна in-band CLEAN → банк-фид v5 (194→+); ожидание ванили
      norm ∈ [−8.0,+1.5] (Л143), выход = инфра-алерт.

Usage: dispatch_474_c12_poire.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = ["round-474-c12-poire", "round-474-c12-poire-r2"]  # ре-ролл ≤2 канон
PIN_SHA = "d791550d9f66a31cf8ef309241588314ef7dc4a7"  # origin/master, docs-only = ваниль

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",  # ваниль-якорь (POI-окно ре-ролл)
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, ok404=False):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code == 404 and ok404:
            return None
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, full_sha):
    existing = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}", ok404=True)
    if existing is not None:
        got = existing["object"]["sha"]
        if got != full_sha:
            raise SystemExit(f"BRANCH ALIAS DRIFT: {branch} live={got} pin={full_sha}")
        print(f"branch exists OK: {branch} @ {got[:8]}", flush=True)
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{branch}", "sha": full_sha})
    got = sha_of(tok, branch)
    if got != full_sha:
        raise SystemExit(f"REF VERIFY FAIL: {branch} -> {got}")
    print(f"branch created+verified: {branch} @ {got[:8]}", flush=True)


def find_run_id(tok, full_sha):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&head_sha={full_sha}&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"))
            for r in runs.get("workflow_runs", [])
            if r.get("head_sha") == full_sha]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    live = sha_of(tok, "master")
    if live != PIN_SHA:
        print(f"NOTE: origin/master moved to {live[:8]} — пин остаётся {PIN_SHA[:8]}", flush=True)
    print(f"preflight OK: pin {PIN_SHA[:8]} (origin/master docs-only = ваниль, 0 код-дельт)", flush=True)
    for br in BRANCHES:
        print(f"  will ensure+dispatch: {br}", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return

    before = {r[0] for r in find_run_id(tok, PIN_SHA)}
    done = []
    for br in BRANCHES:
        ensure_branch(tok, br, PIN_SHA)
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} gc3 vanilla band[6.0,9.5]M (HTTP 204)", flush=True)
        done.append(br)
        time.sleep(3)

    run_ids = {}
    for _ in range(30):
        time.sleep(5)
        hits = {r[0]: r for r in find_run_id(tok, PIN_SHA) if r[0] not in before}
        if len(hits) >= len(done):
            run_ids = hits
            break
    attr = {}
    for rid, st, _ in run_ids.values():
        for br in done:
            runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&branch={br}&per_page=3")
            for r in runs.get("workflow_runs", []):
                if r["id"] == rid:
                    attr[br] = rid
    for br in done:
        print(f"LEG {br}: run {attr.get(br, 'POLL-PENDING')}", flush=True)
    print(f"=== C12 POI-window ре-роллы DISPATCHED: {len(done)} ветки @ {PIN_SHA[:8]} "
          f"gc3/ваниль/band[6.0,9.5]M; runs={json.dumps(attr)} ===", flush=True)


if __name__ == "__main__":
    main()
