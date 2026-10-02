#!/usr/bin/env python3
"""dispatch_483_a04_960s.py — COMMANDER 483-A04 нога2: 960s-марафон ×1 (C45-класс, WILD).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча, round-483):
  960s = длинное окно стресс-лестницы ×1. t_stab-лестница: 150s (A04-нога1) <
  канон 300s < 600s-плато (W7: плато 2.6 к poll 9-11 ≈ 450-550s, t_stab
  [250,500]s). ГЛАВНЫЙ вопрос прегиста: t_stab-хвост >500s — норма стабильна
  на [500,960]s? W1-урок (600s-gc7): young-масса копится — young 168→318,
  STW +4.52s, tail-min 1.8 < плато 2.6 → хвост НЕ бесплатен. На 960s копление
  young-долга ожидаемо сильнее (600s → 960s = ×1.6 окно). 0 код-дельт: vanilla
  @acffa383 (×482-учёт master).

ПЛАН (паттерн dispatch_480_c45_960s.py: ensure_alias FULL-sha GET-verify →
dispatch → run-id; 1 реф=1 диспатч Л188b): алиас round-483-a04-960s → refs
@ FULL-sha acffa3839b09a3388d4949d3767ab0a15b4fcd94, canon x466-C98 ЯВНЫЙ JSON
(640/960s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G, lever=""), band GLOB
[6000000,9500000] fast-fail. Band-miss (fail ≤180s) → ре-ролл ≤2
round-483-a04-960s-r2/-r3 (закон W3). Runs >15 мин → DISPATCHED run-id
(закон 12e/18-iii) — 960s-бенч ≈ 40+ мин НЕ ждать, число пост-хок на абсорбе.

Usage: dispatch_483_a04_960s.py [--dry-run] | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # origin/master ×482-учёт, vanilla lever ∅
BASE_CLAIM = "acffa383"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIAS = "round-483-a04-960s"
REROLLS = [ALIAS + "-r2", ALIAS + "-r3"]   # ≤2 ре-ролла (закон W3)
PAUSE_S = 20
BAND_DEAD_S = 180
BAND_PROOF_S = 240      # in_progress после этого = band-гейт пройден (C24: band-dead пал на 39-41s)
COLLECT_TIMEOUT_S = 330

# канон x466-C98 — ЯВНЫЙ JSON (yml-дефолты = merge-поверхность, урок x466-C73);
# дельта от 300s-канона ТОЛЬКО seconds=960 (марафон-ячейка стресс-лестницы)
INPUTS = {
    "radius": "640", "seconds": "960", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}
# сравнительная лестница (числа-якоря прегиста)
LADDER = {
    "a04_150s":  {"alias": "round-483-a04-150s", "status": "same-wave"},
    "canon_300s": {"runs": "w2 36386470310 / w4 36386521687"},
    "w7_600s":   {"run": 36369278270, "cpu_index": 7022396, "norm_v5": 17.64,
                  "stw_total_s": 27.5845, "young_n": 168,
                  "plateau": 2.6, "t_stab": "[250,500]s"},
    "w1_600s_gc7_lesson": {"run": 36376772554, "dSTW_s": 4.52, "young_n": 318,
                           "tail_min": 1.8, "verdict": "REFUTED_CENS"},
}
TAIL_GATE_MIN = 2.3     # gap W1-tail-min 1.8 (young-mass деградация) ↔ W7-плато 2.6


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
    """0-дельт-страж: CLAIM-база acffa383 → live master не трогает bench-поверхности."""
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


def dispatch_one(tok, ref):
    ensure_alias(tok, ref, PIN)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {ref} seconds={INPUTS['seconds']} "
          f"band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] -> {'204 OK' if ok else r}")
    if not ok:
        sys.exit(2)
    time.sleep(PAUSE_S)
    run = latest_run(tok, ref, mc) or {"id": None, "status": "POLL-NEEDED", "created": mc}
    print(json.dumps({"alias": ref, **run}))
    return run


def band_watch(tok, ref, run):
    """Ждём band-вердикт: failure ≤180s = band-dead (→ ре-ролл W3); in_progress ≥
    BAND_PROOF_S = band-гейт пройден, бенч в полёте → DISPATCHED (закон 12e/18-iii).
    Марафон 960s ≈ 40+ мин НЕ ждём (закон 12e)."""
    t0 = time.time()
    while time.time() - t0 < COLLECT_TIMEOUT_S:
        if run.get("id") and run.get("status") == "completed":
            return run
        time.sleep(30)
        fresh = latest_run(tok, ref, time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 1800)))
        if fresh:
            run = fresh
            el = int(time.time() - t0)
            print(f"{ref}: run {run['id']} status={run['status']} concl={run['conclusion']} (+{el}s)")
            if run["status"] == "completed":
                return run
    return run


def is_band_miss(run):
    """fast-fail failure ≤ BAND_DEAD_S+60 от создания = band-miss."""
    if run.get("conclusion") != "failure":
        return False
    try:
        t0 = time.mktime(time.strptime(run["created"], "%Y-%m-%dT%H:%M:%SZ"))
    except Exception:
        return True
    return time.time() - t0 <= BAND_DEAD_S + 60


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    if dry:
        ensure_alias(tok, ALIAS, live)
        print("DRY-RUN OK — ref GET-verified, 0 диспатчей")
        return
    chain = [(ALIAS, dispatch_one(tok, ALIAS))]
    for rr in REROLLS:
        ref, run = chain[-1]
        run = band_watch(tok, ref, run)
        if is_band_miss(run) and len(chain) <= len(REROLLS):
            print(f"{ref}: BAND-MISS fast-fail → ре-ролл {rr} (закон W3, ≤2)")
            chain.append((rr, dispatch_one(tok, rr)))
        else:
            break
    print("A04-960S-DISPATCH-JSON " + json.dumps(
        {"pin": PIN[:12], "seconds": INPUTS["seconds"], "tail_gate_min": TAIL_GATE_MIN,
         "ladder": LADDER, "chain": {r: x for r, x in chain}}, default=str))


def do_poll():
    tok = token()
    for ref in [ALIAS] + REROLLS:
        run = latest_run(tok, ref, "2026-09-28T00:00:00Z")
        if run:
            print(f"{ref}: run {run['id']} status={run['status']} concl={run['conclusion']}")
        else:
            print(f"{ref}: NO-RUN")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
