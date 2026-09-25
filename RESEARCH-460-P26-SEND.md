# RESEARCH-460-P26-SEND — P26 send-burst coalescing на poi-носителе (TASK-460-38, law-13 CLIMB)

Ветка: round-460-poisend-1 @ 5ecd841a (poi-носитель cmp456_poi, POI-plane живая).
Левер: **cmp456_poi_send** (STRICT-OR; исторический cmp459_p26 оставлен в Java-гейте и cp блоба).
Диспатч: `dispatch_460.py sw round-460-poisend-1 cmp456_poi_send` (БАНК v4: cpu_band [6.0M, 9.5M], 150k, fp=4).

---

## 1. ROOT-CAUSE негатива p26 (run 36157837929, VALID −12.6@7067374) — ХОСТ, НЕ КОД

Числа из лога рана (0_world-bench.txt):

| # | метрика | значение | порог/фон |
|---|---|---|---|
| 1 | GC total pause | **24195.7 ms** | гейт L05 G3-STW: >23.0s → INVALID-STW-HOST |
| 2 | GC avg pause | **216.03 ms** | гейт L05: >200ms → INVALID-STW-HOST; max pause **2526.6 ms** |
| 3 | Full GC | **10** (6 CodeCache + 4 Metadata — детерминизм, не аллокация) | банк 7, «депресс-ноги» 9 |
| 4 | median TPS | **2.0** (поллы [18.0, 1.4, 1.6, 2.0, 2.2, 2.2]) | TPS_exp(7067374)=2.287 → norm −12.6% |
| 5 | ВАНИЛЬ-якорь на том же раннере | anchor-16 **−16.9@7066543** (Δrunner=831!) | рычаг-пустой ранер на **4.3пп ХУЖЕ** p26-ноги |
| 6 | шум полосы | anchor-4 −8.1@7059636 ↔ poi457-9 +5.1@7054948 (Δrunner<12k) | свинг ±13пп внутри полосы |

Вывод: −12.6 = депресс хоста (STW-host + шум полосы ~7.07M), код exculpated:
P26-v1 = accounting-стаб (p26WindowMark = 1×nanoTime + счётчики ПОСЛЕ connection.send,
flush НЕ откладывается) на лейне network(kernel) 2.6% CPU — стаб физически не может дать −12.6.

## 2. ГЛУБОКИЕ НАХОДКИ (2-3 числа)

1. **P26-v1 скаффолд НЕ консолидирует flush** — реальный flush-сайт сидит в
   Connection/netty (kernel-closed); `p26FlushMarks` только считает. Прогноз
   +0.3-0.8пп остаётся НЕПРОВЕРЕННЫМ ни одним раном; следующий лаб-лег = канальный
   flush через javap Connection (research doc §5-риск-1).
2. **Гейт G3-STW не зашит в абсорбер**: absorb_459b.py valid-критерии НЕ содержат
   STW-цензус → STW-host-раны (24.2s/216ms) получают VALID и загрязняют pair-статистику.
   Чинить: scavAvg/soakFull поля в absorb-канон (ledger L05 поля уже названы).
3. **Носитель-зависимость лейна**: joins-burst lane 1.75% (потолок +1.5пп) жив на
   poi-носителе; на chk-оси lane 0.00 — диспатч туда = карма-шум (L07). Poi-база
   5ecd841a подтверждена: chunk_send5 union уже несёт cmp456_poi, poi_plane.rs 621 строка.

## 3. Порт (worktree @5ecd841a, sparse-checkout 17→111MB)

- src/chunk_send6.rs — NEW (из round-459-p26 @dd00730a, lever → cmp456_poi_send,
  тест-иглы дополнены cmp456_poi).
- chunksend/net/.../ChunkSendOps.java — база ⊕ P26-машина; unions 435/437/444/450/
  452/453/**456**/459/**460=cmp456_poi_send**; P26_ACTIVE = cmp456_poi_send || cmp459_p26.
- src/chunk_send5.rs — union + marker cmp456_poi_send (коалисинг едет на encode-cache).
- src/lib.rs — mod/register/activate (после chunk_send5, до queryplane).
- **Блобы пересобраны** scripts/build_438c_chunksend.sh (javac 21, kernel-jar):
  ChunkSendOps 10108→**11305** bytes, major 65, **flat==nested** (cmp-гейт OK),
  raw-cp все маркеры OK (check_blobs_sync.sh EXIT=0, + новые иглы).
- cargo test --release: **344 passed / 0 failed** (8 chunksend6-тестов вкл.
  no-nested-NCDFE-канон, resolution closure, pristine guard, redirect sites:1).
- NCDFE T1=0: zero nested classes (build-dir `$`-гейт OK), selfTest()Z до ARM,
  define retry ×10 — канон fa9054d9/5ecd841a сохранён.

## 4. Прогноз

Нога: норма poi-семьи на валидных ногах ~+3.8..+16.9; P26-стаб на живом лейне
ожидание **0..+0.8пп** (стаб-потолок; полный flush-лег даст заявленные +0.3-0.8пп
netty-хвоста). Пара ≥+20 — цель закона 13: poi-носитель + якорь ≤−4.6 в окне D
[8907260,9007260].

## SOURCES

run 36157837929 логи (API), /tmp/poi_stats.py цензус 13 poi-ног + 70 якорей,
docs/LAB_LEDGER.md L05/L07/L09, RESEARCH-459-P26.md, netty FlushConsolidationHandler.
