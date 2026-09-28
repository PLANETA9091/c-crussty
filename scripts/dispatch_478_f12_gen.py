#!/usr/bin/env python3
"""dispatch_478_f12_gen.py — [478-F12] Commander 19a CHUNK-GEN (тик ×478, мега-цель 19a).

CLAIM (прегистер закон 14a/16, зафиксирован ДО диспатча):
  19a chunk-gen лестница. Факты-наследство: noise in-window = 0 сэмплов
  (worldgen/noise bucket 0.1%, 177/143,414 — Л-BENCH3/packleg2); A/B noise-fill
  переведён на gen_work-ось (c24 legs FAILURE = G3.0 fixture terr-мира, не код);
  A5-тяжесть (Л-478-A5): fresh-gen стенды towers 335s (29.2 чанк/s) < terralith
  641s (15.3) < tectonic 814s (12.0) — стенды меряют гонку без quiesce.

План:
  (1) валидный gen-диспатч на канон-мире: world-bench-parallel pop150k canon
      x466-C98, алиас round-478-f12-gen @master (0 код-дельт), lever "" ;
  (2) gen_work-экстракция: BOTTLENECKS_3.md boot-фаза + server-stdout.log
      forceload-фаза (36×Marked 256 = 9216 чанков) — min-of-N локальных
      канон-ранов уже извлечён (board-число);
  (3) лестница: канон-мир чанк/s (disk-load pregen) vs стенды → 19a-ступень;
  (4) гейт: gen_work (boot+forceload) ≥15% wall → гипотеза-дельта + диспатч
      код-фикса (алиас round-478-f12-impl, cargo 0 err); иначе REFUTED_CENS
      числа. Runs >15 мин → DISPATCHED run-id (закон 18-iii).

H-478-F12 (prereg): канон-мир afb3a0b3 PREGEN → gen_work in-window ≈ 0
  (noise 0.1%), boot+forceload ≈ 44s из ~500-540s wall ≈ 8-9% < 15% →
  REFUTED_CENS: gen-ось не ботлнек канон-мира; лестница 19a замыкается
  сверху канон-disk-load ~326 чанк/s (9216/28.3s med-of-N) ≫ towers 29.2.

Usage: dispatch_478_f12_gen.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-478-f12-gen"
PIN_SHA = "848d8f14"  # 0-дельт пин: live origin/master 478-F6 FINAL (docs-only), FULL-sha берётся при диспатче (урок Л188a: полный sha)

# полный canon x466-C98 ЯВНЫМ JSON (урок C66-C72): 640/300s/fp4/gc3/ic1/fd1/
# rt4/bc1/pop150k/seed42/10G/xms4G, band [6.0,9.5]M fast-fail; 0 дельт = lever ""
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


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
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:300]}", flush=True)
        raise
    return json.loads(body) if body else {}


def ref_sha(tok, branch):
    try:
        return api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")["object"]["sha"]
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return None
        raise


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    master = ref_sha(tok, "master")
    # 0-дельт страж: пин = live origin/master на момент диспатча; ветка-алиас
    # одного sha с master (прецедент Л-470-S52.1 / Л188b)
    commit = api(tok, f"/repos/{REPO}/commits/{master}")
    touched = [f["filename"] for f in commit.get("files", [])]
    code_touch = [f for f in touched if not (f.startswith("BLACKBOARD.md") or f.startswith("docs/") or f.startswith("scripts/") or f == ".github/workflows/world-bench-parallel.yml")]
    print(f"origin/master={master[:8]} head-msg={commit['commit']['message'][:60]!r}", flush=True)
    print(f"head-files={touched} code-touch={code_touch}", flush=True)
    pin = ref_sha(tok, BRANCH)
    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    if dry:
        print(f"DRY: would pin {BRANCH} @ {master[:8]} (0 код-дельт) + dispatch canon pop150k", flush=True)
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        return

    if code_touch:
        raise SystemExit(f"0-DELTA GUARD FAIL: master head touches code {code_touch}")

    if pin != master:
        if pin is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": master})  # Л188a FULL-sha
            print(f"ref CREATED: {BRANCH} @ {master[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": master, "force": True})
            print(f"ref PATCHED: {BRANCH} -> {master[:8]}", flush=True)
        live = ref_sha(tok, BRANCH)
        if live != master:
            raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
        print(f"GET-verify OK: {BRANCH} @ {live[:8]}", flush=True)

    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    print("dispatch 204-OK", flush=True)

    rid = None
    deadline = time.time() + 240
    while time.time() < deadline:
        time.sleep(15)
        for r, st, ca in runs_on_branch(tok, BRANCH):
            print(f"poll: run {r} status={st} created={ca}", flush=True)
            rid = r
            break
        if rid:
            break
    print(json.dumps({"branch": BRANCH, "pin": master, "run_id": rid}, indent=1), flush=True)


if __name__ == "__main__":
    main()
