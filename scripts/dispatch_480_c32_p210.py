#!/usr/bin/env python3
"""dispatch_480_c32_p210.py — COMMANDER 480-C32: 19b-реплика 210k (2-я точка излома).

CLAIM (19b-лестница): излом (200k,205k] подтверждён; якорь 210k = 24.33s STW @12G gc6
×1 (run 36304154523, CLM-C23-474). Миссия: РЕПЛИКА 210k — нужна 2-я точка 24.33
(консистентность полки/young-стены; young_avg 175.7ms@210k, 12G НЕ спасает).

ЗАДАЧ: 1 диспатч world-bench-parallel @686f2258 (0 код-дельт), алиас
round-480-c32-p210, canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc6/ic1/fd1/rt4/bc1/
pop210k/seed42/12G/xms4G, lever=""), band GLOB [6000000,9500000] fast-fail,
band-miss → 1 ре-ролл -r2 (закон W3).

ПРЕГИСТ-ГЕЙТ (закон 14a/16): ваниль-VALID (lever ∅ armed=0, NCDFE=0, AIOOBE=0,
FIXTURE-VALIDITY, world afb3a0b3), band IN. Чтение post-factum: ΣSTW 24.33±бимод-класс
(200k-якорь бимодален 20.91/25.50 — 210k-класс = 24.33 vs DIRTY-хвост); консистент
×1 → канон 210k ×2, полка (205k,220k) подтверждена; M1 STW>23 → DIRTY-ценз нечисла.
Runs >15 мин → DISPATCHED run-id (закон 18-iii), абсорб ×481.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master round-480, 0 код-дельт

ALIAS = "round-480-c32-p210"

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72: yml-дефолты = merge-поверхность);
# миссия 19b: pop 210000, gc_tune 6, server_xmx 12G; lever ∅; world_url НЕ задаём
# → дефолт-мир банка (afb3a0b3).
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
        "User-Agent": "480-c32-p210-dispatch"})
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
        print(f"{name}: GET-proof exists @ {sha[:8]}")
        return
    if cur:
        sys.exit(f"FATAL: alias {name} exists @ {cur[:8]} — drift, not touching")
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    got = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")["object"]["sha"]
    assert got == sha, f"GET-verify failed: {got}"
    print(f"{name}: created FULL-sha @ {sha[:8]} (GET-verify OK)")


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} pop={inputs['population_target']} -> "
          f"{'204 OK' if r == {} else r}")
    return r == {}


def find_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"run_id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"][:8],
                    "created": run["created_at"]}
    return None


def launch(tok, alias):
    """ensure alias @pin, dispatch, discover run; вернёт (info, band_dead)."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    pin = live if live.startswith(PIN[:12]) else PIN
    print(f"origin/master live = {live[:8]} -> pin {pin[:8]}")
    ensure_alias(tok, alias, pin)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    if not dispatch(tok, alias, dict(BASE_INPUTS)):
        sys.exit(2)
    info = None
    for _ in range(6):  # discovery + ранний band-ценз (~100s)
        time.sleep(15)
        info = find_run(tok, alias, mc)
        if info:
            print(json.dumps({"alias": alias, **info}))
            if info["conclusion"] == "failure":
                return info, True  # fast-fail = band-miss
        else:
            print("waiting run discovery ...")
    return info, False


def main():
    dry = "--dry-run" in sys.argv
    tok = token()
    if dry:
        print(json.dumps({"alias": ALIAS, "pin": PIN,
                          "inputs": BASE_INPUTS}, indent=1))
        return
    info, band_dead = launch(tok, ALIAS)
    if band_dead and info:
        r2 = ALIAS + "-r2"
        print(f"BAND-DEAD {info['run_id']} ({info['created']}) -> ре-ролл W3 {r2}")
        info2, dead2 = launch(tok, r2)
        print("C32-DISPATCH-JSON " + json.dumps(
            [{"leg": ALIAS, **(info or {})},
             {"leg": r2, "band_dead": dead2, **(info2 or {})}]))
        return
    print("C32-DISPATCH-JSON " + json.dumps([{"leg": ALIAS, **(info or {})}]))


if __name__ == "__main__":
    main()
