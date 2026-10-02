#!/usr/bin/env python3
"""absorb_479_b4.py — [479-B4] dpfull-стенд перепроверка absorb (v19.0 tick ×479).

RUN: 36378524664 round-479-b4-dpfull @ed705c5d (№17-master, 0 код-дельт,
lever ∅, canon x466-C98 pop150k, world MineShield-3 Min). Диспатч прегист
scripts/dispatch_479_b4_dpfull.py (закон 14a/16).

Гейты чтения (пост-фактум из артефакта world3-bench):
  G1 boot: "Done (" в server-stdout (boot complete, run_world3.sh waiting-for-Done)
  G2 AIOOBE: счёт "ArrayIndexOutOfBounds" (cmp420-biomes-selftest не-гейт Л-474-C88.2)
  G3 POP: "POPULATION INJECT DONE" ∧ "POPULATION FIXTURE-VALIDITY: VALID"
  G4 NCDFE=0; G5 band 6.0-9.5M (run-env runner_cpu_index); G6 BENCH-4 FIXTURE-VALIDITY
  G7 M1: STW-total ≤23.0s ∧ young_avg ≤200ms (v5-FROZEN пороги не тронуты)

Census fn-пайп (методика Л-478-B6, идентичные паттерны): cpu-collapsed.txt →
  ServerFunctionManager ∪ CommandFunction ∪ FunctionManager_other ∪ fn-executor ∪
  commands-union ∪ brigadier-deep ∪ jigsaw/template ∪ pack-load; база B6
  0/348,656 = 0.0000% ALL-CPU (×3 ваниль-ноги). Тик-анкер tickChildren
  27.60-28.05% + harness-приборка (EntityCommand/spark) как константа прибора.
"""
import json, os, re, shutil, subprocess, sys, time, zipfile

REPO = "PLANETA9091/c-crussty"
RUN = 36378524664
BRANCH = "round-479-b4-dpfull"
OUT = "/home/z/rounds/ROUND-479/B4"
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
BASE = "B6: 0/348,656 (0.0000% ALL-CPU, ×3 ваниль-ноги Л-478-B6)"

# census-паттерны — ТОЧНАЯ реплика b6_fn_pipe_census.py (Л-478-B6)
PATTERNS = {
    "FN_QUEUE_ServerFunctionManager": ["ServerFunctionManager"],
    "CommandFunction": ["CommandFunction"],
    "FunctionManager_other": ["FunctionManager"],
    "functions_executor_thread": ["functions-executor", "FunctionLibrary", "ServerFunctionLoader"],
    "commands_union_net_mc_commands": ["net/minecraft/commands", "CommandDispatcher", "brigadier",
                                       "ExecutionContext", "CommandQueueEntry", "runCommandQueue", "ExecuteCommand"],
    "brigadier_dispatch_deep": ["CommandDispatcher.execute", "CommandDispatcher.parse"],
    "jigsaw_structure_dp_path": ["JigsawPlacement", "StructurePoolElement", "SinglePoolElement",
                                 "StructureTemplate", "TemplateStructurePiece", "StructureProcessor",
                                 "StructureManager", "JigsawJunction"],
    "pack_datapack_load": ["PackRepository", "WorldLoader", "PackResources", "datapack"],
    "harness_EntityCommand": ["EntityCommand"],
    "harness_spark": ["me/lucko/spark", "lucko/spark"],
    "kernel_crussty_all": ["dev/crussty", "crussty"],
    "TICK_ANCHOR_tickChildren": ["tickChildren"],
}
DP_PLANE = ["FN_QUEUE_ServerFunctionManager", "CommandFunction", "FunctionManager_other",
            "functions_executor_thread", "brigadier_dispatch_deep",
            "jigsaw_structure_dp_path", "pack_datapack_load"]

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    path = path.lstrip("/")  # GitHub 404-strict: no double slash after repo (2026-09)
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


def run_status(t):
    r = api(t, f"/actions/runs/{RUN}")
    return r.get("status"), r.get("conclusion"), r.get("head_sha", "")


def parse_gc(text):
    pauses = []
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m:
            continue
        um, cm = UPTIME.search(line), CAUSE.search(line)
        pauses.append((float(m.group(1)), cm.group(1) if cm else "?"))
    young = [p for p in pauses if p[1] == "Young"]
    fulls = [p for p in pauses if p[1] == "Full"]
    return {"stw_total_s": round(sum(p[0] for p in pauses)/1000, 2),
            "young_n": len(young),
            "young_avg_ms": round(sum(p[0] for p in young)/len(young), 2) if young else 0.0,
            "fulls": len(fulls)}


