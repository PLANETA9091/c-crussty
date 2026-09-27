#!/usr/bin/env python3
"""dispatch_475_c74_terr.py — C74 / ROUND-475: terr640-реплика на свежем пине.

Канон диспатчера тик-475: argv-guard, sha-pin, canonical anchor inputs,
band 6.0-9.5M fast-fail, POST + run-id discovery (Л188a/Л188b: 1 ветка = 1 ран).

Носитель: round-475-c74-terr @cab66758 (чистый мастер ×474 MAIN-консолидация,
  0 код-дельт). emap-фенс k5b в мастере — армится lever-union
  cmp466_c98ai ∈ nav_plane.armed() (фикс-путь Л-474-C74.3: ре-ролл terr640
  с emap-arm, прогноз Л243 leg-A — инъекция доходит DONE).

Мир: world469-terr-v1.zip sha256 cc1b5b4d... (2.9MB, release v469-terr-v1,
  сервер-рождён level.dat 1515B DataVersion 4556 — DOA-класс снят, genfix
  не нужен; datapacks/ = Terralith_1.21.5_v2.5.13.zip + bukit mcmeta) —
  ТОТ ЖЕ мир, что у фейла 36277729335 (terr640 K1-K6 контекст).
  НЕ 0fd4c132-канон (мир-sha ≠ afb3a0b3-ваниль ≠ 0fd4c132: на свежем пине
  genfix 92a7b66b ОТСУТСТВУЕТ → 0fd4c132-мир = DOA-риск; cc1b5b4d boot-
  верифицирован на CI round-471).

Вектор: terr640-реплика (r640/300s/fp4/gc3 ЗАДАНИЕ ТИКА — девиация от
  S53-канона gc6 задокументирована в пре-регистре/ic1/fd1/rt4/bc1/pop150k/
  seed42/10G/4G, band [6.0,9.5]M обычный). Прегист: мир-sha ≠ afb3a0b3
  (Terralith-dp-мир) → ваниль-гейты (BENCH-4 FIXTURE-VALIDITY, world-
  identity) читать с учетом dp-мира; пар vs ваниль-банк-якоря НЕ ждать.

Usage: dispatch_475_c74_terr.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = ["round-475-c74-terr"]
PIN_SHA = "cab66758"  # master ×474 MAIN-консолидация (свежий пин тика)

TERR_V1_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
               "v469-terr-v1/world469-terr-v1.zip")

INPUTS = {
    "world_url": TERR_V1_URL,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp466_c98ai",  # emap-ARM (Л-474-C74.3 фикс-путь)
    "lever_arg": "",
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
        if r.get("head_sha") == full_sha and r.get("head_branch") in BRANCHES:
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
        print(f"preflight OK: {br} @ {live[:8]} (terr640-реплика, fresh pin)", flush=True)
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
    deadline = time.time() + 240
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
        print(f"=== C74 terr640 leg: branch={br} sha={live[:8]} run={hit[0] if hit else 'POLL-NEEDED'} ===", flush=True)


if __name__ == "__main__":
    main()
