# RECON-14 — absorb ZERO-CURSOR gate лега s7171 (run 35444005075, head 27ef9454f04c)

- PG-Z2: pop VALID, NCDFE=0, pristine=есть, bridges=есть, redirect=Retargeted{sites:1}, armed=rc=0, region_threads=ARMED, batch_collector=defined, OOM=нет -> **PASS**
- PG-Z3: TPS lines=5 median5=1.40 (гейт ≥1.60 -> FAIL); дельта vs банка 1.80 = -22.2% (LEVER-BOOST: <10% — лейн НЕ закрыт (v7))
- PG-Z4a: cursor-alloc лейн 22.96% (база 29.18%, samples=9047) дроп=+21.3% (гейт ≥−50% -> FAIL, ожидание −80..−95%); топ: net.minecraft.world.phys.AABB_[i]=746; net.minecraft.world.phys.Vec3_[i]=621; long[]_[i]=400; net.minecraft.core.BlockPos$$Lambda+0x00007fe6819e5e08_[i]=139; it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]=60
- PG-Z4b: young=142 (гейт ≤154), Full=0 -> **PASS**
- PG-Z4c: cursor-CPU лейн 7.24% (база 6.42%) дроп=-12.8% (гейт ≥−30% -> FAIL); ZeroCursor-фреймы CPU: 1.57%
- CRASH-FREE: Entity threw exception=2 (≤5) -> **PASS**
- ИТОГ гейтов: PG-Z2=PASS, PG-Z3=FAIL, PG-Z4a=FAIL, PG-Z4b=PASS, PG-Z4c=FAIL, CRASH-FREE=PASS
- **ВЕРДИКТ: FAIL → REFUTED + rollback zero_cursor=0 (мосты остаются инфраструктурой); следующий под-лейн ТОП-1 (Vec3/AABB travel-чейн или remset-драйвер)**
