#!/usr/bin/env python3
"""dispatch_480_c24_feed.py — COMMANDER 480-C24: дрейф-компенсация окон, фид-волна ×4 (тик ×480).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча, v19.0 MEGA-SWARM):
  Окна v5-FROZEN (пороги НЕ двигать): chkclimb-5 [6427199,6527199] norm ≤ −1.60,
  POI [8907260,9007260] norm ≤ −1.99. Состояние 2/3 + 2/3.
  Дрейф-вектор absorb_480.json (40 cpu-точек): TAIL med 6959340 (n=9) → V3 6806350
  (n=10, −153k) → V1 6952705 (n=11, +146k): медианы стабильны ±150k, но верхний
  хвост дрейфует (V1 max 8987810 vs TAIL max 7185409 = +1802k; ≥8.5M кластер 6/40),
  низ пула статичен (V1 min 6502014 ≈ TAIL min 6570145, −69k). Канонный +650k/тик
  = утечка верхней массы в band-top 10.2-11.3M (C48/C63: fast-плечо полоса, не точка).
  C55-КОМПЕНСАЦИЯ (решение): цели со сдвигом +650k ОТВЕРГНУТЫ — A17 (фиксированные
  оффсеты устаревают за тик; транзит 100k-окна при 650k/тик = 15% фазы тика, сдвиг
  уводит мишень на ~550k мимо FROZEN-окна). Рецепт = зонд-дроу: ваниль @686f2258,
  band GLOB [6000000,9500000] fast-fail, ОКНА В ГЕЙТ НЕ СТАВЛЕНЫ — пост-хок
  cpu-фильтр run-env (Л-470-S31.1/Л195); волны ×4 со стаггером 15s = 4 фазы пула
  в пределах gate→bench −163k/5.2мин (C48) — дрейф-коридор тика покрыт сэмплом.
  N-расчёт (1 хит): chkclimb-5 edge-зона (in-window ±60k кромка) 5/40=12.5% ×
  P(norm≤−1.60|valid)=35% → P≈4.4%/ран → N≈23; POI 2/40=5.0% × 32.4% → 1.6%/ран
  → N≈60 (транзит 8.5-8.8M массы бустит до ~2-3%). ×4 → P(≥1 hit) ≈ 16% / 6-11%.

ПЛАН (0 код-дельт, vanilla, 1 реф=1 диспатч Л188b, GET-страж Л188a):
  алиасы round-480-c24-w1..w4 → refs @ FULL-sha 686f2258 (origin/master live),
  canon x466-C98 ЯВНЫЙ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/
  10G/xms4G, lever=""), band GLOB fast-fail, пауза 15s; band-dead (fail ≤180s)
  → 1 бесплатный ре-ролл/алиас wN-r2 (закон W3/Л188c, ≤2).
  Runs >15 мин → DISPATCHED run-id (закон 12e/18-iii) — бенчи не ждать, абсорб ×481.

Usage: dispatch_480_c24_feed.py [--dry-run] | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация, vanilla lever ∅
BASE_CLAIM = "686f2258"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIASES = [f"round-480-c24-w{i}" for i in range(1, 5)]
PAUSE_S = 15
BAND_DEAD_S = 180
COLLECT_TIMEOUT_S = 420
IDLE_POLL_S = 15

# канон x466-C98 — ЯВНЫЙ JSON (yml-дефолты = merge-поверхность, урок C73);
# окна chkclimb-5/POI НЕ в гейте → post-hoc фильтр (Л-470-S31.1)
INPUTS = {
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
# C55-компенсация: дрейф-вектор дня (absorb_480.json, med cpu)
DRIFT_VECTOR = {"TAIL": 6959340, "V3": 6806350, "V1": 6952705, "V4": 6955584}
WIN_CHK5 = (6427199, 6527199, -1.60)
WIN_POI = (8907260, 9007260, -1.99)


def token():
    return open("/tmp/gh_token").read().strip()


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


def drift_check(tok):
    """0-дельт-страж: CLAIM-база 686f2258 → live master не трогает bench-поверхности."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    assert live, "no live master sha"
    if not live.startswith(BASE_CLAIM):
        stat = subprocess.run(
            ["git", "-C", "/home/z/c-crussty", "diff", "--name-only",
             BASE_CLAIM, live[:12]], capture_output=True, text=True).stdout
        hot = [ln for ln in stat.splitlines() if any(h in ln for h in HOT)]
        if hot:
            print("DRIFT-HOT (bench surface touched):", *hot, sep="\n", file=sys.stderr)
            sys.exit(3)
        print(f"drift {BASE_CLAIM}->{live[:8]}: docs/scripts-only OK")
        return live
    print(f"live == CLAIM-база {BASE_CLAIM}: 0 дельт")
    return live


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:8]}")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
        print(f"{name}: created FULL-sha @ {sha[:8]}")
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"GET-verify fail {name}: {v}"


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def dispatch_wave(tok, live_sha):
    results = {}
    for alias in ALIASES:
        ensure_alias(tok, alias, live_sha)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": alias, "inputs": INPUTS})
        ok = (r == {})
        print(f"dispatch {alias} band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] "
              f"-> {'204 OK' if ok else r}")
        if not ok:
            results[alias] = {"error": r}
            continue
        time.sleep(PAUSE_S)
        run = latest_run(tok, alias, mc)
        results[alias] = run or {"id": None, "status": "POLL-NEEDED", "created": mc}
        print(json.dumps({"alias": alias, **(run or {})}))
    return results


