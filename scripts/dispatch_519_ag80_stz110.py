#!/usr/bin/env python3
"""dispatch_519_ag80_stz110.py — wave-519 AG-80: СТЗ-110 «С2 датапак-давление» стенд (v1, stateless).

PREREGISTERED GATES (закон 14a — числа/гейты зафиксированы ДО диспатча; залп-лимит и
сатуратед-очередь 98 queued 2026-10-01 → AG-80 финал = DISP-INTENT, POST только при
ALLOW_STZ110_FIRE=1 и несатуратед очереди):
  G0 band [6.0,9.5]M fast-fail; DP-INSTALLED sha256 == 79a0f122c2c77d61a32601ff85ecc98baaf263b738691ef583b4ddbdc301b248;
  G1 fn/тик ≥ 5000 (fixture исполняется: 200 roots × 24 children + 4800 leaves = 9800 ops/тик);
  G2 dp-класс reopen-класс ×485: fn-плоскость (dp-core) ≥1% ALL-CPU — прогноз 2-5ms/тик = 4-10% MSPT
     (Л-482-C13.2 пер-exec бэнды 0.2-0.5µs guard/light) → G-D1-давление для lever-работы волны-520;
  G3 M1 CLEAN (STW ≤23.0, full_n ≤9-12), NCDFE=0, AIOOBE=0, lever ∅;
  G4 world-diff parity: листья stateless (`data get storage`) → 0.0000% block-diff vs ваниль-пара (П-2);
  G5 вердикт НЕ pair (стенд, НЕ банк): dp-класс class-gate F1 → dp-кривая (прецедент СТЗ-82 v506-stz82).

ВЕКТОР: world-bench-parallel.yml ref=swarm-519-80 (полный worktree-пуш 2f795c64+),
x466-C98 канон ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G)
+ datapack_url = release v519-stz110-c2 asset stz110-c2-fixture.zip (881,182B, sha 79a0f122).

Usage: dispatch_519_ag80_stz110.py [--dry-run]   (POST требует ALLOW_STZ110_FIRE=1)
"""
import json, os, sys, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BRANCH = "swarm-519-80"
DP_URL = "https://github.com/PLANETA9091/c-crussty/releases/download/v519-stz110-c2/stz110-c2-fixture.zip"
DP_SHA = "79a0f122c2c77d61a32601ff85ecc98baaf263b738691ef583b4ddbdc301b248"

INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "datapack_url": DP_URL,          # sha256 79a0f122 (fixture v1, 5000 mcfunction, 200 roots)
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",   # стенд: lever ∅ (class-gate F1 → dp-кривая, НЕ банк)
}

def token(): return open("/tmp/gh_token").read().strip()

def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload: req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}

def main():
    if any(a not in ("--dry-run",) for a in sys.argv[1:]):
        raise SystemExit("argv-guard: only --dry-run")
    tok = token()
    ref = api(tok, f"/repos/{REPO}/git/ref/heads/{BRANCH}")
    sha = ref.get("object", {}).get("sha")
    if not sha: raise SystemExit(f"REF MISSING: {BRANCH} (push branch first)")
    print(f"ref {BRANCH} = {sha[:8]}; workflow={WF}; datapack sha256={DP_SHA[:8]}")
    if "--dry-run" or True:
        print("DRY-RUN INPUTS:"); print(json.dumps(INPUTS, indent=1))
    if os.environ.get("ALLOW_STZ110_FIRE") != "1":
        print("NO POST: queue-saturation guard (DISP-INTENT payload ready). set ALLOW_STZ110_FIRE=1 to fire.")
        return
    queued = api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
    n_q = sum(1 for r in queued.get("workflow_runs", []) if r["status"] == "queued")
    if n_q > 40:
        raise SystemExit(f"FIRE BLOCKED: queue saturated ({n_q} queued in first 100)")
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": BRANCH, "inputs": INPUTS})
    print("DISPATCH:", r if r else "204 OK")

if __name__ == "__main__":
    main()
