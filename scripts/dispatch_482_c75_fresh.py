#!/usr/bin/env python3
"""dispatch_482_c75_fresh.py — COMMANDER C75: fresh-gen контроль × new-master (тик 482).

CLAIM (прегист, закон 14a/16): С94-конфаунд fresh-gen young-масса-инвариант
повтор на новом носителе master 3666a793 (МЕРЖ №19 c98ai-компо) — пост-№18/№19.
Датумы: C05-банк pregen afb3a0b3 young_n 109-123 CPU-инвариант, young-STW 21-23s;
СТЗ-2 9b3a3f09 @dd68fee1 (×479) young 247@56.3ms масса 13.9s norm +3.56 STW 21.16s;
C93-прегист ×480 (686f2258): young 200-280@40-80ms масса [12.5,16.5]s.

ФАКТ-ЧЕК inputs (закон-канон): input'а fresh/gen в world-bench-parallel.yml НЕТ;
world_url имеет НЕПУСТОЙ pregen-дефолт (yml:44 + run_world3.sh:29) → fresh-gen
НЕ дефолт-поведение → диспатч С ЯВНЫМ world_url 9b3a3f09 (s51-канон, СТЗ-2/
C93-прецедент: level.dat + ПУСТОЙ region + dp-драйвер forceload duty 0.2;
redstone ∅ → сцена = fresh-gen профиль, C12 H-Δ1 не нарушен).

ЗАДАЧА: 1 диспатч world-bench-parallel @3666a793 FULL-sha (0 код-дельт), алиас
round-482-c75-fresh, canon x466-C98 burst73-ваниль ЯВНЫМ JSON (640/300s/fp4/gc3/
ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G, lever ∅) + world_url=fresh, band
[6.0,9.5]M fast-fail; band-miss → 1 ре-ролл (W3).

ПРЕГИСТ-ГЕЙТ (пороги v5-FROZEN не двигаются) — fresh-gen-профиль vs банк:
  M1: STW ≤23.0s ∧ young_avg ≤200ms.
  P1 young-морфология: young_n 200-280 (банк 109-123), avg 40-80ms, young-МАССА
     ∈ [12.5,16.5]s (mass-preserving инвариант C94).
  P2 STW ∈ [18,23]s (плато-класс).
  P3 norm ВНЕ ваниль-коридора [−8,+1.5] сверху (+2..+6 breach-top) → НЕ банк-фид.
  P4 worldgen/noise тик-сэмплы 0.0000% (gen off-tick).
Эскалации: масса вне [12.5,16.5] → не mass-preserving на 3666a793 → C05/C92-эскалация;
norm в коридоре → fresh-gen norm-конфаунд опровергнут на №19; STW >23 → DIRTY M1.

Usage: dispatch_482_c75_fresh.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-482-c75-fresh"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"  # master post-МЕРЖ №19 (burst73 BASE)

WORLD_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v479-f2-stz-world/world479-f2-stz-v1.zip")
WORLD_SHA256 = "9b3a3f091941e90dbd22cbf4fc34ee3a2345e2655f108b2c1c14e29fc4c0bb9f"

# canon x466-C98 burst73-ваниль ЯВНЫМ JSON (урок C66-C72: yml-дефолты = merge-поверхность)
INPUTS = {
    "world_url": WORLD_URL,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "482-c75-fresh-dispatch"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def verify_fixture(tok):
    req = urllib.request.Request(WORLD_URL, method="GET", headers={
        "Authorization": f"Bearer {tok}", "User-Agent": "482-c75-fresh-dispatch"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            head = r.read(64)
            ok = r.status == 200
    except urllib.error.HTTPError as e:
        ok, head = False, b""
    print(f"fixture GET-verify: {'OK' if ok else 'FAIL'} (head={head[:16]!r})")
    return ok


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]}")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:8]}")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{name}", "sha": sha})
        print(f"{name}: created FULL-sha @ {sha[:8]}")
    r2 = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    got = r2.get("object", {}).get("sha")
    assert got == sha, f"alias GET-verify FAIL: {got} != {sha}"
    print(f"{name}: GET-verified @ {got}")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} world=fresh(9b3a3f09) -> {'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def attempt(tok, pin, reroll_no=1):
    ensure_alias(tok, ALIAS, pin)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    if not dispatch(tok, ALIAS):
        sys.exit(2)
    time.sleep(20)
    run = latest_run(tok, ALIAS, mc)
    print(json.dumps({"alias": ALIAS, "reroll": reroll_no, **(run or {})}, indent=1))
    if not run:
        print("run-id не найден за 20s — повторный GET через 30s")
        time.sleep(30)
        run = latest_run(tok, ALIAS, mc)
        print(json.dumps({"alias": ALIAS, "reroll": reroll_no, **(run or {})}, indent=1))
        if not run:
            sys.exit(3)
    for _ in range(5):
        time.sleep(45)
        run = latest_run(tok, ALIAS, mc)
        if run and run["conclusion"] == "failure":
            return run, True  # band-miss кандидат
        if run and run["status"] in ("in_progress", "queued") and run["conclusion"] is None:
            continue
        if run and run["conclusion"] == "success":
            break
    return run, False


def main():
    dry = "--dry-run" in sys.argv
    tok = token()
    print(f"pin = {PIN[:8]} (FIXED, миссия: ветка от master full sha)")
    if not verify_fixture(tok):
        sys.exit("FATAL: fresh-gen фикстура недоступна — диспатч отменён")
    if dry:
        print(json.dumps({"alias": ALIAS, "pin": PIN, "world_sha256": WORLD_SHA256,
                          "inputs": INPUTS}, indent=1))
        return
    run, band_miss = attempt(tok, PIN, 1)
    if band_miss:
        print("BAND-MISS fast-fail → 1 ре-ролл (W3)")
        run, band_miss2 = attempt(tok, PIN, 2)
        if band_miss2:
            print("RE-ROLL BAND-MISS — квота исчерпана (2/2), фиксируем факт")
    print("C75-DISPATCH-JSON " + json.dumps(
        {"alias": ALIAS, "pin": PIN, "world_sha256": WORLD_SHA256,
         "prereg": {"M1": "STW<=23.0 ∧ young_avg<=200",
                    "P1": "young_n 200-280 @ 40-80ms, масса [12.5,16.5]s",
                    "P2": "STW [18,23]s", "P3": "norm breach-top +2..+6 (не банк-фид)",
                    "P4": "worldgen tick 0.0000%"},
         "run": run}, indent=1))


if __name__ == "__main__":
    main()
