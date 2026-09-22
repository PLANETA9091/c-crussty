# ROOT-CAUSE cmp420_colpush NCDFE ×1902 (ноги cpa/cpb/cpc, 2026-09-23 05:2x +08)

## Симптом
Все 3 ноги: tps 1.40-1.65 (база якорей 2.30-2.40), NCDFE 3270-3804/ран, GC 48-65s (×2 нормы), DELIVERY-FAIL.

## Root-cause (log-строки round-colpusha/server-stdout.log)
- стр.1128 @20:42:45: ПЕРВЫЙ `RegionTickOps.bulkTick threw java.lang.NoClassDefFoundError: net/minecraft/world/entity/ColpushOps`
- стр.1241: `defined net/minecraft/world/entity/ColpushOps in kernel loader` — ПОЗЖЕ первого вызова
- стр.1246: `PATCHED LivingEntity.pushEntities` (define+patch порядок для LivingEntity правильный)
- ⇒ RegionTickOps-bulkTick-хук заармлен ДО define моста ColpushOps; HotSpot кэширует провал
  резолюции CP-сайта → NCDFE навсегда (1902×), каждый вызов = исключение+vanilla-fallback → коллапс.

## Блобы/патчи НЕ виноваты
- include_bytes! colpush/build/.../ColpushOps.class присутствовал (сборка зелёная)
- define состоялся (маркер ×1), RegisterNatives/ARMED ×6 — но ПОЗЖЕ первого вызова

## ФИКС-МАНДАТ (тик-420)
1. Порядок: define_bridge(ColpushOps)+selfTest ДО арма RegionTickOps.bulkTick (паттерн lib.rs
   mod+register(last)+activate(last) как у queryplane)
2. Belt-and-braces: one-shot NCDFE-guard в ColpushOps.bulkTick — поймал → eprintln ×1 →
   vanilla-хвост → DISARM lever (без 1902-шторма)
3. javap-гейт flat==nested + количественный гейт: grep -c NCDFE == 0 в абсорбе до вердикта
