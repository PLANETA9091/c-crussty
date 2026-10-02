#!/usr/bin/env python3
"""dispatch_470_s31_window.py — S31 / ЯКОРЯ (round-470): climb5-p32 3-й якорь,
окно window-ре-роллов [8734563,8834563], порог ≤+2.77 (cert-пересчёт x469),
железная щель пула [8.6,9.0]M (0/20, Л201 hit ≤15% → 3+ попытки).

Канон диспатчера тик-470 (Л188a/b + S20 микро-урок FULL-sha): прямой POST
/git/refs (алиас-ветка на чистый MERGE №11 b3853246, case "6" ЖИВОЙ внутри —
Л197/Л183) + GET-верификация object.sha ДО dispatch; 1 диспатч = 1 ветка
(world-bench-parallel concurrency cancel-in-progress = 1 ран/реф).

Вектор ноги = банк-канон v5 (workflow defaults x466-C98: 640/300s/fp4/ic1/fd1/
rt4/bc1/pop150k/seed42/10G/xms4G) + gc_tune=6 (ценз-энаблер Л183: gc6-окно
[0,3], HOST-ценз-экономика 31.6%→~0 на 150k) + lever_flag ПУСТО (ваниль-якорь)
+ band [6.0,9.5]M fast-fail. Окно НЕ ставим в band-гейт: щель пула = band-dead
×все; канон Л201: банк-фид полным band + пост-хок idx-фильтр.

ГЕЙТ-ПРОТОКОЛ тик-471 (preregister, закон 14a/16):
  (1) band PASS по echo шага-3 (cpu_band [6.0,9.5]M);
  (2) пост-хок idx-фильтр: run-env cpu ∈ [8734563,8834563] → climb5-в-окне
      кандидат; anchor-легален при norm(v5) ≤ +2.77;
  (3) CLEAN-ценз: STW ≤23s (gc6-окно [0,3], Л183) и young avg ≤200ms — иначе
      HOST-excl (BANK §3, в фиты не идёт);
  (4) вне окна, но in-band CLEAN → банк-фид v5 (приоритет плотности 8.5-9.3M,
      S37/BANK §2) — нога не сгорает.

Usage: dispatch_470_s31_window.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# алиас-ветки серии window-ре-роллов (1 диспатч = 1 ветка, Л188b)
BRANCHES = [f"round-470-s31-w{i}" for i in range(1, 6)]
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
    "lever_flag": "", "lever_arg": "",  # ваниль-якорь (банк-фид / climb5-окно)
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
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&head_sha={full_sha}&per_page=20")
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
    print(f"=== S31 window-ре-роллы DISPATCHED: {len(done)} веток-алиасов @ {PIN_SHA[:8]} "
          f"gc6/ваниль/band[6.0,9.5]M; runs={json.dumps(attr)} ===", flush=True)


if __name__ == "__main__":
    main()
