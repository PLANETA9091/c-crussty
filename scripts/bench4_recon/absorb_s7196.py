#!/usr/bin/env python3
"""absorb_s7196.py - TASK-363: absorb RECON-37 P2-PREGATE DIAGNOSTIC leg s7196.

ДИАГНОСТИЧЕСКИЙ лег (прецедент TASK-317 instrument-gate: pure observability,
0 behavior change — НЕ гейт-лег, TPS-вердикта НЕТ). Единственная цель:
THREADED wall-профиль -> worker-дисбаланс I -> развилка RECON-36:
  I<=1.15  -> OFFLOAD-READY (P2 = слив main-бакетов + offload остатков)
  I>=1.30  -> REBALANCE (REGION_CHUNKS/WORKERS)
  серая    -> повторный threaded лег (min-of-2 по знаку развилки)

Гейты (прегистрированы в dispatch_s7196.py):
  PG-A leg-validity: run success + банк-v3 доставка (region_threads=4,
    batch_collector=1, inside_cache=1, flush_diet=1, travel_diet=0)
  PG-B THREADED: wall-collapsed.txt в формате -t (worker/Server thread токены)
  PG-C полнота: >=3 worker-слота + Server thread
  PG-D crash-sanity: NCDFE/s7180-класс отсутствуют в stdout
  RECON-37: I + развилка (recon37_worker_balance.analyze)
Failure roulette: conclusion=failure -> BAND-DISCARD (fast-fail, не вердикт,
ре-ролл) / INFRA-FLAKE (ре-ролл, макс 2 подряд) / TOOL-FAIL (asprof -t
отклонён -> правка флага, ре-ролл). CRASH для банк-конфига не ожидается.
"""
import json, os, re, subprocess, sys, urllib.error, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7196-p2pregate")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url):
    if url.startswith("/"):
        url = API + url
    req = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def _fetch_redirected(tok, url, dest):
    class NoRedir(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            return None
    opener = urllib.request.build_opener(NoRedir)
    try:
        with opener.open(urllib.request.Request(url, headers={
                "Authorization": f"Bearer {tok}"}), timeout=120) as r:
            data = r.read()
    except urllib.error.HTTPError as e:
        if e.code in (301, 302, 307):
            with urllib.request.urlopen(urllib.request.Request(
                    e.headers["Location"], headers={
                        "Authorization": f"Bearer {tok}"}), timeout=300) as r2:
                data = r2.read()
        else:
            raise
    with open(dest, "wb") as f:
        f.write(data)


def fetch_artifact(tok, run_id):
    import zipfile
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    bench = [a for a in arts if a["name"] == "world3-bench"]
    if not bench:
        print(f"no world3-bench artifact (available: {[a['name'] for a in arts]})")
        return False
    a = bench[0]
    os.makedirs(RUN_DIR, exist_ok=True)
    dest = os.path.join(RUN_DIR, a["name"] + ".zip")
    _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/artifacts/{a['id']}/zip", dest)
    print(f"downloaded {dest} ({os.path.getsize(dest)} bytes)")
    with zipfile.ZipFile(dest) as z:
        z.extractall(RUN_DIR)
    return True


def fetch_joblog(tok, run_id):
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs").get("jobs", [])
    if not jobs:
        return ""
    dest = os.path.join(RUN_DIR, "job.log")
    try:
        _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/jobs/{jobs[0]['id']}/logs", dest)
        return open(dest, errors="ignore").read()
    except Exception as e:  # noqa: BLE001
        print(f"job-log fetch failed: {e}", file=sys.stderr)
        return ""


def main():
    if len(sys.argv) < 2:
        print("usage: absorb_s7196.py <run_id>")
        return 9
    run_id = int(sys.argv[1])
    tok = token()
    os.makedirs(RUN_DIR, exist_ok=True)
    run = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    concl = run.get("conclusion")
    head = run.get("head_sha", "")[:7]
    print(f"run {run_id} @ {head}: status={run.get('status')} conclusion={concl}")
    if concl != "success":
        jl = fetch_joblog(tok, run_id)
        if re.search(r"cpu_index.*outside|OUTSIDE band|BAND-DISCARD", jl, re.I) or \
                any(s.get("name", "").startswith("Runner calibration band gate") and
                    s.get("conclusion") == "failure"
                    for j in api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs").get("jobs", [])
                    for s in j.get("steps", [])):
            print("VERDICT: BAND-DISCARD (fast-fail, НЕ вердикт) — ре-ролл dispatch_s7196.py")
            return 1
        print("VERDICT: INFRA-FLAKE (ре-ролл, макс 2 подряд); joblog в run-dir")
        return 2
    if not fetch_artifact(tok, run_id):
        print("VERDICT: INFRA-FLAKE — нет артефакта")
        return 2

    # PG-A доставка банка v3
    envp = os.path.join(RUN_DIR, "run-env.txt")
    env = open(envp, errors="ignore").read() if os.path.exists(envp) else ""
    checks = {
        "region_threads: 4": "region_threads: 4" in env,
        "batch_collector: 1": "batch_collector: 1" in env,
        "inside_cache: 1": "inside_cache: 1" in env,
        "flush_diet: 1": "flush_diet: 1" in env,
        "travel_diet: 0": "travel_diet: 0" in env,
    }
    idx = re.search(r"runner_cpu_index: (\d+)", env)
    print(f"PG-A delivery: {checks} cpu_index={idx.group(1) if idx else '?'}")
    if not all(checks.values()):
        print("VERDICT: DELIVERY-FAIL — конфиг лега не банк v3 (аудит диспатчера)")
        return 6

    # PG-D crash-sanity
    outp = os.path.join(RUN_DIR, "server-stdout.log")
    out = open(outp, errors="ignore").read() if os.path.exists(outp) else ""
    bad = re.findall(r"NoClassDefFoundError|GuardedNavigatingMobs.*NCDFE|s7180", out)
    print(f"PG-D crash-sanity: bad-markers={len(bad)}")
    if bad:
        print("VERDICT: CRASH (неожиданно для банк-конфига) — аудит stdout, ре-ролл после фикса")
        return 7

    # PG-B/PG-C + RECON-37
    import recon37_worker_balance as r37
    rc = r37.analyze(os.path.join(RUN_DIR, "wall-collapsed.txt"))
    if rc == 4:
        print("VERDICT: TOOL-FAIL — asprof -t отклонён; правка harness-флага, ре-ролл")
    elif rc == 5:
        print("VERDICT: INCOMPLETENESS — воркеров <3; ре-ролл")
    elif rc in (0, 1, 2):
        print("VERDICT: P2-PREGATE OK — развилка выше (0=OFFLOAD-READY,1=REBALANCE,2=GRAY)")
    return rc


if __name__ == "__main__":
    sys.exit(main())
