# LEVER_STACK (P501) — живой список кандидатов-рычагов
# Формат: ID | рычаг | файл/узел | ожидание d | уровень сертификации | происхождение | статус
# Правила: <=3 активных рычагов за тик; каждый — своя ветка, бенч в том же коммите;
# FAIL уходит в доску с числом и причиной; стек пуст = майнинг истории/профилей.

| ID | рычаг | файл/узел | ожид. d | уровень | происхождение | статус |
|---|---|---|---|---|---|---|
| LS-1 | chunk_encode_microbench как L1-инструмент (восстановить из bb8cf74, CE его выронил) | native/paper-native-chunk-encode-core/src/bin/ | — (инструмент) | L1 | CRUSSTY bb8cf74 | TODO |
| LS-2 | майнинг TASK-2xx рычагов CRUSSTY (tape identity, TLS output pool, zero-copy c_publish) → кандидаты | CRUSSTY история июнь-сент | по находке | L0→L1 | CRUSSTY master до удаления native/ | TODO |
| LS-3 | RECOVERY-2: недостающие ~100 items закрытого core-2025 | native/paper-native-jni (импорты без тела) | разблокировка рычагов | — | native/RECOVERY.md | ON-DEMAND |
| LS-4 | конвертация находок PROFILE-B2 / PROFILE-C / NOISEFILL_ROOTCAUSE в рычаги | профильные файлы репо | по профилю | L0→L2 | репо | TODO |
