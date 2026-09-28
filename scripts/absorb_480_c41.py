#!/usr/bin/env python3
"""absorb_480_c41.py — C41 gc1-стресс young-морфология absorb (A1/B4-канон parse_gc
= absorb_478_w5.py бит-совместим). Гейты по PREREG_480_C41_GC1.md (закон 14a/16):
band [6.0,9.5]M; M1-гейт young≤200ev (прогноз FAIL = gc1-класс жив);
young ≥400ev → 638ev-класс REPRODUCED; ΣSTW-коридор vs gc2/gc3-банк [18.3,25.6]s.
Usage: absorb_480_c41.py <artifact_dir> <run_id>"""
import json, os, re, sys

BAND = (6.0e6, 9.5e6)
GC23_BANK = (18.3, 25.6)
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")


def parse_gc(text):
    pauses = []
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m:
            continue
        um, cm = UPTIME.search(line), CAUSE.search(line)
        pauses.append((float(m.group(1)), cm.group(1) if cm else "?",
                       cm.group(2) if cm else "?"))
    young = [p for p in pauses if p[1] == "Young"]
    fulls = [p for p in pauses if p[1] == "Full"]
    cause = {}
    for ms, kind, why in pauses:
        key = f"{kind}:{why.split()[0] if why else '?'}"
        cause[key] = [cause.get(key, (0, 0.0))[0] + 1, cause.get(key, (0, 0.0))[1] + ms]
    return {"stw_total_s": round(sum(p[0] for p in pauses)/1000, 2),
            "young_n": len(young),
            "young_sum_s": round(sum(p[0] for p in young)/1000, 2),
            "young_avg_ms": round(sum(p[0] for p in young)/len(young), 2) if young else 0.0,
            "fulls": len(fulls),
            "full_ms": round(sum(p[0] for p in fulls), 1),
            "max_ms": round(max((p[0] for p in pauses), default=0.0), 1),
            "cause": {k: f"{v[0]}x/{v[1]:.0f}ms" for k, v in sorted(cause.items())}}


def main():
    d, rid = sys.argv[1], sys.argv[2]
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    cpu = int(re.search(r"runner_cpu_index:\s*(\d+)", env).group(1))
    gt = re.search(r"^gc_tune:\s*(\d+)", env, re.M)
    tps_all = [float(x) for x in RE_TPS.findall(log)]
    bench = [x for x in tps_all if x < 15]
    g = parse_gc(gct)
    v = {"run": int(rid), "cpu": cpu, "gc_tune": gt.group(1) if gt else None,
         "polls_bench": bench,
         "tps_med": (sorted(bench)[len(bench)//2] if bench else None),
         "ncdfe": log.count("NoClassDefFoundError"),
         "aioobe": log.count("ArrayIndexOutOfBounds"),
         "world": (re.search(r"world sha[: ]*([0-9a-f]{8})", log).group(1)
                   if re.search(r"world sha[: ]*([0-9a-f]{8})", log) else None),
         "lever": (re.search(r"^lever_flag:\s*(.*)$", env, re.M).group(1).strip()
                   if re.search(r"^lever_flag:\s*(.*)$", env, re.M) else None),
         **g}
    v["band"] = BAND[0] <= cpu <= BAND[1]
    v["m1_young_le200"] = v["young_n"] <= 200
    v["gc23_bank"] = GC23_BANK[0] <= v["stw_total_s"] <= GC23_BANK[1]
    if v["band"] and v["ncdfe"] == 0 and v["aioobe"] == 0:
        if v["young_n"] >= 400:
            v["verdict"] = "GC1-CLASS-REPRODUCED (638ev-класс)"
        elif v["young_n"] > 200:
            v["verdict"] = f"PARTIAL-DRIFT ({v['young_n']}ev, класс-порог 400 не добран)"
        else:
            v["verdict"] = "REFUTED_CENS (young≤200 — pause-target-артефакт исчез)"
    else:
        v["verdict"] = "INVALID-RUN (band/NCDFE/AIOOBE)"
    print("C41-ABSORB-JSON " + json.dumps(v, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
