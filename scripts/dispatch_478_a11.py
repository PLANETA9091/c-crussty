#!/usr/bin/env python3
"""dispatch_478_a11.py — ROUND-478-A11 прегист emap-arm диспатч (Л-475-C52).

Usage:
  python3 scripts/dispatch_478_a11.py [--sha <full-or-short>] [--dry-run]

Вектор: свежий-gen нога cmp405_eindex на алиас-ветке round-478-a11-emap
(нести fix-коммит: emap::armed() суперсет — делегационный фикс канона
Л-475-C52.1). Канон 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/
10G/xms4G + band 6.0-9.5M fast-fail (workflow defaults + явные inputs,
Л188a). Прегист-маркеры (ARM-пруф, обязаны быть в артефакте):
  - "region_threads: defined net/minecraft/server/level/EntityMapOps in kernel loader"
  - "emap: entityMap fence composed on ChunkMap (Retargeted { sites: 12 })"
  - "refsync: ServerLevel ReferenceList fence composed (Retargeted { sites: 4 })"
  - "refsync: ... composed (... Retargeted { sites: N })" ×7 (fenced 7/7)
Гейт AIOOBE=0: в server-stdout.log нет "Index -1" / AIOOBE / infinite-probe.

Отличие от dispatch_475.py: ветка УЖЕ существует с код-дельтой (fix-коммит),
поэтому скрипт НИКОГДА не двигает ref на чужой pin — только GET-верификация
object.sha == ожидаемого sha диспатча (Л188b), иначе abort.
"""
import argparse, json, subprocess, sys, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BRANCH = "round-478-a11-emap"

# Канон-вектор банка v5 (world-bench-parallel defaults; dispatch_475-канон):
INPUTS = {
    "lever_flag": "cmp405_eindex", "lever_arg": "",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4", "batch_collector": "1",
    "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token():
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
        print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--sha", default="")
    ap.add_argument("--dry-run", action="store_true")
    a = ap.parse_args()

    tok = token()
    if not a.sha:
        a.sha = subprocess.run(
            ["git", "-C", "/home/z/c-crussty", "rev-parse", "round-478-a11-emap"],
            capture_output=True, text=True).stdout.strip()
    if len(a.sha) < 40:
        full = api(tok, f"/repos/{REPO}/commits/{a.sha}")
        a.sha = full["sha"]
    print(f"dispatch sha = {a.sha}")

    got = api(tok, f"/repos/{REPO}/git/ref/heads/{BRANCH}")
    branch_sha = got.get("object", {}).get("sha", "")
    assert branch_sha == a.sha, (
        f"branch {BRANCH} @ {branch_sha[:12]} != dispatch sha {a.sha[:12]} — "
        "abort (никогда не двигаем чужой/старый ref: Л188b)")
    print(f"verified {BRANCH} object.sha == {a.sha[:12]}")

    disp_body = {"ref": BRANCH, "inputs": INPUTS}
    if a.dry_run:
        print("DRY:", json.dumps(disp_body))
        return

    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data=disp_body)
    print(f"DISPATCHED {BRANCH} @ {a.sha[:12]} lever={INPUTS['lever_flag']} "
          f"arg='' gc={INPUTS['gc_tune']} pop={INPUTS['population_target']}")


if __name__ == "__main__":
    main()