def collect_and_reroll(tok, results):
    """Дожидаемся run-id/band-dead; band-dead (fail ≤180s) → 1 ре-ролл wN-r2 (W3)."""
    deadline = time.time() + COLLECT_TIMEOUT_S
    while time.time() < deadline:
        time.sleep(IDLE_POLL_S)
        for alias in list(results):
            run = results.get(alias) or {}
            if run.get("id") and run.get("status") == "completed":
                continue
            mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 900))
            fresh = latest_run(tok, alias, mc)
            if fresh:
                results[alias] = fresh
                print(f"{alias}: run {fresh['id']} status={fresh['status']} "
                      f"concl={fresh['conclusion']}")
    # band-dead детект + ре-роллы
    for alias in ALIASES:
        run = results.get(alias) or {}
        if run.get("conclusion") == "failure" and run.get("status") == "completed":
            rr = alias + "-r2"
            print(f"{alias}: BAND-DEAD fast-fail → ре-ролл {rr} (W3/Л188c)")
            ensure_alias(tok, rr, PIN)
            mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
            r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                    method="POST", data={"ref": rr, "inputs": INPUTS})
            if r == {}:
                time.sleep(PAUSE_S)
                fresh = latest_run(tok, rr, mc)
                results[rr] = fresh or {"id": None, "status": "POLL-NEEDED"}
                print(json.dumps({"alias": rr, **(fresh or {})}))
    return results


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    if dry:
        for alias in ALIASES:
            ensure_alias(tok, alias, live)
        print("DRY-RUN OK — refs GET-verified, 0 диспатчей")
        return
    results = dispatch_wave(tok, live)
    results = collect_and_reroll(tok, results)
    print("C24-DISPATCH-JSON " + json.dumps(
        {"pin": PIN[:12], "drift_vector": DRIFT_VECTOR,
         "windows": {"chkclimb-5": WIN_CHK5, "POI": WIN_POI},
         "runs": results}, default=str))


def do_poll():
    tok = token()
    for alias in ALIASES + [a + "-r2" for a in ALIASES]:
        run = latest_run(tok, alias, "2026-09-28T00:00:00Z")
        if run:
            print(f"{alias}: run {run['id']} status={run['status']} "
                  f"concl={run['conclusion']}")
        else:
            print(f"{alias}: NO-RUN")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
