#!/usr/bin/env python3
"""dispatch_478_a19_chk5.py — [478-A19] chkclimb-5 окно 2/3 -> 3/3 ФИДЫ (тик x478, v19.0).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча board-коммитом):
  chkclimb-5 (BANK_V5_FREEZE §2 v5-FROZEN, пороги НЕ двигать):
    лег +18.40, порог якоря norm_v5 ≤ −1.60, окно E=[6427199,6527199].
    near-bar страж: a59/a297/a237/a268 = +19.5..+19.7 (якоря 0.3-0.5пп от бара).
    Окно 2/3 хитов — 3-й хит = МЕРЖ-кандидат №18 / цикл закона 18.
  Фон: W2-o12 6,539,455 промах +12k (над верхом окна); пул тика ×478
    шумит 6.32–7.61M (B2 ×6) + Y4 6,986,487 (02:30Z) — медленные раннеры есть.

ПЛАН (0 код-дельт, vanilla @master, ref-push алиасов, 1 реф=1 диспатч Л188b):
  4 ваниль-фида алиасы round-478-a19-c1..c4 @PIN (master 386887a8; дрейф
  848d8f14→HEAD docs/scripts-only = 0 Java/Rust файлов, diff-верифицирован).
  Канон x466-C98 ЯВНЫМ JSON (урок C66-C72): 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/
  pop150k/seed42/10G/xms4G, lever=""/""; gate = ОКНО [6427199,6527199]
  (A18-фид-рецепт: fast-fail вне окна = pairing-discard НЕ вердикт;
  канон-банд 6.0–9.5M = валидность, окно ⊂ банд).
  Band-miss → 1 ре-ролл на промах (алиасы round-478-a19-c5..c8, канон W3).
  norm A1-метод (Л-478-A1.1 bit-exact, b5_norm_verdict.absorb):
  polls-median C55 (<15.0) / tps_exp_v5 interp §2 / HOST-ценз M1
  (STW_total ≤23.0s ∧ young_avg ≤200ms); cpu ТОЛЬКО из run-env.txt.
  hit = cpu∈окно ∧ norm_v5 ≤ −1.60 ∧ CLEAN M1 ∧ FIXTURE-VALID ∧ vanilla
  → pair = 18.40 − norm → board [478-A19] THIRD-HIT {run id, cpu, pair}.
  Runs >15 мин → DISPATCHED run-id (закон 18-iii).

Usage: dispatch_478_a19_chk5.py [dispatch|poll] [--dry-run]
"""
import json, re, subprocess, sys, time, zipfile, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "386887a82f6647311a1d2005172507af09821256"

FEEDS = [f"round-478-a19-c{i}" for i in range(1, 5)]
REROLLS = [f"round-478-a19-c{i}" for i in range(5, 9)]  # по 1 на band-miss

# окно E chkclimb-5 = gate (A18-рецепт); канон-банд 6.0–9.5M = валидность
WIN_MIN, WIN_MAX = "6427199", "6527199"
LEG_V5 = 18.40   # chkclimb-5 лег (BANK_V5_FREEZE §2 FROZEN)
THRESH = -1.60   # порог якоря FROZEN

BASE_INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G", "gc_tune": "3",
    "lever_flag": "", "lever_arg": "",
    "cpu_band_min": WIN_MIN, "cpu_band_max": WIN_MAX,
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, raw=False):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}", flush=True)
        raise
    return body if raw else (json.loads(body) if body else {})


def ensure_ref(tok, br):
    ref = f"refs/heads/{br}"
    try:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": ref, "sha": PIN_SHA})
        print(f"ref CREATED: {br} @ {PIN_SHA[:8]}", flush=True)
    except urllib.error.HTTPError as e:
        if e.code == 422:
            cur = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
            if cur != PIN_SHA:
                if runs_on_branch(tok, br):
                    raise SystemExit(f"REF CONFLICT + runs exist: {br} @ {cur[:8]}")
                # wave-1 висячий реф (0 runs, 1 реф=1 диспатч не потрачен):
                # выравнивание на PIN (master @диспатч) — PATCH + GET-пруф
                api(tok, f"/repos/{REPO}/git/refs/heads/{br}", method="PATCH",
                    data={"sha": PIN_SHA, "force": True})
                print(f"ref ALIGNED: {br} {cur[:8]} -> {PIN_SHA[:8]} (wave-1 no-run)", flush=True)
            else:
                print(f"ref EXISTS-OK: {br} @ {cur[:8]}", flush=True)
        else:
            raise
    live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
    assert live == PIN_SHA, f"GET-verify fail {br}"
    return live


