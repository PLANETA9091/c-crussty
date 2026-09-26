#!/usr/bin/env python3
"""absorb_464.py — TASK-464-53: M1 gc.log-primary STW-канонизация absorb-пайплайна.

Канон (LEDGER-49 §5 ×463 + STW-CENSUS.md ×464 + LAB-STAGE/LEDGER-53.md ×464-53):
  * total STW = сумма длительностей ТОЛЬКО completion-строк `GC(N) Pause ... NNN.Nms`.
    gc,start-строки не несут длительности; [gc,phases] строки = вложенные суб-фазы
    ВНУТРИ top-level паузы (аналог JFR GCPhasePauseLevel1..4) — НЕ суммировать.
    Наивный суммарный reg-exp (Pause+Phase) = +37.8% median STW-инфляция на ×464
    (40/40 gc.log, Δ 33.84-43.59%) → гейты горят ложным INVALID.
  * Full-lane = `Pause Full` (kinds: Metadata/CodeCache/прочие из причинной скобки);
    young-lane = `Pause Young`.
  * **scavAvg_young = средняя YOUNG-пауза** (=(total−full_sum)/young) — канон ×463
    LEDGER-49 (кросс-валидация JFR↔gc.log бит-в-бит; чтение «scavAvg = mean-Full»
    опровергнуто: 28/28 → HOST, гейт вырожден). Поле full_avg_ms дано отдельно.
  * full_cc = число `Pause Full (CodeCache GC Threshold)` — главный источник
    Full-джиттера ×464 (r vs runner +0.413, терцильный градиент +19%).
  * M1 = gc.log-primary: stdout-блок «Garbage Collector statistics» (PS MarkSweep/
    PS Scavenge) НЕ тянуть — завышение avg ~×1.7 и недосчёт full ~×2 (×464 §3).

Гейты (канон G3, exit 42 fail-closed): STW_total > 23.0s ИЛИ avg > 200ms
(=INVALID-STW-HOST) ИЛИ Full ∉ [8,10].
"""
import json
import os
import re
import sys

STW_MAX_S = 23.0
AVG_MAX_MS = 200.0
FULL_WINDOW = (8, 10)

MS = re.compile(r'(\d+\.\d+)ms')
# completion-строка паузы: длительность в конце, теги до/после допустимы
PAUSE_COMPL = re.compile(r'GC\(\d+\) Pause .* (\d+\.\d+)ms$')
KIND_CC = re.compile(r'Pause Full \(CodeCache')
KIND_MD = re.compile(r'Pause Full \(Metadata')


def parse_gclog_canon(path):
    """M1 gc.log-primary парсер (канон LEDGER-49 §5 / STW-CENSUS.md ×464)."""
    stw = {'total_ms': 0.0, 'max_ms': 0.0, 'pauses': 0,
           'full': 0, 'young': 0, 'full_md': 0, 'full_cc': 0, 'full_other': 0,
           'full_sum_ms': 0.0, 'young_sum_ms': 0.0}
    with open(path, errors='replace') as f:
        for line in f:
            if 'Pause' not in line:
                continue
            m = PAUSE_COMPL.search(line)
            if not m:            # gc,start-строки и прочие без длительности — мимо
                continue
            if '[gc,phases' in line:   # вложенная суб-фаза — НЕ top-level (канон)
                continue
            dur = float(m.group(1))
            stw['total_ms'] += dur
            stw['pauses'] += 1
            if dur > stw['max_ms']:
                stw['max_ms'] = dur
            if 'Pause Full' in line:
                stw['full'] += 1
                stw['full_sum_ms'] += dur
                if KIND_CC.search(line):
                    stw['full_cc'] += 1
                elif KIND_MD.search(line):
                    stw['full_md'] += 1
                else:
                    stw['full_other'] += 1
            else:
                stw['young'] += 1
                stw['young_sum_ms'] += dur
    return stw


def census(run_dir):
    """Полный STW-ценз рана: канон-поля + G3-вердикт. stdout-блок игнорируется (M1)."""
    gclog = os.path.join(run_dir, 'gc.log')
    if not os.path.isfile(gclog) or os.path.getsize(gclog) == 0:
        return {'run': os.path.basename(run_dir.rstrip('/')), 'gclog': False,
                'verdict': 'NO-GCLOG'}
    s = parse_gclog_canon(gclog)
    n = s['pauses']
    scav_young = (s['young_sum_ms'] / s['young']) if s['young'] else 0.0
    full_avg = (s['full_sum_ms'] / s['full']) if s['full'] else 0.0
    avg = (s['total_ms'] / n) if n else 0.0
    host = (s['total_ms'] > STW_MAX_S * 1000) or (n and avg > AVG_MAX_MS)
    full_win = FULL_WINDOW[0] <= s['full'] <= FULL_WINDOW[1]
    verdict = 'INVALID-STW-HOST' if host else ('FULL-WINDOW-OUT' if not full_win
                                              else 'CLEAN')
    return {'run': os.path.basename(run_dir.rstrip('/')), 'gclog': True,
            'stw_total_ms': round(s['total_ms'], 3), 'pauses': s['pauses'],
            'avg_ms': round(avg, 1), 'max_ms': round(s['max_ms'], 1),
            'full': s['full'], 'young': s['young'],
            'full_md': s['full_md'], 'full_cc': s['full_cc'],      # v6-ковариата
            'full_other': s['full_other'],
            'full_sum_ms': round(s['full_sum_ms'], 1),
            'full_avg_ms': round(full_avg, 1),
            'scavAvg_young_ms': round(scav_young, 1),               # канон LEDGER-49
            'full_window_ok': full_win, 'verdict': verdict}


