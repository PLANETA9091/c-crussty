#!/usr/bin/env python3
"""dispatch_478_a18_probe_feed.py — [478-A18] 6.59-БИН 2/3 -> 3/3 ФИДЫ + ЗОНД-ДРОУ БЕЗ ГЕЙТА (тик x478, v19.0).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча этим коммитом):
  Пара-пул №17/18 (6.59-бин): лег +23.56@6876945, якорь <=+3.56, Δ<=50k
  pair-fresh, состояние 2/3. Цель: 3/3 фиды.
  A17-урок x478: фиксированные оффсеты устаревают за тик (low-mode live-пула
  дрейфует ~650k/тик) -> рецепт "зонд-дроу без гейта".
  Live-пул ценз на старт тика: STRICT 3/18=16.7%, B-плечо 27.8%.

ПЛАН (0 код-дельт, vanilla @master, ref-push алиасов, 1 реф=1 диспатч Л188b):
  A) 2 зонд-диспатча vanilla @master БЕЗ cpu-гейта
     (алиасы round-478-a18-p1/p2, cpu_band_min/max="") -> ценз live-пула:
     run-env.txt runner_cpu_index = фактическая позиция пула (дрейф-тизер).
  B) 4 фида с мишенями 6.59-бина: границы из прегистов
     (rg "6.59" docs/ -> Л-477-C42/W3): sensn16-бин [6826945,6926945]
     (алиасы round-478-a18-f1..f4, cpu_band gate = бин, порог НЕ двигается).
     Fallback-ветка (не активирована: границы найдены) = соседние STRICT
     кластеры BANK_V5_FREEZE.
  C) Вердикт: hit+pair -> board [478-A18] HIT {run id, cpu, pair};
     0/4 -> DISPATCHED телеметрия пула; runs >15 мин -> DISPATCHED run-id.

ГАЙТЫ АБСОРБА (решает артефакт, run-env.txt Л195/Л240.1):
  G1 в-бин: runner_cpu_index ∈ [6826945,6926945] (Δ<=50k к легу 6876945)
  G2 CLEAN M1: STW ALL <=23.0s, young avg <=200ms
  G3 v5-норма по замороженной таблице BANK_V5_FREEZE §2 (лин-интерп,
     канон-прокси): anchor_norm <= +3.56 (порог пула №17/18), коридор >= -8
  G4 pair = 23.56 - anchor_norm; >= +20 = фид открывает цикл закона 18
  Band канон эры 6.0-9.5M не двигается; зонды без гейта = телеметрия.

Канон x466-C98: 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G,
lever_flag="" (ваниль-якорь). PIN vanilla = 2700571f (master декларации тика).

Usage: dispatch_478_a18_probe_feed.py [dispatch|poll] [--dry-run]
"""
import json, re, subprocess, sys, time, zipfile, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "2700571f5549c3e358724adea2ab99ebb1c14be8"

PROBES = ["round-478-a18-p1", "round-478-a18-p2"]
FEEDS = ["round-478-a18-f1", "round-478-a18-f2", "round-478-a18-f3", "round-478-a18-f4"]
ALL = PROBES + FEEDS

# sensn16/6.59-бин из прегиста Л-477-C42/W3 — порог НЕ двигается
BIN_MIN, BIN_MAX = "6826945", "6926945"
LEG_V5 = 23.56  # лег пары-пула №17/18 @6876945

BASE_INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G", "gc_tune": "3",
    "lever_flag": "", "lever_arg": "",
}
INPUTS_BY_BRANCH = {}
for _b in PROBES:
    INPUTS_BY_BRANCH[_b] = dict(BASE_INPUTS, cpu_band_min="", cpu_band_max="")  # БЕЗ гейта
for _b in FEEDS:
    INPUTS_BY_BRANCH[_b] = dict(BASE_INPUTS, cpu_band_min=BIN_MIN, cpu_band_max=BIN_MAX)

# замороженная таблица tps_exp_v5 (BANK_V5_FREEZE §2): cpu_M -> exp
EXP_V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]


def exp_v5(cpu):
    m = cpu / 1e6
    if m <= EXP_V5[0][0]:
        return EXP_V5[0][1]
    if m >= EXP_V5[-1][0]:
        return EXP_V5[-1][1]
    for (m0, e0), (m1, e1) in zip(EXP_V5, EXP_V5[1:]):
        if m0 <= m <= m1:
            return e0 + (e1 - e0) * (m - m0) / (m1 - m0)
    return EXP_V5[-1][1]


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
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return body if raw else (json.loads(body) if body else {})