def runs_on_branch(tok, br, per=40):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page={per}")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def do_dispatch(dry, branches):
    tok = token()
    for br in branches:
        ensure_ref(tok, br)
        if runs_on_branch(tok, br):
            raise SystemExit(f"Л188a VIOLATION: {br} уже имеет run — 1 реф=1 диспатч")
    if dry:
        print("DRY-RUN OK — refs verified, no dispatches", flush=True)
        return
    for br in branches:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": BASE_INPUTS})
        print(f"dispatched: {br} gate=WIN[{WIN_MIN},{WIN_MAX}] (expect 204)", flush=True)
        time.sleep(20)
    ids, deadline = {}, time.time() + 300
    while time.time() < deadline and len(ids) < len(branches):
        time.sleep(15)
        for br in branches:
            if br in ids:
                continue
            hits = runs_on_branch(tok, br)
            if hits:
                ids[br] = hits[0]
                print(f"run-id: {br} -> {hits[0][0]} status={hits[0][1]}", flush=True)
    for br in branches:
        h = ids.get(br)
        print(f"=== A19 branch={br} pin={PIN_SHA[:8]} run={h[0] if h else 'POLL-NEEDED'} "
              f"status={h[1] if h else '-'} concl={h[2] if h else '-'} ===", flush=True)


def fail_idx(tok, run_id):
    """cpu-ценз fast-fail: IDX из лога гейт-степа (band-miss телеметрия)."""
    try:
        jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs")["jobs"]
        for j in jobs:
            log = api(tok, f"/repos/{REPO}/actions/jobs/{j['id']}/logs", raw=True)
            m = re.search(rb"runner_cpu_index=(\d+)", log)
            if m:
                return int(m.group(1))
    except Exception as e:
        print(f"  log-ERR {e}", flush=True)
    return None


def do_poll(branches):
    tok = token()
    for br in branches:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        line = f"{br}: run {rid} status={st} concl={concl} created={ca}"
        if st == "completed":
            try:
                sys.path.insert(0, "/home/z/c-crussty/scripts")
                from b5_norm_verdict import absorb
                r = absorb(rid, "/home/z/rounds/ROUND-478/A19/art")
                if r.get("verdict") == "NO-ARTIFACT":
                    idx = fail_idx(tok, rid)
                    line += f" BAND-DEAD fast-fail idx={idx}" if idx else " NO-ARTIFACT"
                else:
                    line += (f" cpu={r['cpu_index']} in_win={WIN_MIN}<=cpu<={WIN_MAX}: "
                             f"{int(WIN_MIN) <= r['cpu_index'] <= int(WIN_MAX)} "
                             f"polls={r['tps_polls']} med={r['tps_med']} exp={r['tps_exp_v5']} "
                             f"norm={r['norm_v5']} M1={r['clean_M1']} "
                             f"fix={r['fixture_valid']} van={r['vanilla']['lever_empty']} "
                             f"verdict={r['verdict']}")
                    if int(WIN_MIN) <= r["cpu_index"] <= int(WIN_MAX):
                        line += f" pair={LEG_V5 - r['norm_v5']:+.2f} hit={r['norm_v5'] <= THRESH}"
            except Exception as e:
                line += f" absorb-ERR {e}"
        print(line, flush=True)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] in ("dispatch", "poll") else "dispatch"
    rest = [a for a in sys.argv[2:] if not a.startswith("--")]
    if cmd == "poll":
        do_poll(rest or FEEDS)
    else:
        do_dispatch("--dry-run" in sys.argv, rest or FEEDS)
