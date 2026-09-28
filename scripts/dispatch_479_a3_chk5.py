#!/usr/bin/env python3
"""dispatch_479_a3_chk5.py — [479-A3] chkclimb-5 окно 2/3 -> 3/3: зонд-дроу ×2 БЕЗ гейта
+ фид-волна c9..c12 (A19-рецепт ×478, тик x479, v19.0).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча board-коммитом db887807):
  chkclimb-5 (BANK_V5_FREEZE §2 v5-FROZEN, пороги НЕ двигать):
    лег +18.40, порог якоря norm_v5 ≤ −1.60, окно E=[6427199,6527199].
    near-bar страж: a59/a297/a237/a268 = +19.5..+19.7 (якоря 0.3-0.5пп от бара).
    Окно 2/3 хитов — 3-й хит = МЕРЖ-кандидат / цикл закона 18.
  Факт ×478 (A19): пул на кромках окна — c3 6,527,255 (+56 над верхом),
    c8 −42k под низом, c1 +56,733; A18-урок: low-mode live-пула дрейфует
    ~650k/тик -> на x479 дрейф играет В окно. Пул шума 6.32–7.61M (B2 ×6).

ПЛАН (0 код-дельт, vanilla @master 70d64190 = origin/master тика x479,
ref-push алиасов, 1 реф=1 диспатч Л188b GET-страж):
  (1) 2 зонд-диспатча vanilla БЕЗ cpu-гейта (алиасы round-479-a3-z1/z2,
      cpu_band_min/max="") -> ценз live-пула: runner_cpu_index из run-env.txt
      (ин-флайт ценз = grep джоб-лога "run-env: ... runner_cpu_index=").
  (2) ОТ ФАКТА зонда: 4 фида с мишенями окна (алиасы round-479-a3-c9..c12,
      gate = ОКНО [6427199,6527199]; fast-fail вне окна = pairing-discard,
      НЕ вердикт; канон-банд 6.0–9.5M = валидность, окно ⊂ банд).
  (3) Канон x466-C98 ЯВНЫМ JSON (урок C66-C72): 640/300s/fp4/gc3/ic1/fd1/rt4/
      bc1/pop150k/seed42/10G/xms4G, lever=""/"" (ваниль-якорь).
  (4) norm A1-метод (normtool_478 / b5_norm_verdict.absorb bit-exact):
      polls-median C55 (<15.0) / tps_exp_v5 interp §2 / HOST-ценз M1
      (STW_total ≤23.0s ∧ young_avg ≤200ms); cpu ТОЛЬКО из run-env.txt.
      hit = cpu∈окно ∧ norm_v5 ≤ −1.60 ∧ CLEAN M1 ∧ FIXTURE-VALID ∧ vanilla
      -> pair = 18.40 − norm -> board [479-A3] THIRD-HIT {run id, cpu, pair}.
      0/N в окне -> REFUTED_CENS 0/N + ценз пула. Runs >15 мин -> DISPATCHED
      run-id (закон 18-iii).

Usage:
  dispatch_479_a3_chk5.py dispatch probes|feeds|all [--dry-run]
  dispatch_479_a3_chk5.py census              # ин-флайт ценз зондов (лог-греп)
  dispatch_479_a3_chk5.py poll probes|feeds
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "70d641907769d982497fafbb3c56fb387fe2b204"  # origin/master x479 (vanilla, lever ∅)

PROBES = ["round-479-a3-z1", "round-479-a3-z2"]                 # БЕЗ гейта
FEEDS = [f"round-479-a3-c{i}" for i in range(9, 13)]            # мишени окна

# окно E chkclimb-5 = gate (A18/A19-рецепт); канон-банд 6.0–9.5M = валидность
WIN_MIN, WIN_MAX = "6427199", "6527199"
LEG_V5 = 18.40   # chkclimb-5 лег (BANK_V5_FREEZE §2 FROZEN)
THRESH = -1.60   # порог якоря FROZEN
ART_DIR = "/home/z/rounds/ROUND-479/A3/art"

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
}
INPUTS_BY_BRANCH = {}
for _b in PROBES:
    INPUTS_BY_BRANCH[_b] = dict(BASE_INPUTS, cpu_band_min="", cpu_band_max="")  # БЕЗ гейта
for _b in FEEDS:
    INPUTS_BY_BRANCH[_b] = dict(BASE_INPUTS, cpu_band_min=WIN_MIN, cpu_band_max=WIN_MAX)


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
        api(tok, "/repos/{REPO}/git/refs".format(REPO=REPO), method="POST",
            data={"ref": ref, "sha": PIN_SHA})
        print(f"ref CREATED: {br} @ {PIN_SHA[:8]}", flush=True)
    except urllib.error.HTTPError as e:
        if e.code == 422:
            cur = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
            if cur != PIN_SHA:
                if runs_on_branch(tok, br):
                    raise SystemExit(f"REF CONFLICT + runs exist: {br} @ {cur[:8]}")
                # wave-1 висячий реф (0 runs, 1 реф=1 диспатч не потрачен)
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


def runs_on_branch(tok, br, per=60):
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
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        gate = "NO-GATE(probe)" if br in PROBES else f"WIN[{WIN_MIN},{WIN_MAX}]"
        print(f"dispatched: {br} gate={gate} (expect 204)", flush=True)
        time.sleep(20)
    ids, deadline = {}, time.time() + 420
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
        print(f"=== A3 branch={br} pin={PIN_SHA[:8]} run={h[0] if h else 'POLL-NEEDED'} "
              f"status={h[1] if h else '-'} concl={h[2] if h else '-'} ===", flush=True)


def census_from_logs(tok, run_id):
    """Ин-флайт ценз: runner_cpu_index из джоб-лога (run_world3.sh пишет
    'run-env: ... runner_cpu_index=' до завершения рана)."""
    try:
        jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs")["jobs"]
        for j in jobs:
            log = api(tok, f"/repos/{REPO}/actions/jobs/{j['id']}/logs", raw=True)
            m = re.search(rb"runner_cpu_index[:=]\s*(\d+)", log)
            if m:
                return int(m.group(1))
    except Exception as e:
        print(f"  log-ERR {e}", flush=True)
    return None


def do_census():
    """Ценз live-пула по зондам z1/z2: лог-греп (ин-флайт) или артефакт."""
    tok = token()
    for br in PROBES:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        idx = census_from_logs(tok, rid)
        src = "joblog"
        if idx is None and st == "completed":
            try:
                sys.path.insert(0, "/home/z/c-crussty/scripts")
                from normtool_478 import norm_run
                r = norm_run(rid, ART_DIR)
                idx = r.get("cpu_index") or None
                src = "run-env.txt"
            except Exception as e:
                print(f"  art-ERR {e}", flush=True)
        print(f"{br}: run {rid} status={st} concl={concl} cpu_index={idx} (src={src}) "
              f"in_win={bool(idx) and int(WIN_MIN) <= idx <= int(WIN_MAX)} "
              f"in_band={bool(idx) and 6000000 <= idx <= 9500000}", flush=True)


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
                if br in PROBES:
                    idx = census_from_logs(tok, rid)
                    src = "joblog"
                    if idx is None:
                        sys.path.insert(0, "/home/z/c-crussty/scripts")
                        from normtool_478 import norm_run
                        r = norm_run(rid, ART_DIR)
                        idx = r.get("cpu_index")
                        src = "run-env.txt"
                    line += (f" cpu={idx} (src={src}) "
                             f"in_win={bool(idx) and int(WIN_MIN) <= idx <= int(WIN_MAX)} "
                             f"in_band={bool(idx) and 6000000 <= idx <= 9500000}")
                else:
                    sys.path.insert(0, "/home/z/c-crussty/scripts")
                    from b5_norm_verdict import absorb
                    r = absorb(rid, ART_DIR)
                    if r.get("verdict") == "NO-ARTIFACT":
                        idx = census_from_logs(tok, rid)
                        line += f" BAND-DEAD fast-fail idx={idx}" if idx else " NO-ARTIFACT"
                    else:
                        in_win = int(WIN_MIN) <= r["cpu_index"] <= int(WIN_MAX)
                        hit = (in_win and r["norm_v5"] is not None and r["norm_v5"] <= THRESH
                               and not r["clean_M1"]["host"] and r["fixture_valid"]
                               and r["vanilla"]["lever_empty"])
                        line += (f" cpu={r['cpu_index']} in_win={in_win} "
                                 f"polls={r['tps_polls'][:5]} med={r['tps_med']} "
                                 f"exp={r['tps_exp_v5']} norm={r['norm_v5']} "
                                 f"M1_host={r['clean_M1']['host']} fix={r['fixture_valid']} "
                                 f"van={r['vanilla']['lever_empty']} verdict={r['verdict']}")
                        if in_win and r["norm_v5"] is not None:
                            line += f" pair={LEG_V5 - r['norm_v5']:+.2f} HIT={hit}"
            except Exception as e:
                line += f" absorb-ERR {e}"
        print(line, flush=True)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "dispatch"
    rest = [a for a in sys.argv[2:] if not a.startswith("--")]
    if cmd == "census":
        do_census()
    elif cmd == "poll":
        sel = rest[0] if rest else "all"
        branches = PROBES + FEEDS if sel == "all" else PROBES if sel == "probes" else FEEDS
        do_poll(branches)
    elif cmd == "dispatch":
        sel = rest[0] if rest else "all"
        branches = PROBES + FEEDS if sel == "all" else PROBES if sel == "probes" else FEEDS
        do_dispatch("--dry-run" in sys.argv, branches)
    else:
        raise SystemExit(__doc__)
