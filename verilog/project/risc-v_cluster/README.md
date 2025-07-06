# RISC-V Cluster
## This project is modify from [picorv32](https://github.com/YosysHQ/picorv32)

## Refer
- [用 verilator 輔助數位電路設計](https://yodalee.me/2023/02/verilator_intro/)
- [PicoRV32：A Size-Optimized RISC-V CPU](https://juejin.cn/post/7372245998897774629)
- [Verilator 使用指南](https://soc.ustc.edu.cn/CECS/lab2/verilator/)
- [Verilator Pt.1: Introduction](https://itsembedded.com/dhd/verilator_1/)
- [自學筆記(FPGA)](https://hackmd.io/@KevinT/Hktt6sFvh)
- [GoogleTest](https://github.com/google/googletest)

### docker 環境
```docker
# build riscv_ctnr
docker build -t riscv_img .
# 執行退出後自動刪除
docker run -it --rm -e DISPLAY=$DISPLAY -v ./:/app -v /tmp/.X11-unix:/tmp/.X11-unix --name riscv_ctnr riscv_img 
# 執行 riscv_ctnr
docker run -it -e DISPLAY=$DISPLAY -v ./:/app -v /tmp/.X11-unix:/tmp/.X11-unix --name riscv_ctnr riscv_img 
# 背景執行
docker run -d --name riscv_ctnr riscv_img 
# 進入 riscv_ctnr
docker exec -it riscv_ctnr /bin/bash