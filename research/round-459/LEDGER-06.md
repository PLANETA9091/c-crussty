# LEDGER-459-L06 (3-5 строк, числа)
- L06 paldelta-климб: leg-3 lanes = PalettedContainer.get 3.0% (3145) + Ops.get 2.2% (2352) + serve4 1.9% (1960) + CHM.get 0.9% (935) + gate 0.8% (877); SimpleBitStorage.get <0.5% (демукс срезал с 1.1-1.4% ванили); JDK collections 6.8% (7140).
- P32+P36 capture: лейн 2.35% × 40-60% → Δ +1.0-1.5пп, потолок +2.35пп; НЕ pair-maker. Потолок пары: нога ≥18.7-19.2 (P31 +5-8 — главный, P34 +2-4 — swing) ← якорь ≤−1.3..−0.8 в [6729996,6779996] (a4 −0.8@6737702 Δ7706 или окно-C хвост [6761670,6779996]).
- Спящие блобы гейт: CERT-FIX жив — 10/10 tracked блобов несут cmp457_paldelta Utf8 (javap -v), build_430b_blobs.sh flat==nested gates OK, svorta 10/10 EQUAL, PalettedContainerOps.selfTest=true offline; правило: вайринг без reблоид-gate = placebo (roar-2 прецедент).
- javap-контракты: get(int)=volatile data→invokeinterface BitStorage.get→FastPaletteData aaload; SimpleBitStorage=magic*idx>>20 packed; serve4=2×volatile builtAtGen==gen гейты (цель P36), SNAPS CHM per-claim (цель P32).
- Резерв paletted-лейна: stride-перетюн 4096→2048 на dense-секциях (Ops.get 2.2% slow-path остаток) — отдельный диспатч с прегист-гейтом G5 Δ≥+0.5пп.
