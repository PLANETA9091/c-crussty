# LEDGER-02 (TASK-459-L02, chunk-tick eligibility ID-P22)
1. Sched-lane остаток на монстр-ноге chk-14 (+21.7@8687055): getChunkNow 738/103062 (0.7%) + CLLRCHT.getNode 763 (0.7%) + off-main 1260 (1.2%) видимых; полный лейн 4.3-4.9% (ваниль 4.6-5.2% по 4 профилям: ваниль-bucket 8.0-8.8%, C2-носитель снял ~0.5пп).
2. javap K1-K8: getChunkNow = чистый CHM.get (O(1), ускорять нечем); eligibility = entityTickingChunks/playerTickingChunks ReferenceList raw-array (K3/K6) + per-section isRandomlyTickingBlocks (K5) + hasAnyNearbyNarrow area-map (K7); setFullChunk = единственный feed-пойнт fullChunks (K2).
3. Capture-матем: Δ = 4.6% × 30% захват ≈ +1.4пп (диапазон +1.1…+2.0); ПОТОЛОК лейна = +4.6пп < +20 → P22 только стековый слой, не носитель.
4. Климб-математика chk-14 (закон 13c): P22 (+1.4) ⊕ P31 (+5-8) ⊕ P32/P36 (+1.5-2.5) → нога +29.6…+33.7 → якорный порог окна B ≤+9.6…+13.7 → a26 +12.4@8671791 (Δ15k) проходит → пара +20.2…+21.3 ≥ +20 ✓.
5. Гейты G1-G6 preregistered (RESEARCH-459-L02.md §7): ARM+schedProbe>0; flat==nested бит-в-байт drift=0; young≤128/Full≤9; pop 140-165k+items 0.00; pair ≥+20 min-of-3 band 6.0-9.5M; fail-closed BROKEN-защёлка с offline-lockstep selfTest.
