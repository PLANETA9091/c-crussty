#!/usr/bin/env python3
"""dispatch_481_c34_w8g6_terr.py — [481-C34] Commander W8@r480-GC6 (тик ×481, мега-цель 19a CHUNK-GEN).

CLAIM (прегистр закон 14a/16, зафиксирован ДО диспатча):
  W8@r480 канон S52 — Amdahl +23.7пп на живом gc6-носителе; свежий абсорб C13
  (run 36389080708): r480-стратум 292.6 чанк/s, gen_work 6.7% — лестница
  pregen 307.2 ≫ r480-стратум 292.6 ≫ towers 29.2 ≫ terralith 15.3 ≫ tectonic 12.0,
  «единственный ≥+20 канал остаётся W8@r480 на живом gc6 (S52-домен)» (CLM-C13 п.8).
  Шаг тика: terr@r480 CI-число — 4096 чанков fresh-gen СКВОЗЬ Terralith
  (dp-мир world469-terr-v1) на gc6-носителе (gc_tune=6).

Вектор: canon x466-C98 ЯВНЫМ JSON, дельты radius=480 / gc_tune=6 /
  world_url=terr-v1: 300s/fp4/ic1/fd1/fd_bit0/rt4/bc1/pop150k/seed42/10G/xms4G,
  band GLOB [6.0,9.5]M fast-fail, ветка round-481-c34-w8g6 @ 22919dfc
  (master ×480, база легов из RECIPE, 0 код-дельт vanilla-leg).
Мир: world469-terr-v1.zip sha256 cc1b5b4d (release v469-terr-v1, DataVersion
  4556, datapacks/Terralith_1.21.5_v2.5.13) — тот же, что C74-terr/C13-terr.
emap-ARM: lever_flag=cmp466_c98ai, arg "" — правило Л-474-C82.2 действует для
  terr+pop>0 НЕЗАВИСИМО от ванильности кода (прецеденты dispatch_480_c13_terr.py
  и dispatch_475_c74_terr.py: обе vanilla-leg terr-ноги несут emap-arm; иначе
  TASK-411-A AIOOBE-клин инъекции на terr-мире). AI-плейн сайд-эффект не
  конфаундит 19a-число: pregen (boot+forceload) идёт ДО инъекции.

ГЕЙТЫ тик-481 (prereg): (1) band PASS [6.0,9.5]M; (2) boot Done + forceload
  4096 chunks DONE (r480-квант 256×16); (3) FIXTURE-VALIDITY dp-мира:
  world sha256 = cc1b5b4d ≠ afb3a0b3 → НЕ банк-фид (пар против ваниль-якорей
  НЕ ждать); (4) emap ARM-пруф в stdout («emap … armed/compose»); (5) AIOOBE=0
  (биом-exempt Л-474-C88.2), NCDFE=0; (6) M1: STW_total ≤23.0s ∧ young ≤200ms;
  (7) число = 4096/wall_forceload → чанк/s terralith@r480-gc6, gen_work% =
  (boot+forceload)/total-wall; (8) normtool_478 обязателен.
  Runs >15 мин → DISPATCHED run-<id> (закон 12e/18-iii).
LEDGER: Л-481-C34.

Usage: dispatch_481_c34_w8g6_terr.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-481-c34-w8g6"
PIN = "22919dfc1ae0d91eb6d0d962bfed960c8d8cb884"  # master ×480 (RECIPE база легов)

TERR_V1_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
               "v469-terr-v1/world469-terr-v1.zip")

INPUTS = {
    "world_url": TERR_V1_URL,
    "radius": "480",  # r480-стратум: 4096 чанков (W8@r480-GC6)
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6",  # gc6-носитель (S52-домен)
    "inside_cache": "1", "flush_diet": "1",
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

    # Л188a: пин FULL-sha (RECIPE база легов; live master может уйти вперёд — не мешает)
    live_master = ref_sha(tok, "master")
    pin = PIN
    print(f"origin/master live = {live_master} -> pin {pin} (RECIPE база)", flush=True)

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
    print("C34-DISPATCH-JSON " + json.dumps(
        {"branch": BRANCH, "pin": pin, "run_id": rid,
         "world": "world469-terr-v1 cc1b5b4d", "radius": 480, "gc_tune": 6,
         "lever": "cmp466_c98ai (emap-arm, Л-474-C82.2)"}, indent=1), flush=True)


if __name__ == "__main__":
    main()
