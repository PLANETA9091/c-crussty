#!/usr/bin/env python3
"""dispatch_s7206.py - ARCH-LEVER #16 FLUIDPUSH-BITMASK (RECON-43, TASK-389) на БАНКЕ v4 — архитектурная замена дата-плейна fluid-скана
(директива владельца 2026-09-21 «заменяй реальные архитектуры, без провальных
повторов» + анти-повтор протокол §3: флагман RECON-43, контракт ec880c2,
research/gc-recon-2026-09-19/RECON43_FLUIDPUSH_BITMASK_CONTRACT.md).

ARCHITECTURE: per-entity-per-tick block-итерация Entity.updateFluidHeightAndDo
FluidPushing (лейн 14.6% java = оркестрация 8.1% + чтения 5.1%) заменяется
секционно-резидентными fluid-битмапами (long[128]/секция, WATER|LAVA x 4096
слотов) + median-exact pre-gate ВНУТРИ banked FluidPushGuardHook (fluid_guard
TASK-80, в банке v4). CLEAN => бит-точный negative-tail: fluidHeight.put(tag,
0.0) + return false (put в хвосте slow() БЕЗУСЛОВНЫЙ - верифицировано по
исходнику @318). Не-CLEAN => banked slow-путь без изменений.

НЕ ПОВТОР (реестр запретов §2-3): не per-entity memo (fluid_dirty S7-153:
hit~0% - здесь кэш пер-СЕКЦИЯ, позиционная нестабильность неприменима), не
all-air гейт (inside_bitmask #15: секции не пусты - здесь предикат «секция без
тега-жидкости» структурно истинен), не per-entity JNI (alloc_diet x2 - здесь
0 JNI: гейт чисто Java, бит-тесты по long[]), не read-path demux (REFUTED
s7177-класс - здесь только write-bump ledger через banked-инфра fluid_dirty).

INVALIDATION: FluidPushOps.LEDGER (identity-keyed CHM, bump ТОЛЬКО на реальные
fluid-изменения в secWrite) + identity-сравнение объекта секции. Лег армирует
CRUSSTY_FLUID_DIRTY_LEDGER=1 (LEDGER-ONLY сплит fluid_dirty.rs: LevelChunk-хук
+ stamps, БЕЗ refuted memo-стейджа; fluid_dirty=0). fluid_bitmask.rs defines
FluidBitmaskOps+$Entry в kernel loader ДО fluid_guard-хука (lib.rs порядок).

BUDGET: <=64 rebuild/секцию-окно (~1.05s) + CACHE_CAP 32k; промах бюджета =
fall-through в vanilla (fail-dominant, парити сохраняется).

LEVER INPUTS = БАНК v4 (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1) + gc_tune=3 (ParallelGC) + fluid_bitmask=1
+ fluid_dirty_ledger=1; fluid_dirty=0, inside_bitmask=0, travel_diet=0,
skip_store_bb=0, region_steal=0, bu_defer=0 - ЧИСТАЯ ИЗОЛЯЦИЯ #16.

PREREGISTER GATES (absorb_s7206.py):
  PG-T1 delivery: ARMED-маркеры ПРИСУТСТВУЮТ: "[crussty-plugin] fluid_bitmask:
    defined net/minecraft/world/entity/FluidBitmaskOps in kernel loader",
    "fluid_bitmask: gate armed", "fluid_dirty: LEDGER-ONLY mode";
    dormant-маркер "fluid_bitmask: dormant" ОТСУТСТВУЕТ; 0 NCDFE;
    pop 150000 VALID; collector-proof "Using Parallel"; GC-стейджи чисты.
  PG-T2 crash-free + soak >= 3.
  PG-T3 DUAL BAR vs БАНК v4 (2-точки 2.6@8551924 / 2.2@6653417):
    normalized min по ногам >= +10% И absolute vs TPS_exp(runner) >= +10%
    -> CANDIDATE-GREEN -> min-of-2 -> BANKING v5 = v4 + fluid_bitmask.
  PG-T4 страховочные ParallelGC: young 30..250, Full <= 10, total <= 20.0s.
  PG-T5 профиль-вердикт: fluid-family (updateFluidHeightAndDoFluidPushing
    lane) в cpu-collapsed ↓>=60% vs банк-профиль RECON-42 (15319 сэмплов
    baseline) - архитектурное доказательство замены дата-плейна.
  REFUTED-критерий: PG-T1 PASS но G1 недостижим (family не падает) ИЛИ
    G4-регресс >10% => fluid_bitmask=0 (анти-урок fluid_dirty), вердикт ЭКОНОМИКИ.

ДИСПАТЧ: python3 scripts/bench4_recon/dispatch_s7206.py
(санкция = директива автономности 20:08 + анти-повтор протокол владельца
2026-09-21 §3; диспатч ТОЛЬКО при пустом пуле - runs?per_page=6 проверен).
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
        "fake_players": "4",
        "fluid_guard": "1",
        "gc_tune": "3",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "fluid_dirty_ledger": "1",
        "fluid_bitmask": "1",
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
