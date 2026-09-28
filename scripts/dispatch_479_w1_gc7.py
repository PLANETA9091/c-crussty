#!/usr/bin/env python3
"""dispatch_479_w1_gc7.py — COMMANDER 479-W1 MEGA-SWARM v19.0 WILD (тик ×479).

CLAIM (прегист закон 14a/16, этот коммит): безумие — gc7-канал (RCC 1024M) на
стресс-окне 600s/640. gc2 канон-канал REFUTED (normtool ×8: −35.57), но
стресс-канал ОТКРЫТ: A4 young-стена gc1 638ev→gc2 151ev (аллокационная стена
декелерирует, 600s накапливает young-долг) + A13 G3-гипотеза ΔSTW ≤−1.5s
(CC-fulls RCC512→1024 ≈ 2-3→1, колено-прогноз −1.8..−3.7s). База сравнения =
W7-чистая ячейка run 36369278270 (gc3 600s/640 @round-478-w7-600 e3dbb7e4,
плато 2.6 @poll9-11, norm +17.64 @7,022,396, SUCCESS wall 23.8 мин).

Диспатч: алиас round-479-w1-gc7 @PIN (master 5d724fa0 + этот prereg-скрипт,
0 bench-код-дельт — delta = docs/scripts only), WF world-bench-parallel.yml,
640/600s, канон x466-C98 ЯВНЫМ JSON c ОДНОЙ дельтой gc_tune=3→7 (RCC 1024M):
fp4/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G, lever ""/"" (ваниль-якорь),
band [6.0,9.5]M fast-fail ON (band-dead = free re-roll Л188c, ≤2 попытки).
Закон-5 запреты чисты (не ZGC/THP; gc_tune 4/5/6 не трогаем; ваниль-паритет).

ГАЙТ сравнения с базой 36369278270 (решает артефакт world3-bench):
  G1 band PASS cpu(run-env) ∈ [6.0,9.5]M
  G2 fixture VALID ∧ NCDFE=0 ∧ AIOOBE=0
  G3 ΔSTW = STW(gc7) − STW(gc3-base) ≤ −1.5s  (A13-колено на стресс-окне)
  G4 плато не хуже: tail-поллы (poll9-конец) ≥ базовых 2.6
  ev-рид-аут: pauses/young/full_cc/full_md из gc.log (механизм, не цензор)
Гейт-ПРОХОД → board [479-W1] {STW, плато, ev} число-вердикт ΔSTW;
Гейт-ФЕЙЛ → REFUTED_CENS (числа-потолка закон 18-ii).
Runs >15 мин → DISPATCHED run-id (закон 18-iii; база 23.8 мин — закономерно).

Usage:
  dispatch_479_w1_gc7.py dispatch   # ref + 1 диспатч (204) + run-id
  dispatch_479_w1_gc7.py reroll     # band-dead free re-roll (тот же алиас)
  dispatch_479_w1_gc7.py poll       # фаза/гейт + (при done) вердикт-числа
"""
import json, re, subprocess, sys, time, zipfile, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-479-w1-gc7"
BASE_RUN = 36369278270          # W7 gc3 600s/640 база (плато 2.6, STW из артефакта)
PIN_SHA = "065aa6dab78bd2da72ef690b420aae6fd3722fc4"  # заполняется коммитом prereg (закон 14a/16)

INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "600", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G", "gc_tune": "7",  # ДЕЛЬТА: RCC 1024M
    "lever_flag": "", "lever_arg": "",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, raw=False):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-w1-gc7-dispatcher"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}", flush=True)
        raise
    return body if raw else (json.loads(body) if body else {})


