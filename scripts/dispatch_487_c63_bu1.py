#!/usr/bin/env python3
"""dispatch_487_c63_bu1.py — [487-C63] Commander C63: bu1 3-я нога min-of-3, окно Л142 (тик ×487).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча, /home/z/rounds/ROUND-487/board/CLM-C63.md):
  Бит-в-бит репродукция исторической bu1-ноги round-483-a17-w8bu1-r2
  (run 36444248520, norm +44.36, LEDGER ст.1270/1308) с новым run-id.
  bu1-канон: W8⊕c98ai (C53: r480/rt8/gc6/12G, lever cmp466_c98ai/16) + bu_defer=1,
  ветка = bare pin acffa383 (0 код-дельт). Единственная дельта ×487:
  band [8734563,8834563] (окно Л142) вместо [6.0,9.5]M.
  Fast-fail band → ре-ролл (r2/r3), band-дискард бесплатен (Л224-г).

Usage: dispatch_487_c63_bu1.py <r1|r2|r3> [--dry-run]
"""
import json, os, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# bu1-канон ×483-A17 (dispatch_483_a17_w8bu1.py 1:1), дельта = band (окно Л142)
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # bare pin round-483-a17-w8bu1-r2

INPUTS = {
    "radius": "480",
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6",
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0",
    "bu_defer": "1",  # bu1-дельта канона (при steal=0 инертен, канон-совместим)
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "8734563", "cpu_band_max": "8834563",  # ×487: окно Л142
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",  # c98ai-компо-гейт STRICT-OR 40/40
}

ROLLS = {"r1": "round-487-c63-bu1r1", "r2": "round-487-c63-bu1r2", "r3": "round-487-c63-bu1r3"}


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
    runs = api(tok, f"/repos/{REPO}/actions/runs?branch={br}&per_page=10")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", [])]


def git(*args):
    return subprocess.run(["git", *args], cwd="/home/z/c-crussty",
                          capture_output=True, text=True, timeout=120)


def main():
    args = sys.argv[1:]
    if not args or args[0] not in ROLLS or any(a not in ROLLS and a != "--dry-run" for a in args):
        raise SystemExit(f"argv-guard: usage dispatch_487_c63_bu1.py <r1|r2|r3> [--dry-run], got {args}")
    roll = args[0]
    dry = "--dry-run" in args
    BRANCH = ROLLS[roll]
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} (НЕ трогаем); bu1-нога = bare pin {PIN[:8]} + bu_defer=1, band Л142", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    # git branch+push (приказ) — PIN-коммит существует локально; cwd НЕ checkout (x466-C12.5)
    if git("rev-parse", "-q", "--verify", PIN).returncode != 0:
        raise SystemExit(f"PIN {PIN} отсутствует локально — fetch нужен")
    br = git("branch", BRANCH, PIN)
    if br.returncode != 0 and "already exists" not in br.stderr:
        raise SystemExit(f"git branch failed: {br.stderr}")
    push = git("push", "origin", f"{BRANCH}:{BRANCH}")
    if push.returncode != 0:
        raise SystemExit(f"git push failed: {push.stderr}")
    print(f"git branch+push OK: {BRANCH} -> {PIN[:8]}", flush=True)

    got = ref_sha(tok, BRANCH)
    if got != PIN:
        raise SystemExit(f"POST-PUSH VERIFY FAIL (Л188a): got {got}")
    print(f"GET-verify OK object.sha == {got}", flush=True)

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    if r != {}:
        raise SystemExit(f"dispatch failed: {r}")
    print("dispatch 204-OK", flush=True)

    rid = None
    deadline = time.time() + 300
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, cc, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid = rid_
                break
    out = {"roll": roll, "branch": BRANCH, "pin": PIN, "run_id": rid, "inputs": INPUTS}
    json.dump(out, open(f"/home/z/rounds/ROUND-487/c63_dispatch_{roll}.json", "w"), indent=1)
    if rid is None:
        print("run-id not visible in 300s (204 принят, discovery по head_sha позже)", flush=True)
        return
    print(f"RUN-ID {rid}", flush=True)


if __name__ == "__main__":
    main()
