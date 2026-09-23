# ROUND-403 BOTTLENECK (TASK-403, тик 21:08 +08, cron 40247)

## База
- master f19d5f5 (TASK-402). Банк v4: 2.6@8551924 / 2.2@6653417. Свежие same-day якоря: 2.40@6741944 / 2.20@6728002 (402), 2.2/2.6/2.3 (401), 2.4/2.3/2.4/2.35/2.2 (400).
- БАР = +80% pair-stable (min-of-3, parity PASS, RAM/CPU без регрессий). Лучший реплицированный: stagger +12.5pp (3/3 ARMED).

## TOP-1 (post-J lanes, банк-профиль)
1. **JNI-граница 12-17% wall** (rust-пути: скрытая цена пересечения границы, jnibulk recon не сошёлся) → вектор B: raw-arena/coarse-stamp.
2. **Моб-срез nav+push+collide ≈35% wall** (nav_ai ~14% россыпью, collide 8.68%, push ~6-8%) → вектор C: tickplane whole-body retarget.
3. **stagger только на push+canUse** — незакрытые соседи: collide/sensing-лейны, N-скан (2/8) → вектор A: stagger-варианты на d47d944.

## План тика
- Якоря ×3 (master) + stagcomp ре-ран ×2 (@60fe902, cmp402_stagcomp, ожидание ~+20-25pp при ортогональности) — диспатч немедленно.
- Волна 1: A stagger-варианты / B jnibulk / C tickplane — полные пайплайны субагентов.
- Композиция к бару: stagger +12.5 ⊕ comp +12.5 ⊕ stagcomp(UNPROVEN) — сумма ступеней ~+25-37pp; нужны 2-3 MEGA-вектора класса «замена подсистемы».
