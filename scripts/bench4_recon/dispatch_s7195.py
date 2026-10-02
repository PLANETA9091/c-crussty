#!/usr/bin/env python3
"""dispatch_s7195.py - lever #15 INSIDE-BITMASK ACTIVATION leg (s7195),
option-B флагман (RECON-33 контракт + ORACLE ALL PASS dea9de8).

SAFETY GATE: POST уходит ТОЛЬКО с флагом --sanctioned — санкция владельца
option B (протокол min-MSPT). Без флага скрипт печатает план и exit 0.
Это защита от случайной активации: под ДЕЙСТВУЮЩИМ протоколом v8-REGRESSION
(ДВОЙНОЙ БАР) рычаг #15 не банкингуется (TPS-конверсия ~0, RECON-32/33) —
лег имеет смысл только при переходе владельца на B.

LEG INPUTS = банк v3 (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1) + inside_bitmask=1, travel_diet=0 (изоляция эффекта #15;
v2a/v2b-рука не подмешивается). InsideBitmaskOps верифицирован офлайн-оракулом
(dea9de8: 1,000,000 hull-superset кейсов / 210,513,534 позиций РЕАЛЬНОГО
forEachBlockIntersectedBetween внутри домена РЕАЛЬНОГО sweptHullInto;
20,000 сек-последовательностей / 249,217 setBlockState — hasOnlyAir
бит-согласован; fluids НЕ-air).
Arm markers expected in stdout:
  "[crussty-plugin] inside_bitmask: bridge owner armed"
  "stage inside_bitmask composed (Retargeted { sites: 1 })"
  (dormant-маркер "[crussty-plugin] inside_bitmask: dormant" = ОШИБКА армирования)

WIDE BAND 6.0M..9.5M (protocol v8-REGRESSION band, S7-96d fast-fail only for
extreme landings). ANCHOR-SLOW s7184: median5 1.60 @ 6680195.

VERDICT PROTOCOLS (absorb_s7195.py):
  --protocol v8 (default, current law): DUAL BAR — normalized AND absolute
    both >= +10% vs ANCHOR-SLOW
  --protocol B (owner option B): min-MSPT — normalized speed axis >= +10%
    AND median5 TPS >= anchor (no absolute regression, >= anchor-0)

PREREGISTER GATES: PG-T1 delivery (+ inside_bitmask arm markers, no dormant),
PG-T2 crash-free (0 threw/unexpected/s7180-class; RECON-22 stochastic race
catches classified), PG-T3 verdict protocol, PG-T4 GC (young <= 174, 0 Full),
PG-T5 DONE-park N/A tolerated (AP-PID defect s7178). INJECTS-ONLY, 1
sanctioned CI gate leg. Confirming leg = dispatch_s7195.py --sanctioned
повторно (min-of-2) -> banking.
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
    sanctioned = "--sanctioned" in sys.argv
    tok = token_from_remote()

    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "4",
        "fluid_guard": "1",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "travel_diet": "0",
        "inside_bitmask": "1",
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
    print("plan: bank v3 + inside_bitmask=1 (lever #15 isolation, travel_diet=0), "
          "wide band 6.0M..9.5M, anchor 1.60 @ 6680195")
    if not sanctioned:
        print("SAFETY GATE: без --sanctioned POST НЕ уходит — рычаг #15 активируется "
              "только по санкции владельца option B (протокол min-MSPT). "
              "absorb: python3 scripts/bench4_recon/absorb_s7195.py <run_id>")
        return 0

    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent (SANCTIONED); waiting for the run to appear...")
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
