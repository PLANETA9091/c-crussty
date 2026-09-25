# RESEARCH-459-P34 — INSIDE-QUANTUM-GATE: класс «квант-покой» |Δ|<ε за K тиков (ID-P34, TASK-459-74, закон 11 тик-459)

Lever: `cmp459_p34` (STRICT eq; env `CRUSSTY_INSIDE_QUANTUM`; DORMANT) · carrier: inside_volatile 16.6% wall (inside_cache static-gate семья, cmp424_inside)
Прогноз (карточка ID-P34): подвижные-но-медленные ~30-50% движущихся → **захват 2-4пп из 16.6**; потолок честного захвата при 100%-м memо медленных + бесплатном replay ≤ 8.3пп (16.6 × 0.5) — до потолка далеко, оракул решает.
Статус: **scaffold v1 DORMANT** — референс-модель классификатора (Rust + JDK-only Java-зеркало, selfTest-канон), НОЛЬ байт-хуков, НОЛЬ define_class, НОЛЬ ретаргетов; vanilla bytes НЕ тронуты.

## 1. Механика (что положено в scaffold)

Существующий static-гейт inside_cache (S7-135) обслуживает ТОЛЬКО бит-неподвижные сущности:
`deltaMovement==0 && позиция бит-равна кэшу`. P34 добавляет **класс «квант-покой»**:

| элемент | v1 (этот scaffold) | v2 (после оффлайн-оракула 10k-сцен) |
|---|---|---|
| классификатор | фазовая машина VANILLA→DWELL→REST: `0 < |Δ| < ε` K тиков подряд → SERVE; один скачок `|Δ| ≥ ε` → деклассификация + 1 обязательный ванильный тик (гистерезис) | ε/K пере-выводятся оракулом (константы оффлайн-оракула — карточка); таблица порогов по типам сущностей |
| владение | ДИЗЪЮНКТНО с static-гейтом: точный ноль Δ (биты) → `Decision::Static` (inside_cache); квант-класс = строго-движущиеся 0<|Δ|<ε | без изменений |
| replay | инкрементальный: pending-сегменты (anchor → текущая позиция, бит-точный ре-анкор), флеш на каждом SERVE; overflow > MAXSTEPS → one-shot DISARM навсегда ваниль | флет-слоты InsideBlockOps + JNI bulk-wiring; replay = бит-в-байт (тот же контракт, что static-гейт) |
| фоллбэк | чистая ваниль (fail-closed) | без изменений |

Незыблемые контракты (javap-canon InsideBlockOps.java): moved-порог ванили
`distanceToSqr(from,to) > square(0.9999900000002526)` ≈ 0.99999 бл/тик; ε=1.0e-4 (v1-дефолт)
лежит на 4 порядка ниже — классы не пересекаются с ванильной moved-полосой; budget визитора
16 (MAXSTEPS); deflate 9.999999747378752E-6; слот = eid & (NSLOTS-1), NSLOTS=2^18 (TASK-432-B:
150k живых полностью в мемо), штамп = eid.

## 2. Риск-гейт карточки (почему DORMANT)

«ε-критерий меняет тайминг stateful-эффектов»: freeze/fire-каденс зависит от per-tick
последовательности effect-вызовов. Классификатор продвигает сущность в REST через K тиков
ванили (подмножество — безопасно), но steady-state REST обязан воспроизводить
последовательность вызовов бит-в-байт по накопленному дрейфу — это доказывается ТОЛЬКО
оффлайн-оракулом (10k-сцен lockstep), поэтому класс запрещено армить до оракула (ARMED=false
в Java-зеркале закреплено инвариантом selfTest).

## 3. Web-рисёрч (5 источников, все 200-верифицированы; механика → как легла в дизайн)

1. **https://github.com/CaffeineMC/lithium/issues/125** («Entity suffocation optimizations in
   Lithium 0.5.3», источник карточки) — прецедент лейна: внутри-блочные (suffocation/
   entityInside) эффект-вызовы сущностей — отдельная оптимизируемая плоскость с контрактами
   вызовов, а не результатов. → Дизайн replay «кэшируются ВЫЗОВЫ, не результаты»
   (bit-in-byte транскрипция effect-вызовов), тот же контракт, что static-гейт.
2. **https://github.com/CaffeineMC/lithium/blob/develop/lithium-neoforge-mixin-config.md** —
   «Entity movement uses optimized block access and optimized and delayed entity access. Use
   block listening system to allow skipping stuff in entity code». → Прецедент гейтирования
   inside-discovery по движению: movement-валидность как ворота до полной геометрии —
   наш dwell-гейт тот же паттерн с K-тик-памятью.
3. **https://en.wikipedia.org/wiki/Schmitt_trigger** — канон гистерезиса: ДВЕ раздельные
   пороговые точки + dead band исключают «дребезг» на границе. → Форма классификатора:
   продвижение требует K подряд sub-ε тиков (вход-порог), деклассификация — один скачок ≥ε
   плюс обязательный ванильный hold-тик (выход-порог) — никаких промо/демо-осцилляций
   на границе ε.
