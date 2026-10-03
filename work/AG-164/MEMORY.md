# AG-164 w528 — MEMORY (≤15 уроков)
1. runs-API: `conclusion=`/`status=` фильтры списка молча игнорируются — верили только direct-GET /runs/<id> (мой «9 жертв» был артефактом; поймал GET-контролем).
2. bench-v2-sameboot.yml L68-70: cancel-in-progress:true, group=ref+seed+radius+leg_id — 2-й POST с теми же inputs на той же ветке убивает старший run (даже in_progress).
3. AG-133: 37109372401 отменён через 1s после сиблинг-POST (08:20:43→44) — «2/3 pairs» = 1/2; их ре-POST старых inputs убил бы живой 37109382578.
4. cancel-API: queued = no-op (AG-83 канон), но IN_PROGRESS = работает — w526-rot-пара 37026832903/00733 убита sweep 08:49:51Z (17.5h зомби зачищены).
5. Census 09:01Z: ip40 = 40/40 swarm-528 (27 sb + 6 wbr + 7 bv2); очередь ≈ 25 ног; волна-3 ETA 09:25-40Z, дрен ~11:30Z.
6. Пустые seed/leg_id → anon-run_id fallback = уникальная группа (так спасаются 13 пар в очереди).
7. Доска: CAS-PUT с retry-409, строки ≤120 — штатно, штампед держит.
