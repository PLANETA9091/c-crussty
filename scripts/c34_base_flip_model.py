#!/usr/bin/env python3
"""c34_base_flip_model.py — COMMANDER C34 ×488: dp-jitter / консоль-квант.
Модель флипа бара 58.9pp ↔ 51.8pp при базе-медиане, гуляющей 0.3↔0.4 между ранами
(5/6-полл anti-gaming гейт C06). Офлайн-матем по фактам /tmp/abs488 (закон 5 чист).
"""
from math import comb
import random

random.seed(488)

# ---------- 0. ЭМПИРИКА: пул канон-поллов dp-плоскости (5s-колонка, ramp эксклюд) ----------
# C01-baserep 36506102482: [0.4,0.4,0.4,0.4,0.3]  (n=5)
# C51-dp03    36509535833: [0.3,0.3,0.3,0.3,0.3]  (n=5)
# C78-dps43   36511617786: [0.3,0.3,0.3,0.4,0.4]  (n=5)
# C98-dps600  36513549586: [0.4,0.2,0.3,0.3,0.4,0.3,0.3,0.3] (n=8, 600s)
RUNS_5S = {
    "C01-baserep": [0.4, 0.4, 0.4, 0.4, 0.3],
    "C51-dp03":    [0.3, 0.3, 0.3, 0.3, 0.3],
    "C78-dps43":   [0.3, 0.3, 0.3, 0.4, 0.4],
    "C98-dps600":  [0.4, 0.2, 0.3, 0.3, 0.4, 0.3, 0.3, 0.3],
}
# 1m-колонка той же строки "TPS from last 5s,1m,5m,15m" (пост-ramp, последние n поллов)
RUNS_1M = {
    "C01-baserep": [0.4, 0.4, 0.4, 0.4, 0.4],
    "C51-dp03":    [0.3, 0.3, 0.3, 0.3, 0.3],
    "C78-dps43":   [0.3, 0.3, 0.3, 0.4, 0.4],
    "C98-dps600":  [0.3, 0.3, 0.2, 0.3, 0.3, 0.3, 0.3, 0.3],
}

def pooled_p(runs):
    polls = [v for ps in runs.values() for v in ps]
    return sum(1 for v in polls if v >= 0.4) / len(polls), len(polls)

p5, n5 = pooled_p(RUNS_5S)
p1, n1 = pooled_p(RUNS_1M)

# ---------- 1. БИНОМ-ГЕЙТЫ: P(классификация = 0.4) при per-poll Бернулли(p) ----------
def p_at_least(k, n, p):
    return sum(comb(n, i) * p**i * (1 - p)**(n - i) for i in range(k, n + 1))

def flip_rate(q):  # P(соседние раны классифицированы РАЗНЫМИ барами)
    return 2 * q * (1 - q)

def damped(q, legs=3):  # min-of-3: пара = мажоритарный класс 3 ног
    q3 = sum(comb(legs, i) * q**i * (1 - q)**(legs - i) for i in range(2, legs + 1))
    return q3, flip_rate(q3)

def inconsistency(q, rounds=6):  # P(≥1 вердикт-флип пары за 6 раундов судейства)
    return 1 - (1 - q)**rounds - q**rounds

print("=" * 78)
print(f"[0] ЭМПИРИКА-ПУЛ: 5s-колонка p̂={p5:.4f} ({n5} поллов) | "
      f"1m-колонка p̂={p1:.4f} ({n1})")
for run, ps in RUNS_5S.items():
    k5 = sum(1 for v in ps if v >= 0.4); k1 = sum(1 for v in RUNS_1M[run] if v >= 0.4)
    print(f"    {run:12s} 5s: {k5}/{len(ps)} 0.4  |  1m: {k1}/{len(RUNS_1M[run])} 0.4")

print("=" * 78)
print("[1] P(≥4/5 поллов 0.4) = p^4·(5−4p)  — гейт C06 anti-gaming как написан")
print(f"    {'p':>5} {'P(>=4/5)':>9} {'P(мед>=3/5)':>11} {'P(>=5/8)':>9} {'P(фикс>=4/8)':>12}")
for p in [0.20, 0.30, p5, 0.40, 0.50, 0.60, 0.80]:
    print(f"    {p:5.3f} {p_at_least(4,5,p):9.4f} {p_at_least(3,5,p):11.4f} "
          f"{p_at_least(5,8,p):9.4f} {p_at_least(4,8,p):12.4f}")
print(f"    ^ p̂={p5:.3f} пул-оценка (8/23); фикс-4 гейт на 600s (n=8) "
      f"инфлирует класс-0.4 в {p_at_least(4,8,p5)/p_at_least(4,5,p5):.1f}× vs n=5 — ДЫРА")

