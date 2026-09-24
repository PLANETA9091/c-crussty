#!/usr/bin/env python3
"""golden_443.py — TASK-443-C (tick-443, prep 14:2x +08 2026-09-24) — ЕДИНЫЙ диспатчер
ЗОЛОТОГО СЛОТА 02:08 +08 (2026-09-25). Шаблон: /home/z/rounds/ROUND-439/golden_439.py
(механика 1:1). Мандат владельца 2026-09-23: ≥+20% pair-stable ОБЯЗАТЕЛЬНО — батч
спроектирован как СЕРТИФИКАЦИОННЫЙ (min-of-3 пары для всех живых рычагов).

БАТЧ — 24 запуска, интерлив в 3 трети по 8: якоря ПЕРВЫМИ и ПОСЛЕДНИМИ в каждой трети,
между ними ноги (max 2 ноги подряд, без соседства одинаковых семейств):
  - 12 якорей @master          lever=''          (round-443g-anchor-1..12) — лотерея
      двух зон (депресс ~6.3-6.9M + здоровая ~8.0-9.0M): якорная плотность по ВСЕМУ
      окну 02:2x-03:3x (~70 мин), чтобы обе зоны получили партнёров Δ≤50k
  - 3 ins4d @round-442-b-ins4d   cmp440_ins4d  (round-443g-ins4d-1..3) — диета-носитель,
      линия 3.30 +23.4 / 3.20 +22.2 (RECORD эры), min-of-3 дизайн: нога в каждой трети
  - 2 ins4  @round-436-b-ins6    cmp436_ins4   (round-443g-ins4-1..2)  — пики +22.8/+15.2
  - 2 chunk4 @round-438-c-chunk4b cmp437_chunk4 (round-443g-chunk4-1..2) — GREEN ×6 эры
  - 1 ss    @round-437-a-sscan2  cmp436_sscan2 (round-443g-ss-1)       — ARM ×2 CI-пруф
  - 1 pd    @round-437-b-pdemux  cmp436_pdemux (round-443g-pd-1)       — ФИКС @48362768 обязателен
  - 1 chk3  @round-436-c-chunk3  cmp435_chunk3 (round-443g-chk3-1)     — пара +16.0пп @×442
  - 2 MEGA  @round-443-mega      cmp443_mega   (round-443g-mega-1..2)  — ВЕТКА ПЕРЕДАЁТСЯ
      АГЕНТОМ-B СЕЙЧАС → OPTIONAL: ветки нет ИЛИ sha другой → skip с ПРЕДУПРЕЖДЕНИЕМ,
      НЕ аборта (см. EXPECTED_SHA/OPTIONAL_BASES; перед слотом запинить sha в §0.4 плана)

ВЕРИФИЦИРОВАННЫЕ SHA БАЗ (git ls-remote origin, prep TASK-443-C 14:26 +08 2026-09-24):
  master               c9060b1f  (tick-442 FINAL)
  round-442-b-ins4d    d9d1fb30  (TASK-442-B диета cmp440_ins4d, RECORD-носитель)
  round-436-b-ins6     07078007  (cmp436_ins4)
  round-438-c-chunk4b  c5fe0251  (cmp437_chunk4, GREEN ×6)
  round-437-a-sscan2   3f3b111f  (cmp436_sscan2)
  round-437-b-pdemux   48362768  (ФИКС STRICT-OR — cmp436_pdemux армит, TASK-438-B)
  round-436-c-chunk3   7afe6d17  (cmp435_chunk3)
  round-443-mega       OPTIONAL  (передаётся агентом-B; на origin пока ОТСУТСТВУЕТ —
                        skip-with-warning семантика; пин ставить в OPTIONAL_SHA)
  Ветки round-443g-*   НЕ СУЩЕСТВУЮТ на origin (ls-remote glob пуст, prep 14:26) —
      создадутся при запуске. Ветки round-443-* (батч 13:08 main) НЕ ТРОГАЕМ.

РЕЖИМЫ:
  python3 scripts/golden_443.py --dry-run   (или GOLDEN_DRY_RUN=1)
      → dry-run: печатает полный план ×24 (leg/branch/base/lever/sha по живому API GET),
        НИЧЕГО не диспатчит и не создаёт.
  python3 scripts/golden_443.py
      → реальный диспатч (тик 02:08 +08): preflight SHA → dispatch ×N, sleep 4.

ВЕРДИКТ-ГЕЙТЫ (полный протокол: scripts/GOLDEN_EXEC_PLAN_443.md):
  pair Δ≤50k | band 6.0-9.5M re-roll ≤2 | депресс-гейт якорь norm ≥−2 |
  ARM-маркеры stdout | AIOOBE=0 | NCDFE=0 | selfTest | threw=0 |
  min-of-3 медиана ≥+20% → немедленный --no-ff мерж победителя;
  якорный спред окна ex-outlier >±5пп = окно браковано → перенос на следующий слот.
"""
import json, os, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# (leg, branch, base, lever) — 3 трети × 8: якорь первым и последним в каждой трети;
# ins4d-нога в каждой трети (min-of-3); одиночные семейства разнесены по третям.
BATCH = [
    # --- ТРЕТЬЯ 1 (раны 1-8) ---
    ("anchor-1",  "round-443g-anchor-1",  "master",              ""),
    ("ins4d-1",   "round-443g-ins4d-1",   "round-442-b-ins4d",   "cmp440_ins4d"),
    ("anchor-2",  "round-443g-anchor-2",  "master",              ""),
    ("ins4-1",    "round-443g-ins4-1",    "round-436-b-ins6",    "cmp436_ins4"),
    ("chunk4-1",  "round-443g-chunk4-1",  "round-438-c-chunk4b", "cmp437_chunk4"),
    ("anchor-3",  "round-443g-anchor-3",  "master",              ""),
    ("ss-1",      "round-443g-ss-1",      "round-437-a-sscan2",  "cmp436_sscan2"),
    ("anchor-4",  "round-443g-anchor-4",  "master",              ""),
    # --- ТРЕТЬЯ 2 (раны 9-16) ---
    ("anchor-5",  "round-443g-anchor-5",  "master",              ""),
    ("ins4d-2",   "round-443g-ins4d-2",   "round-442-b-ins4d",   "cmp440_ins4d"),
    ("anchor-6",  "round-443g-anchor-6",  "master",              ""),
    ("pd-1",      "round-443g-pd-1",      "round-437-b-pdemux",  "cmp436_pdemux"),
    ("chunk4-2",  "round-443g-chunk4-2",  "round-438-c-chunk4b", "cmp437_chunk4"),
    ("anchor-7",  "round-443g-anchor-7",  "master",              ""),
    ("ins4-2",    "round-443g-ins4-2",    "round-436-b-ins6",    "cmp436_ins4"),
    ("anchor-8",  "round-443g-anchor-8",  "master",              ""),
    # --- ТРЕТЬЯ 3 (раны 17-24) ---
    ("anchor-9",  "round-443g-anchor-9",  "master",              ""),
    ("ins4d-3",   "round-443g-ins4d-3",   "round-442-b-ins4d",   "cmp440_ins4d"),
    ("anchor-10", "round-443g-anchor-10", "master",              ""),
    ("chk3-1",    "round-443g-chk3-1",    "round-436-c-chunk3",  "cmp435_chunk3"),
    ("mega-1",    "round-443g-mega-1",    "round-443-mega",      "cmp443_mega"),
    ("anchor-11", "round-443g-anchor-11", "master",              ""),
    ("mega-2",    "round-443g-mega-2",    "round-443-mega",      "cmp443_mega"),
    ("anchor-12", "round-443g-anchor-12", "master",              ""),
]

