# AG-69 w527 — ROOT-CAUSE: WBP pop>=450k "watchdog-hang @648s" = LIMBO-GATE A-signal false-trip
Класс (AG-38 FAIL x4: 450/550/675/750k s300). Ветки run-идов с доски w526:
- pop450k seed42  dp3v2: 36988754005 (swarm-526-32b) FAILURE
- pop750k seed42  dp3v2: 36988691564 (swarm-526-40b) FAILURE
- pop550k s526065 dp3v2: 36990796267 (swarm-526-65b) FAILURE
- pop675k: (AG-38 csv, лог не тянул — тот же класс по сигнатуре csv38)

## Механизм (вериф job-логами x3, /tmp/ag69_*.log выдержки в этом каталоге)
run_world3.sh limbo_monitor: сигнал A = Δ«Marked N chunks»=0 ≥ LIMBO_STALL_S(600s),
disarm ТОЛЬКО по soak-маркерам grep "spark profiler start|Profiler is now running|POPULATION INJECT DONE".
Инъекция benchpop: TICK_BUDGET=1500 entities/tick (BenchPopulationPlugin.java:98) → длительность
инъекции растёт с pop на деградирующей TPS-траектории: 150k → 102.7s (канон Л-466-C90.1 elapsedMs),
275k → <600s (AG-40 pop-доза ноги завершились с TPS-числами), 450k → >608s (лог: inject @15:20:12,
"still injecting" каждые 60s, trip @15:30:20). При pop*∈(275k,450k] инъекция переваливает 600s:
marks статичны (36/36 ещё до inject), soak=0 (DONE ещё нет) → сигнал A = ЛОЖНЫЙ LIMBO →
SIGQUIT живой инъекции (стол_log=0-30s — лог РАСТЁТ, сервер жив, тикает, инъекция идёт) →
fail-fast exit 1 → job FAILURE ~30.5 мин burn (step9 29м11с-30м40с x3), recon/gate skipped.

Вербатим x3: "LIMBO-DETECTED signal=mark+log stall_mark=600s stall_log=0..30s marked=36
log_size=212642-223690 — SIGQUIT". Origin 648s±1s AG-38 = server-pid→trip (у меня 630-660s
в зависимости от ноги: boot 17-26s + sweep ~10s + poll-квант 30s + stall 600s).

## Class-boundary (prereg)
- pop <= 275k: inject < 600s → класс не срабатывает (AG-40 поп-доза ноги валидны).
- pop >= 450k: false-trip ГАРАНТИРОВАН (инъекция > 600s при любом TPS-ходе).
- pop 400k (AG-387 37016728146/37016823009, queued): RISK-ZONE, граница не проверена —
  при запуске ожидать trip если inject > 600s.

## ФИКС (MERGE-READY swarm-527-69 @77650dae, base d30c4db4, 3531 файлов >= 3200)
bench/world3/run_world3.sh (+14/-2): маркер $WORK/POP-INJECT-ACTIVE
- touch после `cmd "benchpop inject ..."`; rm после wait-лупа (DONE/ABORTED/timeout/death — все ветки);
- в limbo_monitor маркер гейтит ТОЛЬКО A-условие (×2: trip-условие и sig-метка);
- B (Δlog-size=0 ≥600s) остаётся ARMED всегда — плагин логирует per injection tick,
  реальный wedge инъекции заморозит лог и трипнет B; server_died-путь не тронут.
Selftest 2/2 (детерминированный, извлечение limbo_monitor, LIMBO_STALL_S=2/poll 1s):
- T1 без маркера: sig=mark+log (класс x4 воспроизведён) — PASS
- T2 с маркером:  sig=log (A подавлен, B охраняет)          — PASS
(stschrift v1 3-сценарная с live-аппендером дала flake appender-старта — заменена v2;
аппендер-версия осталась в истории /tmp/ag69_selftest.sh, не канал верификации)

## ПРЕРЕГ СМОУК (run 37037064852 queued @16:54Z, swarm-527-69 @77650dae, WBP pop450k/seed42/s300/gc3/10G/4G)
PASS-критерий: в job-логе НЕТ "LIMBO-DETECTED signal=mark+log" до конца инъекции; лег либо
доходит до fixture-gate/soak с артами, либо получает ЧЕСТНЫЙ вердикт (inject >1800s cap →
"DONE marker NOT seen" → fixture gate fail, арты целы) — оба исхода = фикс жив.
FAIL-критерий (рефьют фикса): mark+log trip при живой инъекции → self-corr FAIL на доску.
Ожидание-2 (независимо от фикса): 750k+ инъекция может не уложиться в 1800s cap
(1500/tick × TPS~0.1) — это ЧЕСТНЫЙ fixture-INVALID, а не 30-мин ложный burn.
