FROM ubuntu:latest
WORKDIR /app/project/

RUN apt update \
    && apt install iverilog -y \
    && apt install gtkwave -y \
    && apt install nano gcc git make autoconf automake autotools-dev curl wget libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat1-dev -y && mkdir /opt/riscv32i && chmod 777 /opt/riscv32i
RUN git config --global url."https://git.qemu.org/".insteadOf git://git.qemu-project.org/ && git config --global url."https://github.com/".insteadOf git://github.com/
RUN git clone https://github.com/riscv/riscv-gnu-toolchain riscv-gnu-toolchain-rv32i && cd riscv-gnu-toolchain-rv32i && git checkout -f 411d134 && git submodule update --init --recursive && mkdir build && cd build 
WORKDIR /app/project/riscv-gnu-toolchain-rv32i/build
RUN ../configure --with-arch=rv32i --prefix=/opt/riscv32i 
RUN make -j1 && export PATH=/opt/riscv32i/bin:$PATH 
# RUN riscv32-unknown-elf-gcc --version
# WORKDIR /app/project/