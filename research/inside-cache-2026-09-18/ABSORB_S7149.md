# ABSORB S7-149 — ALLOC-DIET leg (ран 35328228929): REFUTED (повторное подтверждение S7-133b на валидной базе) — дефолт 0 подтверждён

Дата: 2026-09-18 17:3x +08. Job 394666. Агент agent-7625532f.

## 1. Валидность A/B

Ран 35328228929 (head 59b6bbb, alloc_diet=1, все остальные 0, fp4/300s/150k/
seed42/xmx10G/guard1), SUCCESS 09:11:24→09:27:04 UTC (~16 мин). База = base-b.

- Конфиг точен; INJECT VALID; популяция **148321/148314/148103** стабильна ✓
- **0 NoClassDefFoundError / 0 исключений** ✓
- ALLOC-DIET ARMED живьём: defined EntityQueryOps (kernel loader) → computed
  LivingEntity 186570→186759 (Retargeted{1}) + CollisionUtil 45439→45546
  (Retargeted{1}) → hook serve оба ✓ — механика мостов работает end-to-end
- patched-kernel = e2992d63 (байт-в-бит) ✓

## 2. Гейты (dispatch_s7134 preregistered): FAIL ⇒ REFUTED

| Метрика | base-b | ALLOC-DIET | Вердикт |
|---|---|---|---|
| movement-geom (alloc) | 1603 (22.24%) | 1822 (24.25%) | ❌ +13.7% (вырос) |
| Vec3 leaf | 1388 | 1468 | ❌ +5.8% |
| AABB leaf | 1290 | 1257 | ~шум (−2.6%) |
| inside-blocks | 2915 | 3087 | +5.9% (шум, изолированная нога) |
| flushStep | 299 | 308 | +3% (шум, flush_diet=0) |
| young GC | 125 | 142 | ❌ +13.6% (гейт S7-133 «GC ↓≥10%» провален) |
| TPS | 0.8–0.9 | 0.6–0.7 | ❌ ~−12% |
| entity-фаза | 59.7% | 53.8% | ✅ формально |

Эффект диеты (2 ArrayList на push ~45k/тик + MutableBlockPos ctor ~250k
запросов/тик) НЕ виден ни в одном листе: pushables-ретаргет снял мёртвые
ArrayList (не видны в leaves — они были под movement-семьёй?), но семьи не
упали; total alloc +4.2%; young GC +13.6%; TPS −12% (ретаргет-мосты добавляют
invokestatic-оверхед на push/collision горячих путях без экономической
компенсации).

## 3. Вердикт

**ALLOC-DIET = REFUTED (дефолт 0 подтверждён)** — повторное подтверждение
S7-133b (leg #1: high-water −20%, GC-лейн +21%) уже на валидной базе base-b
с чистым протоколом S7-148 (0 NCDFE, популяция стабильна, ARMED полна):
мосты корректны и живы, экономика отрицательная. Код банкуется (в силе
OFFLINE PASS, 101 тест, харнесс), wave-2 диеты (LazyEntityCollisionContext)
НЕ продолжается — экономика подтверждена дважды.

## 4. Итог конвейера рычагов эры (base-b-эпоха, валидные A/B)

| Рычаг | Вердикт | Основание |
|---|---|---|
| INSIDE-CACHE (S7-135/136) | **GREEN-BY-SAFETY** | alloc-диета −32.5%/−35.9%, entity-фаза −5pp, TPS нейтрально, 0 NCDFE (leg2''' 35322530537) |
| FLUSH-DIET (S7-137) | **GREEN** | flushStep −100%, Object[] −51.2%, TPS нейтрально (35324517090) |
| FLUID-FREE+DEMUX (S7-139/143) | **REFUTED-BY-ECONOMICS** | fluid-лейн 1.8% CPU = микро-класс; fluid-get не снят; TPS −20% от демукса (35326295881) |
| ALLOC-DIET (S7-133) | **REFUTED** | семьи не упали (+3..14%), GC +13.6%, TPS −12%; повтор S7-133b (35328228929) |

Накопительный ран: **inside_cache=1 + flush_diet=1** (два зелёных), диспатчен
(35330129145, head dbb5e8e, 09:33:13 UTC) → absorb → итоговый вердикт эры по
комбинации.

## 5. Артефакты

- run-s7149-allocdiet/: BOTTLENECKS_3.md, alloc-collapsed.txt (f), gc.log,
  run-env.txt, sha256_allocdiet_extras.txt — в git.
- dispatch_s7149b.py (накопительный ран) забанкован.
