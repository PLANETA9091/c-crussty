# LEDGER-L10 (TASK-459-L10) — P24 noise-octave scratch-pool: PARK
1. H10 «noise-alloc бустит Full на soak-хвосте» REFUTED: Full GC = 9 (5 CodeCache + 4 Metadata) на 4/4 прогонах (chk-14/-16/-11, anchor-33), AllocFailure-Full = 0 — пулу нечего снимать; «Full=9 vs банк 7» = JIT-объём/класс-сет, не heap.
2. Noise-lane на soak = 0.020-0.026% CPU (21-30 self из 103062-115503); alloc-окно: 0 октавных фреймов в top-20, единственный «noise» = Reaper-инфра noisesimd 10-11/3029-3896 scaled (0.28-0.36%, чужой носитель).
3. javap (kernel round-396-a, JDK21): PerlinNoise.getValue октав-цикл 0×`new` (аккум = scalar-локал dstore 12), NormalNoise.getValue 0×`new`, ImprovedNoise.noise/sampleAndLerp 0×`new`; аллокации шума только в ctor/boot (NoiseRouter build).
4. CAPTURE: прямой ≤+0.026пп; supermax GC-путь +0.39пп (100% захват всего observable noise-alloc × slope L05 −4.96пп/с) — в 51× ниже бара +20 → PARK без диспатча, real Δ = +0.00пп.
5. Живо в подсистеме: только cmp457_noisesimd @c8156a69 (SIMD/bulk-JNI compute-ось) и P25 2D-router cache; GC-носители искать в Vec3/AABB entity-чурне (28-29% alloc top-сайтов), не в шуме.
