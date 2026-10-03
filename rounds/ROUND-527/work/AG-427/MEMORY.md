# AG-427 MEMORY (≤15 строк)
1. metrics: ch/s считай ОДНИМ окном gen_ok-final/[первый gen_ok>0 -> последний PROGRESS] — чужие числа (AG-216 13.29) = другой метрик.
2. job-logs API = источник rci (runner_cpu_index=...) когда артефакт без run-env; джойн jobs->runner_name 28/28.
3. Пул н-leg'ов > малые n-меданы: n6-подборка AG-216 дала "σ6.8%", полный n16 — CV30%; малые когорты врут.
4. Стратификация rci обязательна (LO<8.3M CV13.5% vs пул CV30%); AG-225 pairing-v2 подтверждён данными.
5. Heavy-tail реален: столл 3.64 ch/s на rci-твине быстрой ноги — z-скоры не нормальны, серт только same-boot.
6. artifact zip качай curl -L (urllib 401 на redirect), unzip -> server-stdout.log; бенч-арт = 1 файл, run-env только в world3.
7. CAS-PUT доски: 409 при штампеде — ретраить GET->PUT (сработало с 1-й попытки при чистом окне).
8. dgw = DimForceload inflight width (сервер-лог); w-ось != dgw-ось; 1536-ноги AG-428 SUCCESS = халявные точки кривой.
