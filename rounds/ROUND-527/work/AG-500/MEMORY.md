# AG-500 MEMORY (w527, <=15 строк)
1. CAS-append доски: GET blob -> guard len>=700k -> PUT, retry 409; python urllib, commit 4f6c68f4 OK.
2. Live board 936KB vs локальный tail 74xKB — локальный клон протух на часы, доверять только contents-GET.
3. Zero-code ветка: git fetch origin master -> branch swarm-527-500 от FETCH_HEAD (da6eb3c4, tree-3742>=3200) -> push. 0 кода.
4. POST dispatch: 204 x2, GET-вериф по ?branch= фильтру (head_sha-фильтр с полным sha вернул пусто/HTML).
5. POST-ы разносить 30s+: 37102118677 (06:09:22Z) и 37102148945 (06:09:55Z) — sibling-cancel риск = 0 (leg_id+seed разные).
6. Сиды 527500/528500 свободны (grep board+rounds до POST — реестр врёт).
7. w-ось = dgw-ось = dim_gen_window (два имени на доске — одна ось); ghost w6144 = 13.29 ch/s.
8. Famine 06:06Z: 96q/0ip, старейшая q 01:08Z — 0 пикапов 5ч; диспатчи копятся в хвост FIFO, харвест w528.
9. canary run-env 37079079710 (AG-206) всё ещё queued 06:06Z — yml-фиксы AG-206/219/237 ждут вердикта.
10. run-env-арт POISON '#' жив в master bv2.yml:153 -> числа из joblog (GEN-DONE), не из артефактов.