def ensure_ref(tok, br):
    ref = f"refs/heads/{br}"
    try:
        api(tok, f"/repos/{REPO}/{ref}", method="POST",
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
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def do_dispatch(dry):
    tok = token()
    for br in ALL:
        ensure_ref(tok, br)
    if dry:
        print("DRY-RUN OK — refs verified, no dispatches", flush=True)
        return
    for br in ALL:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        gate = "NO-GATE(probe)" if br in PROBES else f"bin[{BIN_MIN},{BIN_MAX}]"
        print(f"dispatched: {br} gate={gate} (expect 204)", flush=True)
        time.sleep(20)
    ids, deadline = {}, time.time() + 300
    while time.time() < deadline and len(ids) < len(ALL):
        time.sleep(15)
        for br in ALL:
            if br in ids:
                continue
            hits = runs_on_branch(tok, br)
            if hits:
                ids[br] = hits[0]
                print(f"run-id: {br} -> {hits[0][0]} status={hits[0][1]}", flush=True)
    for br in ALL:
        h = ids.get(br)
        print(f"=== A18 branch={br} pin={PIN_SHA[:8]} run={h[0] if h else 'POLL-NEEDED'} "
              f"status={h[1] if h else '-'} concl={h[2] if h else '-'} ===", flush=True)


def fetch_artifact_metrics(tok, run_id):
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None
    z = api(tok, f"/repos/{REPO}/actions/artifacts/{a['id']}/zip", raw=True)
    path = f"/tmp/a18_{run_id}.zip"
    open(path, "wb").write(z)
    out = {}
    with zipfile.ZipFile(path) as zf:
        names = zf.namelist()
        for want, key in (("run-env.txt", "runenv"), ("BOTTLENECKS_3.md", "bn")):
            match = next((n for n in names if n.endswith(want)), None)
            if not match:
                continue
            txt = zf.read(match).decode("utf-8", "replace")
            if key == "runenv":
                m = re.search(r"runner_cpu_index[=:]\s*(\d+)", txt)
                if m:
                    out["cpu"] = int(m.group(1))
            else:
                for pat, key2 in ((r"runner_cpu_index[=:]\s*(\d+)", "cpu2"),
                                  (r"TPS[^:\n]*med[^:\n]*?[=:]\s*([0-9.]+)", "tps_med"),
                                  (r"STOP[- ]THE[- ]WORLD[^\n]*", "stw_line"),
                                  (r"Full GC[^\n]*", "full_line"),
                                  (r"FIXTURE-VALIDITY[^\n]*", "fixture"),
                                  (r"POPULATION[^\n]*", "pop")):
                    mm = re.search(pat, txt, re.I)
                    if mm:
                        out.setdefault(key2, mm.group(1) if mm.groups() else mm.group(0)[:160])
    return out


def verdict(tok, br, m):
    cpu = m.get("cpu") or m.get("cpu2")
    tps = m.get("tps_med")
    line = f"{br}: cpu={cpu}"
    if cpu:
        inbin = 6826945 <= cpu <= 6926945
        line += f" in-bin={inbin} Δ={cpu-6876945:+d}"
    if tps:
        e = exp_v5(float(tps) and cpu) if cpu else None
        if e:
            norm = (float(tps) / e - 1) * 100
            line += f" tps_med={tps} exp_v5={e:.4f} norm={norm:+.2f}пп"
            if cpu and 6826945 <= cpu <= 6926945:
                line += f" anchor<=+3.56:{norm <= 3.56} pair={LEG_V5 - norm:+.2f}"
    for k in ("stw_line", "fixture", "pop"):
        if m.get(k):
            line += f" | {m[k][:80]}"
    return line


def do_poll():
    tok = token()
    for br in ALL:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        print(f"{br}: run {rid} status={st} concl={concl} created={ca}", flush=True)
        if st == "completed":
            try:
                m = fetch_artifact_metrics(tok, rid)
                print("  " + (verdict(tok, br, m) if m else "  artifact-NOT-FOUND"), flush=True)
            except Exception as e:
                print(f"  artifact-ERR {e}", flush=True)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] in ("dispatch", "poll") else "dispatch"
    if cmd == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
