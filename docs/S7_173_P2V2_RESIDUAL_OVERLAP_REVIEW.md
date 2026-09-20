# S7-173: P2-v2 RESIDUAL OVERLAP — PARITY-FEASIBILITY REVIEW (TASK-373 offline, 2026-09-20)

Статус: LANE-OPEN-подготовка (бранч (4c), лег 35499022752 @ b18b3a4 в полёте).
Вопрос: если P2-OFFLOAD v1 (REGION_STEAL="2") НЕ пройдёт ДВОЙНОЙ БАР — что из
«P2-v2 residual overlap» (GOAL ×56-ADD2) паритет-реализуемо, и стоит ли оно
реализации по правилу ТОП-1 (<5% запрещено)?

## 1. Разложение остатка main в режиме MAIN_OFFLOAD (v1)

В v1 main в фазе 3 работает так: snapshot-fill (сериен, до GO) → GO.release →
`DONE.await()` (парковка на ВСЁ время тиков хелперов) → post-join: retention
hygiene → phase-4 (drain PENDING, FIFO) → phase-4b (drainDeferredBlockUpdates) →
rethrow workerError. Кандидаты на overlap с тиками хелперов:

| Кусок | Паритет при переносе ДО DONE | Потолок |
|---|---|---|
| phase-4 drain PENDING | **БЛОКИРОВАН**: PENDING производится ВО ВРЕМЯ тиков хелперов; ранний drain ломает FIFO-after-join = наблюдаемая семантика ванильного post-tick (RECON-16 эталон) | — |
| phase-4b blockUpdates | **БЛОКИРОВАН**: тот же класс — replay ПОСЛЕ join (S7-168 defect-fix контракт) | — |
| retention hygiene (tail-null) | **БЕЗОПАСЕН**: пишет только `arr[slot][j], j>=len[slot]`; хелпер читает только `j<len[slot]` (граница зафиксирована в tickBucket); рост бакетов — только в fill (main) | <0.3% MSPT (w хвостов массивов) |
| парковка → spin | без изменения семантики | ~0 (park==spin по бюджету) |

## 2. Вердикт по правилу ТОП-1

Паритет-безопасная часть (hygiene-overlap) имеет потолок <0.3% << 5% →
РЕАЛИЗАЦИЯ ЗАПРЕЩЕНА (правило «побочные лейны <5% НЕ реализовывать»).
Паритет-блокированная часть (phase-4/4b) — единственный кусок с реальным
весом (RECON-36 §2: сериальные фазы main), и она НЕ переносима без смены
контракта drain-семантики (отдельный большой лейн, не «v2»).

**ИТОГ: P2-v2 residual overlap как код-рычаг ОТКЛОНЁН (S7-173).**

## 3. Матрица решений TASK-373+ (по вердикту absorb 35499022752)

| Вердикт absorb_s7197 | Действие |
|---|---|
| PG-T1 ok + ДВОЙНОЙ БАР обе оси ≥+10% vs 1.60 @ 6680195 | GREEN → min-of-2 (dispatch_s7197.py повторно, тот же b18b3a4+) → banking v4 (v3+main_offload) |
| Один баровый барьер / LANE-OPEN | РЕБАЛАНС-ветка: I повторно по свежему threaded-wall профилю ЭТОГО лега; REGION_CHUNKS/WORKERS sizing (мировые инпуты существуют, yml на капе не трогаем); при I<1.15 в v1-режиме → потолок исчерпан, RECON main-остатка до под-лейнов ≥5% |
| CRASH (workerError surfaced) | rollback env REGION_STEAL=0, инцидент-док по классу s7192/s7196 |
| BAND-DISCARD (cpu_index вне 6.0..9.5M) | re-roll dispatch_s7197.py (прецедент ×56-ADD2) |

## 4. Мemo на Rebalance-ветку

Rebalance НЕ требует нового кода ядра: меняются входы bench-лега (REGION_CHUNKS/
WORKERS), диспатч = модификация dispatch_s7197.py (workflow-input'ы). Гейт:
после rebalance снова I ≤1.15 И main-доля <10% MSPT — иначе RECON под-лейнов.
Двойной бар считается только на полном леге (bank v3 + travel_diet=0).
