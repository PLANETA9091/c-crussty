#!/usr/bin/env python3
"""dispatch_s7203.py - A/B #3: ParallelGC + TransparentHugePages leg s7203
(TASK-384, директива владельца 20:08 «автономные A/B непривычных конфигов»,
очередь №5 из директивы; RECON-41 обоснование).

ТЕЗИС (RECON-41, свежий профиль банк-v4 leg#2, 114891 сэмплов): после ухода
G1-concurrent (37% -> GC-PARALLEL 1.1% сэмплов) java-сцена = 91.4% CPU и
ПАМЯТЬ-BOUND: PalettedContainer.get 4.3% + SimpleBitStorage.get 1.7%
воркеров (fluid-чтения), HashMap.getNode 1.5%, ChunkEntitySlices.getEntities
3.9% — TLB-miss-тяжёлый профиль. THP (2MB страницы) режет TLB-miss И в сцене,
И в ParallelGC-copy 4.35GB/s (leg#2: young 109 эвакуаций, total 18.8s STW).

LEVER = JVM-флаги ТОЛЬКО (vanilla-parity бит-в-бит): gc_tune=5 =
-XX:+UseParallelGC -XX:+UseTransparentHugePages -XX:+AlwaysPreTouch
(инфраструктура run_world3.sh ветка 5, TASK-384).

LEG INPUTS = банк v4 (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1) + gc_tune=5 (изоляция THP-эффекта поверх
банка), region_steal=0, travel_diet=0, inside_bitmask=0, skip_store_bb=0,
bu_defer=0.

PROTOCOL v8-REGRESSION: WIDE BAND 6.0M..9.5M; DUAL BAR vs БАНК v4
(2-точки: 2.6 @ 8551924, 2.2 @ 6653417): normalized=min по ногам >= +10% И
absolute vs интерполяция TPS_exp(runner) >= +10% -> CANDIDATE-GREEN ->
min-of-2 -> banking v5 = v4 + THP.

PREREGISTER GATES (absorb_s7203.py):
  PG-T1 delivery: gc_tune=5 + банк rest + NCDFE=0 + pop 150k VALID +
    collector-proof "Using Parallel" в gc.log + workers=4 телеметрия
    (+ детект THP-unavailable warning в stdout — не гейт, маркер нейтральности)
  PG-T2 crash-free + soak >= 3 (T1/T2 = предусловие вердикта — урок #167)
  PG-T3 DUAL BAR vs БАНК v4 (2-точечная модель)
  PG-T4 страховочные (ParallelGC-адаптированные): young 30..250, Full <= 10,
    total <= 20.0s, avg <= 200ms, max <= 3000ms — FAIL не отменяет двойной бар
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
        "gc_tune": "5",
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
