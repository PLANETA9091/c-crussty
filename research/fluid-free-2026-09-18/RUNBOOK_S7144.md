# RUNBOOK S7-144 — воспроизводимый офлайн-конвейер FluidFreeHarness после WIPE

Дата: 2026-09-18 ~04:30-04:55 +08. Контекст: тик 12:43 Job 393012 (10-й тик CREDS-BLOCKED).
Цель: доказать воспроизводимость FLUID-FREE OFFLINE PASS (S7-143) из ЧИСТОГО состояния
(только /tmp/toolchain/{jdk-21, purpur-1.21.10.jar} + репо c-crussty @9564838).

## Состояние на входе
- Уцелело: /tmp/toolchain/jdk-21.0.12.1+1, /tmp/toolchain/purpur-1.21.10.jar (paperclip 57353083B).
- Утрачено WIPE: материализованное ядро (29386794B), libraries/, classpath-структуры.

## Шаги (скрипты в scripts_s7144/, банкованы sha256 в artifact_hashes_s7144.txt)
1. `materialize_kernel_v2.sh <paperclip.jar> <workdir> <out>` — paperclip kill-before-main
   (variant: сам скачивает mojang_1.21.10.jar, когда cache пуст). Результат:
   versions/1.21.10/purpur-1.21.10.jar = 29386794B байт-в-бит с цензом
   (sha256 e2992d63abd2c2544a4d1564c6dbe402fb05c12a410d2700a355d2cbe2e87200).
   Побочно: paperclip разворачивает <workdir>/server/libraries/ (125 jar) и cache/mojang_*.jar.
2. `finish_libraries.sh` — страховка: повторный paperclip-проход в тот же workdir с kill-ом
   на строке "Starting …Main" (до main; eula.txt отсутствует ⇒ гонки main не boot'ит).
   Факт: libraries остались 125 — полный набор по манифесту purpur.
3. `run_fluid_free_harness.sh` — сборка shadow-classpath + чистая javac-пересборка
   FluidFreeHarness + прогон. Результат: FLUID-FREE OFFLINE PASS exit 0.

## УРОКИ (критично для будущих прогонов после WIPE)
1. **Порядок classpath = shadow → kernel → libraries**. В libraries/ лежит ЧИСТЫЙ
   mojang-logging-1.5.10, а paper-классы (SimpleProviderStorage и др.) требуют
   getClassLogger(), который есть только в paper-shaded LogUtils ВНУТРИ
   пропатченного kernel jar. Kernel ДОЛЖЕН идти раньше libraries, иначе
   NoSuchMethodError getClassLogger.
2. **Источники библиотек**: вложенные jar'ы в mojang/purpur контейнерах — base+patch
   (например slf4j-api-2.0.16.jar.patch); ФИНАЛЬНОЕ развёрнутое состояние —
   <workdir>/server/libraries/ (создаётся paperclip'ом при материализации). Брать оттуда.
3. **unzip не создаёт вложенный -d путь** — mkdir -p обязателен.
4. **set -euo pipefail**: перезапись LIBCP через `ls "$LIBS"/*.jar` на вложенной
   структуре даёт пусто и молча роняет classpath (ловится только на runtime).
5. Bootstrap.bootStrap() тянет brigadier + paper entrypoints ⇒ прогон харнесса
   требует полный libraries-набор, а не только kernel+fastutil (этого хватало
   только для КОМПИЛЯЦИИ FluidOps, см. build_fluid_ops.sh).
6. INJECTS-ONLY соблюдался: kill-before-main, eula.txt не создаётся, процессов java
   на выходе нет (проверено pgrep после каждого шага).

## Итог прогона (04:54:17 +08)
STRUCTURAL+INJECTED SURFACE PASS / SECTION MATERIALIZED / FLUID-FREE ARMED=true
(section-ff offsets resolved) / FREE-HIT ff=1 ffGen=0 gen=0 / EVENT: gen 0→2 ff=2
ffGen=2 / RESTORE: gen=4 ffGen=4 ff=1 verdict=true / SCATTERED-WATER PARITY PASS ⇒
FLUID-FREE OFFLINE PASS (exit 0). Байт-в-бит соответствует записи S7-143/91fcd70.
