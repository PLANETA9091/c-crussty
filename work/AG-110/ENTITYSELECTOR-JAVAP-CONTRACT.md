# AG-110 w528 — JAVAP-КОНТРАКТ EntitySelector (iter-2 AG-76) — 2026-10-03T07:5xZ
Механизм ДО POST (канон navmath-1). Инструмент: /tmp/jdk21/bin/javap 21.0.12.1 (restore AG-58).
Источник: patched-kernel.jar 29,386,794B = ТОЧНЫЙ ledger-канон (AG-48 w528 арт pop275k_art,
Oct 2 15:14) → net/minecraft/commands/arguments/selector/EntitySelector.class = **15940B = ТОЧНЫЙ
канон AG-76**; major 65, minor 0, class-date 2025-12-11 (ванильная дата сборки — класс НЕ тронут
красти-патчами; кореллят: src/classfile.rs 0 EntitySelector-сайтов, AG-76).

## CP-карта полей (put/get-field index → поле)
- #57 maxResults:I            #59 includesEntities:Z       #61 worldLimited:Z
- #63 contextFreePredicates:Ljava/util/List;               #65 range:MinMaxBounds$Doubles
- #67 position:Function       #69 aabb:AABB                #71 order:BiConsumer
- #73 currentEntity:Z         #75 playerName:String        #77 entityUUID:UUID
- #97 type:EntityTypeTest     #99 usesSelector:Z           #79 ANY_TYPE:EntityTypeTest (Fieldref #2.#78)
- КТОР: type = (param12==null ? ANY_TYPE : param12) — getstatic #79 @73 / putfield #97 @81.

## getResultLimit() (20B код): order==ORDER_ARBITRARY(#306) ? maxResults(#57) : Integer.MAX_VALUE
## ТОЧКА РЕДИРЕКТА — addEntities (private, код 52B, stack=6 locals=6):
  0 aload_0; 1 invokevirtual #293 getResultLimit:()I; 4 istore 5
  6 aload_1; 7 invokeinterface #154 List.size:()I; 12 iload 5; 14 if_icmpge 51
  17 aload_3(box); 18 ifnull 38
  box-ветка: 21 aload_2(level); 23 getfield #97 type; 26 aload_3; 27 aload 4(pred); 29 aload_1;
             30 iload 5; 32 invokevirtual #297 ServerLevel.getEntities:(EntityTypeTest,AABB,Predicate,List,I)V
  no-box:    38 aload_2; 40 getfield #97; 43 aload 4; 45 aload_1; 46 iload 5;
             48 invokevirtual #300 ServerLevel.getEntities:(EntityTypeTest,Predicate,List,I)V
  51 return; StackMapTable 2 entries; LVT: entities/level/box/predicate/resultLimit
## ВОРОНКА (все getEntities-сайты класса = только эти 2; обходы игроков/UUID — отдельные пути):
  findEntities: @2 checkPermissions; @6 !includesEntities → findPlayers; @19 playerName → byName;
  @56 UUID → ServerLevel.getEntity(UUID); @137+ position/aabb/worldLimited → invokevirtual #271
  addEntities ×2 (офсеты 249 и 299 в findEntities). findSingleEntity @7 → findEntities(#145).
## PATCH-SPEC (1 java ops + rust chains поверх SYNC-инфры AG-76):
  1) Новый EntitySelectorOps: body-redirect invoke #297 @32 и #300 @48 в addEntities →
     fast-path при (getfield #97 != ANY_TYPE #79 ∧ resultLimit==1), иначе ванильный delegate
     (superset-parity; предикат применяется ПОСЛЕ индексного фетча — та же семантика фильтра).
  2) Rust: per-type цепочки (type-byte в слот + цепочки) поверх существующих noteAdd/noteRemove/
     noteMove событий eindex-зеркала — НОВЫХ java note-сайтов НЕ нужно (AG-76 reuse-карта).
  3) ПОРЯДОК-СТЕНА (Л58/Л146): перечисление только порядок-сохраняющее (per-type цепочки в
     E-порядке); NO-CACHE-инвариант RECON-39/40 — счётчики монотонные fail-dominant.
## КАПЧУР-МАТЕМАТИКА (AG-19): dp50k 11.6-16.9% CPU × капчур 70-95% = +8.1-16.1пп CPU →
  TPS@dp50k 3.4 → 3.7-4.0 (+9-16% rel); pop150k 30.5-57.6пп (коллапс-класс AG-48/50).
## ГЕЙТЫ: наследуются от prereg clm/AG-19 (parity lockstep ≥20k ops бит-в-байт, ARM index-hit,
  NCDFE=0, AIOOBE=0, threw=0, items-gate, FIXTURE-VALIDITY, min-of-3, same-boot A/B 2-in-1-job
  clm/AG-210, банк-канон 300s/fp4/gc3/seed42/10G/xms4G/r640/ic1/fd1/rt4/bc1, band [6.0,9.5]M).
