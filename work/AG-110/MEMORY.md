# AG-110 MEMORY (≤15 строк)
1. javap жив: /tmp/jdk21/bin/javap (ensure_javap, restore AG-58); канон-jar 29,386,794B лежит в
   /home/z/rounds/ROUND-528/work/AG-48/pop275k_art/ — /tmp-харвесты сметает Д1-свип, ищи в rounds.
2. EntitySelector.class 15940B byte-канон: major 65, date 2025-12-11, 0 красти-сайтов.
3. Воронка селектора: ВСЕ entity-walk пути сходятся в addEntities @32/#297 (box) + @48/#300
   (no-box) — редирект 2 сайтов покрывает всё; игроки/UUID/name — отдельные пути, не трогать.
4. getResultLimit: order!=ORDER_ARBITRARY ⇒ Integer.MAX_VALUE — fast-path условие = resultLimit==1
   И order==ORDER_ARBITRARY (иначе лимит-семантика ломается).
5. type-дискриминатор = getfield #97 vs getstatic #79 ANY_TYPE, ctor @67-81 null-wrap.
6. Board-post: CAS через contents-API, assert len<=120, retry 409 (скрипт /tmp/ag110_*.py).