def fetch_artifact(t):
    arts = api(t, f"/actions/runs/{RUN}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None, "no world3-bench artifact"
    zpath = f"{OUT}/art_{RUN}.zip"
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url, "-o", zpath],
                       capture_output=True, timeout=600)
    if p.returncode != 0 or open(zpath, "rb").read(2) != b"PK":
        return None, "download/PK fail"
    d = f"{OUT}/art"
    os.makedirs(d, exist_ok=True)
    with zipfile.ZipFile(zpath) as z:
        for m in ("run-env.txt", "server-stdout.log", "gc.log", "cpu-collapsed.txt",
                  "BOTTLENECKS_3.md"):
            names = [n for n in z.namelist() if n.endswith(m)]
            if not names:
                if m == "cpu-collapsed.txt":
                    return None, "missing cpu-collapsed.txt (census impossible)"
                continue
            with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                shutil.copyfileobj(src, dst)
    return d, ""


def census(d):
    counts = {k: 0 for k in PATTERNS}
    total = 0
    hits = {k: [] for k in PATTERNS}
    with open(os.path.join(d, "cpu-collapsed.txt"), errors="replace") as f:
        for raw in f:
            line = raw.rstrip("\n")
            i = line.rfind(" ")
            try:
                n = int(line[i+1:])
            except (ValueError, IndexError):
                continue
            if n <= 0:
                continue
            total += n
            stack = line[:i]
            for k, pats in PATTERNS.items():
                if any(p in stack for p in pats):
                    counts[k] += n
                    if len(hits[k]) < 3:
                        hits[k].append(stack[:220])
    return total, counts, hits


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    deadline = time.time() + (int(sys.argv[1]) if len(sys.argv) > 1 else 1200)
    while True:
        st, cc, sha = run_status(t)
        print(f"[{time.strftime('%H:%M:%S')}] run {RUN}: {st}/{cc} @{sha[:8]}", flush=True)
        if st == "completed":
            break
        if time.time() > deadline:
            print(f"DISPATCHED run-id {RUN} (>deadline, стенд в полёте)", flush=True)
            return
        time.sleep(60)
    if cc != "success":
        print(f"FAILURE run {RUN} conclusion={cc} — смотреть job-логи", flush=True)
        return
    d, err = fetch_artifact(t)
    if not d:
        print(f"artifact error: {err}", flush=True)
        return
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    bn = open(os.path.join(d, "BOTTLENECKS_3.md"), errors="replace").read()

    cpu = int(re.search(r"runner_cpu_index:\s*(\d+)", env).group(1))
    polls_all = [float(x) for x in RE_TPS.findall(log)]
    polls = [x for x in polls_all if x < 15]
    med = __import__("statistics").median(polls[:5]) if polls[:5] else None
    g = parse_gc(gct)
    gates = {
        "boot_done": "Done (" in log,
        "ncdfe": log.count("NoClassDefFoundError"),
        "aioobe": log.count("ArrayIndexOutOfBounds"),
        "pop_inject_done": "POPULATION INJECT DONE" in log,
        "pop_fixture_valid": "POPULATION FIXTURE-VALIDITY: VALID" in log,
        "bench4_fixture_valid": "FIXTURE-VALIDITY: VALID" in bn,
        "band": BAND[0] <= cpu <= BAND[1],
        "m1": g["stw_total_s"] <= STW_LIMIT_S and g["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS,
    }
    total, counts, hits = census(d)
    dp_total = sum(counts[k] for k in DP_PLANE)
    res = {
        "run": RUN, "branch": BRANCH, "head_sha": sha[:8], "conclusion": cc,
        "cpu": cpu, "band": gates["band"],
        "tps_med5": med, "polls_n": len(polls),
        "gc": g, "gates": gates,
        "census": {"total_all_cpu": total,
                   "dp_plane_union_samples": dp_total,
                   "dp_plane_pct": round(dp_total*100.0/total, 4) if total else None,
                   "tick_anchor_pct": round(counts["TICK_ANCHOR_tickChildren"]*100.0/total, 4) if total else None,
                   "detail": {k: counts[k] for k in PATTERNS},
                   "base": BASE},
        "hits": hits,
    }
    verdict = "REPRO_CENS" if (gates["boot_done"] and gates["pop_inject_done"]
                               and gates["pop_fixture_valid"] and gates["ncdfe"] == 0
                               and gates["band"] and dp_total == 0) else \
              ("DISPATCHED" if not gates["boot_done"] else "CENS-HIT" if dp_total else "GATE-FAIL")
    res["verdict"] = verdict
    with open(f"{OUT}/absorb_479_b4.json", "w") as f:
        json.dump(res, f, indent=1, ensure_ascii=False)
    print(json.dumps(res, indent=1, ensure_ascii=False), flush=True)
    print(f"saved {OUT}/absorb_479_b4.json", flush=True)


if __name__ == "__main__":
    main()
