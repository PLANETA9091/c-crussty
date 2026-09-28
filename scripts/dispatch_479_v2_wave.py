#!/usr/bin/env python3
"""dispatch_479_v2_wave.py — COMMANDER 479-V2 windows-volley (MEGA-SWARM v19.0, тик ×479).

CLAIM (закон 14a/16, прегистрируется board-коммитом ДО диспатчей):
  Окна-фидерная волна ×12: 12 ваниль-диспатчей @master e503160c (0 код-дельт,
  алиасы round-479-v2-o01..o12, canon x466-C98 ЯВНЫМ JSON, 1 реф=1 диспатч
  Л188b runs_before=0). Мишени — 3 окна ×4 фида:
    chkclimb-5   [6427199,6527199]  (окно 2/3, порог norm_v5 ≤−1.60 FROZEN)
    POI          [8907260,9007260]  (окно 2/3, порог ≤−1.99 FROZEN)
    sensn16-якоря [6807260,6907260] (mxa-08 6,849,418 / s1-d 6,888,701 /
                                    mxa-12 6,905,659 — все 3 в зоне; pair Δ≤50k
                                    коридор, пара-пул №17/18)
  gate = ОКНО fast-fail (A3/A18-фид-рецепт: вне окна = pairing-discard НЕ
  вердикт; канон-банд 6.0–9.5M = валидность, окно ⊂ банд). Band-dead → ре-ролл
  ×1 free (канон W3, алиасы round-479-v2-rNN @PIN, 1 на промах).
  Финал ≤40 мин, бенчи НЕ ждать (закон 18-iii/12e: runs >15 мин → DISPATCHED
  run-id): {run-id ×12+} + cpu-ценз по гейт-логам (runner_cpu_index из
  джоб-лога — гейт пишет его ДО скачивания мира, ин-флайт тоже). Числа-абсорб
  след. тик. Каноны: закон-5 запреты; пороги v5-FROZEN НЕ двигать.

Usage:
  dispatch_479_v2_wave.py dispatch [--dry-run]
  dispatch_479_v2_wave.py census              # cpu-ценз по гейт-логам (ин-флайт)
  dispatch_479_v2_wave.py poll [o|r|all]
  dispatch_479_v2_wave.py reroll              # band-dead ×1 → round-479-v2-rNN
"""
import json, re, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "e503160cdf7a4aef55ac282d13f719d4ca05500a"  # origin/master тика ×479 (vanilla, lever ∅)

GROUPS = {
    "chk5":    ("6427199", "6527199"),
    "poi":     ("8907260", "9007260"),
    "sensn16": ("6807260", "6907260"),  # mxa-якоря зона, Δ≤50k коридор
}
ALIASES = {}   # branch -> group
for _i in range(1, 13):
    _g = "chk5" if _i <= 4 else ("poi" if _i <= 8 else "sensn16")
    ALIASES[f"round-479-v2-o{_i:02d}"] = _g

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
for _br, _g in ALIASES.items():
    lo, hi = GROUPS[_g]
    INPUTS_BY_BRANCH[_br] = dict(BASE_INPUTS, cpu_band_min=lo, cpu_band_max=hi)  # gate=ОКНО


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
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        raise


class NoAuthRedirect(urllib.request.HTTPRedirectHandler):
    """Джоб-логи ин-флайт = 302 на Azure blob: НЕ пересылать Bearer (урок A3)."""

    def redirect_request(self, req, fp, code, msg, headers, newurl):
        if "Authorization" in req.headers:
            del req.headers["Authorization"]
        req.headers.pop("Authorization", None)
        return urllib.request.HTTPRedirectHandler.redirect_request(
            self, req, fp, code, msg, headers, newurl)


_opener = urllib.request.build_opener(NoAuthRedirect)


def job_log_raw(tok, job_id):
    req = urllib.request.Request(
        f"{API}/repos/{REPO}/actions/jobs/{job_id}/logs",
        headers={"Authorization": f"Bearer {tok}",
                 "Accept": "application/vnd.github+json"})
    with _opener.open(req, timeout=120) as r:
        return r.read()


