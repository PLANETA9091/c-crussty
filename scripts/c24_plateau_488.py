#!/usr/bin/env python3
"""c24_plateau_488.py — C24 tps_exp_dp плато-модель ×488: границы, slope, бар-дрейф.
Данные: raw 5s-поллы из /tmp/abs488/<run>/x/server-stdout.log (+run-env cpu_index),
проверены вручную 2026-09-29; канон C06/C24, мастер 13a955a3. Офлайн, 0 диспатчей.
Вывод: (1) домен/ slope по 4 точкам ×488; (2) бар-дрейф 58.9→51.8pp и влияние на
wiring-пары c02/c65/c66/c67 (компо-матрица bitset⊕roaring 31.6-43.9pp ALL, C01).
"""
import re, statistics as st

# --- raw пост-рамп 5s-поллы (C55-фильтр <15.0 применён при извлечении) ---
PTS = {  # label: (run, cpu_index, [5s-поллы пост-рамп])
    "C98-dps600":  (36513549586, 6486164, [0.4, 0.2, 0.3, 0.3, 0.4, 0.3, 0.3, 0.3]),
    "C78-dps43":   (36511617786, 6652875, [0.3, 0.3, 0.3, 0.4, 0.4]),
    "C51-dp03":    (36509535833, 6688790, [0.3, 0.3, 0.3, 0.3, 0.3]),
    "ARB-baserep": (36506102482, 6831804, [0.4, 0.4, 0.4, 0.4, 0.3]),
}
# канон-пул (ledger Л1331/Л1348/CLM-C06 §2): верх домена
POOL_OLD = {"dp-r2": (6.67, 0.3), "dpbase3": (6.94, 0.3), "C03-par": (8.29, 0.3),
            "dpbase6": (8.75, 0.3), "c91": (8.84, 0.3)}

print("== 1. Мед-поллы и домен ==")
meds = {}
for k, (rid, cpu, polls) in PTS.items():
    m = st.median(polls); meds[k] = (cpu, m)
    print(f"{k:12s} run {rid} cpu {cpu/1e6:.3f}M med={m} polls={polls} last2={st.median(polls[-2:])}")

lo = min(c for c, _ in meds.values()); hi_canon = 8.84
print(f"домен ×488-кластер: [{lo/1e6:.3f}, {max(c for c,_ in meds.values())/1e6:.3f}]M; "
      f"новый низ {lo/1e6:.3f}M vs канон 6.67M ({(lo/1e6-6.67)/6.67*100:+.1f}%)")

print("\n== 2. OLS slope по 4 точкам ×488 (med vs cpuM) ==")
xs = [c/1e6 for c, _ in meds.values()]; ys = [m for _, m in meds.values()]
n = len(xs); xm = sum(xs)/n; ym = sum(ys)/n
sxx = sum((x-xm)**2 for x in xs); sxy = sum((x-xm)*(y-ym) for x, y in zip(xs, ys))
slope = sxy/sxx
ssres = sum((y - (ym + slope*(x-xm)))**2 for x, y in zip(xs, ys))
sstot = sum((y-ym)**2 for y in ys)
r2 = 1 - ssres/sstot
se = (ssres/(n-2)/sxx) ** 0.5
t = slope/se
print(f"slope={slope:+.3f} dp/M R2={r2:.2f} SE={se:.3f} t={t:.2f} df=2 t_crit=.05=4.303 -> "
      f"{'ЗНАЧИМ' if abs(t)>4.303 else 'н.з. (совместим с каноном slope≈0)'}")

print("\n== 3. Монотонность vs пул (глобальный slope-тест) ==")
arb = meds["ARB-baserep"][0]/1e6
for name, (c, v) in POOL_OLD.items():
    if c > arb:
        print(f"  арбитр {arb:.3f}M=0.4  VS  {name} {c}M={v}  => наклон между ними "
              f"{(v-0.4)/(c-arb):+.3f} dp/M {'ОТРИЦАТЕЛЬНЫЙ -> глоб. slope ОПРОВЕРГНУТ, 0.4 = квант-флип off-{runner/slot} (класс C07 z+6.9σ, 1-квант)' if v < 0.4 else ''}")
        break

print("\n== 4. Бар Δcpu по классу базы (канон C06: Δcpu=(1−b/(b+0.43))·100) ==")
bar = {}
for b in (0.2, 0.3, 0.4):
    bar[b] = (1 - b/(b+0.43))*100
    q = (1 - b/(b+0.1))*100  # квант вверх, канон C06 (SNR=бар/квант: 2.05/2.36/2.59)
    print(f"  база {b}: цель {b+0.43:.2f} TPS, бар {bar[b]:.1f}pp, квант вверх {q:.1f}pp, SNR {bar[b]/q:.2f}q")

print("\n== 5. Влияние на wiring-пары (компо-матрица 31.6-43.9pp ALL) ==")
for x in (0.316, 0.439):
    for b in (0.3, 0.4):
        # вторая плоскость Y: (1-X)(1-Y)=b/(b+0.43)
        Y = 1 - (b/(b+0.43))/(1-x)
        print(f"  X={x*100:.1f}pp @ база {b}: Y ≥ {Y*100:.1f}pp", end="")
        print(f"   ratio X/бар={x*100/bar[b]:.2f}x  дефицит {bar[b]-x*100:+.1f}pp")
print(f"  FN-окно [{bar[0.4]:.1f}, {bar[0.3]:.1f})pp ширина {bar[0.3]-bar[0.4]:.1f}pp; "
      f"макс-матрица 43.9 {'<' if 43.9 < bar[0.4] else '≥'} {bar[0.4]:.1f} -> "
      f"{'ПУСТО (0 флипов)' if 43.9 < bar[0.4] else 'НЕ ПУСТО'}")
print(f"  collision-REOPEN потолок 14.88pp vs Y≥{ (1-(0.4/0.83)/0.561)*100:.1f}pp (X=43.9, база 0.4): "
      f"{'ОТКРЫТ (capture ≥'+format((1-(0.4/0.83)/0.561)/0.1488*100,'.1f')+'% юниона)' if 14.88 >= (1-(0.4/0.83)/0.561)*100 else 'закрыт'}")
