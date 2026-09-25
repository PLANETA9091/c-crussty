# JAVAP P31/P32 — ground-truth транскрипция (TASK-459-91, инфра-агент)

* 2026-09-25 16:56 | **SOURCE=JAR**: `research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar`
  (канон javac-cp, маркер сохранения eb027a01/ABSORB; 29,386,794 байт, классы 2025-12-11).
* javap: `/home/z/tools/jdk-21.0.12.1+1/bin/javap` (21.0.12.1), команда: `javap -p -c -cp <jar> <cls>`.
* Классы: `net.minecraft.world.entity.Entity` (внутри-entity inside-вызовы),
  `net.minecraft.world.level.chunk.PalettedContainer` (get), `net.minecraft.util.SimpleBitStorage` (get).
* Полные аннотированные дампы: `research/javap_dumps/{entity,paletted,simplebit}.txt` (9 / 48 / 15 методов).

## Сводная таблица: метод → insns → N invoke\* → релевантность

### P31-батч (INSIDE-BATCH bulk-JNI, BLACKBOARD row 56, branch round-459-p31)

| метод | insns | invoke\* (st/v/sp/i/d) | релевантность P31 |
|---|---|---|---|
| `protected void applyEffectsFromBlocks();` | 58 | **16** (st=0/v=7/sp=2/i=7/d=0) | P31-вход: тик-путь Entity (baseTick) → собирает Movement-батч |
| `public void applyEffectsFromBlocks(net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec3);` | 9 | **3** (st=1/v=1/sp=1/i=0/d=0) | P31: перегрузка на move-пути, деградирует в List-вариант |
| `private void applyEffectsFromBlocks(java.util.List<net.minecraft.world.entity.Entity$Movement>);` | 80 | **24** (st=0/v=24/sp=0/i=0/d=0) | P31: батч-агрегатор, 24 invoke — кандидат свёртки в bulk-JNI |
| `private void checkInsideBlocks(java.util.List<net.minecraft.world.entity.Entity$Movement>, net.minecraft.world` | 106 | **27** (st=1/v=20/sp=0/i=6/d=0) | P31-ядро: ось-зависимый проход 3 осей → 3 вызова Vec3-оверлоада; 27 invoke на движение |
| `private int checkInsideBlocks(net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec3, net.minecraft.worl` | 61 | **11** (st=2/v=7/sp=1/i=0/d=1) | P31: AABB.deflate + invokedynamic-visitor → BlockGetter.forEachBlockIntersectedBetween |
| `private boolean lambda$checkInsideBlocks$2(int, java.util.concurrent.atomic.AtomicInteger, boolean, net.minecr` | 161 | **38** (st=3/v=32/sp=2/i=1/d=0) | P31-hot kernel: BlockStepVisitor-лямбда, 38 invoke на шаг — главный hot-цикл inside-батча |
| **Σ P31** | 475 | **119** | 6 методов внутри-entity внутри-цепочки |

### P32-view (SNAP sidecar-реестр, BLACKBOARD row 57, branch round-459-p32)

| метод | insns | invoke\* (st/v/sp/i/d) | релевантность P32 |
|---|---|---|---|
| `public T get(int, int, int);` | 9 | **2** (st=0/v=2/sp=0/i=0/d=0) | P32-view вход: Strategy.getIndex(III) → делегирует get(I) |
| `public T get(int);` | 11 | **2** (st=0/v=1/sp=0/i=1/d=0) | P32-view: BitStorage.get(I) + readPalette(I); interface-dispatch |
| `public final int get(int);` | 29 | **0** (st=0/v=0/sp=0/i=0/d=0) | P32-пол (JNI-floor профиль): 0 invoke, чистая арифметика |
| **Σ P32** | 49 | **4** | 3 метода read-пути палетт-вью |

**ИТОГО: 9 методов транскрибировано, 524 insns, invoke-сумма 123 (P31 119 + P32 4).**

## Call-graph inside-цепочки Entity (для P31 bulk-JNI)

