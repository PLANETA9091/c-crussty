#!/usr/bin/env python3
"""dispatch_481_c39_bn2.py — [481-C39] BN 6.9.8 + Incendium 5.5.2 ВАНИЛЬ-ПОВТОР (стресс-лестница 19c), 1 нога.

Канон диспатчера (Л188a/b, закон 14a/16): алиас FULL-sha + GET-verify, 1 диспатч=1
ветка=1 concurrency, canon x466-C98 ЯВНЫМ JSON (дефолты yml = merge-поверхность),
band GLOB [6.0,9.5]M fast-fail, token /tmp/gh_token.

Мир (C61-класс, fixture 95fa955a — world_url из прегиста ×476, тот же что C39-480):
  https://github.com/PLANETA9091/c-crussty/releases/download/v476-dpstress-v1/world476-dpstress-v1.zip
  sha256 95fa955a9ca75d08d0fed2cb1506082ce1b61e569f9cb46da091e130d0175a54
  = base world469-terr-v1 (cc1b5b4d) + datapacks/: BN 6.9.8 (00589faf, 2112 fn,
  542 schedule-cmd) + Incendium_Legacy 5.5.2 (a2405a74, 2314 fn, 122 schedule-cmd)
  = 4426 fn / 664 schedule-cmd. Asset жив (HTTP 206 range, verify 481-C39).
  НЕ банк-фид (world sha ≠ afb3a0b3 → число = стратум лестницы 19c, НЕ merge-якорь,
  C74-прецедент).

Носитель: master 22919dfc1ae0d91eb6d0d962bfed960c8d8cb884 (×480 MAIN-консолидация,
БАНК 29/30, canary-480 −2.44 PASS). 0 код-дельт.

Дельта-вектор от canon x466-C98 (РЕЦЕПТ-481):
  world_url = BN-фикстура 95fa955a; radius = "320" (r640/2 — повтор r480-C39,
  который r640 упал в "Bottleneck report gate", НЕ band → halve-стресс);
  lever_flag = "" (ВАНИЛЬ: без emap-ARM — чистая ступень лестницы 19c).
  Остальное canon: 300s/fp4/gc3/ic1/fd1/fd_bit0/rt4/bc1/pop150k/seed42/10G/xms4G,
  band [6000000,9500000].

H-481-C39 (прегист, закон 14a/16, зафиксирован ДО диспатча):
  (1) СТРЕСС-ЛЕСТНИЦА 19c (канон ×481: pregen 307.2 ≫ towers 29.2 ≫ terralith
      15.3 ≫ tectonic 12.0 чанк/s): BN-класс НИЖЕ towers → ожидание чанк/s
      < 29.2, зона [8,20] чанк/s (jigsaw-heavy 4426 fn/664 schedule тяжелее
      terralith). Число = ступень лестницы, не merge-якорь.
  (2) Гейты: band runner_cpu_index ∈ [6.0,9.5]M (fast-fail); FIXTURE-VALIDITY
      VALID (BENCH-4 inject DONE ≤900s); AIOOBE=0 (биом-exempt Л-474-C88.2);
      NCDFE T1=0 до вердикта; dp-load-маркер stdout (Incendium format-мисматч
      = pack-refused, НЕ crash-класс — C61-урок pack.mcmeta dual-declaration).
  (3) Паритет 20d (тяжёлый dp-стенд — обязателен): world_diff_parity_v2
      log-режим по артефактам: FIXTURE-VALIDITY / WORLD-sha256 / SEED /
      POP-TOTALS / TICK-BEHIND / FAKE-PLAYERS; jigsaw-RNG → чанк-суммы
      self-consistent одной ногой (Л-475-C44.1).
  (4) wall > 15 мин = DISPATCHED run-<id> (закон 12e/18-iii, gen-шторм
      C63-класса: dp 4426 fn).
  (5) band-miss → 1 ре-ролл same-branch (закон W3/Л188c), ≤2 ре-ролла.

Usage: dispatch_481_c39_bn2.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-481-c39-bn2"
PIN_SHA = "22919dfc1ae0d91eb6d0d962bfed960c8d8cb884"  # ×480 MAIN-консолидация (master tip)

# C61-класс fixture 95fa955a (прегист ×476: dispatch_476_c61_dpstress.py, CLM-C61;
# тот же мир в C39-480 run 36388711386):
WORLD_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v476-dpstress-v1/world476-dpstress-v1.zip")

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72), дельта: world_url=BN, radius=320,
# lever_flag="" (ваниль-повтор — чистая ступень лестницы 19c).
INPUTS = {
    "world_url": WORLD_URL,
    "radius": "320", "seconds": "300", "fake_players": "4",
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
    # носитель жив на origin? (master tip 22919dfc)
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
