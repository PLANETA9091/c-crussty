#!/usr/bin/env python3
"""dispatch_478_a16_legk.py — COMMANDER 478-A16 MEGA-SWARM v19.0 (тик ×478).

CLAIM: sensn16 leg ≥+20 (пара-пул №17/18; якоря mxa-08 −3.70 / mxa-12 +3.32 /
s1-d +0.59 READY; C88 s1-d +0.59 → пара +22.97 PASS).
Задача: диспатч leg-руна sensn16 canon x466-C98 ЯВНЫМ JSON (fp4-канон), алиас
round-478-a16-sensn16-k (семья round-478-a16-sensn16, 1 диспатч = 1 ветка
Л188b; базовый алиас занят leg-a) от master, 0 код-дельт (дрейф d24ba5fd →
2d83f079 docs/scripts-only diff-верифицирован: 0 non-md/py/txt/yml/json, 0
yml/bench дельт). Pair = leg_norm − anchor_norm, Δcpu ≤50k, min-of-3;
pair ≥+20 → MERGE-READY. Порог лега ≥+23.32 (худший якорь mxa-12 +3.32).
Прегист закон 14a/16 — этот docstring, LEDGER Л-478-A16. Пороги/каноны
v5-FROZEN НЕ двигаются.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-478-a16-sensn16-k"
PIN = "2d83f079c9a717cc5ea1d1fe54cd4f9d60aed62b"  # origin/master FULL sha

# canon x466-C98 — ЯВНЫЙ JSON (урок C66-C73: yml-дефолты = merge-поверхность)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",  # cert n16 Л-470-S20.1
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
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def main():
    tok = token()
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    pin = live if live.startswith(PIN[:12]) else PIN
    print(f"origin/master live = {live} -> pin {pin}", flush=True)
    # 1 диспатч = 1 ветка (Л188b): создаём -k FULL-sha, GET-пруф
    ref = api(tok, f"/repos/{REPO}/git/refs", method="POST",
              data={"ref": f"refs/heads/{ALIAS}", "sha": pin})
    if ref.get("object", {}).get("sha") != pin:
        g = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")
        if g.get("object", {}).get("sha") == pin:
            print(f"{ALIAS}: GET-proof exists @{pin[:8]}", flush=True)
        else:
            print(f"REF-FAIL {json.dumps(ref)[:200]}", file=sys.stderr)
            sys.exit(1)
    else:
        print(f"{ALIAS}: ref CREATED @{pin[:8]}", flush=True)
    before = {x["id"] for x in api(tok,
        f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={ALIAS}&per_page=20"
        ).get("workflow_runs", [])}
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    print(f"dispatch explicit-JSON -> {'204 OK' if r == {} else r}", flush=True)
    time.sleep(30)
    runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={ALIAS}&per_page=20"
               ).get("workflow_runs", [])
    new = [x for x in runs if x["id"] not in before and x["created_at"] > mc]
    for x in new:
        print(json.dumps({"id": x["id"], "status": x["status"],
                          "conclusion": x["conclusion"],
                          "sha": x["head_sha"][:8]}), flush=True)
    if not new:
        print("NO-NEW-RUN", file=sys.stderr)
        sys.exit(2)


if __name__ == "__main__":
    main()
