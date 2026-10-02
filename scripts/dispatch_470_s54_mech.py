#!/usr/bin/env python3
"""dispatch_470_s54_mech.py — S54 / КЛИМБ-stress (round-470): mech 4.2.4 bench повтор.

Диагноз провала 36263907486 (job-логи API, артефакт world3-bench):
  НЕ level.dat-DOA (boot Done 15.861s, SEEN_DONE=1; level.dat сервер-рождён
  gzip 1576B — S47-канон уже в ассете stress-worlds-v1; mcmeta dual-decl 88 +
  min26/max88 + supported_formats верифицирован в zip) и НЕ GC (91 пауза
  9.75s, Full 8×5.1s, heap 5.9G/10G). Провал = ТАСК-411-A класс:
  x150k-инъекция встала 90000/150000 (окно таблицы n=131072: 49153-98304),
  32,769× BenchPopulation "spawn failed" AIOOBE "Index -1 ... length 131073"
  = fastutil Int2ObjectOpenHashMap downscan (docs/TASK411_A_AIOOBE_ROOTCAUSE.md
  сигнатура 1:1), watchdog "not responded for 870s" (вечный containsKey probe
  ChunkMap.addEntity:953) — emap-фенс был VANILLA ("emap vanilla (lever off)"
  в логе 18:54:16, lever пуст). Terr-нога того же класса (42k в окне n=65536,
  "length 65537" ×16385). INJECT DONE 0 → gates 1a/1b/1c FAIL.

Фикс-путь: эмул-фенс уже в мастере (TASK-411-A k5b, emap.rs probe-then-patch,
  fail-dominant) — армится lever-union'ом nav_plane.armed(): cmp466_c98ai
  (№11 мастер-лейн) входит. Carrier round-470-s54-mech* @a846dd58 = b3853246 +
  genfix 92a7b66b (level.dat-DOA + seed-pin; для mech-мира no-op — level.dat
  уже сервер-рождён) — sha-идентичен S53-ногам.

Ноги A/B (мир и carrier идентичны, дельта = lever):
  leg A mech  (c98ai):  измеримая ступень лестницы 19c — emap ARM, инъекция
                        должна дойти до DONE (~40-100s фаза).
  leg B mechv (пустой): строгий повтор конфига тика-469 на новой базе —
                        прогноз-фальсификатор: клин снова ~84-98k, run FAILED.
Мир: world466-stress-mech-v1.zip sha256 013b3842a3289eb9... верифицирован
  (region 4×MCA pregen + MechanizationDatapack_v4.2.4_mc88x.zip 1332fn +
  bukit pack.mcmeta [88,0]).

Вектор: канон S53 (640/300s/fp4/gc6 Л217/ic1/fd1/rt4/bc1/pop150k/seed42/10G/4G,
  band [6.0,9.5]M). Usage: dispatch_470_s54_mech.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = [("round-470-s54-mech", "cmp466_c98ai"),   # leg A: emap-ARM (№11 lane)
            ("round-470-s54-mechv", "")]              # leg B: vanilla повтор
PIN_SHA = "a846dd58"  # b3853246 + genfix 92a7b66b + seed-identity pin

MECH_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
            "stress-worlds-v1/world466-stress-mech-v1.zip")

INPUTS = {
    "world_url": MECH_URL,
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
    out = []
    for br, lever in BRANCHES:
        live = sha_of(tok, br)
        if not live.startswith(PIN_SHA[:7]):
            raise SystemExit(f"SHA MISMATCH: {br} live={live[:8]} pin={PIN_SHA}")
        inputs = dict(INPUTS, lever_flag=lever)
        print(f"preflight OK: {br} @ {live[:8]} lever={lever or '(empty)'}", flush=True)
        out.append((br, lever, live, inputs))
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for br, lever, live, inputs in out:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": inputs})
        print(f"dispatched: {br} (HTTP 204)", flush=True)
    run_ids = {}
    deadline = time.time() + 300
    while time.time() < deadline and len(run_ids) < len(out):
        time.sleep(10)
        for br, lever, live, inputs in out:
            if br in run_ids:
                continue
            hit = find_run_id(tok, live)
            if hit and hit[2] in ("queued", "in_progress", "completed"):
                run_ids[br] = hit
                print(f"run-id discovered: {br} -> {hit[0]} status={hit[2]}", flush=True)
    for br, lever, live, inputs in out:
        hit = run_ids.get(br)
        print(f"=== S54 mech leg: branch={br} lever={lever or '(empty)'} sha={live[:8]} "
              f"run={hit[0] if hit else 'POLL-NEEDED'} ===", flush=True)
    # HEADS-UP: same-sha branches make find_run_id ambiguous (head_sha match);
    # canonical ids are head_branch-scoped, captured 21:12Z tick-470:
    #   leg A round-470-s54-mech  (c98ai) -> run 36272150400
    #   leg B round-470-s54-mechv (empty) -> run 36272151752


if __name__ == "__main__":
    main()
