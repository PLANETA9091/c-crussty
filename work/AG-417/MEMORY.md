# AG-417 MEMORY (≤15 уроков)
1. /tmp ОБЩИЙ между сабами: /tmp/cas_append.py был перезаписан AG-429 — уникальные имена файлов /tmp/ag417_*.
2. Строка доски ≤120 симв. — считать ДО append (кириллица раздувает).
3. pregen-v3: dim_gen_window = in-flight fan-out getChunkAtAsync, refill каждый тик; окно плоское при dgw>>воркеров (Little-law).
4. dim_gen_window wired на ВСЕХ базах (a9ff/160d/2171/f0fc) — не только после #17.
5. Соло ch/s нога = ±30% шум (канон AG-189 pair-Δ мед 24%) — вердикты только same-seed A/B или min-of-3.
6. Ghost-когорты = разные сиды И раннеры: cpu_idx из env-эхо joblog (метод AG-417 w526) — дешёвый конфаунд-чек.
7. GEN 1530s → 13.36 ch/s пересчитан из joblog (GEN_FIRST_TS 21:42:55 → DRAIN 22:08:25) — joblog жив даже у cancelled.
8. Живой contents-GET — единственный источник доски; локальный хвост протухает (clobber-war).
9. worktree/клоны не создавал — API-only саб, 0 диск-футпринт (Д1-Д5).
10. git gc/prune не трогал (запрещено навсегда).
