# WAVE_MEMORY — сквозная память роя (волна N → волна N+1). Формируется: (a) сидом координатора, (b) консолидацией work/AG-*/MEMORY.md.

## ВОЛНА-515 (первая волна эры v22: 500 агентов, единый SWARM_PROMPT.md, без ролей) — СИД от волны-514
### База лестницы S (композит: TPS@150k + ch/s@chunk-gen + TPS@dp50k)
- chunk-gen: 22.0 ch/s @cadence 12.4s (V1 worker-saturation ЗАКРЫТ: busy 0.1-1.2% ≪85%)
- entities: 150k канон; P31-IB hist popcnt/ents≈51%; блокер pack-guard: 10-бит упаковка sx/sz<0 теряет биты
- datapack: dp@20k=4.8 канон; dp@50k G-B2 k=8/18 (4-й тик); dp900 0.3 n=15; dp@100k клифф ~207k
- BENCH-V2 базы ещё НЕТ — волна-515 обязана его родить (20k forceload × 3 измерения × Terralith/Tectonic/Incendium/Stellarity × Piper-бешеный спавн × view/sim 32) и снять базовые числа

### Горячие вилки ≥+20 (проверенные прогнозы, из board ×513)
1. SWAR-X re-arm: 1 rust-строка → +11.7..+19.5 (drain-гейт desync: eq_epoch широкий union vs узкий 5-флаг)
2. Компо OCC+P31: 1 java-строка occFlagArmed на c5bfix-базе; min-of-3 прогноз 23.93 [22.43;25.53] ≥ 22.9 req; seeds ib7c-ib9c 1669-1671
3. pack-guard WILD-01: 2 java-строки + selfTest-кейс C pack(−1,4,0)⇒all-ones; ~50% сущностей r640 запад/север
4. W8-φ wiring: ханки ДО POST (cmp511_w8feed на master отсутствует = PLACEBO); φ-шапка N_regions 4→8 → +18..+47пп
5. №24 GATE-3: леги ≥22.74; текущий min 20.93 (3-й тик); pair-сертификаты эродируют (канон pair@worst)
6. Г3-srv пары: fen −0.2/−9.43 vs unf +6.99/+19.99 @d091f96; кросс-сид Δ(fen−unf) при Δ≤50k
7. dp G-B2: dp17-20 в полёте (полный URL sha 16fa1a32 — алиас=FATAL урок dp13-16); 4/4 CLEAN → FIRE k=12/22 P≈0.37

### Инфра-каноны (не наступать повторно)
- эпохи ≤40 POST/залп (fleet 43-49); 429/403 → DISP-INTENT с payload; same-seed re-roll бесполезен при band-dead
- sparse-worktree только; cp клонов запрещён (диск 85%, 1.5G free); worktree remove после push
- dp_url алиас=FATAL, ∅=подмена 9.0×; ledger sha-префикс = арбитр кэша
-Offline-агенты: патчи .patch в work/AG-<N>/; координатор мёржит только MERGE-READY с гейтами (cargo 0 err CI, blobs ALL IN SYNC, canary)

### Директива владельца (2026-10-01) — цели эры v22
- 500 агентов/волну ОДНИМ сообщением; единый SWARM_PROMPT.md; без ролей; рой сам решает
- каждая волна улучшает предыдущую на ≥+20% (S-лестница); память волна→волна; рост 500→20000/тик
- BENCH-V2: 20k форс-лоад все измерения, Terralith/Tectonic/Incendium/Stellarity, бешеный спавн, высокие настройки, генерация чанков ОЧЕНЬ быстрая
- все коммиты ТОЛЬКО от лица PLANETA9091 (PLANETA9091@users.noreply.github.com)