print("=" * 78)
print("[2] ФЛИП БАРА 58.9<->51.8 МЕЖДУ РАНАМИ (f = 2q(1-q)) и min-of-3 демпфирование")
for name, q in [("гейт >=4/5 (C06)", p_at_least(4, 5, p5)),
                ("чистая медиана >=3/5", p_at_least(3, 5, p5)),
                ("строгий мажоритар >=5/8", p_at_least(5, 8, p5))]:
    q3, f3 = damped(q)
    print(f"    {name:26s} q={q:7.4f}  f(ран-пары)={flip_rate(q):7.4f}  "
          f"min-of-3: q3={q3:6.4f} f3={f3:6.4f}  P(инконсист.6раундов)={inconsistency(q):6.4f}")

print("=" * 78)
print("[3] ANTI-GAMING: ресьпл base-rep до прохода гейта, P(>=1 pass за K попыток)")
qg = p_at_least(4, 5, p5)
for K in [1, 2, 3, 4, 5]:
    print(f"    K={K}: 1-(1-q)^K = {1 - (1 - qg)**K:.4f}")

print("=" * 78)
print("[4] ГЕТЕРОГЕННОСТЬ РАН-УРОВНЯ: MC-тест однородной Бернулли против наблюдаемого")
obs = [sum(1 for v in ps if v >= 0.4) / len(ps) for ps in RUNS_5S.values()]
print(f"    набл. фракции по ранам: {[f'{o:.2f}' for o in obs]}  спред={max(obs)-min(obs):.2f}")
N = 200000
ns = [len(ps) for ps in RUNS_5S.values()]
hits = {"spread>=0.8": 0, "extremes": 0}
for _ in range(N):
    fr = []
    for n in ns:
        k = sum(1 for _ in range(n) if random.random() < p5)
        fr.append(k / n)
    if max(fr) - min(fr) >= 0.8:
        hits["spread>=0.8"] += 1
    if any(f >= 0.8 for f in fr) and any(f == 0.0 for f in fr):
        hits["extremes"] += 1
print(f"    P(спред фракций >=0.8 | однородная p̂) = {hits['spread>=0.8']/N:.4f}")
print(f"    P(ран >=4/5 И ран 0/5 одновременно | однородная) = {hits['extremes']/N:.4f}")

print("=" * 78)
print("[5] КОНСОЛЬ-КВАНТ: 5s vs 1m колонка одной строки (дивергенция)")
diffs = 0; gaps5 = 0; gaps1 = 0
for run in RUNS_5S:
    a, b = RUNS_5S[run], RUNS_1M[run]
    diffs += sum(1 for x, y in zip(a, b) if x != y)
    gaps5 += sum(1 for i in range(1, len(a)) if a[i] != a[i - 1])
    gaps1 += sum(1 for i in range(1, len(b)) if b[i] != b[i - 1])
print(f"    поллов пост-ramp: {n5}; 5s!=1m дивергенций: {diffs} ({diffs/n5:.1%})")
print(f"    флип соседних поллов: 5s {gaps5}/{n5-4} = {gaps5/(n5-4):.1%} | "
      f"1m {gaps1}/{n5-4} = {gaps1/(n5-4):.1%}")
for run in RUNS_5S:
    med5 = sorted(RUNS_5S[run])[len(RUNS_5S[run]) // 2]
    med1 = sorted(RUNS_1M[run])[len(RUNS_1M[run]) // 2]
    k1 = sum(1 for v in RUNS_1M[run] if v >= 0.4)
    nn = len(RUNS_1M[run])
    cls = "0.4-класс" if k1 >= nn // 2 + 1 else "0.3-класс"
    print(f"    {run:12s} медиана 5s={med5} 1m={med1}  1m-гейт(>=n/2+1): {cls} ({k1}/{nn})")

print("=" * 78)
print("[6] БАР-МАТЕМ (C06-цепочка): Δcpu(b)=1-b/(b+0.43)")
for b in [0.2, 0.3, 0.35, 0.4]:
    print(f"    база {b}: бар {100*(1 - b/(b + 0.43)):.1f}pp")
print(f"    зазор баров 0.3<->0.4: {100*(1 - 0.3/0.73) - 100*(1 - 0.4/0.83):.1f}pp = "
      f"{(100*(1 - 0.3/0.73) - 100*(1 - 0.4/0.83))/25:.3f} кванта (1q=25pp)")
print(f"    компо C100-цель 43.9^26.7 = {100*(1 - (1-0.439)*(1-0.267)):.2f}pp "
      f"— садится РОВНО на канон-бар 58.9: FN-окно [51.8,58.9) — зона компо-вердиктов")

print("=" * 78)
print("[7] ЗАКЛЮЧЕНИЕ-ЧИСЛА (канон 58.9 фиксирован):")
print(f"    p̂={p5:.3f} -> P(>=4/5)={qg:.3f}; ран-флип f={flip_rate(qg):.3f}; "
      f"min-of-3 f3={damped(qg)[1]:.4f}; 6-раундова инконсистентность {inconsistency(qg):.3f}")
print(f"    ран-уровень эмпирика: 1/4 ранов 0.4-класс -> q_emp~0.25 -> f={flip_rate(0.25):.3f} "
      f"(гетерогенность доминирует над полл-шумом)")
