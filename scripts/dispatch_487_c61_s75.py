#!/usr/bin/env python3
"""dispatch_487_c61_s75.py — [487-C61] sensn16 [7.5,7.7]M vanilla-draw добивка ×2 ролла (тик ×487, Job 415026).

CLAIM (prereg, CLM-C61.md): ваниль-нога (lever_flag=∅) канон-вектора BANK-V5,
band-дельта = ЕДИНСТВЕННАЯ: cpu_band [7500000, 7700000] (плоскость sensn16; canon-компо
у C36 run 36507467885 — НЕ дублируем lever). МАКС 2 ролла: round-487-c61-s75r1/-r2
@ PIN 85a06f2f (1 ветка = 1 ран, Л188b). Band fast-fail = бесплатный pairing-дискард
(S7-96d) → фид cpu_index пула → ре-ролл (макс 2). Цель: окна-фиды для canon-питчей
(подокно пары [7.547,7.647]M пусто 0/22) + сбор данных пула.
Поверхность: world-bench-parallel.yml (инфра-канон ×486, parallel per-ref).

Usage: dispatch_487_c61_s75.py [--dry-run]
"""
import json, re, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)
GATE_STEP = "Runner calibration band gate (pair-hunter fast-fail, S7-96d pairing law)"

BRANCHES = ["round-487-c61-s75r1", "round-487-c61-s75r2"]  # макс 2 ролла (fast-fail → учёт)

# x466-C98 BANK-V5 канон; band [7.5,7.7]M = единственная дельта (плоскость sensn16)
INPUTS = {
    "radius": "640", "seconds": "300",
    "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4", "batch_collector": "1",
    "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "7500000",   # ← дельта: плоскость sensn16 [7.5,7.7]M
    "cpu_band_max": "7700000",   # ← дельта: плоскость sensn16 [7.5,7.7]M
    "lever_flag": "", "lever_arg": "",  # vanilla path (canon-компо у C36, НЕ дублируем)
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
    runs = api(tok, f"/repos/{REPO}/actions/runs?branch={br}&event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", [])]


def gate_verdict(tok, run_id):
    """None = ещё не решено; 'PASS' = гейт прошёл (бенч идёт); 'FAIL' = fast-fail."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=5")
    for j in jobs.get("jobs", []):
        for s in j.get("steps", []):
            if s.get("name") == GATE_STEP:
                c = s.get("conclusion")
                if c == "success":
                    return "PASS"
                if c in ("failure", "cancelled"):
                    return "FAIL"
    return None


def cpu_index_feed(tok, run_id):
    """Фид пула: достаём runner_cpu_index из лога gate-джоба (дискард-данные)."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=5")
    for j in jobs.get("jobs", []):
        try:
            req = urllib.request.Request(
                f"{API}/repos/{REPO}/actions/jobs/{j['id']}/logs",
                headers={"Authorization": f"Bearer {tok}"})
            with urllib.request.urlopen(req, timeout=60) as r:
                m = re.search(r"runner_cpu_index=(\d+)", r.read().decode("utf-8", "replace"))
            if m:
                return int(m.group(1))
        except Exception as e:
            print(f"log-fetch fail job {j['id']}: {e}", file=sys.stderr)
    return None


def roll(tok, branch, dry=False):
    cur = ref_sha(tok, branch)
    if cur != PIN:
        raise SystemExit(f"REF-VERIFY FAIL {branch}: got {cur}, want PIN (Л188a)")
    print(f"GET-verify OK {branch} object.sha == {cur[:8]}", flush=True)

    if dry:
        print(f"DRY-INPUTS {branch}: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        return None

    existing = [rid_ for rid_, st, ca, hs_ in runs_on_branch(tok, branch) if hs_ == PIN]
    if existing:  # re-entrant: адопт in-flight/готовый ран вместо повторного диспатча
        rid = existing[0]
        print(f"ADOPT existing run {rid} on {branch}", flush=True)
    else:
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": branch, "inputs": INPUTS})
        if r != {}:
            raise SystemExit(f"dispatch failed {branch}: {r}")
        print(f"dispatch 204-OK {branch}", flush=True)

        rid = None
        deadline = time.time() + 300
        while time.time() < deadline and rid is None:
            time.sleep(10)
            for rid_, st, ca, hs_ in runs_on_branch(tok, branch):
                if hs_ == PIN:
                    rid = rid_
                    break
        if rid is None:
            print(f"run-id not visible in 300s {branch} (204 принят)", flush=True)
            return {"branch": branch, "run_id": None, "verdict": "UNKNOWN", "cpu_index": None}
    print(f"RUN-ID {rid} ({branch})", flush=True)

    # Ждём вердикт band-гейта (fast-fail pre-download ~1 мин после старта job)
    deadline = time.time() + 900
    while time.time() < deadline:
        time.sleep(20)
        v = gate_verdict(tok, rid)
        if v == "PASS":
            print(f"GATE PASS {branch} run {rid} — бенч продолжается (окна-фид)", flush=True)
            return {"branch": branch, "run_id": rid, "verdict": "HIT", "cpu_index": None}
        if v == "FAIL":
            idx = cpu_index_feed(tok, rid)
            print(f"GATE FAST-FAIL {branch} run {rid} — pairing discard, cpu_index={idx}", flush=True)
            return {"branch": branch, "run_id": rid, "verdict": "DISCARD", "cpu_index": idx}
        st = api(tok, f"/repos/{REPO}/actions/runs/{rid}").get("status")
        if st == "completed":
            concl = api(tok, f"/repos/{REPO}/actions/runs/{rid}").get("conclusion")
            print(f"run {rid} completed early conclusion={concl}", flush=True)
            return {"branch": branch, "run_id": rid,
                    "verdict": "DISCARD" if concl != "success" else "HIT",
                    "cpu_index": cpu_index_feed(tok, rid)}
    print(f"gate verdict timeout 900s {branch} run {rid} — in-flight (DISPATCHED 18-iii)", flush=True)
    return {"branch": branch, "run_id": rid, "verdict": "UNKNOWN", "cpu_index": None}


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN[:8]} (×486 учёт)", flush=True)

    for br in BRANCHES:
        rs = runs_on_branch(tok, br)
        if rs:
            print(f"RUN-SNAPSHOT: {br} has runs {[r[0] for r in rs]} — адопт-путь (ре-энтрант)", flush=True)

    results = []
    for br in BRANCHES:
        res = roll(tok, br, dry=dry)
        if dry:
            continue
        results.append(res)
        json.dump({"pin": PIN, "inputs": INPUTS, "rolls": results},
                  open("/home/z/rounds/ROUND-487/c61_dispatch.json", "w"), indent=1)
        if res["verdict"] in ("HIT", "UNKNOWN"):
            break  # hit = бенч идёт; unknown = in-flight, ре-ролл не оправдан
        print(f"ре-ролл после дискард {br}", flush=True)

    if not dry:
        print("SUMMARY " + json.dumps(results), flush=True)


if __name__ == "__main__":
    main()
