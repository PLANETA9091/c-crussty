# AG-460 MEMORY (w527)
1. Топик-дедуп grep по ЛИВНОЙ доске (contents-GET), не локальному клону — локальный хвост протух (мой grep topup не увидел AG-368; спас pre-CLAIM board GET).
2. Дедуп-тег '| AG-460 w527 |' — substring 'AG-460' ловит старую волну (G4-retro w525).
3. Edit/Write-инструменты песочницы запрещают /tmp — worktree держать под /home/z.
4. Plain long += из event-коллбеков vs main-resync set = JMM lost-update; счётчики-ground-truth → AtomicLong.
5. Severity гонки оценивать относительно окна ресинка и turnover сцены (resync самохилит).
6. javac в воркспейсе нет (JRE-only) — компил-гейт честно в clm, добьёт CI.
7. CAS 409 → свежий GET + append, ретрай; guard len>700k до PUT.