```
applyEffectsFromBlocks()                        [58 insns / 16 invoke]
  └─(1 call)→ applyEffectsFromBlocks(List<Movement>)   [80 / 24]
applyEffectsFromBlocks(Vec3,Vec3)               [ 9 insns /  3 invoke]
  └─(1 call)→ applyEffectsFromBlocks(List<Movement>)
applyEffectsFromBlocks(List<Movement>)
  └─(1 call)→ checkInsideBlocks(List<Movement>, StepBasedCollector)   [106 / 27]
                 ├─(×3: offsets 171/204/229)→ checkInsideBlocks(Vec3,Vec3,Coll,LongSet,I)  [61 / 11]
                 │              └─ invokedynamic → BlockGetter.forEachBlockIntersectedBetween
                 │                            (visitor = lambda$checkInsideBlocks$2)
                 └─ lambda$checkInsideBlocks$2 (BlockStepVisitor)   [161 / 38]
```

Invoke-цели checkInsideBlocks(List,Coll) (27): 4× Movement.to(), 3× checkInsideBlocks(Vec3…),
2× Movement.from(), 2× Movement.axisDependentOriginalMovement(), 4× Iterator.next/hasNext,
Vec3.subtract/relative/lengthSqr/get(Axis), Direction.axisStepOrder, Axis.getPositive, Optional×2.

Invoke-цели checkInsideBlocks(Vec3,…): makeBoundingBox, AABB.deflate, Vec3.distanceToSqr, Mth.square,
ServerLevel.getServer→debugSubscribers→hasAnySubscriberFor, new AtomicInteger, invokedynamic visit,
BlockGetter.forEachBlockIntersectedBetween, AtomicInteger.get — 11 invoke, из них 1 invokedynamic.

## Плоские списки инструкций (ядро, без аргументов)

### P32-пол: SimpleBitStorage.get(int) — 29 insns, 0 invoke
```
0:aload_0 1:getfield 4:iload_1 5:imul 6:istore_2 7:iload_2 8:bipush 10:iushr 11:istore_3 12:iload_2 13:ldc 15:iand
16:aload_0 17:getfield 20:imul 21:bipush 23:iushr 24:istore 26:aload_0 27:getfield 30:iload_3 31:laload 32:iload 34:lushr
35:aload_0 36:getfield 39:land 40:l2i 41:ireturn
```

### P32: PalettedContainer.get(int,int,int) — 9 insns, 2 invoke
```
0:aload_0 1:aload_0 2:getfield 5:iload_1 6:iload_2 7:iload_3 8:invokevirtual 11:invokevirtual 14:areturn
```

### P32: PalettedContainer.get(int) — 11 insns, 2 invoke
```
0:aload_0 1:getfield 4:astore_2 5:aload_0 6:aload_2 7:aload_2 8:getfield 11:iload_1 12:invokeinterface 17:invokevirtual 20:areturn
```

### P31: checkInsideBlocks(Vec3,Vec3,Coll,LongSet,I) — 61 insns, 11 invoke
```
0:aload_0 1:aload_2 2:invokevirtual 5:ldc2_w 8:invokevirtual 11:astore 13:aload_1 14:aload_2 15:invokevirtual 18:ldc2_w 21:invokestatic 24:dcmpl
25:ifle 28:iconst_1 29:goto 32:iconst_0 33:istore 35:aload_0 36:getfield 39:astore 41:aload 43:instanceof 46:ifeq 49:aload
51:checkcast 54:astore 56:aload 58:invokevirtual 61:invokevirtual 64:getstatic 67:invokevirtual 70:ifeq 73:iconst_1 74:goto 77:iconst_0 78:istore
80:new 83:dup 84:invokespecial 87:astore 89:aload_1 90:aload_2 91:aload 93:aload_0 94:iload 96:aload 98:iload 100:aload_1
101:aload_2 102:aload 104:iload 106:aload 108:aload_3 109:invokedynamic 114:invokestatic 117:pop 118:aload 120:invokevirtual 123:iconst_1 124:iadd
125:ireturn
```

