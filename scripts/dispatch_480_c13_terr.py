#!/usr/bin/env python3
"""dispatch_480_c13_terr.py — [480-C13] Commander 19a CHUNK-GEN (тик ×480, мега-цель 19a).

CLAIM (прегистер закон 14a/16, зафиксирован ДО диспатча):
  19a-лестница: канон 307.2 чанк/s pregen (min-of-N ×10) ≫ towers 29.2 ≫
  terralith 15.3 ≫ tectonic 12.0; gen_work 9.6% wall (REFUTED код-фикс
  ×478-F12: burst boot-only, поллы не видят — NOISEFILL RC1/RC2, capture≈0).
  Шаг тика: r480-стратум-повтор ×1 на ТЕРР-мире — CI-число terralith@r480
  (4096 чанков fresh-gen сквозь Terralith) в лестницу 19a.

Вектор: canon x466-C98 ЯВНЫМ JSON с ЕДИНСТВЕННОЙ дельтой radius=480:
  300s/fp4/gc3/ic1/fd1/fd_bit0/rt4/bc1/pop150k/seed42/10G/xms4G,
  band GLOB [6.0,9.5]M fast-fail, алиас round-480-c13-terr1 @master
  686f2258 (0 код-дельт).
Мир: world469-terr-v1.zip sha256 cc1b5b4d (release v469-terr-v1,
  сервер-рождён level.dat DataVersion 4556, DOA-класс снят; datapacks/
  Terralith_1.21.5_v2.5.13) — тот же, что C74-terr/S53-канон.
emap-ARM: lever_flag=cmp466_c98ai, arg "" (правило Л-474-C82.2: pop>0
  terr/stress-ноги обязаны нести emap-arm — иначе TASK-411-A AIOOBE-клин
  инъекции на terr-мире; прецедент dispatch_475_c74_terr.py). AI-плейн
  сайд-эффект не конфаундит 19a-число: pregen (boot+forceload) идёт ДО
  инъекции, gen-фаза не зависит от levers.

ГЕЙТЫ тик-481 (prereg): (1) band PASS; (2) boot Done + forceload 4096
  chunks DONE (не 9216 — r480-квант 256×16); (3) FIXTURE-VALIDITY с учётом
  dp-мира (world sha ≠ afb3a0b3 → НЕ банк-фид, пар против ваниль-якорей
  НЕ ждать); (4) emap ARM-пруф в stdout («emap … armed/compose»);
  (5) число лестницы = forceload-фаза wall / 4096 → чанк/s terralith@r480,
  gen_work% = (boot+forceload)/total-wall — в claim + лестницу 19a.
  AIOOBE/NCDFE-ценз обязателен. Runs >15 мин → DISPATCHED run-id (закон 18-iii).
LEDGER: Л-480-C13.

Usage: dispatch_480_c13_terr.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-480-c13-terr1"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация (0 код-дельт к раунд-базе)

TERR_V1_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
               "v469-terr-v1/world469-terr-v1.zip")

INPUTS = {
    "world_url": TERR_V1_URL,
    "radius": "480",  # r480-стратум: 4096 чанков (19a-стратум-повтор)
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp466_c98ai",  # emap-ARM (Л-474-C82.2, terr+pop>0 обязателен)
    "lever_arg": "",
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

    # Л188a: пин FULL-sha, верифицируем live master
    live_master = ref_sha(tok, "master")
    pin = live_master if live_master and live_master.startswith(PIN[:12]) else PIN
    print(f"origin/master live = {live_master} -> pin {pin}", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != pin:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": pin})  # FULL-sha (урок S20: короткий = 422)
            print(f"ref CREATED {BRANCH} @ {pin[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": pin, "force": True})
            print(f"ref PATCHED {BRANCH} -> {pin[:8]}", flush=True)
    got = ref_sha(tok, BRANCH)
    if got != pin:
        raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
    print(f"GET-verify OK object.sha == {got[:8]}", flush=True)

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
        time.sleep(15)
        for i, st, ca, hs in runs_on_branch(tok, BRANCH):
            print(f"poll: run {i} status={st} created={ca} sha={hs[:8]}", flush=True)
            if st in ("queued", "in_progress", "completed"):
                rid = i
            break
    print("C13-DISPATCH-JSON " + json.dumps(
        {"branch": BRANCH, "pin": pin, "run_id": rid,
         "world": "world469-terr-v1 cc1b5b4d", "radius": 480,
         "lever": "cmp466_c98ai (emap-arm)"}, indent=1), flush=True)


if __name__ == "__main__":
    main()
