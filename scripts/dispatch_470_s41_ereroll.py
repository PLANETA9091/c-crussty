#!/usr/bin/env python3
"""dispatch_470_s41_ereroll.py — S41 / ЯКОРЯ (round-470): chkclimb-5 3-й якорь,
ДУБЛИРУЮЩИЙ E-ре-ролл с конфиг-дельтой против S34: population_seed 43/44
(банк-канон 42) — независимая лотерея узла + независимый pop-layout draw,
алиасы round-470-s41-*; окно E=[6427199,6527199], порог ≤−1.60 v5-frozen
(BANK_V5_FREEZE §2, НЕ пересматривать); gc_tune=6 (claim S41 / ценз-энаблер
Л183, доминантный пресет Л217).

Канон диспатчера тик-470 (Л188a/b + S20 микро-урок FULL-sha): прямой POST
/git/refs (алиас-ветка на чистый MERGE №11 b3853246, case "6" ЖИВОЙ внутри) +
GET-верификация object.sha ДО dispatch; 1 диспатч = 1 ветка (concurrency
cancel-in-progress = 1 ран/реф = независимая лотерея узла).

Вектор ноги = банк-канон v5 (workflow defaults x466-C98: 640/300s/fp4/ic1/fd1/
rt4/bc1/pop150k/10G/xms4G) + ДЕЛЬТА против S34: seed=43 (нога a) / seed=44
(нога b) + gc_tune=6 + lever_flag ПУСТО (ваниль-якорь) + band [6.0,9.5]M
fast-fail. Окно НЕ в band-гейт: канон Л201/S31 — банк-фид полным band +
пост-хок idx-фильтр E-окна.

ГЕЙТ-ПРОТОКОЛ тик-471 (preregister, закон 14a/16):
  (1) band PASS по echo шага-3 (cpu_band [6.0,9.5]M);
  (2) пост-хок idx-фильтр: run-env cpu ∈ [6427199,6527199] → chkclimb-5
      в-окне кандидат; якорь-легален при norm(v5) ≤ −1.60 (v5-порог §2 frozen);
  (3) CLEAN-ценз: STW ≤23s (gc6-окно [0,3], Л183) и young avg ≤200ms — иначе
      HOST-excl (BANK §3, в фиты не идёт);
  (4) вне E-окна, но in-band CLEAN → банк-фид v5 §3 (приоритет плотности
      8.5-9.3M, S37) + v6-ковариата-канал: (norm, STW)-пара по gc_parse_canon
      (Л205) = кандидат-v6 stw-ковариата BANK §4 — нога не сгорает;
  (5) seed-дивергенция 43/44 vs банк-42: пара легальна стандартными гейтами
      (Δcpu ≤50k, min-of-3, G3 HOST-excl); seed-дельта регистрируется как
      config-ковариата v6, авто-§3-адмит В БАНК только seed-42-канон
      (seed-43/44 точки — v6-датасет, не v5-фит).

Usage: dispatch_470_s41_ereroll.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# алиас-ветки серии дублирующего E-ре-ролла (1 диспатч = 1 ветка, Л188b):
# a = seed 43, b = seed 44 — независимые concurrency-группы (лотерея узла)
LEGS = [("round-470-s41-a", "43"), ("round-470-s41-b", "44")]
PIN_SHA = "b385324677ac76f4f9f8ef942a3835c8004aef78"  # MERGE №11, case "6" живой

INPUTS_BASE = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",  # ваниль-якорь (банк-фид / E-окно)
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, ok404=False):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code == 404 and ok404:
            return None
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, full_sha):
    """Л188a: POST /git/refs FULL-sha (S20-урок 422) + GET-верификация."""
    existing = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}", ok404=True)
    if existing is not None:
        got = existing["object"]["sha"]
        if got != full_sha:
            raise SystemExit(f"BRANCH ALIAS DRIFT: {branch} live={got} pin={full_sha}")
        print(f"branch exists OK: {branch} @ {got[:8]}", flush=True)
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{branch}", "sha": full_sha})
    got = sha_of(tok, branch)
    if got != full_sha:
        raise SystemExit(f"REF VERIFY FAIL: {branch} -> {got}")
    print(f"branch created+verified: {branch} @ {got[:8]}", flush=True)


def runs_for_branch(tok, branch):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&branch={branch}&per_page=5")
    return [(r["id"], r.get("status"), r.get("created_at"))
            for r in runs.get("workflow_runs", [])]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    live = sha_of(tok, "master")
    if live != PIN_SHA:
        print(f"NOTE: origin/master moved to {live[:8]} — пин остаётся MERGE №11 {PIN_SHA[:8]} "
              f"(чистый ваниль-якорь пары, case '6' живой внутри; канон S31/S44/S36)", flush=True)
    print(f"preflight OK: pin {PIN_SHA[:8]} (MERGE №11, gc6 case '6' live)", flush=True)
    for br, seed in LEGS:
        print(f"  will ensure+dispatch: {br} population_seed={seed}", flush=True)
    print(f"inputs_base: {json.dumps(INPUTS_BASE, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return

    attr = {}
    for br, seed in LEGS:
        ensure_branch(tok, br, PIN_SHA)
        inputs = dict(INPUTS_BASE)
        inputs["population_seed"] = seed  # конфиг-дельта против S34 (канон 42)
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": inputs})
        print(f"dispatched: {br} gc6 vanilla seed={seed} band[6.0,9.5]M (HTTP 204)", flush=True)
        time.sleep(3)

    # атрибуция run→ветка по head_branch (poll до 150s)
    for _ in range(30):
        time.sleep(5)
        pending = [br for br, _ in LEGS if br not in attr]
        if not pending:
            break
        for br in pending:
            hits = runs_for_branch(tok, br)
            if hits:
                attr[br] = hits[0][0]
    for br, seed in LEGS:
        print(f"LEG {br} (seed={seed}): run {attr.get(br, 'POLL-PENDING')}", flush=True)
    print(f"=== S41 дублирующий E-ре-ролл DISPATCHED: {len(LEGS)} веток-алиасов @ {PIN_SHA[:8]} "
          f"gc6/ваниль/seed43-44/band[6.0,9.5]M; runs={json.dumps(attr)} ===", flush=True)


if __name__ == "__main__":
    main()