### P31: checkInsideBlocks(List<Movement>,Coll) — 106 insns, 27 invoke
```
0:aload_0 1:invokevirtual 4:ifeq 7:aload_0 8:getfield 11:astore_3 12:aload_1 13:invokeinterface 18:astore 20:aload 22:invokeinterface 27:ifeq
30:aload 32:invokeinterface 37:checkcast 40:astore 42:aload 44:getfield 47:astore 49:aload 51:invokevirtual 54:aload 56:invokevirtual 59:invokevirtual
62:astore 64:bipush 66:istore 68:aload 70:invokevirtual 73:invokevirtual 76:ifeq 79:aload 81:invokevirtual 84:dconst_0 85:dcmpl 86:ifle
89:aload 91:invokevirtual 94:invokevirtual 97:checkcast 100:invokestatic 103:invokevirtual 106:astore 108:aload 110:invokeinterface 115:ifeq 118:aload 120:invokeinterface
125:checkcast 128:astore 130:aload 132:aload 134:invokevirtual 137:dstore 139:dload 141:dconst_0 142:dcmpl 143:ifeq 146:aload 148:aload
150:invokevirtual 153:dload 155:invokevirtual 158:astore 160:iload 162:aload_0 163:aload 165:aload 167:aload_2 168:aload_3 169:iload 171:invokevirtual
174:isub 175:istore 177:aload 179:astore 181:goto 184:goto 187:iload 189:aload_0 190:aload 192:invokevirtual 195:aload 197:invokevirtual
200:aload_2 201:aload_3 202:bipush 204:invokevirtual 207:isub 208:istore 210:iload 212:ifgt 215:aload_0 216:aload 218:invokevirtual 221:aload
223:invokevirtual 226:aload_2 227:aload_3 228:iconst_1 229:invokevirtual 232:pop 233:goto 236:aload_3 237:invokeinterface 242:return
```

### P31: lambda$checkInsideBlocks$2 — 161 insns, 38 invoke
```
0:aload_0 1:invokevirtual 4:ifne 7:iconst_0 8:ireturn 9:iload 11:iload_1 12:if_icmplt 15:iconst_0 16:ireturn 17:aload_2 18:iload
20:invokevirtual 23:aload_0 24:invokevirtual 27:aload 29:invokevirtual 32:astore 34:aload 36:invokevirtual 39:ifeq 42:iload_3 43:ifeq 46:aload_0
47:aload_0 48:invokevirtual 51:checkcast 54:aload 56:invokevirtual 59:iconst_0 60:iconst_0 61:invokevirtual 64:iconst_1 65:ireturn 66:aload 68:aload_0
69:invokevirtual 72:aload 74:aload_0 75:invokevirtual 78:astore 80:aload 82:invokestatic 85:if_acmpeq 88:aload_0 89:aload 91:aload 93:aload
95:new 98:dup 99:aload 101:invokespecial 104:invokevirtual 107:invokevirtual 110:invokevirtual 113:ifeq 116:iconst_1 117:goto 120:iconst_0 121:istore
123:aload_0 124:aload 126:invokevirtual 129:aload 131:aload 133:aload 135:invokevirtual 138:istore 140:iload 142:ifne 145:iload 147:ifeq
150:aload 152:aload 154:invokevirtual 157:invokeinterface 162:ifeq 165:iload 167:ifeq 170:iload 172:ifne 175:aload 177:aload 179:invokevirtual
182:ifeq 185:iconst_1 186:goto 189:iconst_0 190:istore 192:aload 194:iload 196:aload 198:invokevirtual 201:aload 203:aload_0 204:invokevirtual
207:aload 209:aload_0 210:aload 212:iload 214:invokevirtual 217:aload_0 218:aload 220:invokevirtual 223:goto 226:astore 228:aload 230:ldc_w
233:invokestatic 236:astore 238:aload 240:ldc_w 243:invokevirtual 246:astore 248:aload 250:aload_0 251:invokevirtual 254:aload 256:aload 258:invokestatic
261:aload 263:ldc_w 266:invokevirtual 269:astore 271:aload_0 272:aload 274:invokevirtual 277:new 280:dup 281:aload 283:invokespecial 286:athrow
287:iload 289:ifeq 292:aload 294:iload 296:aload 298:invokevirtual 301:aload 303:invokevirtual 306:aload_0 307:invokevirtual 310:aload 312:aload_0
313:aload 315:invokevirtual 318:iload_3 319:ifeq 322:aload_0 323:aload_0 324:invokevirtual 327:checkcast 330:aload 332:invokevirtual 335:iload 337:iload
339:invokevirtual 342:iconst_1 343:ireturn 344:iconst_1 345:ireturn
```

