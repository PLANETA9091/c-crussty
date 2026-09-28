#!/usr/bin/env python3
"""dispatch_482_c14_w8.py — [482-C14] Commander C14 W8@r480 перенос на new-master (тик ×482, мега-цель 19a).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча):
  W8-канал перенос на new-master + донабор ячеек. S52-канон: rt8@r480 Amdahl-потолок
  +23.7пп (φ 0.567→0.713, 8 тредов = 1/(0.287+0.713/8) = 2.66×; W8@r640 REFUTED ×3 —
  N_regions≤4 шапка). Л-481-C34 подтверждение на gc6-terr: gen_work 6.7→30.4% = +23.7пп.
  Свежие якоря окна ×482: 15 ваниль-фидов burst73-канона, медиана norm ≈−2.9 →
  pair ≥+20 требует leg_norm ≥ +17.1. Точка S52: стратум-класс r480a −3.00 + 23.7 = +20.7.
  БАНК-v5 norm MAE 5.91 (Л-470-S06.2/Л201) → честный band [+14.8, +26.7] — бар берётся
  при полном capture потолка; min-of-3 канон: этот ран = 1/3.

Вектор: burst73.py INPUTS (канон x466-C98 vanilla) с ЕДИНСТВЕННЫМИ дельтами
  radius=480 (r480-стратум, 4096 чанков) / region_threads=8 (W8 env-ARM).
  lever_flag/arg ПУСТЫЕ (env-only нога, 0 Java/0 Rust дельт — S52-класс;
  1 диспатч = 1 дельта-пара rt8+r480 на новом носителе master 3666a793).
  Мир: MineShield-3 Min (default URL,_afb3a0b3-класс) — НЕ terr → emap-arm не нужен.

ARM-пруфы lever жив на master 3666a793 (проверено до диспатча):
  - rust: src/region_threads.rs workers_from_env CRUSSTY_REGION_THREADS>=2 (L160-165),
    lib.rs:94/281/595 register+activate — git show pin подтверждён.
  - блобы: entityinside/build{,/net/...}/RegionTickOps.class javap -p -c flat==nested
    md5 5c290de156b8fe0636d4f06bd6b9d4e9, cp-маркеры CRUSSTY_REGION_THREADS(1)/
    CRUSSTY_REGION_STEAL(3)/CRUSSTY_BATCH_COLLECTOR(1), сайты tickBucket:(I)V +
    midTickTasks ретаргет живы (canon C07: cp-presence = lever жив в носителе).
  - код-дельт НЕТ → push-гейты cargo/javap-before-push не применимы (env-only).

ГЕЙТЫ рана (prereg): (1) band PASS [6.0,9.5]M fast-fail → бесплатный дискард/ре-ролл;
  (2) POPULATION 150k seed42 VALID; (3) ncdfe_real=0/aioobe=0; (4) M1 STW ≤23.0s /
  young ≤200ms (HOST-excl ценз); (5) ARM-эхо region_threads в run-env.txt;
  (6) norm = bank v5 (absorb-канон), Δcpu ≤50k к якорю для pair;
  (7) runs >15 мин → DISPATCHED run-<id> (закон 12e/18-iii).
LEDGER: «## ТИК-482 ЛАБ-C14».

Usage: dispatch_482_c14_w8.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-482-c14-w8"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"  # master post-merge №19 (burst73 BASE)

INPUTS = {
    "radius": "480",  # r480-стратум: 4096 чанков (W8@r480, S52-домен)
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3",  # burst73/bank canon (якоря окна — gc3-класс)
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",  # W8 — ЕДИНСТВЕННЫЙ живой ARM (env-only, >=2)
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha")


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN} (burst73 BASE, mission)", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": PIN})  # FULL-sha (урок S20)
            print(f"ref CREATED {BRANCH} @ {PIN[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": PIN, "force": True})
            print(f"ref PATCHED {BRANCH} -> {PIN[:8]}", flush=True)
    got = ref_sha(tok, BRANCH)
    if got != PIN:
        raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
    print(f"GET-verify OK object.sha == {got}", flush=True)

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    if r == {}:
        print("dispatch 204-OK", flush=True)
    else:
        raise SystemExit(f"dispatch failed: {r}")

    rid = None
    deadline = time.time() + 240
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid = rid_
                break
    if rid is None:
        print("run-id not visible in 240s (204 принят, discovery по head_sha позже)", flush=True)
        json.dump({"branch": BRANCH, "pin": PIN, "run_id": None,
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-482/c14_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-482/c14_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
