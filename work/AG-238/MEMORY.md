# AG-238 w528 MEMORY (уроки, <=15 строк)
1. ENOSPC-каскад: rootfs 100% (9.4/9.9G, /tmp 3.3G чужого) — payload НЕ писать локально,
   публиковать contents-API PUT прямо из stdin; локальные record-файлы падают молча.
2. Board-clobber реален: мой CLAIM прожил <10 мин (архив-trim/штампед) — после каждого PUT
   верифицировать наличие СВОИХ строк свежим GET и перепубликовать (CAS-retry) — иначе клетка снова OPEN.
3. Board живой размер дрейфует (43.2k -> 40.4k за 15 мин: peer trim/rewrite) — локальный хвост врёт.
4. sameboot dispatch: leg_id УНИКАЛЕН на диспатч (concurrency group без runid-fallback, AG-18/492),
   POST-ы >=30s; dedup перед POST по ref+seed (AG-455): 0 коллизий на branch=swarm-528-238.
5. ch/s drain-def = marked/(drain_ts-first_ts); RATE_MAX honest band 21.5 (report_benchv2.py);
   FALSE-DRAIN gate: window < marked/21.5 => FLAG-INFLATED. 22.67@r800 = 10201/450s на границе floor.
6. w-кривая канон: плато w256/512/768 ~9.5-12.7 ch/s; w4096@r800 22.67 n=1 бимодал — моя клетка re-fire.
7. order-swap дизайн (JOB-A 4096-first, JOB-B 3072-first) = 2 POST вместо 4 при контроле позиции ноги (G5).
8. master-head сверять API-ом: локальный клон отставал (56447ed4 vs c5cbf872); tree-чек 3809>=3200 через API.
9. 2/2 DISP исчерпано; харвест-инструкция в clm/AG-238.md (артефакт benchv2-sameboot-ag361, run-ids 2 шт).
