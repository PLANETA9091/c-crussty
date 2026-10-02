#!/usr/bin/env python3
"""dispatch_s7207.py - ЦЕЛЕВОЙ профиль-лег: конкарренси игроков 32 (s7207) на БАНКЕ v4 — диагностический (ЦЕЛЬ-ДЕПЛОЙ falix: много игроков; НЕ гейт-лег, без банкинга)
(TASK-392, эра-план TASK-386-аменд: «точка players-TPS -> 2-я точка (32/48)»).
После REFUTED #16 (RECON43_FINAL_VERDICT.md, s7206#3 35528326290) актуальное
ТОП-1 = ЦЕЛЬ-ДЕПЛОЙ лейны (RECON-42/TASK-387: однородных точечных >=5% java
больше нет; владелец: falix, бесплатный хостинг, МНОГО ИГРОКОВ).

ТОЧКА: fake_players=32 на банке v4 vs якорь-банка fp=4 (2.6 @ 8551924 /
2.2 @ 6653417) и закрытая точка fp=16 (-0.2%, 35519072184 @ 0c0fbc4) —
карта скейлинга конкарренси под ParallelGC-экономикой. Банкинга нет —
профиль-данные: где растёт цена игрока (broadphase? пафеты? чанк-сеты?)
-> под-лейн >=5% при игроко-нагрузке = следующий архитектурный рычаг.

НЕ ПОВТОР (реестр запретов §2-3): players-16 — закрытая ТОЧКА (один профиль
fp=16), fp=32 — новая точка sanctioned свипа, ни классом ни механизмом не
совпадает с REFUTED-рычагами. fluid_bitmask=0 (REFUTED #16), fluid_dirty=0,
inside_bitmask=0 — банк v4 в чистом виде + игроки.

PREREGISTER (диагностический): PG-T1 delivery (банк v4, fp=32, pop VALID,
ParallelGC, ARMED-маркеры банка, 0 NCDFE); PG-T2 crash-free; PG-T3
ИНФОРМАЦИОННЫЙ — дельта vs TPS_exp-интерполяция якоря при fp=4 (ожидание:
конкарренси-цена суб-линейна до 16; 32 — первый стресс-профиль); PG-T4
страховочные ParallelGC; PG-T5 профиль-карта: top-лейны при fp=32 vs
банк-профиль RECON-42 (что растёт с игроками) -> RECON следующего рычага.

ДИСПАТЧ: python3 scripts/bench4_recon/dispatch_s7207.py
(санкция = директива автономности 20:08 + «видеть и делать сразу» §b;
диспатч ТОЛЬКО при пустом пуле — concurrency guard внутри).
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
        "fake_players": "32",
        "fluid_guard": "1",
        "gc_tune": "3",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "fluid_dirty_ledger": "0",
        "fluid_bitmask": "0",
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
