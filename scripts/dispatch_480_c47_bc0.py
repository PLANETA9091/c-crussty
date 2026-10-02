#!/usr/bin/env python3
"""dispatch_480_c47_bc0.py — COMMANDER 480-C47: WILD bc0-изоляция (BLACKBOARD C47).

CLAIM (изоляция-семья ×480, сиблинг C46-ic0): canon x466-C98 держит
batch_collector="1"; bc0 возвращает per-entity сбор внутрь RegionTickOps.forEach —
топ-лейна профиля ×479-F1 (22.19% CPU → 1.49пп wall). Δnorm(bc0−bc1|same-model)
= прямая оценка стоимости сбора; capture-потолок ≤1.49пп ⇒ P(pair≥+20) ≤ 2%,
pair-ось НЕ заявляется. ПРЕГИСТ: /home/z/rounds/ROUND-480/c47/
PREREG_480_C47_BC0.md (закон 14a/16, ДО диспатча).

ЗАДАЧ: 1 диспатч world-bench-parallel.yml @686f2258 (0 код-дельт), алиас
round-480-c47-bc0, canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/
bc0/pop150k/seed42/10G/xms4G, lever="" ваниль), band GLOB [6000000,9500000]
fast-fail, band-miss → 1 ре-ролл (закон W3).

ПРЕГИСТ-ГЕЙТЫ: G1 delivery (bc0∧canon∧FULL-sha GET-verified Л188a); G2 NPE-FREE
(bc0 — не дефолт-тестируемая ветка, честный ценз AIOOBE/NCDFE); G3 M1 STW≤23s ∧
young_avg≤200ms (v5-FROZEN); G4 norm_v5 ∈ коридор [−8.0,+1.5] Л143 ⇒ bank-
eligible иначе отдельный v6-канал (fp0-C52/seed45-C90.1-прецедент); G5 band
fail-окно ≤180s (Л188c: >200s in_progress = band-pass). LEDGER Л-480-C47.
Runs >15 мин → DISPATCHED run-id (закон 18-iii). Финал {run id, ветка+хеш, число}
— иначе SLACKER.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # ×479-консолидация, 0 код-дельт

ALIAS = "round-480-c47-bc0"

# canon x466-C98 ЯВНЫЙ JSON; ЕДИНСТВЕННАЯ дельта = batch_collector "0" (bc0-изоляция)
BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "0",         # C47-ось: bc0-изоляция (canon bc1)
    "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}
BAND_DEAD_S = 180  # fast-fail окно band-гейта (наблюдение Л188c)


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
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"alias verify failed: {v} != {sha}"
    print(f"{name}: created FULL-sha @ {sha[:8]}, GET-verified")


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} bc={inputs['batch_collector']} pop={inputs['population_target']} -> "
          f"{'204 OK' if r == {} else r}")
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
    tok = token()
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    pin = live if live.startswith(PIN[:12]) else PIN
    ensure_alias(tok, ALIAS, pin)
    inputs = dict(BASE_INPUTS)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    for attempt in (1, 2):  # 1 retry на transient (закон W3: только POST-фейл)
        if dispatch(tok, ALIAS, inputs):
            break
        time.sleep(20)
    else:
        sys.exit(2)
    time.sleep(20)
    run = latest_run(tok, ALIAS, mc)
    print(json.dumps({"alias": ALIAS, "bc": inputs["batch_collector"],
                      **(run or {})}, indent=1))
    print("C47-DISPATCH-JSON " + json.dumps(
        {"alias": ALIAS, "pin": pin, "run": run or {}, "inputs": inputs}))


if __name__ == "__main__":
    main()
