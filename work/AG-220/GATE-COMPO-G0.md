# AG-220 w528 — compo-G0 anti-placebo gate (0-POST, offline)

CLAIM: b37bd531 (доска). Ячейка: pre-POST гейт для любого re-fire lever=cmp528_compo
(AG-236/202/221/235 фикс-лейны). Против плацебо-класса navmath-1 / compo-canary-1.

## Канарейка-смерть (run 37107843533, swarm-528-95, step8 rust-build, 08:57Z)
- CI: E0308 src/sb_r1.rs:84:67 + E0425 src/sb_r1.rs:598:8 (job 111159760957).
- Blob swarm-528-95: 07ec548a (40606B, sha256 afa31bd5b566efc4) — скачан contents-API.

## ROOT (байт-пруф)
- L84+L85 — ДУБЛЬ сигнатуры `pub fn r1_enabled_with(...) -> bool {`.
- Следствие: тело L84 = вложенные items (L85-113) → fn возвращает () (E0308),
  `fn sbarm_selected()` L93-98nested → не виден activate() L598 (E0425),
  `}` L113 закрывает r1_enabled_with, а не armed_retarget_order.
- master sb_r1 (локальная копия 40531B, sha256 6a808e3892255853) — чистый.

## GATE (воспроизведён offline, sandbox /tmp/ag220_gate)
- Cargo 1.99.0 (AG-172 deploy). Крейт-скелет: Cargo.toml+lock, src/, cplug-abi/sdk,
  ВСЕ 25 */build*/ dirs (include_bytes .class) — `ls -d */build */build-*`.
- RED  (blob 07ec548a as-is): ровно 2 err E0308@84 E0425@598 == CI. Пруф /tmp/ag220_red.log
- FAIL -1L (только дубль): `unexpected closing delimiter }` — все еще RED.
- GREEN -2L (del L85-дубль + del L113-stray): `Finished dev in 11.21s`, 0 err.
- Время: warm-check 11s — гейт дешевле одной очереди canary на часы.

## G0-ЧЕКЛИСТ (обязателен ДО любого POST lever=cmp528_compo)
- G0-R cargo check --lib = 0 err (11s offline, скелет выше; rust 1.99).
- G0-J javac-21 SelectorBulkOps/bulkjni 0 err + .class в bench-пути (AG-176 FACT:
  bench НЕ собирает bulkjni — placebo-плечо; рецепт AG-176/105; лейны AG-221/235).
- G0-CP javap -v собранных блобов: CP-хит cmp528_compo в sb_r1+SelectorBulkOps
  (канон Л115/Л122: нет CP-хита = плацебо A/A гарантирован).
- G0-D DORMANT-статика master не тронута (AG-119 4-way mirror/arms/mods/lever&&arm=1).
- G0-L lever в inputs ВЕРИФИЦИРОВАН до запуска (navmath-1: ARM-маркер 0 = A/A).

## Вердикт-число
- 2 (структурные строки до GREEN); 11.21s (warm cargo-check); 0 POST.
