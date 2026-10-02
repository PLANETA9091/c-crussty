#!/usr/bin/env python3
"""gen_s7194_pair.py — генерирует dispatch_s7194.py + absorb_s7194.py из
лейн-#14 пары (TASK-350): lever #10 ZERO-ALLOC-INSIDE isolation (S7-164)
на фоне банка v3 + S7-170 (гонка закрыта, маркер ОБЯЗАТЕЛЕН).

Заменяет: флаги ноги (travel_diet 1->0, zero_alloc 0->1), arm-маркер
(traveldiet -> zeroin), RUN_DIR, требование [S7-170] маркера в PG-T1,
тексты вердикт-веток (banking v4 = v3+zero_alloc(+S7-170); <+10% -> REFUTED
под-лейна -> RECON jdk-collections/serde 25.5%).
"""
import re

BASE = "/home/z/c-crussty/scripts/bench4_recon"


def patch(src, dst, subs):
    txt = open(src).read()
    for old, new in subs:
        if old not in txt:
            raise SystemExit(f"NOT FOUND in {src}: {old[:70]!r}")
        txt = txt.replace(old, new)
    open(dst, "w").write(txt)
    print(f"written {dst}")


# ---------- dispatch_s7194.py ----------
patch(f"{BASE}/dispatch_s7189.py", f"{BASE}/dispatch_s7194.py", [
    ('"""dispatch_s7189.py - lever #14 TRAVEL-ALLOC-DIET v2a ISOLATION leg (TASK-346),',
     '"""dispatch_s7194.py - lever #10 ZERO-ALLOC-INSIDE ISOLATION leg (TASK-350),'),
    ('LEG INPUTS = v2a ISOLATION: bank v3 (inside_cache=1 + flush_diet=1 +\nregion_threads=4 + batch_collector=1) + travel_diet=1, zero_alloc=0,\nskip_store_bb=0 (pure v2a effect, no v1 admixture). TravelDietOps verified\noffline by TravelDietLockstepHarness (GOAL x29, 1.05M bit-exact cases).\nArm markers expected in stdout: "stage traveldiet composed (Retargeted { sites: 1 })".',
     'LEG INPUTS = #10 ISOLATION: bank v3 (inside_cache=1 + flush_diet=1 +\nregion_threads=4 + batch_collector=1) + S7-170 NAV-MOBS-GUARD (committed blob,\nrace closed s7193: threw=0) + zero_alloc=1, travel_diet=0, skip_store_bb=0\n(pure #10 effect). Target = RECON-24 top actionable alloc sub-lane\ninside-blocks/fluid-scan 22.3% (Vec3/AABB/BlockPos churn) + GC barrier channel.\nArm marker expected: "stage zeroin composed (Retargeted { sites: 3 })".'),
    ('PG-T1 delivery (+ traveldiet arm marker),', 'PG-T1 delivery (+ zeroin arm marker + [S7-170] marker MANDATORY),'),
    ('PG-T3 REGRESSION DUAL-BAR, PG-T4 GC\n(young <= 174, 0 Full), PG-T5 DONE-park N/A tolerated (AP-PID defect s7178).',
     'PG-T3 REGRESSION DUAL-BAR, PG-T4 GC\n(young <= 174, 0 Full; young DROP = secondary signal for alloc-diet),\nPG-T5 DONE-park N/A tolerated (AP-PID defect s7178).'),
    ('"travel_diet": "1",\n        "zero_alloc": "0",',
     '"travel_diet": "0",\n        "zero_alloc": "1",'),
])

