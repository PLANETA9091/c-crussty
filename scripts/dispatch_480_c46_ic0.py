#!/usr/bin/env python3
"""dispatch_480_c46_ic0.py — COMMANDER 480-C46: inside_cache=0 изоляция ×1 (ic-квота канона).

CLAIM: канон x466-C98 несёт inside_cache="1" (мемоизация checkInsideBlocks); checkInside-лейн
11.41%CPU уже С кэшем. ic0-изоляция снимает кэш → raw-checkInside всплывает → ожидаемый
минус нормы. Миссия = проверка ic-квоты канона; гейт: ТОЛЬКО ЧИСЛО Δnorm(ic0−ic1), НЕ merge.
Прегист: /home/z/rounds/ROUND-480/c46/PREREG_480_C46_IC0.md (закон 14a/16, ДО диспатча).

ЗАДАЧ: 1 диспатч world-bench-parallel @master 686f2258 (0 код-дельт), алиас
round-480-c46-ic0, canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic0/fd1/rt4/bc1/pop150k/
seed42/10G/xms4G, lever="" ваниль), band GLOB [6000000,9500000] fast-fail,
band-miss → 1 ре-ролл (закон W3).

ПРЕГИСТ-ГЕЙТЫ: G1 delivery (ic0∧canon-вектор∧686f2258); G2 band in [6.0,9.5]M;
G3 readout norm_v5 (selftest, NCDFE=0); G4 IC0-PRICE = Δnorm(ic0−ic1|same-pin,
w2 36386470310 / w4 36386521687 якоря) — только число, не merge. LEDGER Л-480-C46.
Runs >15 мин → DISPATCHED run-id (закон 18-iii).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # master live, 0 код-дельт

LEGS = [
    {"alias": "round-480-c46-ic0"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3",
    "inside_cache": "0",            # C46-ось: ЕДИНСТВЕННАЯ дельта от канона (ic-квота-тест)
    "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4",
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
    print(f"dispatch {ref} ic={inputs['inside_cache']} -> "
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
        alias = leg["alias"]
        ensure_alias(tok, alias, pin)
        inputs = dict(BASE_INPUTS)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            sys.exit(2)
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, **(run or {})}, indent=1))
        results.append({"alias": alias, **(run or {})})
        time.sleep(5)
    print("C46-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
