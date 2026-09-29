#!/usr/bin/env python3
"""dispatch_487_c57_mednorm.py — [487-C57] Commander C57: мед-норма 150k канон-якорь ×2 (тик ×487, Job 415026).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча в board/CLM-C57.md):
  БАНК-ФИД: 2 ваниль-диспатча world-bench-parallel.yml @master 85a06f2f (0 код-дельт,
  ваниль-ноги бит-идентичны — закон-5). GLOB-полоса [6.0,9.5]M (широкая, высокий hit-rate),
  norm_v5 admit-гейт [−8.0,+1.5] → банк 66/30+ → +2 при двойном admit.
ВЕТКИ: round-487-c57-mna, round-487-c57-mnb (git branch+push @ FULL-sha 85a06f2f…, master НЕ тронут).
INPUTS = полный канон x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/
  seed42/10G/xms4G, lever="" ваниль, band [6000000,9500000] fast-fail); world_url default
  MineShield-3 Min (norm_v5-банк = ванильный MineShield-стенд — канон-уточнение ×487).
  ⚠️ input inside_bitmask на пине НЕ существует (x484 dp-door swap → datapack_url) — исключён.
ГЕЙТЫ рана (prereg): (1) band PASS fast-fail; (2) CLEAN M1 STW ≤23.0; (3) norm_v5 ∈
  [−8.0,+1.5] → admit; вне коридора → drift-разбор (gate→bench дрейф +1.94%).
Поверхность: parallel per-ref — единственная (инфра-канон ×486); 1 ветка = 1 ран (Л188b).

Usage: dispatch_487_c57_mednorm.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = ["round-487-c57-mna", "round-487-c57-mnb"]
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)

# canon x466-C98 — ЯВНЫЙ JSON, yml-дефолты = merge-поверхность (урок x466-C73)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha")


def run_on_branch(tok, br):
    """run-id GET runs?branch=<br> (миссия-канон discovery)."""
    runs = api(tok, f"/repos/{REPO}/actions/runs?branch={br}&per_page=10")
    for r in runs.get("workflow_runs", []):
        if r.get("head_sha") == PIN:
            return r["id"], r.get("status"), r.get("created_at")
    return None, None, None


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    # POST-CREATE VERIFY (Л188a): обе ветки на пине
    for br in BRANCHES:
        got = ref_sha(tok, br)
        if got != PIN:
            raise SystemExit(f"REF-VERIFY FAIL {br}: {got} != PIN")
        print(f"GET-verify OK {br} @ {got[:8]}", flush=True)
    pre = {br: run_on_branch(tok, br) for br in BRANCHES}
    dirty = {br: r for br, r in pre.items() if r[0] is not None}
    if dirty:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {dirty}")

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    result = {}
    for br in BRANCHES:
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": br, "inputs": INPUTS})
        if r == {}:
            print(f"dispatch {br}: 204-OK", flush=True)
        else:
            raise SystemExit(f"dispatch {br} failed: {r}")
        result[br] = {"pin": PIN, "inputs": INPUTS}
        time.sleep(3)  # анти-дребезг между POST

    # run-id discovery: GET runs?branch= (×2)
    ids = {br: None for br in BRANCHES}
    deadline = time.time() + 300
    while time.time() < deadline and any(v is None for v in ids.values()):
        time.sleep(12)
        for br in BRANCHES:
            if ids[br] is None:
                rid, st, ca = run_on_branch(tok, br)
                if rid:
                    ids[br] = {"run_id": rid, "status": st, "created_at": ca}
                    print(f"RUN-ID {br} {rid} ({st})", flush=True)

    for br in BRANCHES:
        result[br].update(ids[br] or {"run_id": None})
        if ids[br] is None:
            print(f"{br}: run-id not visible in 300s (204 принят, discovery по head_sha позже)", flush=True)
    json.dump(result, open("/home/z/rounds/ROUND-487/c57_dispatch.json", "w"), indent=1)
    print("SUMMARY: " + json.dumps({br: (ids[br] or {}).get("run_id") for br in BRANCHES}), flush=True)


if __name__ == "__main__":
    main()
