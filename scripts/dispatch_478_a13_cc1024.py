#!/usr/bin/env python3
"""dispatch_478_a13_cc1024.py — [478-A13] CODECACHE-СКЕЙЛ-КРИВАЯ Л-475-C48.1 (тик ×478, v19.0 MEGA-SWARM).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча этим коммитом):
  Гипотеза C48.1: N_fulls(max) = C_new/(15%·max), C_new≈190-216M →
    RCC 512M (gc6): CC-fulls ≈ 2-3   (канон, эмпирика Л183/Л-474-C11.2)
    RCC 1024M (gc7): CC-fulls ≈ 1
    Колено-прогноз ΔSTW_ALL = −1.8..−3.7s; медиан-иммунно (C48: медиана
    не движется — гейт на медиану НЕ ставится, фиксируем как факт);
    norm Δпп +0.1-0.3 (суб-бар → компо-ступень-кандидат, collision E2/C01,
    цикл закона 18).

A/B-ВЕКТОР (1 дельта между ногами = RCC 512M vs 1024M):
  - ветки-алиасы ОДНОГО sha (прецедент Л-470-S52.1 «код бит-идентичен»):
    round-478-a13-cc1024a = gc_tune=6 (512M, контроль — case-6 путь байт-идентичен
    мастеру, дельта-коммит additive-only elif)
    round-478-a13-cc1024b = gc_tune=7 (1024M, лечение)
  - 0 Java/Rust код-дельт; delta = bench/world3/run_world3.sh case-7 + docs
  - вектор банк-v5 канон x466-C98: 640/300s/fp4/ic1/fd1/rt4/bc1/pop150k/
    seed42/10G/xms4G + lever_flag=""/lever_arg="" (ваниль-якорь) +
    band [6.0,9.5]M fast-fail (band-miss = free re-roll, Л188c)

ГАЙТЫ АБСОРБА (решает артефакт, не gate-эхо; run-env.txt — Л195/Л240.1):
  G1 band PASS по run-env runner_cpu_index ∈ [6.0,9.5]M
  G2 M1-канон CLEAN ОБЕ ноги: STW ALL ≤23.0s ∧ young avg ≤200ms (gc6/gc7-окно
     fulls [0,3]); fulls-счётчик = механизм-рид-аут, НЕ цензор
  G3 прегейс-эффект: ΔSTW_ALL(cc7−cc6) ≤ −1.5s (C48 колено −1.8..−3.7s)
  G4 norm-дельта: Δnorm(v5) ≥ +0.1пп → компо-ступень-кандидат (суб-бар цикл 18)
  G5 медиан-иммунность: Δ(tail med) ожидается 0 ±0.1 кванта — НЕ гейт, факт
  G6 pair-legality: Δcpu(run-env) ≤50k; legs = независимые лотереи runner'а,
     вне окна → банк-фид §3 + канон re-roll
  Контроль-валидация: нога a (gc6) norm ∈ [−8,+1.5] (Л143 vanilla-коридор),
     выход = инфра-алерт, ноги не pair-данные.

Runs >15 мин → вердикт DISPATCHED run-id (финал ≤15 строк).

Usage: dispatch_478_a13_cc1024.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = ["round-478-a13-cc1024a", "round-478-a13-cc1024b"]
PIN_SHA = "d24ba5fd"  # master ×477 MAIN-консолидация + A13-дельта (см. коммит)

# вектор: банк-v5 канон, 1 дельта между ногами = gc_tune 6-vs-7
BASE_INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "",  # ваниль-якорь: 1 дельта = только JVM-флаг RCC
    "lever_arg": "",
}
INPUTS_BY_BRANCH = {
    "round-478-a13-cc1024a": dict(BASE_INPUTS, gc_tune="6"),  # контроль RCC=512M
    "round-478-a13-cc1024b": dict(BASE_INPUTS, gc_tune="7"),  # лечение RCC=1024M
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
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
    """Снапшот/атрибуция ТОЛЬКО по head_branch (S31-урок: head_sha ловит чужие
    алиасы — обе ноги шарят один sha)."""
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    out = []
    for r in runs.get("workflow_runs", []):
        if r.get("head_branch") == br:
            out.append((r["id"], r.get("status"), r.get("created_at")))
    return out


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    shas = {}
    for br in BRANCHES:
        live = sha_of(tok, br)  # GET-верификация object.sha ДО диспатча (Л188a)
        # пин = base-пруф: master d24ba5fd должен быть АНЦЕСТОРОМ ветки
        # (баз-пруф канон Л-470-S31.1: git merge-base --is-ancestor)
        mb = subprocess.run(["git", "-C", "/home/z/c-crussty", "merge-base", "--is-ancestor",
                             PIN_SHA, live])
        if mb.returncode != 0:
            raise SystemExit(f"SHA MISMATCH: {br} live={live[:8]} не содержит pin {PIN_SHA}")
        pre = runs_on_branch(tok, br)
        if pre:
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br} уже имеет runs {pre} — атрибуция сломана")
        shas[br] = live
        print(f"preflight OK: {br} @ {live[:8]} runs_before=0", flush=True)
    print(f"inputs a(gc6=512M ctrl): gc_tune={INPUTS_BY_BRANCH[BRANCHES[0]]['gc_tune']}", flush=True)
    print(f"inputs b(gc7=1024M trt):  gc_tune={INPUTS_BY_BRANCH[BRANCHES[1]]['gc_tune']}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        print(f"dispatched: {br} (HTTP 204)", flush=True)
    run_ids = {}
    deadline = time.time() + 240
    while time.time() < deadline and len(run_ids) < len(BRANCHES):
        time.sleep(10)
        for br in BRANCHES:
            if br in run_ids:
                continue
            hits = runs_on_branch(tok, br)
            if hits:
                rid, st, ca = hits[0]
                if st in ("queued", "in_progress", "completed"):
                    run_ids[br] = (rid, st, ca)
                    print(f"run-id discovered: {br} -> {rid} status={st} created={ca}", flush=True)
    for br in BRANCHES:
        hit = run_ids.get(br)
        print(f"=== A13 cc-leg: branch={br} sha={shas[br][:8]} "
              f"run={hit[0] if hit else 'POLL-NEEDED'} status={hit[1] if hit else '-'} ===", flush=True)


if __name__ == "__main__":
    main()
