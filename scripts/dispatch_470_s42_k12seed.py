#!/usr/bin/env python3
"""dispatch_470_s42_k12seed.py — S42 / ЯКОРЯ (round-470): chkclimb-12 K12-окно
ре-ролл на gc6 с seed-ковариатом 45 (отличен от S33/seed42) — независимая
попытка, алиасы round-470-s42-{a,b,c}.

Канон диспатчера тик-470 (Л188a/b + S20 микро-урок FULL-sha + S31-урок
атрибуции): прямой POST /git/refs (алиас-ветка на чистый MERGE №11 b3853246,
case "6" ЖИВОЙ внутри — git-пруф S31/S32: run_world3.sh:587) + GET-верификация
object.sha ДО dispatch; 1 диспатч = 1 ветка = 1 concurrency-группа
(cancel-in-progress = 1 ран/реф = независимая лотерея узла); атрибуция
run→ветка ТОЛЬКО по head_branch (алиасы №11 шарят head_sha — sha-запрос ловит
чужие раны).

Окно K12=[8133686,8233686], порог norm_v5 ≤ −2.57 — v5-FROZEN
(BANK_V5_FREEZE §2, НЕ пересматривать); leg_v5 chkclimb-12 = +17.43
(x464 ре-мер, VALID selftest-шум). Окно НЕ в band-гейт (S31/S33-канон):
банк-фид полным band [6.0,9.5]M + пост-хок idx-фильтр.

Вектор ноги = банк-канон v5 (workflow defaults x466-C98: 640/300s/fp4/ic1/fd1/
rt4/bc1/pop150k/10G/xms4G) + ДЕЛЬТА против S33: population_seed=45 (канон 42)
+ gc_tune=6 (ценз-энаблер Л183, доминантный пресет Л217) + lever_flag ПУСТО
(ваниль-якорь). 3 ноги ОДНОГО конфига = 3 независимых лотереи узла (S33-канон:
каждая нога = независимая лотерея runner'а; hit глубоких хвостов 3-5%/диспатч
Л160 — серия ≥3 канон Л201/S31).

ГЕЙТ-ПРОТОКОЛ тик-471 (preregister, закон 14a/16):
  (1) band PASS по echo шага-3; band-dead = free discard 37-40s (Л188c),
      немедленный ре-ролл (memoryless P(in|out)=0.88, Л176);
  (2) cpu ТОЛЬКО из run-env.txt артефакта (Л195/Л224-а; gate-эхо не аргумент);
  (3) пост-хок idx-фильтр: cpu ∈ K12 [8133686,8233686] ∧ norm(v5) ≤ −2.57
      → K12 pass 0→1 = B-якорь chkclimb-12 (leg +17.43 → пара ≥+20 закон 18;
      пар-матем vs leg @8183686, Δcpu ≤50k, min-of-3, G3 HOST-excl);
      сателлит-окна той же ногой: E [6427199,6527199] ≤−1.60 /
      POI [8907260,9007260] ≤−1.99 / climb5 [8734563,8834563] ≤+2.77 (cert x469);
  (4) CLEAN-ценз M1 gc.log-primary: STW ≤23.0s (gc6-окно [0,3], Л183/Л217) ∧
      young avg ≤200ms — иначе HOST-excl (BANK §3) + re-roll;
  (5) vanilla-validity (Л209): armed=null ∧ n=null ∧ ncdfe_real=0 ∧ aioobe=0;
  (6) SEED-КОВАРИАТА 45 (дельта против S33/42): пара легальна стандартными
      гейтами (Δcpu ≤50k, min-of-3, HOST-excl); seed-дельта регистрируется как
      config-ковариата v6 — авто-§3-адмит В БАНК v5 только seed-42-канон
      (seed-45 ваниль-точки = v6-датасет (norm,STW) gc_parse_canon Л205, не
      v5-фит); K12-якорный hit при seed-45 → pair-канал легален, финальный
      CERT добор min-of-3 против seed-42-ног S33 при их завершении;
  (7) вне окон, но in-band CLEAN → банк-фид §3 (seed-42) / v6-датасет
      (seed-45) + приоритет плотности 8.5-9.3M (S37).

Usage: dispatch_470_s42_k12seed.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# алиас-ветки K12-ре-ролла seed-ковариата 45 (1 диспатч = 1 ветка, Л188b):
# все ноги = seed 45, независимые concurrency-группы (лотерея узла)
LEGS = [("round-470-s42-a", "45"),
        ("round-470-s42-b", "45"),
        ("round-470-s42-c", "45")]
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
    "lever_flag": "", "lever_arg": "",  # ваниль-якорь (банк-фид / K12-окно)
}
SEED = "45"  # конфиг-дельта против S33 (канон 42) — независимая попытка


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
        print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, full_sha):
    """Л188a: POST /git/refs FULL-sha (S20-урок: короткий = 422) + GET-верификация."""
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
              f"(чистый ваниль-якорь пары, case '6' живой внутри; канон S31/S33/S41)", flush=True)
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
        inputs["population_seed"] = seed  # seed-ковариат 45 vs S33 (канон 42)
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": inputs})
        print(f"dispatched: {br} gc6 vanilla seed={seed} band[6.0,9.5]M (HTTP 204)", flush=True)
        time.sleep(3)

    # атрибуция run→ветка по head_branch (S31-урок; poll до 150s)
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
    print(f"=== S42 K12-ре-ролл seed45 DISPATCHED: {len(LEGS)} веток-алиасов @ {PIN_SHA[:8]} "
          f"gc6/ваниль/seed45/K12[8133686,8233686]≤−2.57/band[6.0,9.5]M; runs={json.dumps(attr)} ===",
          flush=True)


if __name__ == "__main__":
    main()