def runs_on_branch(tok, br, per=60):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page={per}")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def ensure_ref(tok, br):
    """1 реф=1 диспатч Л188b: FULL-sha, GET-верификация ДО диспатча."""
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
                api(tok, f"/repos/{REPO}/git/refs/heads/{br}", method="PATCH",
                    data={"sha": PIN_SHA, "force": True})
                print(f"ref ALIGNED: {br} {cur[:8]} -> {PIN_SHA[:8]} (no-run)", flush=True)
            else:
                print(f"ref EXISTS-OK: {br} @ {cur[:8]}", flush=True)
        else:
            raise
    live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
    assert live == PIN_SHA, f"GET-verify fail {br}"
    return live


def census_cpu(tok, run_id):
    """cpu-ценз по гейт-логам: runner_cpu_index пишется гейтом ДО бенча."""
    try:
        jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs")["jobs"]
        for j in jobs:
            log = job_log_raw(tok, j["id"])
            m = re.search(rb"runner_cpu_index[:=]\s*(\d+)", log)
            if m:
                return int(m.group(1))
    except Exception as e:
        print(f"  log-ERR {e}", flush=True)
    return None


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
        lo, hi = GROUPS[ALIASES[br]]
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        print(f"dispatched: {br} WIN[{lo},{hi}] (expect 204)", flush=True)
        time.sleep(20)


def do_census(branches):
    tok = token()
    for br in branches:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        idx = census_cpu(tok, rid)
        g = _group_of(br)
        win = f"[{GROUPS[g][0]},{GROUPS[g][1]}]" if g else "?"
        in_win = idx is not None and g and int(GROUPS[g][0]) <= idx <= int(GROUPS[g][1])
        print(f"{br}: run {rid} status={st} concl={concl} win={win} "
              f"cpu={idx} in_win={in_win}", flush=True)


def _group_of(br):
    if br in ALIASES:
        return ALIASES[br]
    m = re.match(r"round-479-v2-r(\d+)$", br)
    if m:
        i = int(m.group(1))
        return "chk5" if i <= 4 else ("poi" if i <= 8 else "sensn16")
    return None


def do_poll(branches):
    tok = token()
    for br in branches:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        g = _group_of(br)
        lo, hi = GROUPS[g] if g else ("?", "?")
        idx = census_cpu(tok, rid)
        in_win = bool(idx) and idx.isdigit() if False else (
            idx is not None and int(lo) <= idx <= int(hi))
        print(f"{br}: run {rid} status={st} concl={concl} win=[{lo},{hi}] "
              f"cpu={idx} in_win={in_win} created={ca}", flush=True)


def do_reroll():
    """Band-dead ×1 free (канон W3): completed failure = fast-fail вне окна."""
    tok = token()
    n = 0
    for br in sorted(ALIASES):
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN (poll first)", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        if st == "completed" and concl == "failure":
            idx = census_cpu(tok, rid)
            rb = br.replace("-o", "-r")
            print(f"band-dead {br} run {rid} cpu={idx} -> re-roll {rb}", flush=True)
            ensure_ref(tok, rb)
            if runs_on_branch(tok, rb):
                print(f"  SKIP: {rb} уже имеет run", flush=True)
                continue
            lo, hi = GROUPS[ALIASES[br]]
            api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
                data={"ref": rb, "inputs": dict(
                    INPUTS_BY_BRANCH[br], cpu_band_min=lo, cpu_band_max=hi)})
            print(f"  re-roll dispatched 204 @WIN[{lo},{hi}]", flush=True)
            time.sleep(20)
            n += 1
    print(f"re-rolls dispatched: {n}", flush=True)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "dispatch"
    rest = [a for a in sys.argv[2:] if not a.startswith("--")]
    if cmd == "census":
        do_census(list(ALIASES))
    elif cmd == "poll":
        sel = rest[0] if rest else "all"
        if sel == "o":
            do_poll(list(ALIASES))
        elif sel == "r":
            do_poll([f"round-479-v2-r{i:02d}" for i in range(1, 13)])
        else:
            do_poll(list(ALIASES) + [f"round-479-v2-r{i:02d}" for i in range(1, 13)])
    elif cmd == "reroll":
        do_reroll()
    elif cmd == "dispatch":
        do_dispatch("--dry-run" in sys.argv, list(ALIASES))
    else:
        raise SystemExit(__doc__)