def selftest():
    """Инварианты канона — упавшие assertion-ы = non-zero exit (fail-closed)."""
    tmp = '/tmp/absorb464_selftest'
    os.makedirs(tmp, exist_ok=True)
    log = '\n'.join([
        # start-строка: БЕЗ длительности — канон обязан её пропустить
        '[t][info][gc,start] GC(0) Pause Young (Metadata GC Threshold)',
        # completion young
        '[t][info][gc] GC(0) Pause Young (Metadata GC Threshold) 898M->13M(3925M) 6.559ms',
        # вложенные суб-фазы Full (gc,phases) — НЕ считать в total
        '[t][info][gc,phases] GC(1) Marking Phase 7.586ms',
        '[t][info][gc,phases] GC(1) Compaction Phase 7.105ms',
        '[t][info][gc] GC(1) Pause Full (Metadata GC Threshold) 13M->13M(3925M) 18.614ms',
        # CodeCache-kind Full
        '[t][info][gc] GC(2) Pause Full (CodeCache GC Threshold) 100M->99M(3925M) 40.000ms',
        # young для scavAvg
        '[t][info][gc] GC(3) Pause Young (Allocation Failure) 498M->63M(3925M) 93.441ms',
    ]) + '\n'
    p = os.path.join(tmp, 'gc.log')
    with open(p, 'w') as f:
        f.write(log)
    r = census(tmp)
    assert r['pauses'] == 4, r                      # 3 completion + 1 young, start-lines 0
    assert abs(r['stw_total_ms'] - 158.614) < 0.01, r   # 6.559+18.614+40.0+93.441, БЕЗ phases
    assert r['full'] == 2 and r['young'] == 2, r
    assert r['full_md'] == 1 and r['full_cc'] == 1, r   # full_cc канон-поле
    assert abs(r['scavAvg_young_ms'] - 50.0) < 0.01, r  # (6.559+93.441)/2 — mean YOUNG
    assert abs(r['full_avg_ms'] - 29.307) < 0.01, r     # mean-Full = ОТДЕЛЬНОЕ поле
    assert r['verdict'] == 'FULL-WINDOW-OUT', r            # full=2 ∉ [8,10]
    # HOST-путь по avg (G3): 9 Full × 300ms → avg 300 > 200
    log4 = '\n'.join(
        f'[t][info][gc] GC({i}) Pause Full (Metadata GC Threshold) 1M->1M(1G) 300.000ms'
        for i in range(9)) + '\n'
    with open(p, 'w') as f:
        f.write(log4)
    r4 = census(tmp)
    assert r4['verdict'] == 'INVALID-STW-HOST', r4         # avg 300ms > 200 = HOST
    # гейт-тест: CLEAN в окне [8,10]
    log2 = '\n'.join(
        f'[t][info][gc] GC({i}) Pause Full (CodeCache GC Threshold) 1M->1M(1G) {100.0+i:.3f}ms'
        for i in range(9)) + '\n'
    with open(p, 'w') as f:
        f.write(log2)
    r2 = census(tmp)
    assert r2['full'] == 9 and r2['full_cc'] == 9, r2
    assert r2['verdict'] == 'CLEAN', r2                 # avg 104.4ms ≤ 200, total 0.94s
    # Full вне окна [8,10]
    log3 = '\n'.join(
        f'[t][info][gc] GC({i}) Pause Full (Metadata GC Threshold) 1M->1M(1G) {1.0+i:.3f}ms'
        for i in range(12)) + '\n'
    with open(p, 'w') as f:
        f.write(log3)
    r3 = census(tmp)
    assert r3['verdict'] == 'FULL-WINDOW-OUT', r3       # full 12 > 10, total/avg малы
    assert r3['full'] == 12 and not r3['full_window_ok'], r3
    print('selftest OK: 5/5 инвариантов (start-excl, phases-excl, full_cc+scavAvg_young-канон, G3-host, окна)')
    return 0


def main(argv):
    if '--selftest' in argv:
        return selftest()
    if '--batch' in argv:
        base = argv[argv.index('--batch') + 1]
        for d in sorted(os.listdir(base)):
            p = os.path.join(base, d)
            if os.path.isdir(p):
                print(json.dumps(census(p)))
        return 0
    if len(argv) < 2:
        print('usage: absorb_464.py <run_dir> [--batch <dir>] [--selftest]', file=sys.stderr)
        return 2
    out = census(argv[1])
    print(json.dumps(out))
    return 42 if out.get('verdict') == 'INVALID-STW-HOST' else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
