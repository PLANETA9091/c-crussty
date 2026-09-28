#!/usr/bin/env python3
"""dispatch_482_c32_p210.py — COMMANDER ×482-C32: 210k-реплика ре-ролл (band-miss ×2 → 3-я попытка).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча, /home/z/rounds/ROUND-482/board/CLM-C32.md):
  LEDGER 19b: 210k-якорь 24.33s STW @12G gc6 (run 36304154523, ×474). Реплика ×2 ОБА
  band-miss (Л-481-C31-C33: 10.32M↑/5.46M↓, 0 bench-cost) → перенос ×482. Миссия: 3-я
  попытка; band-miss здесь = ТРОЙНОЙ miss (систематика: раннер-пул slow-фаза burst-73;
  population на pre-download калибровку не влияет). Ре-ролл -r2 ×1 по W3, дальше стоп.

ЗАДАЧ: 1 диспатч world-bench-parallel @3666a793 (master post-merge №19 = burst73 BASE,
0 код-дельт), алиас round-482-c32-p210, ваниль-канон x466-C98 ЯВНЫМ JSON (640/300s/fp4/
fg1/ic1/fd1/rt4/bc1/seed42/xms4G, lever="") с дельтами миссии pop210k/gc6/xmx12G
(линия якоря C23), band GLOB [6000000,9500000] fast-fail.

ГЕЙТЫ рана (prereg): (1) ваниль-VALID (armed=∅, NCDFE=0, AIOOBE=0, world afb3a0b3);
(2) ΣSTW 24.33±бимод-класс, young ~176ms; M1 STW>23 → HOST-CENS ожидаемо — точка меряется
в STW-секундах-классе, НЕ raw-TPS; (3) колено [220k,280k]-гипотеза C11: 210k ДО колена →
ΣSTW 20-26s; ΣSTW>27s @210k = колено сдвинуто вниз, гипотеза REFUTED снизу;
(4) runs >15 мин → DISPATCHED run-<id> (закон 12e/18-iii), абсорб ×482.
LEDGER: «## ТИК-482 ЛАБ-C32».

Usage: dispatch_482_c32_p210.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

PIN = "3666a7931703e24a36af4887c41a585918667b7a"  # master post-merge №19 (burst73 BASE, миссия-пин)
ALIAS = "round-482-c32-p210"
OUTJSON = "/home/z/rounds/ROUND-482/c32_dispatch.json"

# ваниль-канон x466-C98 (burst73-матрица) ЯВНЫМ JSON (урок C66-C72: yml-дефолты =
# merge-поверхность); дельты миссии ×482: pop 210000 / gc_tune 6 / server_xmx 12G;
# lever ∅; world_url НЕ задаём → дефолт-мир банка (afb3a0b3).
BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "210000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "482-c32-p210-dispatch"})
    payload = json.dumps(data).encode() if data is not None else None
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]}", flush=True)
        return
    if cur:
        sys.exit(f"FATAL: alias {name} exists @ {cur[:8]} — drift, not touching")
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (урок S20/Л-470-S20.1)
    got = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    if got != sha:
        sys.exit(f"GET-verify FAIL: {got}")
    print(f"{name}: created FULL-sha @ {sha[:8]} (GET-verify OK)", flush=True)


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} pop={inputs['population_target']} gc={inputs['gc_tune']} "
          f"xmx={inputs['server_xmx']} -> {'204 OK' if r == {} else r}", flush=True)
    return r == {}


def find_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"run_id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"][:8],
                    "created": run["created_at"]}
    return None


def launch(tok, alias):
    """ensure alias @pin, dispatch, discovery + ранний band-ценз; вернёт (info, band_dead)."""
    ensure_alias(tok, alias, PIN)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    if not dispatch(tok, alias, dict(BASE_INPUTS)):
        sys.exit(2)
    info = None
    for _ in range(7):  # discovery + ранний band-ценз (~105s: fast-fail умирает на шаге 2)
        time.sleep(15)
        info = find_run(tok, alias, mc)
        if info:
            print(json.dumps({"alias": alias, **info}), flush=True)
            if info["conclusion"] == "failure":
                return info, True  # fast-fail = band-miss (0 bench-cost)
        else:
            print("waiting run discovery ...", flush=True)
    return info, False


def main():
    dry = "--dry-run" in sys.argv
    tok = token()
    if dry:
        print(json.dumps({"alias": ALIAS, "pin": PIN, "inputs": BASE_INPUTS}, indent=1))
        return
    legs = []
    info, band_dead = launch(tok, ALIAS)
    legs.append({"leg": ALIAS, "band_dead": band_dead, **(info or {})})
    if band_dead:
        print(f"BAND-MISS #{3} подряд ({ALIAS} run {info['run_id']}, cpu-index вне "
              f"[6.0,9.5]M) — систематика slow-фазы; W3 ре-ролл -r2 (последний)", flush=True)
        r2 = ALIAS + "-r2"
        info2, dead2 = launch(tok, r2)
        legs.append({"leg": r2, "band_dead": dead2, **(info2 or {})})
    json.dump({"pin": PIN, "legs": legs, "inputs": BASE_INPUTS},
              open(OUTJSON, "w"), indent=1)
    print("C32-DISPATCH-JSON " + json.dumps(legs), flush=True)


if __name__ == "__main__":
    main()
