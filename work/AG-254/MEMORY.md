# AG-254 MEMORY (уроки ≤15 строк)
1. /tmp/harvest* протухает между волнами — re-harvest через actions/artifacts API (id живы ≥90д), run-env.txt из zip — источник inputs.
2. Коллапsed-сплит: классифицируй по ПЕРВОММУ кадру после цели (ИЕ.tick), иначе double-count baseTick/applyEffects.
3. merge-лейн мёртв и на dp50k (0.01%) — не воскрешать (канон Л145 подтверждён).
4. item-fluid-скан 7.3-8.1% total CPU = топ-суб-таргет S#3; fluid_guard/bitmask/dirty лейны его НЕ покрывают (другой сайт).
5. inside_cache=1 НЕ убивает checkInsideBlocks (4.35-4.51%) — гейт кэширует discovery, свип жив; внутри_bitmask #15 = кандидат.
6. Суб-профили dp50k стабильны ≤0.6пп между ногами — капture по 2 ногам легален, в отличие от TPS (σ17%).
7. Соло-потолок item-оси ~+6-8%, комбо ~+13-18% — sub-бар; POST до волны-527 для dp50k запрещён (слоты 6/6).
