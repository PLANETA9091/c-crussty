#!/usr/bin/env python3
"""dispatch_s7202.py - COLLECTOR A/B #2: ZGC generational leg s7202 (TASK-383,
директива владельца 20:08 «автономные A/B непривычных конфигов», очередь №2).

ТЕЗИС: ZGC-generational = concurrent-коллектор с sub-ms паузами ценой
собственного concurrent-CPU (~10-15% сэмплов на 4-ядерном раннере).
ParallelGC выиграл банк v4 тем, что отдал G1-concurrent-ядра (37%) воркерам
при median5-толерантности к редким длинным STW. ZGC возвращает concurrent-CPU,
НО убирает все STW-паузы > 1ms. Чистый A/B против нового якоря банка v4.

LEVER = JVM-флаги ТОЛЬКО (vanilla-parity бит-в-бит): -XX:+UseZGC -XX:+ZGenerational
(gc_tune=4, инфраструктура run_world3.sh уже слита TASK-380; JDK21 temurin
подтверждён TASK-381 pre-flight).

LEG INPUTS = банк v4 (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1) + gc_tune=4 (изоляция коллектор-эффекта),
region_steal=0, travel_diet=0, inside_bitmask=0, skip_store_bb=0, bu_defer=0.

PROTOCOL v8-REGRESSION: WIDE BAND 6.0M..9.5M; DUAL BAR vs ANCHOR-БАНКА v4
(2.60 @ 8551924 leg#1 + подтверждающий лег #167 — см. GOAL ×68):
normalized AND absolute BOTH >= +10% -> CANDIDATE-GREEN -> min-of-2 -> banking v5.

PREREGISTER GATES (absorb_s7202.py):
  PG-T1 delivery: gc_tune=4 + банк rest + NCDFE=0 + pop 150k VALID +
    collector-proof: "Using The Z Garbage Collector" в gc.log + workers=4
  PG-T2 crash-free + soak >= 3
  PG-T3 DUAL BAR (обе оси >= +10% vs ANCHOR-БАНКА v4)
  PG-T4 ZGC-гейты (пререгистр TASK-381): Full <= 2; total_pause <= 10.0s;
    max_pause <= 50ms (sub-ms pause-модель; длиннее = ZGC не работает как
    задумано); pause-count sanity <= 3000
  PG-T5 DONE-park N/A tolerated (AP-PID defect).
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
        "gc_tune": "4",
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
