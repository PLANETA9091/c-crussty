#!/usr/bin/env python3
"""dispatch_479_a4_wave.py — COMMANDER 479-A4: STRICT-добивка банка Л201, волна ×4 ваниль-рунов.

CLAIM (банк Л201 n=27/30 после s2-адмита, дефицит 3): STRICT-окно [6.9,7.2]M
(Л201-локальный узел 2.1293) — 4 ваниль-руна bank-feed. Задача: 4 ваниль-руна
@origin/master 0 код-дельт (алиасы round-479-a4-s1..s4, pin FULL-sha origin/master
19b45ac5a49330a87f62a76df841be7fca896184, дрейф 70d64190→19b45ac5 docs/scripts-only
верифицирован diff-статом: BLACKBOARD.md + dispatch_479_a7/a8 — 0 src/|.github|Cargo|pom),
canon x466-C98 ЯВНЫМ JSON (урок C66-C72), band GLOB [6000000,9500000] fast-fail —
окно [6.9,7.2]M в band-гейт НЕ ставится (канон Л201/Л-470-S31.1: банк-фид полным
band + пост-хок idx-фильтр; щель → band-dead ×все запрещён). В-точка STRICT ⇔
cpu(run-env.txt, Л195) ∈ [6.9,7.2]M ∧ CLEAN M1 (STW_total ≤23.0s ∧ young_avg ≤200ms,
gc.log-primary Л118) ∧ ваниль-VALID (armed-lever ∅, NCDFE=0, AIOOBE=0, FIXTURE VALID,
Л209) ∧ norm ∈ ваниль-коридор [−8.0,+1.5] (Л143/Л240.1); CLEAN вне окна → банк-фид
§3 полным band. Пороги BANK_V5_FREEZE §2 v5-FROZEN НЕ двигать (§5), закон-5: lever ∅.
Band-miss → free discard (Л188c), ре-ролл канон W3. Атрибуция run-ids: снапшот ДО
диспатча + фильтр по branch+head_sha (Л-470-S31.1 кросс-агентский урок). 1 диспатч =
1 ветка (Л188b), POST /git/refs FULL-sha + GET-верификация object.sha ДО диспатча
(Л188a/S20-урок: короткий sha = 422).
Прегист закон 14a/16 — этот docstring; LEDGER Л-479-A4.
"""
import json, subprocess, sys, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIASES = ["round-479-a4-s1", "round-479-a4-s2", "round-479-a4-s3", "round-479-a4-s4"]
PIN = "19b45ac5a49330a87f62a76df841be7fca896184"  # origin/master FULL sha (479-A8 board)
BASE_CLAIM = "70d64190"  # база из CLAIM командира — дрейф до PIN проверяем
STRICT_LO, STRICT_HI = 6_900_000, 7_200_000  # STRICT-окно Л201 (пост-хок фильтр)

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


def drift_check(tok, pin):
    """0-дельт-страж: diff-stat BASE_CLAIM→pin не должен трогать bench-поверхности."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    pin = live if live.startswith(PIN[:12]) else PIN
    stat = subprocess.run(
        ["git", "-C", "/home/z/c-crussty", "diff", "--stat", BASE_CLAIM, pin[:12]],
        capture_output=True, text=True).stdout
    hot = [ln for ln in stat.splitlines()
           if ("src/" in ln or ".github/" in ln or "Cargo" in ln or "pom" in ln
               or "native/" in ln or "bench/" in ln)]
    if hot:
        print("DRIFT-HOT files (bench surface touched!):", *hot, sep="\n", file=sys.stderr)
        sys.exit(3)
    n_files = stat.strip().splitlines()[-1] if stat.strip() else "0 files"
    print(f"drift {BASE_CLAIM}->{pin[:8]}: docs/scripts-only OK ({n_files})")
    return pin


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
        data={"ref": f"refs/heads/{name}", "sha": sha})
    # GET-верификация ДО диспатча (Л188a)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha", "")
    ok = v == sha
    print(f"{name}: created FULL-sha @ {sha[:8]} GET-verify {'OK' if ok else 'MISMATCH ' + v[:8]}")
    if not ok:
        sys.exit(4)


def snapshot_run_ids(tok):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=30")
    return {x["id"] for x in r.get("workflow_runs", [])}


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} -> {'204 OK' if r == {} else r}")
    return r == {}


def main():
    tok = token()
    pin = drift_check(tok, PIN)
    for a in ALIASES:
        ensure_alias(tok, a, pin)
    before = snapshot_run_ids(tok)
    print(f"run-id snapshot BEFORE: {len(before)} runs")
    ok = 0
    for a in ALIASES:
        ok += dispatch(tok, a)
    if ok != len(ALIASES):
        print(f"DISPATCH-INCOMPLETE {ok}/{len(ALIASES)}", file=sys.stderr)
        sys.exit(5)
    print("waiting 25s for run creation...")
    import time
    time.sleep(25)
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=40")
    mine = {}
    for x in r.get("workflow_runs", []):
        if x["head_branch"] in ALIASES and x["head_sha"].startswith(pin[:12]) \
                and x["id"] not in before:
            mine.setdefault(x["head_branch"], []).append(x["id"])
    print("RUN-IDS (branch-filtered, post-snapshot — канон Л-470-S31.1):")
    for a in ALIASES:
        print(f"  {a}: {mine.get(a, 'NOT-FOUND')}")


if __name__ == "__main__":
    main()
