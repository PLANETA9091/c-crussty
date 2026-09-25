# LEDGER-09 (TASK-459-L09, GC/STW-инструмент)
1. STW-таблица 33 ног ×457/×458: STW 17.7-25.2s (p50 20.3), Full=9 на 27/33, soak-Full=1 (boot 4-5 + teardown 3-4 детерминизм).
2. Slope −4.96пп/с (L05, n=5, r=−0.93) ОПРОВЕРГНУТ: n=33 → −1.09пп/с, 95%CI [−3.11,+0.93], r=−0.187, R²=0.04; L05-пятёрка = selection-байас хвостов (воспроизведена −4.98/r−0.965).
3. Потолок STW-эффекта: soak-STW median 8.35s/300s → 2.78пп (max 3.28пп, roar-2); slope −1.09 = механика пересчёта soak/total, сверх неё GC-штрафа нет.
4. «Депресс-кластер chk-11/12 Full=9» = миф (Full=9 у всех, вкл. chk-14 +21.7); банк-справка 18.8s/Full=7 = p6 популяции co-run-эры.
5. STW-жертвы (причинно) = 0; маркер-аномалии окна (гейт G3-STW: STW>23.0s ИЛИ avg>200ms) = 3: anchor458-33 −11.8 (якорь из pair-пула out), roar458-2 −10.7, chkmono457-11 −3.6 (+chk-12 парой); true-RED (GC-clean): poi457-14 −18.1, a36 −5.0, poi457-5r2 −5.3, chk-12 −3.7, poi456-3 −0.5.
6. javap: GcHeapStat$Summary.gcOverHead/gcTotalDuration/totalGCs + JfrStatsResult.heapSummary/asJson + jfc jdk.GCPhasePause threshold 0ms — in-kernel JFR-цензус уже есть, spark/gc.log не единственный путь.
