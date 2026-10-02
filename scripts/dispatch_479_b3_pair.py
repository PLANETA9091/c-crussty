#!/usr/bin/env python3
"""dispatch_479_b3_pair.py — [479-B3] collision E2 dedup (C01) pair-class re-roll.

CLAIM (прегист закон 14a/16, зафиксирован ДО диспатча):
  Пара-класс ×478-A7 ре-ролл: leg-A armed = coll-dedup алиас @b24f48766d52eda3965c338371848e6385546c93
  (round-478-a7-coll-dedup bench-ша; diff vs контроль = ТОЛЬКО C01 CollideBatchOps.java
  +327/.class 23883B — верифицировано diff-статом b3e2f4f4→b24f4876), lever cmp401_collide;
  leg-B vanilla @b3e2f4f420dfdc01fa9692fda1f1de665c5bcde1, lever ∅.
  ×478: norm +12.71/−4.00 → Δraw +16.71, но Δrci 54,512 > 50k pair-fresh + 6-poll шум ±7пп
  → НЕ пара-класс (Л-478-A12.2). Ре-ролл ×2 целит same-node: Δrci ≤ 50k.
  canon x466-C98 дефолтами yml (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G,
  band [6.0,9.5]M fast-fail) — дефолты world-bench-parallel.yml = канон (x466-C73),
  постится ТОЛЬКО lever_flag/lever_arg (канон заголовка yml, как ×478-A7).

ПРЕГИСТ-ГЕЙТЫ абсорба (закон 16, до выстрела):
  G1 band: cpu(run-env) ∈ [6.0,9.5]M обе ноги; band-miss = free re-roll (закон W3).
  G2 validity: NCDFE=0 / AIOOBE=0 / world afb3a0b3 / pop150k / seed42 обе ноги;
     ARM-пруф armed-ноги: "cmp401_collide: ARMED" в server-stdout; vanilla armed=∅.
  G3 M1 HOST-ценз: STW ≤23.0s ∧ young_avg ≤200ms обе ноги (иначе HOST-CENSORED).
  G4 pair-легальность: Δrci(arm,van) ≤ 50k pair-fresh (same-node цель).
  G5 pair = normA − normV (min-of-2 базис; банк-гейт min-of-3):
     pair ≥ +20 → board PAIR {pair, Δcpu} (2/3, 3-й якорь след. тик);
     иначе → REFUTED_CENS с числами (residual-потолок E2 3.5-6.6пп < бар +20
     → композит-агенда climb5 на board).
  G6 Δdedup merge-гейт ≥+2пп (прогноз Л-477-C11.1 +4.2 [2.6..5.7]) — report-only sanity,
     коридор leg-A [−8,+3] breach = шум-флаг (урок ×478 leg-A +12.71), не авто-фейл.
  Runs >15 мин → DISPATCHED run-id (закон 18-iii).

C66-C72 УРОК: полный canon — дефолтами yml на пиновых ша (обе ветки несут canon-yml).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

ARMS = {
    "round-479-b3-arm": ("b24f48766d52eda3965c338371848e6385546c93",
                         {"lever_flag": "cmp401_collide", "lever_arg": ""}),
    "round-479-b3-van": ("b3e2f4f420dfdc01fa9692fda1f1de665c5bcde1", {}),
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
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
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=60")
    return [(r["id"], r.get("status"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    if not all(a in ("--dry-run",) for a in sys.argv[1:]):
        raise SystemExit(f"argv-guard: {sys.argv[1:]}")
    dry = "--dry-run" in sys.argv[1:]
    tok = token()
    for br, (pin, inputs) in ARMS.items():
        live = sha_of(tok, br)
        if live != pin:
            raise SystemExit(f"SHA MISMATCH {br}: live={live[:8]} pin={pin[:8]}")
        if runs_on_branch(tok, br):
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br}")
        print(f"preflight OK: {br} @ {live[:8]} runs_before=0 "
              f"lever={inputs.get('lever_flag', '(vanilla)')}", flush=True)
    if dry:
        print("DRY-RUN OK", flush=True)
        return
    for br, (_, inputs) in ARMS.items():
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": inputs})
        print(f"dispatched: {br} lever={inputs.get('lever_flag', '(vanilla)')}", flush=True)
        time.sleep(20)  # пауза 20s, 1 диспатч=1 ветка (Л188b)
    found = {}
    deadline = time.time() + 300
    while time.time() < deadline and len(found) < len(ARMS):
        time.sleep(20)
        for br in ARMS:
            if br in found:
                continue
            for rid, st, ca in runs_on_branch(tok, br):
                found[br] = {"run": rid, "status": st, "created_at": ca}
        print(f"poll: {json.dumps(found)}", flush=True)
    print(json.dumps(found, indent=1))


if __name__ == "__main__":
    main()
