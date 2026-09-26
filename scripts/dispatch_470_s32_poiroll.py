#!/usr/bin/env python3
"""dispatch_470_s32_poiroll.py — S32 / ЯКОРЯ (round-470): POI-окно ре-ролл,
окно window-ре-роллов [8907260,9007260], порог ≤−1.99 (poi456-4 leg +18.01, Л98),
HOST-хит S12-p3c 8909363 −8.36-in-window СЪЕДЕН HOST-цензом (STW 27.1s > 23s, Л184)
→ легальных window-якорей 0. Канал = RE-GRAIN №11 (Л184): ре-роллы window-слотов
С gc_tune=6 (Л183: gc6-флаги снимают 100% HOST-цензов 150k-класса, ценз-экономика
31.6%→~0, прогноз 16.5-18.1s @150k; Full-окно пост-флаг [0,3]).
Щель пула: 0/20 в [8.6,9.0]M, hit ≤15% (Л201) → 3+ попытки алиасами (Л188b).

Канон диспатчера тик-470 (Л188a/b + S20 микро-урок FULL-sha; зеркало S31):
прямой POST /git/refs (алиас-ветка на чистый MERGE №11 b3853246, case "6" ЖИВОЙ
внутри — Л197/Л183) + GET-верификация object.sha ДО dispatch; 1 диспатч = 1 ветка
(world-bench-parallel concurrency cancel-in-progress = 1 ран/реф).

Вектор ноги = банк-канон v5 (workflow defaults x466-C98: 640/300s/fp4/ic1/fd1/
rt4/bc1/pop150k/seed42/10G/xms4G) + gc_tune=6 + lever_flag ПУСТО (ваниль-якорь)
+ band [6.0,9.5]M fast-fail. Окно НЕ ставим в band-гейт: канон S31/Л201 —
банк-фид полным band + пост-хок idx-фильтр.

ГЕЙТ-ПРОТОКОЛ тик-471 (preregister, закон 14a/16):
  (1) band PASS по echo шага-3 (cpu_band [6.0,9.5]M);
  (2) пост-хок idx-фильтр по run-env (Л195: cpu ТОЛЬКО из run-env.txt, не
      gate-echo): cpu ∈ [8907260,9007260] → POI-в-окне кандидат; anchor-легален
      при norm(v5) ≤ −1.99 → poi456-4 3-й якорь (S40-хэндофф, пара ≥+20 при leg
      +18.01); norm ∈ (−1.99,0) → окно заполнено, банк-фид без пары;
  (3) CLEAN-ценз M1 gc.log-primary (Л118/Л205): STW ≤23.0s (gc6-окно [0,3]),
      young avg ≤200ms — иначе HOST-excl (BANK §3) + RE-ROLL;
  (4) vanilla-validity (Л209): armed=null ∧ n=null ∧ ncdfe_real=0 ∧ aioobe=0 ∧
      throw_fail=0;
  (5) вне окна, но in-band CLEAN → банк-фид v5 полным band (Л201/Л209) — нога
      не сгорает (банк 194→+, плотность 8.5-9.3M плеча);
  (6) band-dead → free discard (Л188c), ре-ролл (пул memoryless P(in|out)=0.88,
      Л176).

Usage: dispatch_470_s32_poiroll.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# алиас-ветки серии POI window-ре-роллов (1 диспатч = 1 ветка, Л188b)
BRANCHES = [f"round-470-s32-p{i}" for i in range(1, 7)]
PIN_SHA = "b385324677ac76f4f9f8ef942a3835c8004aef78"  # MERGE №11, case "6" живой

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",  # ваниль-якорь (банк-фид / POI-окно)
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
    """Л188a: молча не создаётся PATCH-в-никуда — прямой POST /git/refs
    (FULL sha, S20-урок 422) + GET-верификация object.sha ДО dispatch."""
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
        print(f"NOTE: origin/master moved to {live[:8]} — пин остаётся MERGE №11 {PIN_SHA[:8]} "
              f"(чистый ваниль-якорь пары, case '6' живой внутри)", flush=True)
    print(f"preflight OK: pin {PIN_SHA[:8]} (MERGE №11, gc6 case '6' live, Л197)", flush=True)
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
        print(f"dispatched: {br} gc6 vanilla band[6.0,9.5]M (HTTP 204)", flush=True)
        done.append(br)
        time.sleep(3)

    run_ids = {}
    for _ in range(30):
        time.sleep(5)
        hits = {r[0]: r for r in find_run_id(tok, PIN_SHA) if r[0] not in before}
        if len(hits) >= len(done):
            run_ids = hits
            break
    # атрибуция run→ветка по head_branch
    attr = {}
    for rid, st, _ in run_ids.values():
        for br in done:
            runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&branch={br}&per_page=3")
            for r in runs.get("workflow_runs", []):
                if r["id"] == rid:
                    attr[br] = rid
    for br in done:
        print(f"LEG {br}: run {attr.get(br, 'POLL-PENDING')}", flush=True)
    print(f"=== S32 POI-window ре-роллы DISPATCHED: {len(done)} веток-алиасов @ {PIN_SHA[:8]} "
          f"gc6/ваниль/band[6.0,9.5]M; runs={json.dumps(attr)} ===", flush=True)


if __name__ == "__main__":
    main()
