# RECON-37: worker-balance вердикт (s7196 run 35488526730 @ ?)

Дата: 2026-09-20 | артефакт world3-bench (id 10598539161, ~29MB) | банд [6.0M,9.5M]

**РАЗВИЛКА: OFFLOAD-READY (I<=1.15): P2 = слив main-бакетов + offload остатков**

```
wall-collapsed: 2998 стеков, 69652 сэмплов, 61 уникальных имён потоков

-- WORKERS (duty = tickBucket-активность / все сэмплы потока):
  crussty-region-worker-1      tot=  1201 active=   809 ( 67.4%) park=   390 (32.5%)
          41  PalettedContainer.get
          37  Entity.updateFluidHeightAndDoFluidPushing
          33  syscall
  crussty-region-worker-2      tot=  1201 active=   796 ( 66.3%) park=   404 (33.6%)
          37  syscall
          35  PalettedContainer.get
          25  Entity.updateFluidHeightAndDoFluidPushing
  crussty-region-worker-3      tot=  1201 active=   819 ( 68.2%) park=   380 (31.6%)
          35  PalettedContainer.get
          30  Entity.updateFluidHeightAndDoFluidPushing
          29  syscall

-- ДИСБАЛАНС I = max/avg duty = 0.682/0.673 = ** 1.01 **
-- MAIN (Server thread): tot=1201 active-entity=745 (62.0%) park=151 (12.6%) — DONE-wait residual

-- РАЗВИЛКА RECON-36 (прегистр):
   I=1.01 <= 1.15 -> ** OFFLOAD-READY **: P2 = слив main-бакетов в workers + offload main-остатков; потолок +25..33% wall-clock

```