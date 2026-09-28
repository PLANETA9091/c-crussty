#!/usr/bin/env python3
"""dispatch_478_f9_bn.py — [478-F9] Commander BN 6.9.8 датапак-стенд «мир под давлением».

CLAIM (прегистер закон 14a/16, зафиксирован ДО диспатча):
  19c dp-ось на тяжёлом датапак-стенде. Факты-наследство: dp-ось 0.0000% после
  МЕРЖА №16 (C22/C23) на канон-мире (мертв-lane на pregen-ванили); C61 Incendium
  5.5.2 fixture-incompatible (пак не парсится 1.21.10 реестром, dpstress-мир
  v476-dpstress-v1 36345841211 — НЕ используется); A5: fresh-gen стенды валидны
  ТОЛЬКО с quiesce/emap-фиксом (G3.1 @e97e8167 или после мержа A2-носителя).

Фикс-класс (галочка (1) задания): A2-носитель ЖИВ на origin —
  round-478-a2-conf @38d1ede9, носитель-union @1185959a (полный
  1185959afa0b287c977df4c5b0329a6db90885d0, tip алиасов round-478-a2-{dp2,totem,trek},
  ancestor a2-conf head; A2 FINAL 36367745677/36367994605/36367810959 in-band
  SUCCESS) = G3.1 quiesce-arm-all-paths + emap-arm суперсет + GAP_REGISTER/
  BlockScheduleOps live-embed. БЕРЕМ @1185959a. master НЕ берём (A2-фикс в
  master ОТСУТСТВУЕТ: master..38d1ede9 непуст).

Мир (галочка (2) — «фикс sha … если парсится 1.21.10»): stress/MANIFEST.md
  (round-477-c94c97-stress) BN-пак НЕ содержит → fallback: мир BN-стенда из
  релиза v470-s55 = world466-bn-stress-v3.zip
  sha256 f34b9242011ee97501675212d9994b18d8692eee6978f53dd5d3ee25fa59af0e
  (верифицирован скачиванием) — Brutal Nightmare 6.9.8 (1072fn/16280 строк,
  schedule-петли) + Terralith 2.5.13 + BACAP + Structory 1.3.7 + tectonic
  3.0.13, БЕЗ Incendium (C61 снят: BN pack.mcmeta dual-declaration v3
  pack_format 48 + supported_formats [48,88] — единственная форма, парсящаяся
  1.21.10; v1/v2 metadata-REJECTED = runs 36263017786/36263816472). level.dat
  server-born канон 19133/4556, region/ ПУСТОЙ → полный r640 fresh-gen под
  датапаками = ось «мир под давлением».

План:
  (1) алиас round-478-f9-bn -> @1185959a FULL-sha (1 реф=1 диспатч, Л188a/b);
  (2) диспатч world-bench-parallel 150k, canon x466-C98 ЯВНЫМ JSON (640/300s/
      fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G), lever=''/'' (стенд
      меряет dp-мир, не lever), band [6.0,9.5]M fast-fail;
  (3) dp-ось числа {function-executions, scheduler-load} из артефактов
      (BOTTLENECKS/spark/stdout) → board 19c-ступень;
  (4) band-miss → 1 ре-ролл (закон W3, алиас round-478-f9-bn-r2).

H-478-F9 (prereg): fresh-gen BN-мира под 5 паками = gen-шторм класса C63
  (terrlith 641s/tectonic 814s локально) → wall > 15 мин → вердикт
  DISPATCHED run-id (закон 18-iii). Если SUCCESS ≤15 мин: dp-ось жива на
  стенде (function-executions > 0, scheduler-load > 0 — противопоставление
  канон-миру 0.0000% C22/C23) → ваниль-гейты читать с учетом dp-мира
  (C74-прецедент: пар vs ваниль-банк-якоря НЕ ждать). Закон-5 запреты
  соблюдены: пороги/окна v5-FROZEN не трогаем, носитель = enumerated A2-union
  (не полный юнион), ваниль-наследование emap.

Usage: dispatch_478_f9_bn.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-478-f9-bn"
PIN_SHA = "1185959afa0b287c977df4c5b0329a6db90885d0"  # A2-носитель union (quiesce+emap-superset), FULL sha

BN_V3_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v470-s55/world466-bn-stress-v3.zip")

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72): дефолты yml = merge-поверхность
INPUTS = {
    "world_url": BN_V3_URL,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
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
    # A2-носитель жив на origin? (галочка (1))
    r = api(tok, f"/repos/{REPO}/git/ref/heads/round-478-a2-dp2")
    live = r.get("object", {}).get("sha", "")
    print(f"A2-carrier round-478-a2-dp2 live = {live}")
    if not live.startswith(PIN_SHA[:12]):
        print("A2-носитель НЕ жив — STOP (fallback master не выполняется автоматически)")
        sys.exit(3)
    if dry:
        print(json.dumps({"branch": BRANCH, "pin": PIN_SHA, "inputs": INPUTS}, indent=1))
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
