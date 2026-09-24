# RESEARCH-A-439 — cmp438_sense: sensing+brain плоскость ЦЕЛИКОМ (TASK-438-A2 рестарт)

> Рестарт застрявшего TASK-438-A (ветка round-438-a-sense @bcfb18fb = 0 коммитов
> сверх master — реальный застой, подтверждено: worktree ROUND-438/agent-a
> содержит только untracked копию round-396-a jar). Дизайн TASK-438-A
> продолжен 1:1 (брифа мандат): сенсорный скан + goal-selector-приоритизация
> одним bulk-JNI/тик над SoA-снимком; паттерн-носитель = round-437-a-sscan2
> (MobScanOps/nearest-колонка/selfTest-до-ARM).

## 1. Lane-обоснование (закон-3 рисёрч; ЧЕСТНО)

Профили раунда-438 (research/gc-recon-2026-09-19/round-round-438-*/ABSORB.md +
BOTTLENECKS_3.md; raw collapsed пурджены локально, выжимки живы):

| слайс | источник | %wall |
|---|---|---|
| Sensing.tick (LOS-кэш мозгов) self | anchor-4 (vanilla master) top-40 leaf | **0.6%** |
| Brain.tick presence + tickEachRunningBehavior | anchor-4 F3-сплит | **0.81 + 0.20%** |
| GoalSelector.tick self (+tickRunningGoals) | ss-1 top-40 leaf | **0.6%** |
| goal/target broadphase (getEntitiesOfClass chokepoint) | RESEARCH-C-k3 (entity_query.rs docs) | **2.7–3.0%** |
| targeting-тесты (TargetingConditions.test: LOS-clip в collision-фреймах) | распорото по collision/clip — отдельно не меряется, ограничено сверху плоскостью AI | ~1–2% |
| Mob.serverAiStep subtree ЦЕЛИКОМ (sensing/targeting/goals/navigation/brain) | comp3 subtree attribution (mobs_ai.rs docs) | 9.73% |

**ВЕРДИКТ lane-гейта брифа: sensing-слайс ≈ 2.2% прямых self (Sensing+Brain+GoalSelector)
+ ~2.7–3.0% goalquery-chokepoint + targeting-тесты = ~5–7% wall суммарно — СЛАЙС ≥2%,
ПЕРЕКЛЮЧЕНИЕ НА ДРУГОЙ ЛИСТ НЕ ТРЕБУЕТСЯ** (порог <2% не пробит; honest-запись в RESULT.json).

**Потенциал**: СОЛО sense-плоскость (обнуление всей плоскости) < +20% — честно.
≥+20% достижим ТОЛЬКО композицией носителя: cmp438_sense = STRICT-UNION carrier
(cerтифицированное семейство cmp430_inside-эры: mobsoa⊕inside⊕colpush⊕chunk_parse⊕
goalquery(cmp410_eindexq)⊕goal-selector(GoalOps) — pair-stable эффект-линия +27.3%,
canon ×432) + НОВЫЙ sense-батч сверху. Это прецедент-паттерн эры (cmp424_mobfeed,
cmp430_inside носители; «certified-mega slice», «16 rust + 7 java gate sites, brain3 pattern»).

## 2. javap ground truth (patched-kernel.jar round-396-a, purpur-1.21.10, Mojang-mapped)

КЛЮЧЕВАЯ РАЗГАДКА: targeting-conditions nearest-picks НЕ на ServerLevel — это
DEFAULT-методы интерфейса **`net/minecraft/server/level/ServerEntityGetter`**
(ServerLevel его реализует, getNearest*/getNearestEntity НЕ переопределяет —
javap ServerLevel/Level: 0 вхождений):

```java
public interface ServerEntityGetter extends EntityGetter {
  public default Player getNearestPlayer(TargetingConditions, LivingEntity);                 // → getNearestEntity(List,...)
  public default Player getNearestPlayer(TargetingConditions, LivingEntity, double,double,double); // → getNearestEntity(List,...)
  public default Player getNearestPlayer(TargetingConditions, double,double,double);         // → getNearestEntity(List,...)
  public default <T> T getNearestEntity(Class, TC, LivingEntity, DDD, AABB);                 // → getEntitiesOfClass → getNearestEntity(List,...)
  public default LivingEntity getNearestEntity(TagKey, TC, LivingEntity, DDD, AABB);         // → getEntitiesOfClass → getNearestEntity(List,...)
  public default <T> T getNearestEntity(List<? extends T>, TC, LivingEntity, double,double,double); // ★ chokepoint
}
```

ВСЯ targeted/look-at/tempt-семейство сходится в ОДИН метод
`getNearestEntity(List, TargetingConditions, LivingEntity, x, eyeY, z)`:

- invokevirtual-сайты `ServerLevel.getNearestPlayer:(LTargetingConditions;LLivingEntity;DDD)`:
  NearestAttackableTargetGoal.findTarget @124 (hostile-таргетинг игрока — zombie/drowned/
  husk/skeleton/spider/creeper бенча), LookAtPlayerGoal @91 (почти все мобы), Panda @77,
  dragon-фазы ×4; 2-arg вариант (TemptGoal @48 — 3.4k chicken бенча, BegGoal, Bat,
  EndermanLook, Dolphin, PandaBreed) депегается в 5-arg → chokepoint;