# Ожидаемые sha (verify prep TASK-443-C; при расхождении живого API-ша — АБОРТ диспатча,
# НЕ молча). round-443-mega — OPTIONAL-база (None = пин не задан: ветка передаётся
# агентом-B; нет ветки / sha ≠ пину → ноги mega SKIP с предупреждением, НЕ аборта).
# Перед реальным запуском можно запинить: "round-443-mega": "<8-char sha>".
EXPECTED_SHA = {
    "master":               "c9060b1f",
    "round-442-b-ins4d":    "d9d1fb30",
    "round-436-b-ins6":     "07078007",
    "round-438-c-chunk4b":  "c5fe0251",
    "round-437-a-sscan2":   "3f3b111f",
    "round-437-b-pdemux":   "48362768",
    "round-436-c-chunk3":   "7afe6d17",
    "round-443-mega":       None,  # OPTIONAL (cmp443_mega, агент-B)
}
OPTIONAL_BASES = {"round-443-mega"}

# Банк INPUTS РОВНО (без travel_diet / fluid_dirty_ledger; concurrency-гвардов нет)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code != 404:  # 404 = ожидаемая семантика "ветки нет" (branch_exists / ensure_branch)
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


def branch_exists(tok, branch):
    try:
        sha_of(tok, branch)
        return True
    except urllib.error.HTTPError:
        return False
    except Exception:
        return False


def live_sha_safe(tok, ref):
    try:
        return sha_of(tok, ref)[:8]
    except Exception as e:
        return f"<API-ERR {type(e).__name__}>"


