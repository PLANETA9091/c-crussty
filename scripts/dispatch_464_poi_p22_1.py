#!/usr/bin/env python3
"""dispatch_464_poi_p22_1.py — TASK-464-34 (КЛИМБ poi456-4⊕P22 нога-1).

ГИПОТЕЗА P22-capture (chunk-sched capture):
  В NewChunkHolder тик-планировщик (66291B) на каждый тик гонит по всем
  отслеживаемым чанкам серию JNI-вызовов за due-ness (due-scan/dequeue
  полей по одному). Замена на CAPTURE-срез: ОДИН JNI/тик снимает
  due-ness маску целиком — 6561 чанков упаковываются в 824B bitmask
  (zero-alloc, повторное использование буфера), дальше джава и раст
  читают due-ness из маски без нативных вызовов.

  Срезы-источники выигрыша (оценка 1.5-2.5% общего тика):
    - due-ness маска: 1 JNI/тик вместо ~6561 полевых чтений (основной срез);
    - queue: обход очереди планировщика по маске вместо per-chunk peek;
    - Long2Ref: отсутствие boxing/unboxing лонг-ключей при поготовке среза.

  Носитель: cmp456_poi @5ecd841a (POI-план chunkmono-семьи,
  poi456-4 leg +15.4 v4/+18.01 v5). До бара −2.0пп; P22 ожидаемо
  закрывает срез 1.5-2.5 → выход в положительный коридор клмба.

РЕЖИМ: python3 scripts/dispatch_464_poi_p22_1.py
  (одна нога: ветка round-464-poi-p22-1, lever_flag=cmp456_poi, lever_arg=1;
   канон-банк клмба radius=640/300s/4fp/10G/150k/seed42, cpu_band 6.0-9.5M).
"""
import json, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
BENCH_WF = "world-bench-parallel.yml"

BRANCH = "round-464-poi-p22-1"
BASE_SHA = "5ecd841a"          # cmp456_poi carrier (LEDGER греф)
LEVER_FLAG = "cmp456_poi"
LEVER_ARG = "1"

INPUTS = {
    "lever_flag": LEVER_FLAG, "lever_arg": LEVER_ARG,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1",
    "flush_diet": "1", "region_threads": "4", "batch_collector": "1",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}

def tok():
    return open("/tmp/gh_token").read().strip()

def main():
    payload = {"ref": BRANCH, "inputs": INPUTS}
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        f"{API}/repos/{REPO}/actions/workflows/{BENCH_WF}/dispatches",
        data=data, method="POST",
        headers={"Authorization": f"token {tok()}",
                 "Accept": "application/vnd.github+json",
                 "Content-Type": "application/json",
                 "User-Agent": "task464-poi-p22-1"})
    try:
        with urllib.request.urlopen(req) as r:
            print(f"DISPATCH {BRANCH} lever={LEVER_FLAG}/{LEVER_ARG} base={BASE_SHA} HTTP {r.status}")
            return 0 if r.status == 204 else 1
    except urllib.error.HTTPError as e:
        print(f"DISPATCH FAIL HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return 1

if __name__ == "__main__":
    sys.exit(main())
