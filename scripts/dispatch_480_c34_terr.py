#!/usr/bin/env python3
"""dispatch_480_c34_terr.py — [480-C34] Commander 19a-STRATUM (тик ×480, мега-цель 19a).

CLAIM (прегистер закон 14a/16, зафиксирован ДО диспатча):
  r480-стратум-повтор ×1 (371-класс) — 3-я точка ансамбля S06:
  r480a −3.00@6749854 (gc3, валид) vs r480b −38.95@7083980 (G1-фолбэк
  через gc6-дыру, REFUTED_CENS Л-470-S06.1-.5). ГИПОТЕЗА-ДЕЛЬТА (закон 14a):
  разброс стратумов = ФАЗА gen_work/parse-события vs poll-окно. На канон-
  фиксстуре afb3a0b3 (ПРЕГЕН) parse-бурст boot-only (22:13:43-45), первый
  TPS-полл через ~130s — поллы фазу НЕ перекрывают (RC1/RC2, ×480-C13
  capture_math) → прогноз: чистый gc3-повтор на свежем мастере 686f2258
  падает в r480a-банд (−3±MAE 5.91), НЕ у −38.95; если ≈−30..−45 на
  clean-STW/gc3 → фаза-гипотеза получает вес, W8-квант ре-открывается.
  Сайд-гейт: стратум-стабильность сквозь эволюцию мастера 40068dbe(№11)
  → 686f2258 (№17+R0+F3) — не слепой ре-ролл.

Вектор: БАНК-V5 канон x466-C98 ЯВНЫМ JSON (s51-реплика, точная репликация
  чистой ноги r480a) с ЕДИНСТВЕННОЙ дельтой radius=480: 300s/fp4/gc3/
  ic1/fd1/fd_bit0/rt4/bc1/pop150k/seed42/10G/xms4G, band GLOB
  [6.0,9.5]M fast-fail, lever_flag/arg ПУСТЫЕ (armed=null — банк-фид
  легален; любое число в банк обязан быть на bank-feed пресете).
Мир: world_url ЯВНО = MineShield-3__Min--Normal.zip (терра-фикстура
  r480-стратума, world_sha256 afb3a0b3 — та, что диспатчит
  dispatch_470_s51_r480.py воркфлоу-дефолтом; Л-480-C15: s51-ран =
  afb3a0b3 r480 gc3). ЯВНЫЙ input > дефолт (урок x466-C73: дефолты —
  merge-поверхность). НЕ cc1b5b4d-terr-dp: там точка уже в полёте
  (round-480-c13-terr1 run 36386283052) и canon lever="" на terr+pop>0
  запрещён (Л-474-C82.2 TASK-411-A).
База: алиас round-480-c34-terr1 @origin/master 686f2258 (FULL-sha, POST
  refs + GET-verify Л188a; 1 диспатч = 1 ветка Л188b). Band-miss → 1
  ре-ролл same-branch (W3/Л188c). Runs >15 мин = DISPATCHED run-id
  (закон 12e/18-iii), абсорб чисел тик-481.

ГЕЙТЫ тик-481 (prereg): band PASS по echo шага-3; POPULATION VALID
  150k/seed42; ncdfe_real=0/aioobe=0; clean-STW ценз M1 (gc.log-primary:
  «Using ParallelGC», young_avg≤200); world_sha256==afb3a0b3 в run-env
  (фиксстура-парность к r480a/b/d/e); cpu по run-env (Л195), Δcpu≤50k к
  6749854 → пара-легальность, иначе bank-norm-only.

LEDGER: Л-480-C34.

Usage: dispatch_480_c34_terr.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-480-c34-terr1"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация (0 код-дельт)

STRATUM_WORLD_URL = ("https://storage.shield.land/public.php/dav/files/"
                     "twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip")

INPUTS = {
    "world_url": STRATUM_WORLD_URL,  # терр-фикстура r480-стратума (afb3a0b3), как в dispatch_470_s51_r480.py
    "radius": "480",  # r480-стратум: 4096 чанков (единственная дельта от канона)
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",  # armed=null — точная реплика r480a (банк-канон)
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
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
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def ref_sha(tok, ref):
    try:
        return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return None
        raise


def runs_on_branch(tok, branch):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    out = []
    for r in runs.get("workflow_runs", []):
        if r.get("head_branch") == branch:
            out.append((r["id"], r["status"], r["created_at"], r["head_sha"]))
    return out


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    # Л188a: пин FULL-sha, верифицируем live master
    live_master = ref_sha(tok, "master")
    pin = live_master if live_master and live_master.startswith(PIN[:12]) else PIN
    print(f"origin/master live = {live_master} -> pin {pin}", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != pin:
        if cur is None:
            api(tok, "/repos/PLANETA9091/c-crussty/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": pin})  # FULL-sha (урок S20: короткий = 422)
            print(f"ref CREATED {BRANCH} @ {pin[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": pin, "force": True})
            print(f"ref PATCHED {BRANCH} -> {pin[:8]}", flush=True)
    got = ref_sha(tok, BRANCH)
    if got != pin:
        raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
    print(f"GET-verify OK object.sha == {got[:8]}", flush=True)

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
    deadline = time.time() + 300
    while time.time() < deadline and rid is None:
        time.sleep(15)
        for i, st, ca, hs in runs_on_branch(tok, BRANCH):
            print(f"poll: run {i} status={st} created={ca} sha={hs[:8]}", flush=True)
            if st in ("queued", "in_progress", "completed"):
                rid = i
            break
    print("C34-DISPATCH-JSON " + json.dumps(
        {"branch": BRANCH, "pin": pin, "run_id": rid,
         "world": "MineShield-3__Min--Normal afb3a0b3 (терр-фикстура r480-стратума, s51-канон)",
         "radius": 480, "gc_tune": 3, "lever": "(empty, armed=null)",
         "hypothesis": "spread = gen_work-phase vs poll-window; predict r480a-band, not -38.95"},
        indent=1), flush=True)


if __name__ == "__main__":
    main()
