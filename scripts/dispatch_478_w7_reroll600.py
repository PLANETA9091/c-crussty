#!/usr/bin/env python3
"""dispatch_478_w7_reroll600.py — [478-W7] IN-BAND РЕ-РОЛЛ 600s/640 (тик x478, WILD, v19.0).

CLAIM-ПРЕГИСТЕР (закон 14a/16; docs/PREREG_478_W7_REROLL600.md):
  Лестница 19a (Л-478-A4): ячейка 600s/640 ГРЯЗНАЯ — x7 36358134849 cpu 10,393,093
  ВНЕ канона [6.0,9.5]M (W4 шёл band-open). Задача: in-band ре-ролл той же ячейки.
  A17-рецепт: live-пул дрейфует ~650k/тик -> фиксированных оффсетов НЕТ, канон-GLOB
  band-gate [6.0,9.5]M ON; band-dead fast-fail (pre-download ~1 мин) = FREE re-roll
  (Л-477-C63 x3 / Л-477-W3 / A18 f1,f4), ре-ролл ДО in-band, <=4 попыток.
  Каждая попытка = зонд-телеметрия (runner_cpu_index в логе шага калибровки).

Диспатч: алиас round-478-w7-600 @PIN (0 код-дельт, 1 реф = 1 диспатч Л188b),
WF world-bench-parallel.yml, 640/600s, канон x466-C98 ЯВНЫМ JSON (C66C72):
gc3/fp4/ic1/fd1/guard1/rt4/bc1/pop150k/seed42/10G/xms4G, lever ""/"".
Прогноз H-478-A4a: плато <=2.9 (не 3.0), стабилити не раньше полла 9 (~400-500s).
Длительность >15 мин -> вердикт-класс DISPATCHED run-id; плато добьет тик x478+1.

Usage:
  dispatch_478_w7_reroll600.py dispatch   # ref + 1 диспатч
  dispatch_478_w7_reroll600.py reroll     # повторный диспатч того же алиаса (free band-discard)
  dispatch_478_w7_reroll600.py poll       # статус + шаг band-gate + (при done) метрики артефакта
"""
import json, re, subprocess, sys, time, zipfile, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-478-w7-600"
PIN_SHA = "e3dbb7e40acf478c6d3b1ba0a54abc5eb9c0714e"  # W7 START-коммит (прег включен, tree+2 docs/tooling, 0 bench-код-дельт)

# канон x466-C98 явным JSON + band-GLOB гейт ON (отличие от грязного x7 band-open)
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "600", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G", "gc_tune": "3",
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
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
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
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": ref, "sha": PIN_SHA})
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
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    print(f"[{label}] dispatch 204: {ALIAS} band=[6.0,9.5]M seconds=600 (expect 204)", flush=True)
    deadline = time.time() + 240
    while time.time() < deadline:
        time.sleep(15)
        hits = runs_on_branch(tok, ALIAS)
        if hits:
            rid, st, concl, ca = hits[-1]
            print(f"run-id: {rid} status={st} concl={concl} created={ca}", flush=True)
            return rid
    print("run-id: POLL-NEEDED (не поднялся за 4 мин)", flush=True)
    return None


def gate_status(tok, run_id):
    """Вернуть (phase, idx) — фаза рана по шагам job'а."""
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
    for rid, st, concl, ca in hits[-3:]:
        phase, gate = None, None
        if st != "completed":
            phase, gate = gate_status(tok, rid)
        print(f"run {rid} status={st} concl={concl} created={ca} phase={phase} band-gate={gate}", flush=True)
        if st == "completed":
            try:
                m = fetch_artifact_metrics(tok, rid)
                print("  " + (verdict(m) if m else "  artifact-NOT-FOUND"), flush=True)
            except Exception as e:
                print(f"  artifact-ERR {e}", flush=True)


def fetch_artifact_metrics(tok, run_id):
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None
    z = api(tok, f"/repos/{REPO}/actions/artifacts/{a['id']}/zip", raw=True)
    path = f"/tmp/w7_{run_id}.zip"
    open(path, "wb").write(z)
    out = {}
    with zipfile.ZipFile(path) as zf:
        for n in zf.namelist():
            if n.endswith("run-env.txt"):
                txt = zf.read(n).decode("utf-8", "replace")
                m = re.search(r"runner_cpu_index[=:]\s*(\d+)", txt)
                if m:
                    out["cpu"] = int(m.group(1))
            elif n.endswith("server-stdout.log"):
                txt = zf.read(n).decode("utf-8", "replace")
                polls = re.findall(r"TPS[^\n]*?:\s*([0-9]+\.[0-9])", txt)
                if polls:
                    out["polls_tail"] = polls[-12:]
    return out


def verdict(m):
    cpu = m.get("cpu")
    line = f"cpu={cpu} in-band={6_000_000 <= (cpu or 0) <= 9_500_000}"
    if m.get("polls_tail"):
        line += f" polls_tail={m['polls_tail']}"
    return line


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "dispatch"
    tok = token()
    if cmd == "poll":
        do_poll()
    elif cmd == "reroll":
        dispatch_once(tok, "REROLL")
    else:
        dispatch_once(tok, "DISPATCH")
