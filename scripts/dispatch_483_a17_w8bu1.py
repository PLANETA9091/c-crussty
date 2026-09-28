#!/usr/bin/env python3
"""dispatch_483_a17_w8bu1.py — [483-A17] Commander A17 КОМПО-СУП: W8⊕c98ai + bu_defer=1 (тик ×483).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча):
  Новая клетка компо-супа: C53-канон W8⊕c98ai (r480/rt8/gc6/12G, lever cmp466_c98ai/16)
  + ДЕЛЬТА bu_defer=1. Прегист: bu-ось была −канон bu0; G2 NPE-риск s7176 при bu1
  — честный ценз: если NPE, это данные (не band-miss по коду, а исход самой клетки).

Вектор: C53-канон копируется 1:1 из dispatch_482_c53_w8compo.py, единственные дельты:
  bu_defer 0->1, алиас round-483-a17-w8bu1, PIN = master acffa383 (учёт ×482).
  Код-дельт НЕТ (ветка = bare pin acffa383) -> push-гейты cargo/javap не применимы.

Usage: dispatch_483_a17_w8bu1.py [--dry-run]
"""
import json, os, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = os.environ.get("A17_BRANCH", "round-483-a17-w8bu1")
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master ×482-учёт (БОРД: acffa383)

INPUTS = {
    "radius": "480",
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6",
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0",
    "bu_defer": "1",  # ДЕЛЬТА A17: bu-ось флип (пегист: −канон bu0; G2 NPE-риск s7176 = данные)
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",  # c98ai-компо-гейт STRICT-OR 40/40
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
    print(f"origin/master live = {live_master} -> pin {PIN} (A17: C53-канон + bu_defer=1)", flush=True)

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
    deadline = time.time() + 240
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid = rid_
                break
    if rid is None:
        print("run-id not visible in 240s (204 принят, discovery по head_sha позже)", flush=True)
        json.dump({"branch": BRANCH, "pin": PIN, "run_id": None,
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-483/a17_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-483/a17_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
