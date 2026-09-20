#!/usr/bin/env python3
"""dispatch_s7205.py - ЦЕЛЕВОЙ профиль-лег: конкарренси игроков 16 (s7205) на БАНКЕ v4 — диагностический (ЦЕЛЬ-ДЕПЛОЙ falix: много игроков; НЕ гейт-лег, без банкинга)
(TASK-385-аменд, ЦЕЛЬ-ДЕПЛОЙ владельца: чанки/генерация + куча мобов + МНОГО
ИГРОКОВ на бесплатных хостингах (falix). Диагностический лег: банк v4 при
fake_players=16 vs якорь-банка при fake_players=4 — первая точка скейлинга
игроков под ParallelGC-экономикой. Банкинга нет — профиль-данные.

ТЕЗИС (RECON-41, свежая экономика ParallelGC): лейн volatile/inside вырос
3.2% -> 13.2% java-сцены после свапа коллектора (G1-вердикты RECON-32/33
«потолок ~9.1%, конверсия ~0» измерены ДО ParallelGC-экономики; по правилу
REFUTED-не-закрывает-лейн нужен свежий A/B). Рычаг #15 = единственный
приготовленный флагман: контракт RECON-33 + impl 6eb3274/1c703f7 + оракул
ALL PASS dea9de8 (1M кейсов / 210.5M позиций hull-superset бит-в-бит).

LEVER = env CRUSSTY_INSIDE_BITMASK=1 (vanilla-parity: median-exact гейт
checkInsideBlocks через hasOnlyAir() fast-path — прецедент LevelChunk
.getFluidState bc13..38; фоллбэк MethodHandle на исходный private метод;
fail-dominant, dormant-invisible).

LEG INPUTS = БАНК v4 (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1) + gc_tune=3 (ParallelGC, банк v4) +
inside_bitmask=1 (изоляция эффекта #15), travel_diet=0, skip_store_bb=0,
region_steal=0, bu_defer=0, fluid_dirty=0.

PROTOCOL v8-REGRESSION: WIDE BAND 6.0M..9.5M; DUAL BAR vs БАНК v4 (2-точки:
2.6 @ 8551924, 2.2 @ 6653417): normalized=min по ногам >= +10% И absolute vs
интерполяция TPS_exp(runner) >= +10% -> CANDIDATE-GREEN -> min-of-2 ->
banking v5 = v4 + inside_bitmask.

PREREGISTER GATES (absorb_s7204.py):
  PG-T1 delivery: gc_tune=3 + inside_bitmask=1 + банк rest + NCDFE=0 +
    pop 150k VALID + collector-proof "Using Parallel" + ARMED-маркеры #15:
    "[crussty-plugin] inside_bitmask: bridge owner armed" ПРИСУТСТВУЕТ +
    "stage inside_bitmask composed (Retargeted { sites: 1 })" ПРИСУТСТВУЕТ +
    dormant-маркер "[crussty-plugin] inside_bitmask: dormant" ОТСУТСТВУЕТ
    (dormant-маркер в stdout = ОШИБКА армирования, delivery-FAIL)
  PG-T2 crash-free + soak >= 3 (T1/T2 = предусловие вердикта — урок #167)
  PG-T3 DUAL BAR vs БАНК v4 (2-точечная модель)
  PG-T4 страховочные ParallelGC: young 30..250, Full <= 10, total <= 20.0s,
    avg <= 200ms, max <= 3000ms — FAIL не отменяет двойной бар
  PG-T5 DONE-park N/A tolerated.

ДИСПАТЧ: python3 scripts/bench4_recon/dispatch_s7204.py
(санкция = директива автономности 20:08, HIGHEST PRIORITY: «без санкций,
без ожиданий»; диспатч ТОЛЬКО при пустом пуле — cancel-in-progress:true).
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL — run rule (1b) remote set-url first")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def main():
    tok = token_from_remote()

    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "16",
        "fluid_guard": "1",
        "gc_tune": "3",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "travel_diet": "0",
        "inside_bitmask": "0",
        "skip_store_bb": "0",
        "region_steal": "0",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "0",
        "cpu_band_min": "6000000",
        "cpu_band_max": "9500000",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=1")
        runs = d.get("workflow_runs", [])
        if runs:
            r = runs[0]
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
            return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
