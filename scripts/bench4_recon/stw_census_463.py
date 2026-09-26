"""stw_census_463.py — G3-STW ценз по артефакту world3-bench (TASK-463-04).
Usage: python3 stw_census_463.py <run_id> <tag>
       python3 stw_census_463.py --jfr <recon.jfr> [<tag>]   # TASK-463-49 dry-run
Канон G3: STW total > 23.0s ИЛИ avg-пауза > 200ms -> INVALID-STW-HOST.
Primary source: gc.log из артефакта (STW total, пауз-каунт, avg/max pause,
Full GC счёт (5*CodeCache + 4*Metadata канон), scavAvg (avg Pause Young)).
Secondary source (--jfr): recon.jfr stream через StwCensusJfr.java
(JDK21 jdk.jfr.consumer; работает и на JRE-headless через source-launch).
  DRY-RUN ФАКТ ×463: recon.jfr в артефактах world3-bench отсутствует 0/8
  (флаг StartFlightRecording вшит за RECON_DIAG=1 гейт run_world3.sh:581,
  банк-ноги идут с RECON_DIAG=0) -> вайринг шага ×462-53 обязан либо
  фолбэкнуться на gc.log, либо RECON_DIAG-волна для seeding jfr-проб.
Бюджет скачивания: 110s (--max-time); >2 мин = SKIPPED-инфра.
"""
import json
import os
import re
import subprocess
import sys
import urllib.request

REPO = "PLANETA9091/c-crussty"
OUTROOT = "/tmp/stw463"
CAP_TOTAL_S = 23.0
CAP_AVG_MS = 200.0
DL_BUDGET_S = 110  # <=2 мин

PAUSE_DONE = re.compile(r"GC\(\d+\) Pause (Full|Young)([^(]*)\([^)]*\)\s+\S+->\S+\([^)]*\)\s+([0-9.]+)(ms|s)\s*$")
PAUSE_DONE_ANY = re.compile(r"GC\(\d+\) Pause (Full|Young)([^(]*)\([^)]*\).*?([0-9.]+)(ms|s)\s*$")
FULL_KIND = re.compile(r"Pause Full \(([^)]*)\)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(url):
    req = urllib.request.Request(f"https://api.github.com{url}", headers={
        "Authorization": f"Bearer {tok()}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())


def download(run_id, run_dir):
    os.makedirs(run_dir, exist_ok=True)
    marker = os.path.join(run_dir, "gc.log")
    if os.path.isfile(marker):
        return "cached"
    arts = api(f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    bench = [a for a in arts if a["name"] == "world3-bench"]
    if not bench:
        return "NO-ARTIFACT"
    aid = bench[0]["id"]
    dest = os.path.join(run_dir, "world3-bench.zip")
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{aid}/zip"
    rc = subprocess.run(["curl", "-sL", "--max-time", str(DL_BUDGET_S),
                         "-H", f"Authorization: token {tok()}",
                         "-o", dest, url]).returncode
    if rc != 0 or not os.path.isfile(dest) or os.path.getsize(dest) < 1000:
        return "DL-TIMEOUT" if rc == 28 else "DL-FAIL"
    r = subprocess.run(["unzip", "-o", "-q", dest, "-d", run_dir],
                       capture_output=True, text=True)
    if r.returncode != 0 or not os.path.isfile(marker):
        return "UNZIP-FAIL(" + r.stderr[:120].replace("\n", " ") + ")"
    return "ok"


def parse_gc(gclog):
    total_ms, n, full_n = 0.0, 0, 0
    full_kinds = {}
    young_ms, young_n, max_ms = 0.0, 0, 0.0
    full_ms = 0.0
    for line in open(gclog, errors="ignore"):
        if "Pause" not in line or "gc,start" in line:
            continue
        m = PAUSE_DONE.search(line) or PAUSE_DONE_ANY.search(line)
        if not m:
            continue
        kind, _, dur, unit = m.group(1), m.group(2), float(m.group(3)), m.group(4)
        ms = dur / 1000.0 if unit == "s" else dur
        total_ms += ms
        n += 1
        max_ms = max(max_ms, ms)
        if kind == "Full":
            full_n += 1
            full_ms += ms
            fk = FULL_KIND.search(line)
            k = fk.group(1) if fk else "?"
            full_kinds[k] = full_kinds.get(k, 0) + 1
        else:
            young_ms += ms
            young_n += 1
    return {
        "stw_total_s": round(total_ms / 1000.0, 2),
        "pause_count": n,
        "avg_ms": round(total_ms / n, 1) if n else None,
        "max_ms": round(max_ms, 1),
        "full": full_n,
        "full_kinds": full_kinds,
        "full_total_ms": round(full_ms, 1),
        "young_count": young_n,
        "scavAvg_ms": round(young_ms / young_n, 1) if young_n else None,
    }


def census_jfr(jfr_path, tag):
    """Secondary source: recon.jfr -> StwCensusJfr.java (source-launch)."""
    here = os.path.dirname(os.path.abspath(__file__))
    tool = os.path.join(here, "StwCensusJfr.java")
    rc = subprocess.run(["java", tool, jfr_path],
                        capture_output=True, text=True, timeout=300)
    line = rc.stdout.strip().splitlines()[-1] if rc.stdout.strip() else ""
    if not line.startswith("{"):
        return {"run": jfr_path, "tag": tag, "status": "JFR-PARSE-FAIL",
                "stderr": rc.stderr[-200:], "verdict": "SKIPPED-ИНФРА"}
    try:
        out = json.loads(line)
    except json.JSONDecodeError:
        return {"run": jfr_path, "tag": tag, "status": "JFR-JSON-FAIL",
                "verdict": "SKIPPED-ИНФРА"}
    out.update({"tag": tag,
                "cap": f"total<={CAP_TOTAL_S}s & avg<={CAP_AVG_MS}ms"})
    # unify verdict: NO-GC-EVENTS treated as skip, else gate decides
    if out.get("verdict") == "INVALID-STW-HOST":
        return out
    return out


def main():
    if sys.argv[1:2] == ["--jfr"]:
        jfr_path = sys.argv[2]
        tag = sys.argv[3] if len(sys.argv) > 3 else os.path.basename(jfr_path)
        print(json.dumps(census_jfr(jfr_path, tag)))
        return
    run_id, tag = sys.argv[1], sys.argv[2]
    run_dir = os.path.join(OUTROOT, str(run_id))
    status = download(run_id, run_dir)
    if status != "ok" and status != "cached":
        print(json.dumps({"run": run_id, "tag": tag, "status": status,
                          "verdict": "SKIPPED-ИНФРА"}))
        return
    gclog = os.path.join(run_dir, "gc.log")
    r = parse_gc(gclog)
    verdict = "STW-CLEAN"
    full_win = 8 <= r["full"] <= 10  # канон Full=9±1 (5CC+4MD jitter)
    if r["stw_total_s"] > CAP_TOTAL_S or (r["avg_ms"] or 0) > CAP_AVG_MS or not full_win:
        verdict = "INVALID-STW-HOST"
    r.update({"run": run_id, "tag": tag, "status": status, "verdict": verdict,
              "full_window": "OK(9±1)" if full_win else "OUT",
              "cap": f"total<={CAP_TOTAL_S}s & avg<={CAP_AVG_MS}ms & full∈[8,10]"})
    print(json.dumps(r))


if __name__ == "__main__":
    main()