# ---------- absorb_s7194.py ----------
patch(f"{BASE}/absorb_s7189.py", f"{BASE}/absorb_s7194.py", [
    ('"""absorb_s7189.py - TASK-346: absorb lever #14 TRAVEL-ALLOC-DIET v2a ISOLATION\nleg s7189 (bank v3 + travel_diet=1, zero_alloc=0/skip_store_bb=0) and deliver the',
     '"""absorb_s7194.py - TASK-350: absorb lever #10 ZERO-ALLOC-INSIDE ISOLATION\nleg s7194 (bank v3 + S7-170 + zero_alloc=1, travel_diet=0/skip_store_bb=0) and deliver the'),
    ('RUN_DIR = os.path.join(RESDIR, "run-s7189-traveldiet-v2a")',
     'RUN_DIR = os.path.join(RESDIR, "run-s7194-zeroalloc-v1")'),
    ('PG-T1 delivery: travel_diet=1 + zero_alloc=0 + skip_store_bb=0 +',
     'PG-T1 delivery: travel_diet=0 + zero_alloc=1 + skip_store_bb=0 +'),
    ('    ARM MARKER "stage traveldiet composed" present (fail-dominant chain must\n    have actually composed the redirect, not skipped it)',
     '    ARM MARKER "stage zeroin composed" present (strict sites:3, fail-dominant\n    chain must have actually composed the redirect, not skipped it) +\n    [S7-170] nav-mobs guarded marker MANDATORY (race-closed bank, s7193) +'),
    ('    (min-of-2) -> banking v4; else LANE-OPEN -> v2b travel-math (RECON-21\n    contract: handleRelativeFrictionAndCalculateMovement + travelInFluid +\n    getInputVector) implemented and dispatched in the same tick',
     '    (min-of-2) -> banking v4 = v3 + zero_alloc(+S7-170); else LANE-OPEN ->\n    REFUTED for the fluid-scan alloc sub-lane attempt -> fresh RECON of the\n    next GC-family sub-lane (jdk-collections/serde 25.5% per RECON-24)'),
    ('CRASH-REFUTED (NPE/throw markers -> rollback travel_diet + verdict doc +\nS7-170 prioritized: RECON-22 bank race is foundational)',
     'CRASH-REFUTED (NPE/throw markers -> rollback zero_alloc (yml default 0) +\nverdict doc; S7-170 guard stays: race closure verified s7193)'),
    ('        # ---- PG-T1 delivery (v2a isolation + arm marker)\n        want = {"travel_diet": "1", "zero_alloc": "0", "skip_store_bb": "0",',
     '        # ---- PG-T1 delivery (#10 isolation + zeroin arm marker + S7-170 marker)\n        want = {"travel_diet": "0", "zero_alloc": "1", "skip_store_bb": "0",'),
    ('        s7170 = stdout_txt.count("[S7-170] nav-mobs guarded")\n        s7170_note = ("guarded-marker=" + ("OK" if s7170 > 0 else "MISSING")) \\\n            if "nav-mobs" in stdout_txt or "[S7-170]" in stdout_txt else "guarded-marker=N/A (pre-S7-170 head)"\n        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and arm and not strict',
     '        s7170 = stdout_txt.count("[S7-170] nav-mobs guarded")\n        s7170_note = f"guarded-marker={' + "'OK' if s7170 > 0 else 'MISSING'" + '} (ОБЯЗАТЕЛЕН: гонка закрыта s7193)"\n        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and arm and s7170 > 0 and not strict'),
    ('''                verdict = "CANDIDATE-GREEN"
                rep.append("  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 "
                           "(dispatch_s7189.py повторно) -> banking v4 = v3 + travel_diet")''',
     '''                verdict = "CANDIDATE-GREEN"
                rep.append("  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 "
                           "(dispatch_s7194.py повторно) -> banking v4 = v3 + "
                           "zero_alloc(+S7-170)")'''),
    ('''                verdict = "LANE-OPEN"
                rep.append("  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> "
                           "#14 v2b travel-math (RECON-21 контракт: "
                           "handleRelativeFrictionAndCalculateMovement 48 строк + "
                           "travelInFluid + getInputVector) — реализация+диспатч в том же тике")''',
     '''                verdict = "LANE-OPEN"
                rep.append("  -> **< +10% хотя бы по одной оси** -> REFUTED попытки "
                           "под-лейна inside-blocks/fluid-scan (22.3% alloc, RECON-24) "
                           "рычагом #10 -> свежий RECON следующего GC-под-лейна "
                           "(jdk-collections/serde 25.5%) до под-лейнов >=5%")'''),
    ('''                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail экстремального "
                           "лендинга, не вердикт) — ре-диспатч dispatch_s7189.py "
                           "(макс 2 подряд)")''',
     '''                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail экстремального "
                           "лендинга, не вердикт) — ре-диспатч dispatch_s7194.py "
                           "(макс 2 подряд)")'''),
    ('''                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу, rollback "
                           "travel_diet (yml-дефолт 0), вердикт-док, S7-170 "
                           "(RECON-22 race в банке) приоритизируется")''',
     '''                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу, rollback "
                           "zero_alloc (yml-дефолт 0), вердикт-док; S7-170 остаётся "
                           "(закрытие гонки верифицировано s7193)")'''),
    ('    out = os.path.join(RESDIR, "ABSORB_S7189.md")',
     '    out = os.path.join(RESDIR, "ABSORB_S7194.md")'),
    ('    rep = [f"# absorb #14 v2a ISOLATION s7189 (run {run_id}, head "',
     '    rep = [f"# absorb #10 ZEROALLOC ISOLATION s7194 (run {run_id}, head "'),
])
print("pair generated OK")
