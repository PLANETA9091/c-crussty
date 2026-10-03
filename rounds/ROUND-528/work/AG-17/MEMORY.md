# AG-17 w528 MEMORY (уроки, ≤15 строк)
1. w-ось ≡ dgw-ось ≡ DIM_GEN_WINDOW (вериф yml→env→plugin сам, не верь номенклатуре ног).
2. Окно = in-flight кап ТОЛЬКО gen-фазы; sustain-TPS инертен → ось не двигает TPS-компоненты S.
3. Little-Law: окно не лимитирует при W>>воркеров; burst-столл 0-42.6 ch/s = батч-режим глубокой очереди.
4. Dose не-монотонен (192/384 ниже 256, 1024 клифф, 6144 +2.0σ n=1) = boot-шум σ15-25% + heap, НЕ lever.
5. prereg: 4 queued оси-ноги, вердикт |z|<2σ (FW порог 2.5) → потолок 0 → CENS оси.
6. Solo-dgw POST = слот-жог (cross-boot запрещён каноном AG-189); серт только same-boot A/B (aa480s1 канал).
7. Конфиг-кноба chunk-system worker-threads в харнессе НЕТ (Moonrise авто из CPU) → lever ch/s = cpu-пулинг.
8. Дозор дешевле диспатча: 9 run-id статусов за 1 curl-цикл ~5с.
