# AG-248 w530 — snapreg java pre-gate iter-3 (v24-2)
Гипотеза: P36 pre-gate в HEAD InsideSnapOps.snapGet (site 1) замыкает java-ногу связки P32+P36 с rust decision-core serve_gate (iter-2, merge 0bc291ac): per-thread warm slot на lanes[0], ОДИН long-cmp pregEpoch vs Snap.gen ДО serve/serve4, ANY doubt → exact legacy continuation.
Ожидание: связка +1.5..2.5пп тика (P32: −40-60% CHM-части → +0.8-1.2пп; P36: +0.5-1пп); вклад iter-3 = site-1 fail-closed fast-gate (site-2 secWrite и site-3 Entity.lambda не тронуты).
Сделал:
- entityinside/net/minecraft/world/entity/InsideSnapOps.java +144 стр: PREGATE (static volatile, default false = byte-for-byte dormant), pregate() slot-serve (guards ref/int + ОДИН long-cmp + long builtAtGen==gen anchor, SAME BlockState object = token identity), pregStamp() на 2 fresh-serve точках serve4, STAT_PREGATE/pregateHits(); ThreadLocal plain new сохранён, <clinit> indy=0 (javap).
- entityinside/net/minecraft/world/entity/InsideSnapRegistryOps.java +11 стр: arm() флипает InsideSnapOps.PREGATE после define-order+selfTest; lever off → класс не дефайнится → pre-gate инертен.
- src/inside_snap_registry.rs +44 стр: тест pregate_site1_slot_epoch_mirror (warm slot → flat-serve == serve_chm same-object на 0/1/4095; sec_write_bump stale → fail-closed full path; re-warm → flat).
- блобы пересобраны ПИННЕННЫМ scripts/build_459_p32_blobs.sh: 8 .class (nested+flat, flat==nested OK), rebuild байт-воспроизводим; javap: snapGet descriptor/receiver-first unchanged, HEAD = PREGATE&&ARMED→pregate.
Тесты: cargo test inside_snap_registry = 6 passed / 0 failed (5 старых + 1 новый); cargo check --workspace = 0 err.
Итог: гейт lever НЕ расширен (STRICT cmp459_snapreg + carrier-compo как в iter-2); PREGATE=false до arm() → ванильный путь байт-в-байт; miss/stale/bounds/cap/negative → serve/serve4 → ваниль (FAIL-CLOSED).
iter-4 handoff: парная сертификация малым same-boot A/B (pregateHits() vs hits() coverage), STRICT-on оракул, hit-инвариант N>=3 позиций при арме, include_bytes пикап в wiring-фазе.