4. **https://martinfowler.com/eaaDev/EventSourcing.html** — канон replay: состояние
   восстанавливается переигрыванием событий с детерминированным порядком. → Инкрементальный
   replay: anchor-позиция (бит-точная) + список накопленных микросегментов; флеш по SERVE
   переигрывает ТОЛЬКО недоигранное (инкрементальность), порядок сегментов = порядок тиков.
5. **https://minecraft.wiki/w/Entity** (Entity §Motion; curl 403 = bot-защита, страница живая)
   — физика сущностей покомпонентна (позиция/скорость/deltaMovement double-биты). →
   Бит-точные сравнения |Δ|² (raw f64 bits, никакой tolerate-математики) и владение по
   точному нулю deltaMovement — соответствие v1-контракту static-гейта.

## 4. javap/профильные числа (глубина ≥5, канон round-458/459 inside-семьи)

1. moved-порог: `square(0.9999900000002526)` (javap checkInsideBlocks, intersected-флаг,
   distanceToSqr(from,to)) — квант-полоса ε=1.0e-4 на 4 порядка ниже.
2. Бюджет визитора 16 позиций (javap 9-16 → InsideBlockOps.MAXSTEPS=16) — overflow-дискriminator
   disarm-латча.
3. Deflate-бокс 9.999999747378752E-6 (javap #2462 inner checkInsideBlocks) — ширина
   replay-коридора сегмента.
4. X150K alloc-ценз (run 35275967738, §155): 66% young-gen чёрна = movement-геометрия +
   inside: LongOpenHashSet 6.1% (dedup на сущность на тик), BlockPos$6 corner-lambda 5.2%,
   flushStep Arrays.copyOf 4.6%, PalettedContainer.get 3.1% + readPalette 0.8% — всё это
   платится медленными движущимися КАЖДЫЙ тик.
5. Статичная сущность исполняет ДВЕ полных traversal за тик (main from-to + финальный to-to
   с бюджетом 1) — вторая даёт только visitedBlocks-дюпы; медленный движущийся платит то же
   множество + swept-геометрию.
6. Лейн inside_volatile = 16.6% wall (карточка/RECON); TASK-432-B: 2^18 слотов покрывают
   150k сущностей, ~13% вечнованильных при 2^17 — ёмкость флет-машериири подтверждена.

## 5. Capture-матем

Лейн 16.6% wall. Медленные-но-движущиеся ~30-50% движущейся популяции (карточка); их вклад в
лейн ≈ пропорционален полному discovery за тик. SERVE после K-тик dwell снимает обе traversal +
визитор + dedup, оставляя инкрементальный replay ~O(pending×стоимость-вызова) вместо
O(2×traversal). Прогноз: **+2-4пп** (карточка). Потолок: 16.6 × 50% × (1 − replay-доля) — при
replay-доле 30% ≈ 5.8пп; честный потолок при консервативных долях ≈ 4-6пп. Не merge-бар
(+20) — вектор-компонент к климбу P31+P32+P36 (канон 13c).

## 6. Parity-план + preregistered гейты

- **G1 (oracle)**: офлайн lockstep 10k сцен (slow-mover популяция: jitter-in-place,
  crowd-pressed, gravity-settling) — транскрипция effect-вызовов бит-в-байт vs ваниль;
  расхождение любой сцены → класс не армится.
- **G2 (arm)**: lever `cmp459_p34` STRICT eq + env CRUSSTY_INSIDE_QUANTUM; STRICT-off до
  оракула; ARMED=false инвариант в Java-зеркале.
- **G3 (bench)**: world-bench-parallel якорь-канон inputs, lever='' (этот тик: dormant =
  корректный A-arm); после оракула — lever-нога A/B min-of-2/3, GC-debt monotone.
- **G4 (latch)**: 0 disarm-событий на оракульном множестве (overflow-free ε/K); любой
  disarm → константы бракуются.
- **G5 (population)**: популяция-паритет + TPS-бар (канон S7-150), young/Full GC без регрессий.

## 7. Дельта-файлы (scaffold v1)

- `src/inside_quantum_gate.rs` — референс-модель: ε-полоса бит-строго, фазовая машина,
  гистерезис, дизъюнкт с static-гейтом, инкрементальный replay-план, one-shot disarm,
  флет-слоты 2^18; 8 unit-тестов; register/activate dormant-канон inside_cache.rs.
- `entityinside/quantum/net/minecraft/world/entity/InsideBlockOps.java` — JDK-only
  Java-зеркало (FQCN без клэша, прецедент P43 `…ai.flat.BrainOps`), selfTest-main exit(1);
  NCDFE-канон задокумечен (v1 никогда не define_class'ится).
- `scripts/build_inside_quantum_ops.sh` — javac --release 21 (kernel JVM major 65) +
  прогон selfTest.
- `src/lib.rs` — wiring register/activate (no-op при выключенном гейте).
