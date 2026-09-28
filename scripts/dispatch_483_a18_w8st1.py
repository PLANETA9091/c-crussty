#!/usr/bin/env python3
"""dispatch_483_a18_w8st1.py — [483-A18] Commander A18 ТРОЙНАЯ КЛЕТКА: W8⊕c98ai + steal=1 ×1 (тик ×483).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча в /home/z/rounds/ROUND-483/board/CLM-A18.md):
  Компо-гипотеза W8⊕c98ai+steal=1 — тройная клетка:
    - C53-база: W8⊕c98ai компо-лег norm +40.08 (run 36427166966, min-of-6 +40.31..+45.61, крупнейшая пара эры);
    - C43-ось: steal — прецедент rt8+steal +20.49 (M1 PASS, z+2.95), реплика ×483 не снята;
    - C92-канон: rt8+steal +20.49 → W8(r480)+steal может дать ≥бар-компо.
  ПРЕГИСТ: pair = leg_norm − anchor_norm ≥+20 → вердикт-мерж-кандидат config-leg; pair <0 → клетка закрыта.

Вектор: канон W8C (копия INPUTS ×482-C53) с ОДНОЙ дельтой region_steal 0→1.
  Код-дельт НЕТ (ветка = bare pin acffa3839b09a3388d4949d3767ab0a15b4fcd94, master ×482-учёт)
  -> push-гейты cargo/javap не применимы. Запреты закона 5 соблюдены (fluid_dirty 0,
  fluid_bitmask 0, inside_bitmask 0, ZGC нет, players 4, bu_defer 0, skip_store_bb 0).

Usage: dispatch_483_a18_w8st1.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-483-a18-w8st1"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master, ×482-учёт (live GET-verified 15:35Z)

INPUTS = {
    "radius": "480",  # r480-стратум (W8-домен S52)
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6",  # ParallelGC+MetaspaceSize256M+RCC512M
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",  # W8 Amdahl +23.7пп gen-ось
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "1", "bu_defer": "0",  # ДЕЛЬТА: region_steal 0→1 (C43/C92-ось rt8+steal)
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",  # BAND fast-fail; band-miss → ре-ролл ≤2
    # c98ai-компо-гейт: STRICT-OR 40/40 — один флаг армит climb5⊕sensn16⊕collide
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",  # cert-конфиг МЕРЖ №19
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


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN} (×482-учёт base, mission)", flush=True)
    assert live_master == PIN, "master moved since claim — re-pin required"

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": PIN})  # FULL-sha (урок S20: 422 на коротком)
            print(f"ref CREATED {BRANCH} @ {PIN[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": PIN, "force": True})
            print(f"ref PATCHED {BRANCH} -> {PIN[:8]}", flush=True)
    got = ref_sha(tok, BRANCH)
    if got != PIN:
        raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
    print(f"GET-verify OK object.sha == {got}", flush=True)

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    if r == {}:
        print("dispatch 204-OK", flush=True)
    else:
        raise SystemExit(f"dispatch failed: {r}")

    rid = None
    created = None
    deadline = time.time() + 240  # run id в окне 40-90с, даём запас до 240с
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid, created = rid_, ca
                break
    if rid is None:
        print("run-id not visible in 240s (204 принят, discovery по head_sha позже) = DISPATCHED (12e)", flush=True)
    else:
        print(f"RUN-ID {rid} created {created}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid, "created": created,
               "inputs": INPUTS},
              open("/home/z/rounds/ROUND-483/a18_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
