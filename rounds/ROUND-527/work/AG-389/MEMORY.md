# AG-389 MEMORY (≤15 строк)
1. Board PUT = stampede: GET-sha→PUT 409 x2-3 norm, retry до OK; guard len>700k перед PUT.
2. base64 850KB в argv валился ("Argument list too long") — body через файл + curl --data-binary @.
3. run-env heredoc = точка правды атрибуции: добавлять knob-строки ТУДА (до L56 cp, оба копии).
4. report_benchv2.py читает run-env только по точечным regex (radius_blocks, dims) — append-safe.
5. GS-attribution: AG-354/337/456 судят GS-пары по dispatch-yml реконструкции; artifacts-echo = фикс.
6. Мой w526-хвост: host_model/nproc форвард (baeeefb4) — серверная строка AG-301 re-land уже в heredoc L54.
7. 0 POST при famine: PATCH-паттерн AG-206/219/237 (MERGE-READY + canary note) легален и достаточен.
8. sim53/64 клетки закрыты (301+307, kernel-eq 87f70193=2d2e6e7f) — не залезать, w528 добить по 1.
