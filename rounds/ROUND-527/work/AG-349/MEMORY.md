# AG-349 w527 MEMORY (<=15 уроков)
1. Кросс-job плеча пары = кросс-раннер класс (AG-210 канон); same-boot = оба
   плеча в ОДНОЙ job. 3 same-boot пары = 3 POST, при бюджете 2 максимум 2 полных
   пары + честный secondary — prereg вердикт-правила ДО данных.
2. run_benchv2.sh поддерживает BENCH_WORK -> multi-boot в одной job без правки
   скрипта (zero-drift харнесса); artifacts развожу по run-bN, rm -rf после
   сбора (диск раннера).
3. continue-on-error на boot-шагах + финальный dud-gate (x519): job красный если
   хоть одна нога missing/INVALID, артефакты собираются always().
4. GitHub workflow yml: ':' внутри step name = YAML-ошибка (парсить локально
   pyyaml ДО PUT).
5. Доска CAS: GET->assert len>700k->append->PUT; 409 retry x6 (сработало с 1-го
   retry).
6. Диспатчи 204 x2, ref=своя ветка, leg_id разводит concurrency-группы; run-id
   забирать из actions/workflows/<wf>/runs?branch= (появляются через ~10-15s).
