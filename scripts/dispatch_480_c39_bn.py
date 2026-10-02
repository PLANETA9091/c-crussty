#!/usr/bin/env python3
"""dispatch_480_c39_bn.py — [480-C39] BN 6.9.8 + Incendium 5.5.2 повтор (C61-класс), 1 нога.

Канон диспатчера (Л188a/b, закон 14a/16): алиас FULL-sha + GET-verify, 1 диспатч=1
ветка=1 concurrency, canon x466-C98 ЯВНЫМ JSON (дефолты yml = merge-поверхность),
band GLOB [6.0,9.5]M fast-fail, token /tmp/gh_token.

Мир (C61-класс, fixture 95fa955a — world_url из прегиста ×476):
  https://github.com/PLANETA9091/c-crussty/releases/download/v476-dpstress-v1/world476-dpstress-v1.zip
  sha256 95fa955a9ca75d08d0fed2cb1506082ce1b61e569f9cb46da091e130d0175a54
  = base world469-terr-v1 (cc1b5b4d, POP-VALID PASS-прецедент, preg-terra, level.dat
  DataVersion 4556 server-рождён) + datapacks/: BN 6.9.8 (00589faf, 2112 fn, 542
  schedule-cmd, tick.json) + Incendium_Legacy 5.5.2 (a2405a74, 2314 fn, 122
  schedule-cmd) = 4426 fn / 664 schedule-cmd. Asset жив (HTTP 206, verify C39).

Носитель: master 686f225830a40570fd7dddcf77c2e1e64e4ecb88 (×479 MAIN-консолидация:
МЕРЖ №17 conf f66feb1b = G3.1 quiesce-arm-all-paths + emap-superset RESIDENT —
закрывает game-logic-плечо класса G3.1 на retry; R0/PIN-28; F3 ×93). 0 код-дельт.

Вектор: r640/300s/fp4/gc3/ic1/fd1/fd_bit0/rt4/bc1/pop150k/seed42/10G/xms4G,
band [6000000,9500000], lever cmp466_c98ai/"" (emap-ARM обязателен на
terr+pop>0 — Л-474-C82.2; прецеденты dispatch_475_c74_terr.py,
dispatch_476_c61_dpstress.py, CLM-C13-480 на том же terr-базисе).

H-480-C39 (прегист, закон 14a/16, зафиксирован ДО диспатча):
  (1) НАЛОГ-ЧИСЛО dp-оси (S72-класс стратум, НЕ merge-якорь: world sha 95fa955a
      ≠ банк-мир afb3a0b3 → пар к ваниль-якорям/банку НЕ ждать, C74-прецедент):
      dp-налог = wall-цена dp-мира = {function-executions, scheduler-load,
      Commands/jigsaw-лейны} из артефактов (BOTTLENECKS/spark/stdout), повтор
      C61 36345841211 (4426 fn/664 schedule) → ожидание ПОВТОРЯЕМОСТИ числа на
      №17-мастере. Прогноз dp-налога +0.9пп, вилка [−0.5,+2.5]пп; наблюдение
      ≥+3.0пп = G-D1 (диспатч-квота на фикс-агенду НЕ открывается автоматически
      — только вердикт-класс в board).
  (2) ПАРИТЕТ 20d (закон 20d, тяжёлый датапак-стенд — обязателен): log-режим
      world_diff_parity_v2 на артефактах (server-stdout.log + BOTTLENECKS_3.md +
      run-env.txt): FIXTURE-VALIDITY / WORLD sha256 / SEED / POP-TOTALS /
      TICK-BEHIND / FAKE-PLAYERS; dp-стенд → чанк-суммы vs pregen-ванили
      недетерминированы (jigsaw-RNG) → строгие чанк-чек-суммы в ОДНОЙ ноге
      self-consistent (seed-идентичный повтор шага запрещён без 20d-модели —
      Л-475-C44.1), гейт = parity FAIL-дельта-хантинг, не мерж-аргумент.
  (3) Гейты: boot Done ∧ AIOOBE=0 ∧ NCDFE T1=0 (до вердикта) ∧ POP-VALID
      (BENCH-4 FIXTURE-VALIDITY: inject DONE ≤900s) ∧ dp-load-маркер stdout
      (format-мисматч Incendium = pack-refused, НЕ crash-класс — C61-урок:
      pack.mcmeta dual-declaration 48+[48,88]).
  (4) wall > 15 мин = DISPATCHED run-id (закон 12e/18-iii — gen-шторм класса
      C63: terralith 641s локально + dp 4426 fn).
  (5) band-miss → 1 ре-ролл (закон W3): алиас round-480-c39-bn-r2, тот же
      носитель/вектор/мир; ≤2 ре-ролла НЕ тратим (Л188c).

Usage: dispatch_480_c39_bn.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-480-c39-bn"
PIN_SHA = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # ×479 MAIN-консолидация (master tip)

# C61-класс fixture 95fa955a (прегист ×476: dispatch_476_c61_dpstress.py, CLM-C61):
# release v476-dpstress-v1, world476-dpstress-v1.zip 7,963,099 B, sha-round-trip.
WORLD_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v476-dpstress-v1/world476-dpstress-v1.zip")

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72), lever cmp466_c98ai — emap-ARM
# обязателен на terr+pop>0 (Л-474-C82.2; C13-480 прецедент same-terr-базис).
INPUTS = {
    "world_url": WORLD_URL,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp466_c98ai", "lever_arg": "",
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


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created FULL-sha @ {sha}")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} -> {'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def main():
    dry = "--dry-run" in sys.argv
    tok = token()
    # носитель жив на origin? (master tip 686f2258)
    r = api(tok, f"/repos/{REPO}/commits/{PIN_SHA}")
    if "sha" not in r:
        print("PIN_SHA не найден на origin — STOP")
        sys.exit(3)
    print(f"PIN {PIN_SHA[:12]} live on origin (msg-шейк через коммит-объект)")
    if dry:
        print(json.dumps({"branch": BRANCH, "pin": PIN_SHA,
                          "world": WORLD_URL,
                          "world_sha256": "95fa955a9ca75d08d0fed2cb1506082ce1b61e569f9cb46da091e130d0175a54",
                          "inputs": INPUTS}, indent=1))
        return
    ensure_alias(tok, BRANCH, PIN_SHA)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    ok = dispatch(tok, BRANCH)
    if not ok:
        sys.exit(2)
    time.sleep(30)
    run = latest_run(tok, BRANCH, mc)
    print(json.dumps(run, indent=1))


if __name__ == "__main__":
    main()
