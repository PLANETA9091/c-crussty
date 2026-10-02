#!/usr/bin/env python3
"""dispatch_487_c64_bu1.py — [487-C64] bu1-семья №20 #5 rep-докатка (тик ×487, окно [8734563,8834563]).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча, см. /home/z/rounds/ROUND-487/board/CLM-C64.md):
  Бит-в-бит реплика bu1-канона №20 (LEDGER Л1270/Л1308: нога A17-bu1 +44.36, run 36444248520,
  ветка round-483-a17-w8bu1-r2 @ acffa3839b09a3388d4949d3767ab0a15b4fcd94, bare pin 0 код-дельт).
  Вектор скопирован 1:1 из scripts/dispatch_483_a17_w8bu1.py; ЕДИНСТВЕННЫЕ дельты (приказ командира):
    (1) алиасы round-487-c64-bu1r1/-r2/-r3 (1 диспатч = 1 ветка, канон Л188b — 3 независимые
        concurrency-группы, parallel per-ref — единственная поверхность ×486);
    (2) cpu_band 6000000/9500000 -> 8734563/8834563 (окно Л142 в gate; fast-fail -> ре-ролл).
  bu_defer=1 = S7-168 STEAL-v2 фикс; при region_steal=0 инертен, канон-совместим (Л1270, NPE s7176=0).

Координация C63: CLM-C63.md нет, веток round-487-c63* нет (fetch 08:4xZ) — нумерация r1/r2/r3 за C64.

Usage:
  dispatch_487_c64_bu1.py dispatch   # refs ×3 (FULL-sha + GET-verify) + dispatches ×3 + run-id discovery
  dispatch_487_c64_bu1.py poll       # gate-вердикты по каждому алиасу (calibration step)
  dispatch_487_c64_bu1.py reroll     # ре-ролл band-dead алиасов (attempt+1, макс 3 попыток/ролл)
"""
import json, os, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
STATE = "/home/z/rounds/ROUND-487/c64_dispatch.json"

PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # bu1-канон пин (bare, 0 код-дельт, ancestor master 85a06f2f)
ALIASES = ["round-487-c64-bu1r1", "round-487-c64-bu1r2", "round-487-c64-bu1r3"]
MAX_ATTEMPTS = 3  # ролл + до 2 ре-роллов после band-dead (Fast-fail -> ре-ролл, приказ)

# БИТ-В-БИТ из dispatch_483_a17_w8bu1.py (canon bu1), дельта только cpu_band (окно).
INPUTS = {
    "radius": "480",
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6",
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0",
    "bu_defer": "1",  # bu1-ось (канон №20 config-leg)
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "8734563", "cpu_band_max": "8834563",  # ДЕЛЬТА C64: окно Л142 в gate
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",
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
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha")


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40&branch={br}")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"), r.get("run_attempt"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br and r.get("head_sha") == PIN]


