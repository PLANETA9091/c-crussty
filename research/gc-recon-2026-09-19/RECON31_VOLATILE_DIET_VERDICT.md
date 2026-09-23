# RECON-31 — дрилл volatile-лейна (NEXT 354) + вердикт volatile-диеты

## s7194: lane-итоги (сцена 128104)

| домен | сэмплов | % сцены |
|---|---|---|
| chunk-map-concurrent | 1947 | 1.52% |
| synced-entity-data | 2481 | 1.94% |
| other-volatile | 69 | 0.05% |

### SynchedEntityData под-сайты

-  1.08%   1380 getValue
-  0.78%   1000 getItem
-  0.07%     85 packDirty
-  0.01%      9 sendDirtyEntityData
-  0.00%      6 ?
-  0.00%      1 getAirSupply

## s7189: lane-итоги (сцена 127150)

| домен | сэмплов | % сцены |
|---|---|---|
| chunk-map-concurrent | 1710 | 1.34% |
| synced-entity-data | 2546 | 2.00% |
| other-volatile | 65 | 0.05% |

### SynchedEntityData под-сайты

-  1.06%   1348 getValue
-  0.85%   1082 getItem
-  0.07%     91 packDirty
-  0.01%     15 sendDirtyEntityData
-  0.01%      9 ?
-  0.00%      1 getAirSupply

## ВЕРДИКТ (модель потолка volatile-диеты)

1. Лейн 3.51/3.40% сцены распадается на две семьи (кросс-раннер стабильно):
   - synced-entity-data 1.94/2.00% сцены: доступ к SynchedEntityData
     (getItem/get/getValue — javap-контракт: itemsById[accessor.id()] AALOAD
     → DataItem.value GETFIELD — ПУТЬ ПОЛНОСТЬЮ PLAIN, НИ ОДНОЙ volatile-
     инструкции; волатильность у DataItem отсутствует и в поле value);
     sub-сайты: generic getValue 1.06/1.08%, getItem 0.78/0.85%,
     packDirty/sendDirtyEntityData ~0.08% — tracker-семейство;
   - chunk-map-concurrent 1.52/1.34% сцены: ConcurrentLong2ReferenceChainedHashTable
     ($TableEntry.getValueVolatile guard_L_L + getAtIndexAcquire guard_LI_L)
     — конкурентная chunk-карта ServerChunkCache (getChunk/getChunkNow/
     getChunkHolder/getChunkAtIfLoadedImmediately) — часть ИНФРАСТРУКТУРЫ
     region-threads (компонент БАНКА v3): регион-воркеры читают chunk-карту
     конкурентно, acquire/volatile-семантика обязательна; диета = атака на
     собственный банк;
   - other-volatile 0.05% — шум.
2. ВЫВОД-КОРРЕКТИРОВКА: «volatile-диета» на этой сцене НЕ ИМЕЕТ ПРЕДМЕТА:
   (a) sync-семья уже plain — убирать нечего, любая выгода = только
   редизайн двухуровневой косвенности itemsById→DataItem.value (структурный
   редизайн SynchedEntityData ради ≤2% = микро-зона, запрещён);
   (b) chunk-семья конкурентна по необходимости (банк v3) — снятие
   acquire/volatile ломает thread-safety чтения чанков воркерами.
   Честный потолок диеты ≈ 0-1% сцены даже при частичной легальности.
3. Hard ceiling лейна 3.4-3.5% < ДВОЙНОГО БАРА +10%; обе семьи залочены
   (plain-по-конструкции / конкурентность-по-необходимости) →
   **volatile-диета = 6-я ДОК-ВЕРИФИКАЦИЯ закрытия лейна (прецедент
   RECON-17/20/23/26/30), патчер не реализуется**.
4. Для эскалации: компонент A «volatile-диета ~3.4%» реально стоит ~0-1%
   → потенциал ЭПОХИ-2 пересчитан: 10-13% → **~9-12% сцены** (верхняя
   граница, TPS-конверсия по эмпирике ~0). Рекомендация B/C усилена.
