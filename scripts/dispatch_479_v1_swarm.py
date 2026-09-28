#!/usr/bin/env python3
"""dispatch_479_v1_swarm.py — COMMANDER 479-V1 MEGA-SWARM (тик ×479, волна 12c): MASS-ВОЛНА диспатчей.

CLAIM (банк Л201 27/30 → 30, дефицит 3): STRICT-добивочная волна — 12 ваниль-диспатчей
@master e503160c 0 код-дельт (алиасы round-479-v1-s01..s12 — уникальные ветки →
per-ref concurrency group world-bench-round-${{ github.ref }} concurrency-safe,
cancel-in-progress не задевает соседей), canon x466-C98 ЯВНЫМ JSON (урок C66-C73),
band GLOB [6000000,9500000] fast-fail; окно STRICT [6.9,7.2]M НЕ в гейте —
post-hoc STRICT-фильтр по run-env runner_cpu_index (канон Л-470-S31.1 банк-фид
полным band). STRICT-hit ⇔ cpu(run-env) ∈ [6.9,7.2]M ∧ CLEAN ∧ ваниль-VALID ∧
norm ∈ ваниль-коридор [−8.0,+1.5] (Л143) → банк n+1; CLEAN in-band вне STRICT →
в-точка банк-фид §3. Пороги НЕ двигать (закон-5 запреты). Пауза 15s между POST
(secondary rate-limit гигиена). Ре-роллы ТОЛЬКО по band-dead (fast-fail ≤180s)
— 1 бесплатный ре-ролл на алиас (закон W3), диспатчил = финал DISPATCHED run-id
(закон 12e) — бенчи НЕ ждём, абсорб чисел след. тик.
Прегист закон 14a/16 — этот docstring; LEDGER Л-479-V1.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BASE_CLAIM = "e503160c"  # база CLAIM командира (origin/master live верифицирован fetch)
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIASES = [f"round-479-v1-s{i:02d}" for i in range(1, 13)]  # s01..s12
PAUSE_S = 15           # пауза между POST (claim 12c)
BAND_DEAD_S = 180      # fast-fail окна band-гейта (наблюдение Л188c: ~20s)
COLLECT_TIMEOUT_S = 480
IDLE_POLL_S = 15

# canon x466-C98 — ЯВНЫЙ JSON, yml-дефолты = merge-поверхность (урок C73)
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
    """0-дельт-страж: CLAIM-база → live master не трогает bench-поверхности."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    assert live, "no live master sha"
    if not live.startswith(BASE_CLAIM):
        stat = subprocess.run(
            ["git", "-C", "/home/z/c-crussty", "diff", "--name-only",
             BASE_CLAIM, live[:12]],
            capture_output=True, text=True).stdout
        hot = [ln for ln in stat.splitlines() if any(h in ln for h in HOT)]
        if hot:
            print("DRIFT-HOT files (bench surface touched!):", *hot, sep="\n", file=sys.stderr)
            sys.exit(3)
        print(f"drift {BASE_CLAIM}->{live[:8]}: docs/scripts-only OK "
              f"({len(stat.strip().splitlines()) if stat.strip() else 0} files)")
    else:
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
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"alias verify failed: {v} != {sha}"
    print(f"{name}: created FULL-sha @ {sha[:8]}, GET-verified")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {ref} -> {'204 OK' if ok else r}", flush=True)
    return ok


def list_runs(tok, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=60")
    out = {}
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] in ALIASES and run["created_at"] > min_created:
            prev = out.get(run["head_branch"])
            if prev is None or run["created_at"] > prev["created"]:
                out[run["head_branch"]] = {"id": run["id"], "status": run["status"],
                                           "conclusion": run["conclusion"],
                                           "sha": run["head_sha"][:8],
                                           "created": run["created_at"]}
    return out


def main():
    tok = token()
    pin = drift_check(tok)
    for a in ALIASES:
        ensure_alias(tok, a, pin)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))

    # --- MASS-ВОЛНА: 12 POST с паузой 15s, 1 retry на transient ---
    posted = []
    for i, a in enumerate(ALIASES):
        if i:
            time.sleep(PAUSE_S)
        if not dispatch(tok, a):
            time.sleep(20)
            if not dispatch(tok, a):
                print(f"FATAL dispatch {a}", file=sys.stderr)
                sys.exit(2)
        posted.append(a)

    # --- сбор run-id ×12 (не ждём бенчей — только факт регистрации run) ---
    deadline = time.time() + COLLECT_TIMEOUT_S
    found = {}
    while time.time() < deadline and len(found) < len(ALIASES):
        found = list_runs(tok, mc)
        print(f"run-ids {len(found)}/{len(ALIASES)}: "
              + ", ".join(f"{k.split('-')[-1]}={v['id']}" for k, v in sorted(found.items())),
              flush=True)
        if len(found) < len(ALIASES):
            time.sleep(IDLE_POLL_S)

    # --- band-dead сторож: fast-fail ≤BAND_DEAD_S → 1 free ре-ролл (закон W3) ---
    rerolled = []
    watch_until = time.time() + BAND_DEAD_S + 120
    while time.time() < watch_until:
        time.sleep(20)
        snap = list_runs(tok, mc)
        for k, v in snap.items():
            if (v["conclusion"] == "failure" and k not in rerolled
                    and v["status"] == "completed"):
                # fast-fail = заключение в пределах BAND_DEAD_S от создания
                t0 = time.mktime(time.strptime(v["created"], "%Y-%m-%dT%H:%M:%SZ"))
                if time.time() - t0 <= BAND_DEAD_S + 60:
                    print(f"band-dead {k} (run {v['id']}) → free re-roll W3", flush=True)
                    time.sleep(10)
                    dispatch(tok, k)
                    rerolled.append(k)
        if all(v["status"] != "completed" or v["conclusion"] != "failure"
               for v in snap.values()) and not snap:
            break

    print("SWARM-RESULT " + json.dumps({
        "aliases": ALIASES, "pin": pin, "runs": found,
        "band_dead_rerolled": rerolled,
        "strict_window": [6900000, 7200000], "strict_hits": "post-hoc (абсорб след. тик)",
    }, indent=1))


if __name__ == "__main__":
    main()