def gate_verdict(tok, run_id):
    """calibration step verdict: 'pass' | 'band-dead' | 'pending' | 'other-fail'."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=5")
    for j in jobs.get("jobs", []):
        if j.get("status") != "completed":
            return "pending"
        for s in j.get("steps", []):
            if "calibration" in (s.get("name") or "").lower():
                if s.get("conclusion") == "failure":
                    return "band-dead"
                if s.get("conclusion") == "success":
                    return ("gate-pass" if j.get("conclusion") == "success" else "other-fail")
    return "pending"


def load_state():
    return json.load(open(STATE)) if os.path.exists(STATE) else {"legs": {a: {"attempts": []}} for a in ALIASES}


def save_state(st):
    json.dump(st, open(STATE, "w"), indent=1)


def dispatch_alias(tok, br, st):
    leg = st["legs"].setdefault(br, {"attempts": []})
    if len(leg["attempts"]) >= MAX_ATTEMPTS:
        print(f"{br}: attempt-cap {MAX_ATTEMPTS} reached — skip")
        return
    pre = runs_on_branch(tok, br)
    if any(r[1] not in ("completed",) for r in pre):
        print(f"{br}: prior run still {pre} — NOT re-dispatching (cancel-guard)")
        return
    cur = ref_sha(tok, br)
    if cur != PIN:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{br}", "sha": PIN})  # FULL-sha (урок S20: 422 на коротком)
            print(f"ref CREATED {br} @ {PIN[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{br}", method="PATCH",
                data={"sha": PIN, "force": True})
            print(f"ref PATCHED {br} -> {PIN[:8]}", flush=True)
    got = ref_sha(tok, br)
    if got != PIN:
        raise SystemExit(f"POST-CREATE VERIFY FAIL {br} (Л188a)")
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {br} -> {'204-OK' if ok else r}", flush=True)
    if not ok:
        return
    rid = None
    deadline = time.time() + 200
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, status, conc, ca, att in runs_on_branch(tok, br):
            rid = rid_
            break
    leg["attempts"].append({"run_id": rid, "dispatched_at": time.strftime("%H:%M:%SZ", time.gmtime())})
    print(f"{br}: RUN-ID {rid}", flush=True)


def cmd_dispatch():
    tok = token()
    master = ref_sha(tok, "master")
    print(f"origin/master live = {master} (НЕ трогаем) -> пин роллов {PIN[:8]} (bu1-канон acffa383)", flush=True)
    st = load_state()
    for a in ALIASES:
        if ref_sha(tok, a) is not None and not st["legs"].get(a, {}).get("attempts"):
            raise SystemExit(f"ALIAS COLLISION: {a} already exists on remote — C63/C-other pre-empt?")
    for a in ALIASES:
        dispatch_alias(tok, a, st)
    st["inputs"] = INPUTS
    st["pin"] = PIN
    save_state(st)


def cmd_poll():
    tok = token()
    st = load_state()
    for a in ALIASES:
        leg = st["legs"].setdefault(a, {"attempts": []})
        runs = runs_on_branch(tok, a)
        verdicts = []
        for rid, status, conc, ca, att in runs:
            if status != "completed":
                verdicts.append({"run_id": rid, "state": "in-flight", "attempt": att})
                continue
            v = gate_verdict(tok, rid)
            verdicts.append({"run_id": rid, "state": conc, "gate": v, "attempt": att})
        leg["verdicts"] = verdicts
        print(f"{a}: {json.dumps(verdicts)}", flush=True)
    save_state(st)


def cmd_reroll():
    tok = token()
    st = load_state()
    for a in ALIASES:
        leg = st["legs"].setdefault(a, {"attempts": []})
        runs = runs_on_branch(tok, a)
        band_dead = [r for r in runs if r[1] == "completed" and gate_verdict(tok, r[0]) == "band-dead"]
        live = [r for r in runs if r[1] != "completed"]
        passed = any(r for r in runs if r[1] == "completed" and gate_verdict(tok, r[0]) == "gate-pass")
        if passed:
            print(f"{a}: gate-pass run exists — roll SETTLED, no re-roll")
            continue
        if live:
            print(f"{a}: run {live} in-flight — wait (cancel-guard)")
            continue
        if band_dead and len(leg["attempts"]) < MAX_ATTEMPTS:
            print(f"{a}: band-dead x{len(band_dead)} -> RE-ROLL (attempt {len(leg['attempts'])+1}/{MAX_ATTEMPTS})")
            dispatch_alias(tok, a, st)
        elif band_dead:
            print(f"{a}: band-dead x{len(band_dead)}, attempt-cap {MAX_ATTEMPTS} — roll DEAD")
    save_state(st)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else ""
    if cmd == "dispatch":
        cmd_dispatch()
    elif cmd == "poll":
        cmd_poll()
    elif cmd == "reroll":
        cmd_reroll()
    else:
        raise SystemExit(__doc__)