- invokevirtual `ServerLevel.getNearestEntity(List,...)` @84 в NearestAttackableTargetGoal
  (non-player ветка) — ТОТ ЖЕ chokepoint;
- тело chokepoint (javap -c, байт-в-байт):
  `d=-1.0; best=null; for c in list: if (!conditions.test((ServerLevel)this, target, c)) continue;
  e=c.distanceToSqr(x,y,z); if (d==-1.0 || e<d) {d=e; best=c;} return best;`
- TargetingConditions.test(ServerLevel, LivingEntity, LivingEntity) — чистый предикат
  (range/invisibility/idleTimeout/selector/LOS-clip; RandomSource НЕ потребляет) →
  переупорядочивание тестов кандидатов не меняет решений (закон 4).

## 3. Дизайн cmp438_sense (закон 6: ОДИН bulk-JNI/тик; закон 4: ваниль бит-в-байт)

1. **BODY-SWAP chokepoint**: `ServerEntityGetter.getNearestEntity(List;TC;LE;DDD)` →
   14-байт straight-line `aload_0..3; dload 4/6/8; invokestatic SenseOps.nearestEntityGate; areturn`
   (прецедент patch_brain_start_each; без ветвлений ⇒ пустой StackMapTable; idempotent).
2. **SenseOps.nearestEntityGate** (java, ECJ, без indy):
   - fail-closed лестница → байт-точная ваниль-реплика тела (итератор + test + лестница);
   - fast-path: best-first по rust-колонке (identity-guard: entities[i]==SNAPSHOT[i] O(n))
     + ТОЧНАЯ дистанция-обрезка свипа: `d>=bestD → тест пропущен` — кандидат с e ≥ best
     НЕ МОЖЕТ выиграть лестницу `(d==-1.0||e<d)` ⇒ решение = min-dist passing с
     tie-keep-earlier, БИТ-В-БАЙТ равное ванили при ЛЮБОЙ свежести rust-снапшота
     (stale-порядок деградирует только ВЫИГРЫШ, не корректность);
   - EFFECT-маркер `sense EFFECT first gate hit tick N, sensed:M` (AtomicLong, one-shot).
3. **sense_epoch (rust, mobs_sense.rs)**: ОДИН bulk-JNI/тик — DOD-проход по
   `mobs_soa::sscan_snapshot()` (flat x/y/z, seqlock global-version, bounded retry —
   паттерн sscanEpoch) пишет `nearest[denseId] = argmin dist² | -1` (ties → младший
   индекс players() порядка). Игроки — flat [x,y,z]×n f64 снапшот java-стороны
   (БЕЗ предиката: test применяется java-стороной к каждому кандидату как в ванили).
4. **selfTest==true ДО ARM** (TASK-437-A паттерн): probe-magic + vanilla-pick oracle
   на локальном ref только что define_class-нутого SenseOps; false → dormant.
5. **STRICT-UNION носитель**: `|| f == "cmp438_sense"` добавлен ко ВСЕМ гейт-сайтам
   семейства (rust 16 файлов + java 8 файлов + check_blobs_sync маркеры +
   run_world3.sh arming case). Пустой/чужой флаг = ваниль бит-в-байт (ничего не
   регистрируется, класс-байты идентичны).
6. **ARM/EFFECT маркеры**: `[crussty-plugin] cmp438_sense: sense armed (...)` rust +
   `sense EFFECT first gate hit tick N, sensed:M` java — вердикты только по ним
   (урок ×409/PROFILE-B2), stdout-капчер обязателен в диспатчере (урок ×437/438).

## 4. Гейты локальной верификации

- javac --release 21 (major 65), cp = kernel round-396-a + fastutil + paper-api +
  adventure (+ entityinside/mobpush build для межбриджных ref — один javac-проход);
- build_430b_blobs.sh (nested+flat, flat==nested) + check_blobs_sync.sh (маркеры
  cmp438_sense во всех семействых блобах + SenseOps native decl);
- javap: 14-байт body-swap в ServerEntityGetter (контраст 252→~в-) + SenseOps gate desc;
- cargo CARGO_TARGET_DIR=/tmp/cargo-439a check --lib + test (target удаляется СРАЗУ);
- py_compile dispatch_439a_sense.py; py-диспатчер BANKED (запуск только main/golden).

## 5. Виндикация скоупа (что НЕ входит — честно)

- Sensing.tick/Brain-сенсоры LOS-матрицы — НЕ батчится (rust не имеет chunk-данных
  для clip-рейкастов; LOS остаётся java, экономится КОЛИЧЕСТВО тестов);
- Brain.tick behaviors / navigation / moveControl — вне sense-среза (цена — nav_plane
  семейство, отдельные гейты);
- getEntitiesOfClass enumeration — УЖЕ в семействе (cmp410_eindexq, чойкпоинт K3);
  sense-плоскость НЕ дублирует его, а добавляет nearest-pick слой СВЕРХУ.
