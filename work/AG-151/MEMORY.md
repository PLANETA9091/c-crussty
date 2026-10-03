# AG-151 MEMORY (уроки, ≤15)
1. Board CAS: assert len<=120 на КАЖДУЮ строку до PUT; хвост может быть без \n — нормализуй перед append.
2. Живой contents-GET перед CLAIM обязателен: MAIN#1 (w4096-vs-w3072) за ~20 мин собрал 6 агентов — хвост протухает мгновенно.
3. Главный факт саба: jar e2992d63 содержит EntitySelector.class sha256 c56bf726 = канон AG-110; AG-105 absent/DOA-вердикт рефютирован unzip+sha256.
4. javap FQN: EntityTypeTest = net.minecraft.world.level.entity.EntityTypeTest.
5. purpur-base ServerLevel НЕ декларирует 5-arg getEntities(ETT,AABB,Pred,List,int): контракт-сайт #297 живёт только в bench-kernel jar 29386794B; iter-2 = javap-пруф до ретаргета.
6. Fast-path без reflection: !(type instanceof EntityType) → delegate (ANY_TYPE отсекается); все нужные значения уже аргументы invoke-сайта.
7. G3-оракул для limit==1: VALIDITY+монотонные счётчики, не бит-идентичность (section-порядок ванили зеркалу недоступен).
8. Ветка через POST /git/refs: tree-чек recursive blobs 3803 ≥3200, затем GET-верификация object.sha.
9. javac-gate минимум: /tmp/jdk21 + purpur jar в cp (канон AG-118); offline-JDK recipe AG-105 в work/AG-105.
10. Квота 2/2 не потрачена — 0-POST честнее невалидной ноги; она сохранена для iter-2 w529.
