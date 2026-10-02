#!/usr/bin/env python3
"""dispatch_480_c71_leg.py — COMMANDER C71 КЛИМБ-sensn16 MEGA-SWARM v19.0 (тик ×480).

CLAIM (sensn16 pair-пул, окно якорей [6.80,6.96]M): серия 13 легов 0 legal пар,
best pair-кандидат +13.51; leg-l1 +32.01 CLEAN @7,601,999 — NO-PAIR (Δ753k).
P-матем CLM-C23 (Л-480-C23): p_leg = P(бин 0.225) × P(norm ≥ +16.7..+23.7 | бин)
≈ 2.3–4.5%/ран → E[пар из 2] ≈ 0.05–0.09, P(≥1) ≈ 4.5–8.8%; пара жива только с
leg-дроу в окне [6.80,6.96]M. Добор-волна l1a-c (стюард) + эта ×2 = лег-фид.

ЗАДАЧ: диспатч leg ×2 — алиасы round-480-c71-l1/l2 (1 диспатч = 1 ветка Л188b),
pin = origin/master 686f2258 FULL-sha 686f225830a40570fd7dddcf77c2e1e64e4ecb88
(POST refs FULL-sha + GET-verify object.sha, Л188a; 0 код-дельт от bench-канона),
workflow world-bench-parallel.yml, canon x466-C98 ЯВНЫМ JSON (урок x466-C73:
yml-дефолты = merge-поверхность): 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/
seed42/10G/xms4G + lever_flag "cmp466_c98ai" lever_arg "16" (cert n16
Л-470-S20.1). Band GLOB [6000000,9500000] fast-fail.

ПОСТ-ХОК ОКНО [6800000,6960000] — НЕ В ГЕЙТ (гейт = band GLOB only): окно
используется ТОЛЬКО пост-хок при пар-матем против якорей mxa-08@6849418 −3.70 /
mxa-12@6905659 +3.32 / s1-d@6888701 +0.59. Пороги v5-FROZEN не двигаются.

ПРЕГИСТ-ГЕЙТ (закон 14a/16): band PASS ∧ M1 STW≤23.0s ЖЁСТКО (урок leg-k
23.37→LEG-INVALID) ∧ young_avg≤200ms ∧ ARM-пруф «cmp466_c98ai: ARMED» из
АРТЕФАКТА server-stdout.log (A16-урок false-negative по log-zip) ∧ NCDFE=0 ∧
AIOOBE=0 (биом-exempt Л-474-C88.2) ∧ FIXTURE-VALIDITY VALID.
PAIR = leg_norm − anchor_norm ≥ +20 ∧ Δcpu ≤50k, min-of-3 по {mxa-08, mxa-12,
s1-d} → МЕРЖ №19 (board [480-C71] MERGE-READY {run id, pair}); иначе
DISPATCHED run-id / REFUTED_CENS (закон 12e/18-iii, абсорб тик-481).
BAND-MISS ПРОТОКОЛ: ≤1 ре-ролл/точку same-branch (закон W3/Л188c, сужен для
C71 по тик-плану; прецедент S06 36269776506→36270314846).
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master FULL sha

ALIASES = ["round-480-c71-l1", "round-480-c71-l2"]

# canon x466-C98 — ЯВНЫЙ JSON; окно [6.80,6.96]M ПОСТ-ХОК, в гейт НЕ входит
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
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "480-c71-sensn16-dispatcher"})
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
    run_ids = {}
    for alias in ALIASES:
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
        before = {x["id"] for x in api(tok,
            f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={alias}&per_page=20"
            ).get("workflow_runs", [])}
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": alias, "inputs": INPUTS})
        print(f"dispatch {alias} explicit-JSON -> {'204 OK' if r == {} else r}", flush=True)
        time.sleep(20)
        runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={alias}&per_page=20"
                   ).get("workflow_runs", [])
        new = [x for x in runs if x["id"] not in before and x["created_at"] > mc]
        if not new:
            print(f"NO-NEW-RUN {alias}", file=sys.stderr)
            sys.exit(2)
        for x in new:
            run_ids[alias] = x["id"]
            print(json.dumps({"alias": alias, "id": x["id"], "status": x["status"],
                              "sha": x["head_sha"][:8]}), flush=True)
    print("RUN_IDS_JSON=" + json.dumps(run_ids, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