def optional_preflight(tok):
    """OPTIONAL-базы: ветки нет ИЛИ sha ≠ пину → (skip-set, warnings), НЕ аборта."""
    skipped, warns = set(), []
    for base in OPTIONAL_BASES:
        pin = EXPECTED_SHA.get(base)
        try:
            live = sha_of(tok, base)[:8]
        except Exception as e:
            n = sum(1 for b in BATCH if b[2] == base)
            skipped.add(base)
            warns.append(f"OPTIONAL SKIP: {base} отсутствует на origin ({type(e).__name__}) "
                         f"→ {n} mega-ног НЕ диспатчится (не аборт)")
            continue
        if pin and not live.startswith(pin):
            n = sum(1 for b in BATCH if b[2] == base)
            skipped.add(base)
            warns.append(f"OPTIONAL SKIP: {base} sha {live} ≠ пин {pin} → {n} mega-ног НЕ "
                         f"диспатчится (не аборт)")
            continue
        note = "пин не задан (ветка от агента-B принята как есть)" if pin is None else f"пин {pin} OK"
        print(f"preflight OPTIONAL OK: {base} @ {live} ({note})", flush=True)
    return skipped, warns


def main():
    dry = "--dry-run" in sys.argv or os.environ.get("GOLDEN_DRY_RUN") == "1"
    tok = token_from_remote()

    if dry:
        print("=== GOLDEN_DRY_RUN — план золотого батча TASK-443 (только GET, 0 диспатчей) ===")
        print(f"{'#':>2}  {'leg':<9} {'branch':<21} {'base':<20} {'lever':<14} {'base_sha':<10} branch")
        for i, (leg, branch, base, lever) in enumerate(BATCH, 1):
            bsha = live_sha_safe(tok, base)
            exp = EXPECTED_SHA.get(base)
            if base in OPTIONAL_BASES:
                flag = "  [OPTIONAL-mega: нет/ша≠пин → SKIP]" if bsha.startswith("<") else \
                       "  [OPTIONAL-mega: пин не задан → диспатч]"
            else:
                flag = "" if (exp is None or bsha.startswith(exp) or bsha.startswith("<")) else "  <<SHA-DRIFT!"
            state = "EXISTS" if branch_exists(tok, branch) else "new (создастся)"
            print(f"{i:>2}  {leg:<9} {branch:<21} {base:<20} {lever!r:<14} {bsha:<10} {state}{flag}")
        n_anchor = sum(1 for b in BATCH if b[3] == "")
        n_mega = sum(1 for b in BATCH if b[2] in OPTIONAL_BASES)
        print(f"=== итог: {len(BATCH)} запусков в 3 третях (якоря {n_anchor} — первым/последним в "
              f"каждой трети; ноги {len(BATCH) - n_anchor}, из них MEGA OPTIONAL {n_mega}); "
              f"диспатчей 0 ===")
        return

    # --- РЕАЛЬНЫЙ ДИСПАТЧ (тик 02:08 +08) ---
    print(f"=== GOLDEN DISPATCH (tick-443 golden slot 02:08 +08, мандат ≥+20% pair-stable) ===",
          flush=True)
    # префлайт 1: обязательные базы — несовпадение = ОТМЕНА, не молча
    for base, exp in EXPECTED_SHA.items():
        if base in OPTIONAL_BASES:
            continue
        live = sha_of(tok, base)[:8]
        if not live.startswith(exp):
            raise SystemExit(f"SHA MISMATCH: base={base} live={live} expected={exp} "
                             f"— диспатч ОТМЕНЁН (ре-верификация ls-remote обязательна)")
        print(f"preflight OK: {base} @ {live}", flush=True)
    # префлайт 2: OPTIONAL-базы → skip-with-warning, не аборта
    skipped, warns = optional_preflight(tok)
    for w in warns:
        print(f"WARNING: {w}", flush=True)
    batch = [leg for leg in BATCH if leg[2] not in skipped]
    if not batch:
        raise SystemExit("пустой батч после OPTIONAL-фильтра — диспатч ОТМЕНЁН")
    for leg, branch, base, lever in batch:
        sha = ensure_branch(tok, branch, base)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
    skipped_n = len(BATCH) - len(batch)
    print(f"=== GOLDEN BATCH COMPLETE: {len(batch)} диспатчей, {skipped_n} OPTIONAL-skip, "
          f"0 ошибок ===", flush=True)


if __name__ == "__main__":
    try:
        main()
    except BrokenPipeError:  # вывод запайпили в head/less — не ошибка исполнения
        try:
            sys.stdout.close()
        except Exception:
            pass
        sys.exit(0)
