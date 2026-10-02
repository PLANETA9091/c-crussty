#!/usr/bin/env python3
"""dispatch_480_c92_heap.py — COMMANDER 480-C92: heap-геометрия ×2 (19b-стенд, 200k-класс).

CLAIM (C92-плоскость, BLACKBOARD-ростер «ЛАБ heap: 10G vs 12G young-частота,
C21-стена микробенч-план»): heap-размер → young-частота → STW-масса @200k.
C21-канон: 200k-gc6-@12G = 25.50s DIRTY (Л-474-C21.1), young-стена 250k,
young-частота = единственный STW-рычаг (C05: STW_total = young_mass + 0.954×fulls,
r01 fulls=0 → young 98.2% STW; β=1.934±0.249 Л-475-C24.1 / 1.96 C31).

ЗАДАЧ: 2 диспатча world-bench-parallel @master 686f2258 (0 код-дельт, diff-страж
scripts/board-only), алиасы round-480-c92-h10 (xmx 10G) / round-480-c92-h14
(xmx 14G), pop 200000, gc_tune 6, canon x466-C98 ЯВНЫМ JSON
(640/300s/fp4/ic1/fd1/rt4/bc1/seed42/xms4G, lever="" ваниль), band GLOB
[6000000,9500000] fast-fail, band-miss → ≤1 ре-ролл/точку (тайтер W3).

ПРЕГИСТ-ГЕЙТ (закон 14a/16, пороги v5-FROZEN не двигаются):
- Гипотеза инварианта массы: 14G → young-gen больше → young_n ↓ ×0.7-0.85,
  young_avg ↑ ×1.15-1.4, STW_mass ≈ инвариант ±10% (copy-work = f(live-set)).
- M1 STW≤23s; young_n∈[100,130] ∧ young_avg∈[100,150] при xms4G-фиксе (C05 G4
  окно @150k; @200k young_avg прогноз ≈161ms по β-модели).
- Вердикт-дерево: (a) 14G STW<23 ∧ 10G STW>23 → 14G ставит 200k CLEAN-лучше =
  ЛЕСТНИЦА-ИНСАЙТ (heap-геометрия двигает young-колено; НЕ merge-лег, 0 код-дельт);
  (b) оба CLEAN → лестница держится, ΔSTW-числа = young-частота-рычаг калибровка;
  (c) оба DIRTY / 14G STW≥10G → инвариант-массы подтверждён, heap-размер НЕ
  STW-рычаг @200k → honest REFUTED_CENS {STW10,STW14,Δyoung_n,Δyoung_avg}.
LEDGER Л-480-C92. Runs >15 мин → DISPATCHED run-id (закон 12e/18-iii).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация, 0 код-дельт

LEGS = [
    {"alias": "round-480-c92-h10", "pop": "200000", "xmx": "10G"},
    {"alias": "round-480-c92-h14", "pop": "200000", "xmx": "14G"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_seed": "42",
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
        print(f"{name}: GET-proof exists @ {sha}")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created FULL-sha @ {sha}")


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} pop={inputs['population_target']} xmx={inputs['server_xmx']} -> "
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
    results = []
    for leg in LEGS:
        alias, pop, xmx = leg["alias"], leg["pop"], leg["xmx"]
        ensure_alias(tok, alias, pin)
        inputs = dict(BASE_INPUTS)
        inputs["population_target"] = pop
        inputs["server_xmx"] = xmx
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            sys.exit(2)
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, "pop": pop, "xmx": xmx, **(run or {})}, indent=1))
        results.append({"alias": alias, "pop": pop, "xmx": xmx, **(run or {})})
        time.sleep(5)
    print("C92-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
