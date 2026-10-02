#!/usr/bin/env python3
"""dispatch_s7199.py - GC-TUNE leg s7199 (TASK-375, owner sanction 2026-09-20
"я тебе разрешаю всё, главное чтобы было как в ваниле, но всё реально ускоренно").

ПАРАДОКС-ФИКС: единственные поверхности, где -CPU конвертируется в +TPS 1:1 по
построению — GC STW (паузы ВНУТРИ тикового wall-clock; база s7198: 246 пауз /
300s, 19.5s = 6.5% окна, avg 79ms, max 178ms) и параллельный GC-CPU (37% сэмплов
на 4-ядерном раннере — прямая конкуренция с 4 тик-воркерами). Точечные кодовые
диеты давали 0% конверсии (5 диет) или отрицательную (#10: -12.5%) именно
потому, что их CPU был вне крит-пути; STW/GC-CPU — НА крит-пути.

LEVER = JVM-флаги ТОЛЬКО (vanilla-parity бит-в-бит — семантика игры не трогается):
  -XX:MaxGCPauseMillis=40            (цель паузы; default 200)
  -XX:InitiatingHeapOccupancyPercent=35  (раньше concurrent-цикл; high-water 63%)
  -XX:G1HeapRegionSize=8m            (меньше card/refine bookkeeping; авто=4m)
  -XX:+AlwaysPreTouch                (pre-commit страниц при буте)

LEG INPUTS = банк v3 (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1) + gc_tune=1, region_steal=0, travel_diet=0,
inside_bitmask=0, skip_store_bb=0, bu_defer=0 (изоляция GC-эффекта).

PROTOCOL v8-REGRESSION: WIDE BAND 6.0M..9.5M; DUAL BAR vs ANCHOR-SLOW
(s7184: median5 1.60 @ 6680195): normalized AND absolute BOTH >= +10% ->
CANDIDATE-GREEN -> confirming min-of-2 leg -> banking v4 = v3 + gc_tune.

PREREGISTER GATES (absorb_s7199.py):
  PG-T1 delivery: gc_tune=1 + банк v3 rest + NCDFE=0 + pop 150k VALID +
    JVM-flag-effect proof: "Heap Region Size: 8M" в gc.log (только -XX:G1HeapRegionSize
    меняет эту строку) + workers=4 телеметрия
  PG-T2 crash-free (0 threw/unexpected/NPE incl. s7180/RECON-22 классы) + soak >= 3
  PG-T3 REGRESSION DUAL-BAR (обе оси >= +10%)
  PG-T4 GC-гейты (preregister на базе s7198): 0 Full; total_pause <= 14.0s
    (-28% vs 19.5188s); avg_pause <= 60ms (vs 79.34ms); young <= 320
    (малый young под target-40 может учащать — жёсткий гейт только на total)
  PG-T5 DONE-park N/A tolerated (AP-PID defect s7178 precedent).
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
        "gc_tune": "1",
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
