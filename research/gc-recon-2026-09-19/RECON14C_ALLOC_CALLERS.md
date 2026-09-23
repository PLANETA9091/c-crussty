# RECON-14c — атрибуция аллок-листьев и CPU по вызывателям (s7173, всё окно)

## ALLOC окно: 9,047 сэмплов; по вызывателям:
- other: 3,568 = 39.44%
- inside-blocks: 2,821 = 31.18%
- travel-collide: 1,482 = 16.38%
- fluid: 1,111 = 12.28%
- push: 65 = 0.72%

## Топ-листья по вызывателям:
- AABB: 1,977 = 21.85% окна | inside-blocks=1094 (55%); travel-collide=518 (26%); other=365 (18%)
- Vec3: 1,942 = 21.47% окна | inside-blocks=718 (37%); fluid=602 (31%); other=371 (19%); travel-collide=248 (13%); push=3 (0%)
- BlockPos: 933 = 10.31% окна | other=512 (55%); fluid=220 (24%); travel-collide=157 (17%); inside-blocks=44 (5%)
- long[]: 416 = 4.60% окна | inside-blocks=403 (97%); other=13 (3%)
- char[]: 449 = 4.96% окна | other=449 (100%)

## CPU окно: 127,109 сэмплов; по вызывателям:
- other: 88,484 = 69.61%
- fluid: 12,657 = 9.96%
- inside-blocks: 11,697 = 9.20%
- travel-collide: 10,571 = 8.32%
- push: 3,700 = 2.91%