### P31: applyEffectsFromBlocks(List) — 80 insns, 24 invoke
```
0:aload_0 1:invokevirtual 4:ifeq 7:aload_0 8:invokevirtual 11:ifeq 14:aload_0 15:invokevirtual 18:astore_2 19:aload_0 20:invokevirtual 23:aload_2
24:invokevirtual 27:astore_3 28:aload_3 29:invokevirtual 32:aload_0 33:invokevirtual 36:aload_2 37:aload_3 38:aload_0 39:invokevirtual 42:aload_0 43:invokevirtual
46:istore_2 47:aload_0 48:invokevirtual 51:istore_3 52:aload_0 53:invokevirtual 56:istore 58:aload_0 59:aload_1 60:aload_0 61:getfield 64:invokevirtual
67:aload_0 68:getfield 71:aload_0 72:invokevirtual 75:aload_0 76:invokevirtual 79:ifeq 82:aload_0 83:invokevirtual 86:iload_2 87:ifeq 90:aload_0
91:invokevirtual 94:ifeq 97:iload_3 98:ifeq 101:aload_0 102:invokevirtual 105:ifne 108:aload_0 109:invokevirtual 112:aload_0 113:invokevirtual 116:iload
118:if_icmple 121:iconst_1 122:goto 125:iconst_0 126:istore 128:aload_0 129:invokevirtual 132:invokevirtual 135:ifne 138:aload_0 139:invokevirtual 142:ifne
145:iload 147:ifne 150:aload_0 151:aload_0 152:invokevirtual 155:ineg 156:invokevirtual 159:return
```

## Выводы для P31-батча / P32-view

* **P31**: до 62 invoke\* на движение до фактического сканирования блоков
  (24+27+11 снаружи lambda; сама lambda 38 → всего 100 invoke\* на одно движение с inside-сканом). Свёртка цепочки applyEffectsFromBlocks→checkInsideBlocks
  в bulk-JNI (1 вызов на List<Movement>) убирает 2 интерфейсных + ~40 виртуальных вызовов на движение.
  Точка интеграции — invokedynamic-visitor на offset 109 checkInsideBlocks(Vec3…): единственный
  visitor-объект, значит bulk-версия может принять List<Vec3>-пар без изменений семантики StepBasedCollector.
* **P32**: read-путь палетт-вью = 4 invoke\* (Strategy.getIndex → readPalette → BitStorage.get + get(i)
  readPalette). Сам SimpleBitStorage.get(int) — JNI-пол: 29 insn чистой арифметики (imul/iushr/laload/
  lushr/land/l2i), 0 вызовов — JNI-обёртка здесь проигрывает инлайну, выигрыш P32 только в устранении
  двух dispatch-переходов PalettedContainer.get(x,y,z)→get(i)→readPalette (sidecar-снапшот реестра срезает
  виртуальность Strategy.getIndex + readPalette).
* Карточный путь (SOURCE=CARD) не понадобился: RESEARCH-458-P.md в ROUND-458 отсутствует, jar найден.

--- RUN/BRANCH: см. финал TASK-459-91 (round-459-javap1, SOURCE=JAR).
