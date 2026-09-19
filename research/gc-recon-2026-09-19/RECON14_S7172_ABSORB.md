# RECON-14 — absorb ZERO-CURSOR gate лега s7171 (run 35442145580, head 89d1df995945)

- PG-Z2: pop VALID, NCDFE=0, pristine=есть, bridges=есть, redirect=Retargeted{sites:1}, armed=НЕТ, region_threads=ARMED, batch_collector=defined, OOM=нет -> **FAIL**
- PG-Z3: TPS lines=6 median5=1.30 (гейт ≥1.60 -> FAIL); дельта vs банка 1.80 = -27.8% (LEVER-BOOST: <10% — лейн НЕ закрыт (v7))
- PG-Z4a: cursor-alloc лейн 27.72% (база 29.18%, samples=10143) дроп=+5.0% (гейт ≥−50% -> FAIL, ожидание −80..−95%); топ: net.minecraft.world.phys.AABB_[i]=736; net.minecraft.world.phys.Vec3_[i]=656; long[]_[i]=449; net.minecraft.core.BlockPos$6_[i]=334; java.util.ImmutableCollections$ListItr_[i]=166
- PG-Z4b: young=151 (гейт ≤154), Full=0 -> **PASS**
- PG-Z4c: cursor-CPU лейн 6.80% (база 6.42%) дроп=-5.9% (гейт ≥−30% -> FAIL); ZeroCursor-фреймы CPU: 0.00%
- CRASH-FREE: Entity threw exception=2 (≤5) -> **PASS**
- ИТОГ гейтов: PG-Z2=FAIL, PG-Z3=FAIL, PG-Z4a=FAIL, PG-Z4b=PASS, PG-Z4c=FAIL, CRASH-FREE=PASS
- **ВЕРДИКТ: FAIL → REFUTED + rollback zero_cursor=0 (мосты остаются инфраструктурой); следующий под-лейн ТОП-1 (Vec3/AABB travel-чейн или remset-драйвер)**