def ensure_ref(tok, br):
    ref = f"refs/heads/{br}"
    try:
        api(tok, f"/repos/{REPO}/git/refs", method="POST", data={"ref": ref, "sha": PIN_SHA})
        print(f"ref CREATED: {br} @ {PIN_SHA[:8]}", flush=True)
    except urllib.error.HTTPError as e:
        if e.code == 422:
            cur = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
            if cur != PIN_SHA:
                raise SystemExit(f"REF CONFLICT: {br} @ {cur[:8]} != PIN")
            print(f"ref EXISTS-OK: {br} @ {cur[:8]}", flush=True)
        else:
            raise
    live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
    assert live == PIN_SHA, f"GET-verify fail {br}"
    return live


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=60")
    return sorted([(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"))
                   for r in runs.get("workflow_runs", []) if r.get("head_branch") == br],
                  key=lambda x: x[0])


def dispatch_once(tok, label):
    ensure_ref(tok, ALIAS)
    before = {h[0] for h in runs_on_branch(tok, ALIAS)}
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    print(f"[{label}] dispatch 204: {ALIAS} seconds=600 gc_tune=7 band=[6.0,9.5]M", flush=True)
    deadline = time.time() + 240
    while time.time() < deadline:
        time.sleep(15)
        for rid, st, concl, ca in runs_on_branch(tok, ALIAS):
            if rid not in before:
                print(f"run-id: {rid} status={st} concl={concl} created={ca}", flush=True)
                return rid
    print("run-id: POLL-NEEDED (не поднялся за 4 мин)", flush=True)
    return None


def gate_status(tok, run_id):
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs").get("jobs", [])
    for j in jobs:
        steps = {s["name"]: s.get("conclusion") for s in j.get("steps", [])}
        gate = next((v for k, v in steps.items() if "band gate" in k.lower()), None)
        return j.get("status"), gate
    return None, None


def do_poll():
    tok = token()
    hits = runs_on_branch(tok, ALIAS)
    if not hits:
        print(f"{ALIAS}: NO-RUN", flush=True)
        return
    for rid, st, concl, ca in hits[-2:]:
        phase = gate = None
        if st != "completed":
            phase, gate = gate_status(tok, rid)
        print(f"run {rid} status={st} concl={concl} created={ca} phase={phase} band-gate={gate}", flush=True)
        if st == "completed" and concl == "success":
            try:
                m = w1_metrics(tok, rid)
                print("  " + (verdict_line(m) if m else "  artifact-NOT-FOUND"), flush=True)
            except Exception as e:
                print(f"  artifact-ERR {e}", flush=True)


def w1_metrics(tok, run_id):
    """{cpu, plateau_tail, stw, ev...} из артефакта world3-bench (normtool-канон)."""
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None
    z = api(tok, f"/repos/{REPO}/actions/artifacts/{a['id']}/zip", raw=True)
    path = f"/tmp/w1_{run_id}.zip"
    open(path, "wb").write(z)
    sys.path.insert(0, "/home/z/c-crussty/scripts")
    import normtool_478 as nt
    r = nt.norm_run(run_id, "/tmp/w1_art")
    with zipfile.ZipFile(path) as zf:
        env = next((n for n in zf.namelist() if n.endswith("run-env.txt")), None)
        idx = int(re.search(r"runner_cpu_index[:=]\s*(\d+)",
                            zf.read(env).decode("utf-8", "replace")).group(1)) if env else 0
    r["cpu_index_api"] = idx
    return r


def verdict_line(m):
    tail = m["raw_polls_stdout"][-6:] or m["polls_valid_c55"][-6:]
    plateau = min(tail) if tail else None
    h = m.get("host_M1", {})
    return (f"cpu={m['cpu_index']} plateau_tail={tail} min={plateau} "
            f"STW={h.get('stw_total_s')}s ev={h.get('young_n')}y+{h.get('full_n')}f"
            f"({h.get('full_cc')}cc/{h.get('full_md')}md) young_avg={h.get('young_avg_ms')}ms "
            f"verdict={m['verdict']}")


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "dispatch"
    tok = token()
    if cmd == "poll":
        do_poll()
    elif cmd == "reroll":
        dispatch_once(tok, "REROLL")
    else:
        dispatch_once(tok, "DISPATCH")
