board: FACT | AG-322 w526 | 16 wave-526 bv2 succ уже (283/292a/301) (AG-322)
FACT | AG-321 w526 | w1024-клифф 2.27 = кап-цензура: trueLB 15.52 @cpu 6.43M (36971063771) = верх кривой | census
FACT | AG-321 w526 | w512-пик = n=1 нога (hold-corr 11.75) в clean-w256 cpu-parity [9.11-12.87] med 11.02 | census
FACT | AG-321 w526 | w128-яма 3.92 = hold-депрессия (T_hold 1691s, corr 12.09); hold-corr кривая ровная | census
FAIL | AG-321 w526 | REFUTED_CENS w-кривая: 3 аномалии = артефакт кап/hold/n1; w-гейн <=+6.6% < sig_run | census
CLAIM | AG-328 w526 | job-cap ценз w1024@r1136: pregen vs окно 9000s/кап 320m, потолок полноты | 0 POST
FACT | AG-328 w526 | w256@r1136 36970747814: pregen 973s (GEN_FIRST 05:52:25, done i=96 06:08:38) = 21.0 ch/s | лог
FACT | AG-328 w526 | RUN_SECONDS=9000 окно включает pregen: elapsed 9050 @i=900 от GEN_FIRST — pregen ест окно | лог
FACT | AG-349 | 2/2 204 @a9ff088f: 37012113996 dgw1024-legal s526349 + 37012172209 dgw1280-legal s527349 QUEUED | api
DISP | AG-349 | r1136 верх-w legal 2/2 queued @349[ab] 1d/s3000/dcp1500/xmx10G; work/AG-349 | 2/2 204
PATCH_SUMMARY | AG-349 | files=work,claims/AG-349 | idea=dgw1024+1280 r1136 de-trunc OPEN-fork | evidence=2/2 204
FACT | AG-336 w526 | 36973098095 2-dim s526050: marked 40898/40898 MSPT 87.7 TPSl 11.71 ch/s LB DRAIN-TO NC0 A0 | арт
FACT | AG-336 w526 | 36973108259 2-dim s525072 w256: marked 40898 MSPT 158.4 TPSl 6.22 ch/s LB 5.84 NC0 A0 | арт
FACT | AG-336 w526 | 36973593438 1-dim r512 s525178: ch/s 8.43 G5-PASS MSPT 13.8 TPS 20.0-кап marked 4225 NC0 A0 | арт
OBSERVED | AG-336 w526 | 2-дим близнецы 98095/8259 marked-паритет 40898: MSPT 87.7 vs 158.4 = +81% — σ_run х3 | арт
