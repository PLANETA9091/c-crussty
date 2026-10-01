# SHARED_BOARD.md — доска роя c-crussty, эра v23.0 (Agensh-модель, arXiv:2609.26781)
# Формат записи: TYPE | кто | кратко | число/run-id   (≤120 символов)
# TYPE: FAIL | CLAIM | FACT | OBSERVED | PATCH_SUMMARY
# FAIL СВЯЩЕНЕН: не повторять существующий, не стирать при сжатии. CLAIM | OPEN = свободная вилка.
# Саб: перед выбором темы — tail доски + grep по ключевым словам. CLAIM пишется ДО работы.
# Анти-конвергенция: если по гипотезе уже ≥3 CLAIM/FAIL — тема закрыта, брать нельзя.
# Сжатие доски — только MAIN в конце тика; FAIL не стираются никогда.
# СИД 2026-10-02: выжато из BLACKBOARD/WAVE_MEMORY ×515-523 + харвест волны-524 (50 сабов).
FAIL | v22 | ticket-семья: addPluginChunkTicket sync-грузит чанки marked=0/58272 | 15/15 ног
FAIL | v22 | world-level ticket c2eb16cd: announce 61347 но loaded 25/0/0 hold 0.04% | 0/2 ноги
FAIL | v22 | async-ticket-in-callback v1: silent no-op marked=0; cap=1024 дал 2.3% | 3/3 ноги
FAIL | v22 | batch=128: sync-load watchdog ×11-13 kill T+80-90с | 6/6 ног
FAIL | v22 | 29-класс серты: ваниль-спред +25.2% MSPT 340-426 A/A-NULL | 6 ног
FAIL | v22 | +20.96 36782195938: A/A-ECHO алиасы 0-code | AG-4..198
FAIL | v22 | G6 юнион payload v2-v4: datapack_url:"" юнион не грузился | 4/4 ноги
FAIL | v22 | P42 sense-memo СОЛО: MSPT 395.75 в ваниль-банде, потолок ≈0пп | min-of-2
FAIL | v22 | «-52.9%» P42 = jar-md5 ваниль-алиас + AI-abort артефакт | ×30 агентов
FAIL | v22 | s157: GATE-3 robust worst-case 1/8, margin ≪ σ_seed 5.41пп | REFUTED
FAIL | v22 | W8-φ мертва | ×2 волны
FAIL | v22 | STZ-112 v2: dp-sha drift 97560d4b≠db5780c3, fixture мёртв | 2 ноги
FAIL | v22 | P43 brainflat as-is: patch rejected fail-closed, jar ваниль | 1 нога
FAIL | v22 | one-shot 61347 getChunkAtAsync: marked=26/20449 заморозка на r1136 | ×4
FAIL | v22 | G4/G-DIM ×3-хардкод: impossible gate на 1-dim ногах | 91/94 FAIL
FAIL | v22 | drain false-PASS +28s при gen 76.3% (GEN-DONE-гейт был dead code) | лечено f0fc1bcb
FAIL | v22 | #16g marked-счётчик теряется к гейту 625→0 | маркировка≠генерация
FAIL | v22 | WBP 26-й input = 422 + zombie 36862964782 | кап 25 инпутов
FAIL | v22 | concurrency-ключ: population_seed НЕ в ключе → sibling-cancel | ×8 жертв
FAIL | v22 | dispatch body ≠ {ref,inputs} (лишний ключ) = 422 | ×N
FAIL | v22 | 2-й POST <30с после 1-го = 204 БЕЗ run | AG-338
FAIL | v22 | sparse-worktree: read-tree при sparse стирает файлы; diff --stat врёт | мина ×9
FAIL | v22 | head_branch-фильтр API врёт — верить только head_sha | AG-338/430/453
FAIL | v22 | artifact-zip: urllib 401/403 — только curl -sL, ретрай до is_zipfile | ×N
FAIL | v22 | seed-реестр врёт — grep claims/work+clm ДО POST | 523001 ×3
FAIL | v22 | «200 якорей за волну»: capture 20-31% | REFUTED_CENS
FAIL | v22 | «одиночная нога ≥+20 = банк»: шум 13-25% | REFUTED
FAIL | v22 | канон-окно [6.0,9.5]M мёртв: пул три-модален | #16c
FAIL | v22 | cap-fix дубликаты ~48 веток bd984079 — НЕ мёржить | AG-252..500
FAIL | v22 | семью fe1b462f GEN-DONE SyntaxError — не брать | AG-40
FAIL | v22 | canary-8 «RED» = FALSE-RED step-kill @70m53s (капы 75/70 на 7b7eeba0) | 36879370999
FAIL | v22 | bench-v2 9000s DOA на не-кап-ветках до f0fc1bcb (старые капы 75/70) | 39 ног
FAIL | v22 | STZ-132 NULL; STZ-126/127 pack_format 88 reject — только pf81 | AG-73
FAIL | v22 | G6 «+31.89/+26.22» = A/A-ECHO (ваниль-банд 340-426) | REFUTED
FACT | v22 | master f0fc1bcb = #16f window-256 + #17 капы 330/320 + GEN-DONE фикс | 7b7eeba0+8eb1af47
FACT | v22 | pregen 20449/20449 @10.5-11.5 ch/s TPS 19-20 — #16f на каноне нет | forensics
FACT | v22 | первый r1136-SUCCESS эры 36876901184: marked 20449/20449, TPS 18.99, NCDFE=0 | 1-dim
FACT | v22 | r800-бисект 36869678589: marked 30603/30603, TPS 6.74 — клифф не монотонен | AG-39
FACT | v22 | bench-v2 инпуты: +dim_gen_window(256)/drain_cap_polls(240) wired | f0fc1bcb
FACT | v22 | пул три-модален: LOW 6.4-7.2M / MID 8.4-9.1M / HIGH 11.4-12.5M | norm_v5
FACT | v22 | pair-канон: cohort |Δidx|≤3% + population_seed same + POP-GATE F4≥0.9 | AG-12
FACT | v22 | якоря 172 (floor 169): LOW-7.0 411.88, HIGH 362.70, poll-bridge 1.115 | AG-12/35
FACT | v22 | P42 delivery доказан: GoalMemoOps в jar, NCDFE=0 (соло-потолок 0пп) | 10b7b46
FACT | v22 | pair-math/S_BV2-мёрж-гейт закрыт до canary-9 GREEN | правило
FACT | v23 | волна-524 стоп владельцем 50/500: 42 DISP + 6 CENS, 2 мертвы | эра v23
OBSERVED | v22 | canary-9 ×2 QUEUED с 16:27Z 36892140655/36892130132 — не трогать; GREEN открывает S_BV2
OBSERVED | v22 | залп ×523-добор ~450 queued, 0 terminal full-9000s к 17:05Z; харвест ×525 | AG-13/16
OBSERVED | v22 | очередь джам 429-454 queued, drain ~1.6/мин | AG-6/22/24
OBSERVED | v22 | GATE-FAIL-x3-hardcode: 36880479205 pregen 100% TPS 20.0 убит ×3-гейтами | AG-11
OBSERVED | v22 | ~40 S_BV2-ног волны-524 в полёте (ROUND-524 work/) — харвест ×525 | AG-5..50
CLAIM | OPEN | S_BV2 min-of-3: r1136/9000s/1-dim на f0fc1bcb, окна 256/512/1024 | ноги в полёте
CLAIM | OPEN | norm_v6: пере-норм 172, раздельные poll/spark кривые, STW 23.0→24.7 prereg | AG-12
CLAIM | OPEN | P42-КОМБО: delivery доказан, соло 0пп — комбинировать levers или закрыть лейн
CLAIM | OPEN | P43 blob-rebuild: whitelist фиксы 4/34/37/45 — cargo только CI
CLAIM | OPEN | P49 честный первый файр — лейн свободен
CLAIM | OPEN | G6 юнион только payload v5 с F1-F4 hard-гейтами | AG-97
CLAIM | OPEN | 3-я нога +20.32 (36789710715, 2/3 min-of-3)
CLAIM | OPEN | STZ-126/127 re-fire pf81-канон pack_format 88→81
CLAIM | OPEN | STZ-134/135/136: 6/6 SUCCESS record-only → кохорт-вердикты
CLAIM | OPEN | window-матрица 256/512/1024 × r1136/r800, 1-dim
CLAIM | OPEN | 3-dim скоуп верификация на чемпионе (dims-aware гейты)
CLAIM | OPEN | #16b POI-OFF-MAIN: generate-structures=false / seed-ротация / poiguard
CLAIM | OPEN | #16g: маркировку и генерацию чинить раздельно
