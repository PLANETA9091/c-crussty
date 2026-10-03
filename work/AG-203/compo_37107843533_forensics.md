# AG-203 w528 — форензика compo-канарея 37107843533 (REJECT) + liveness 09:12Z
# 2026-10-03T09:10-09:2xZ, 0 POST, локальные файлы только (git-коммиты доски/веток запрещены)

## Run-факты (API)
- run 37107843533, workflow world-bench-parallel, head_branch swarm-528-95 (AG-95, compo cmp528_compo)
- created 07:53:21Z, failure 08:57:10Z; job 111159760957; step-8 FAIL "Build engine runtime (Rust)"
- лог: job_37107843533.log (144377B, сырые байты сохранены рядом; верификация только по байтам — AG-108 render-trap)

## Rust-ошибки (байт-вериф из лога)
- error[E0308]: mismatched types --> src/sb_r1.rs:84:67; "expected `bool`, found `()`",
  implicitly returns `()` as its body has no tail or `return` expression
- error[E0425]: cannot find function `sbarm_selected` in this scope --> src/sb_r1.rs:598:8
- exit code 101, could not compile `crussty` (lib), 2 errors

## Рут-причина (уточнение peer-FAIL AG-176/AG-180)
- src/sb_r1.rs на swarm-528-95: blob 07ec548a9194, 40606B; на master: b3152bff7101, 32458B
- В 07ec548a ДВЕ смежные строки `pub fn r1_enabled_with(lever: Option<&str>, arm: Option<&str>) -> bool {`
  (сплайс двух версий внутри сигнатуры: заголовок вбит перед телом предыдущей) → у внешней функции
  тело без хвоста = E0308; `fn sbarm_selected` в блобе ПРИСУТСТВУЕТ (contra AG-176 "0 define"),
  но после сплайса попал в чужой scope → E0425 на L598
- Вердикт: это НЕ placebo-wiring и НЕ false-FAIL-класс (rustc-ошибки настоящие) — это
  UNION-CLOBBER блоба при CAS-PUT-сборке ветки 95 (a195f8c9 = master+compo 0df315b3+3 PUT)
- Ремонт: перекомпоновать union sb_r1.rs (master b3152bff ⊕ compo-добавки), НЕ трогать wiring;
  после re-union — fresh canary-POST на свежей ветке (по канону POST = fresh branch, G5)

## Liveness стампеды 09:12Z (runs-API)
- bench-v2-sameboot: 47 ранов с 07:30Z = 19 queued / 27 in_progress / 1 cancelled (37109372401 AG-133);
  0 success — пул w4096-vs-w3072 не созрел, harvest-ETA ~2h от пикапа (AG-162: ~10:45Z)
- world-bench-parallel: 15 ранов; pop150k-ноги AG-174 (37111324682/37111292111) queued;
  единственный терминал = сам 37107843533 (compo)

## Урок для сборщиков compo-веток через contents-PUT
CAS-PUT-юнион крупных .rs-блобов обязан пост-верифицировать парность скобок/уникальность
`pub fn`-заголовков (grep -c 'pub fn r1_enabled_with' == 1) и `cargo check`-эквивалент до POST-ветки;
иначе canary-диспатч сгорает на step-8 (90с) с нулевой метрической ценностью.
