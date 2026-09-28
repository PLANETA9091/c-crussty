#!/usr/bin/env python3
"""dispatch_479_b1.py — COMMANDER 479-B1 MEGA-SWARM v19.0 (тик ×479).

CLAIM: climb5-p32-1 пост-№18 пара-контроль (закон 21). МЕРЖ №18 (2d84ede9,
min-of-3 +26.96, lever cmp456_chunkmono_p31snap) + №17 (f66feb1b) в мастере;
canary №17 PASS −1.6 (A1). Повторная пара на текущем master.
Задача: свежий ваниль-якорь round-479-b1-a1 + свежий leg-рун climb5-семантики
lever cmp456_chunkmono_p31snap round-479-b1-leg (1 реф = 1 диспатч Л188b),
pin = origin/master 065aa6da FULL-sha (0 код-дельт от 1a0fb21d — docs-only),
canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/
10G/xms4G, band 6.0–9.5M), world_url = workflow-default.
Pair = leg_norm − anchor_norm (normtool_478), Δcpu ≤50k; pair ≥+20 →
POST-MERGE-PAIR; пороги FROZEN, закон-5 запреты не тронуты.
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "af542aec57f6f831aad2243b8a3e00872c1486ef"  # origin/master FULL sha (docs-only chain vs 1a0fb21d, 0 code-delta)

# алиас → (lever_flag, lever_arg): a1 = ваниль-якорь (''/'), leg = climb5-компо
ALIASES = {
    "round-479-b1-a1": ("", ""),
    "round-479-b1-leg": ("cmp456_chunkmono_p31snap", ""),
}

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
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-b1-pair-dispatcher"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}
    return json.loads(body) if body else {}


def main():
    tok = token()

    # 0) EXPECTED_SHA-гард против HEAD-гонок (метод 478-A10)
    master = api(tok, f"/repos/{REPO}/commits/master")["sha"]
    assert master == PIN, f"master drift: {master} != {PIN}"
    print(f"master pin OK {master[:8]} (0 code-delta vs 1a0fb21d)", flush=True)

    run_ids = {}
    for alias, (lf, la) in ALIASES.items():
        # 1) alias @PIN (FULL-sha, Л188a)
        ref = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                  data={"ref": f"refs/heads/{alias}", "sha": PIN})
        if ref.get("object", {}).get("sha") == PIN:
            print(f"{alias}: ref CREATED @{PIN[:8]}", flush=True)
        else:
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{alias}")
            if g.get("object", {}).get("sha") == PIN:
                print(f"{alias}: GET-proof exists @{PIN[:8]}", flush=True)
            else:
                print(f"REF-FAIL {alias}: {json.dumps(ref)[:200]}", file=sys.stderr)
                sys.exit(1)
        # 2) диспатч — явный JSON-канон (1 реф = 1 диспатч Л188b)
        inputs = dict(INPUTS)
        inputs["lever_flag"], inputs["lever_arg"] = lf, la
        before = {x["id"] for x in api(tok,
            f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={alias}&per_page=20"
            ).get("workflow_runs", [])}
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": alias, "inputs": inputs})
        print(f"dispatch {alias} lever='{lf}'/'{la}' explicit-JSON -> "
              f"{'204 OK' if r == {} else r}", flush=True)
        time.sleep(20)
        # 3) run-id discovery строго по head_branch (S31) + created_at
        runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={alias}&per_page=20"
                   ).get("workflow_runs", [])
        new = [x for x in runs if x["id"] not in before and x["created_at"] > mc]
        if not new:
            print(f"NO-NEW-RUN {alias}", file=sys.stderr)
            sys.exit(2)
        run_ids[alias] = new[0]["id"]
        print(json.dumps({"alias": alias, "id": new[0]["id"],
                          "status": new[0]["status"], "sha": new[0]["head_sha"][:8]}),
              flush=True)
    print("RUN_IDS_JSON=" + json.dumps(run_ids, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
