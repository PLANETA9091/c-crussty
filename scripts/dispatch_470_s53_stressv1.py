#!/usr/bin/env python3
"""dispatch_470_s53_stressv1.py — S53 / КЛИМБ-stress (round-470): стресс-мир v1 bench повтор.

Канон диспатчера тик-470: argv-guard, sha-pin, canonical anchor inputs,
band 6.0-9.5M fast-fail, POST + run-id discovery (Л188a/Л188b: 1 ветка = 1 ран).

Носитель: round-470-s53-stressv1a/b @a846dd58 = чистый мастер b3853246
  + cherry genfix 92a7b66b (level.dat-DOA workaround: 0fd4c132-миры шлют
  hand-craft level.dat с Compound-версией под lowercase 'version' ->
  NbtFormatException 'Unknown data version: 0' t+9s = 594s zombie; genfix
  двигает его в level.dat.stale = НАСТОЯЩИЙ gen-from-scratch)
  + seed-identity pin (level-seed=42 sha-gated 0fd4c132 — A/A parity pair
  сравнима, world_diff_parity_v2 SEED-гейт; pregenerated миры не тронуты).
  0 java/rust дельт, lever_flag="" — мир сам стрессор (19c канал).

Мир: release-канон world466-stress-v1.zip sha256 0fd4c132... (8.43MB,
  Terralith 2.5.13 + Tectonic 3.0.13 + BACAP + Structory 1.3.7, пустой
  region/, bukit mcmeta 88/min88/max88), asset v466-stress-c100 (hosting).

Вектор: канон-вектор тика (640/300s/fp4/gc6 Л217/ic1/fd1/rt4/bc1/pop150k/
  seed42/10G/4G, band [6.0,9.5]M) — ДВЕ A/A-ноги (parity-пара для
  world_diff_parity_v2; SEED-идентичность 42/42, чанк-хеш raw = blind
  0/169-класс Paper #14125 при region_threads>0 — S12 struct-gate канон).

Usage: dispatch_470_s53_stressv1.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = ["round-470-s53-stressv1a", "round-470-s53-stressv1b"]
PIN_SHA = "a846dd58"  # b3853246 + genfix 92a7b66b + seed-identity pin

STRESS_V1_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
                 "v466-stress-c100/world466-stress-v1.zip")

INPUTS = {
    "world_url": STRESS_V1_URL,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
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


def find_run_id(tok, full_sha):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&head_sha={full_sha}&per_page=20")
    for r in runs.get("workflow_runs", []):
        if r.get("head_sha") == full_sha:
            return r["id"], r["html_url"], r.get("status")
    return None


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    for br in BRANCHES:
        live = sha_of(tok, br)
        if not live.startswith(PIN_SHA[:7]):
            raise SystemExit(f"SHA MISMATCH: {br} live={live[:8]} pin={PIN_SHA}")
        print(f"preflight OK: {br} @ {live[:8]} (stress-v1 carrier)", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    out = []
    for br in BRANCHES:
        live = sha_of(tok, br)
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} (HTTP 204)", flush=True)
        out.append((br, live))
    run_ids = {}
    deadline = time.time() + 300
    while time.time() < deadline and len(run_ids) < len(out):
        time.sleep(10)
        for br, live in out:
            if br in run_ids:
                continue
            hit = find_run_id(tok, live)
            if hit and hit[2] in ("queued", "in_progress", "completed"):
                run_ids[br] = hit
                print(f"run-id discovered: {br} -> {hit[0]} status={hit[2]} {hit[1]}", flush=True)
    for br, live in out:
        hit = run_ids.get(br)
        print(f"=== S53 stress-v1 leg: branch={br} sha={live[:8]} run={hit[0] if hit else 'POLL-NEEDED'} ===", flush=True)


if __name__ == "__main__":
    main()
