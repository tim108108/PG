# git config --global url.https://github.com/.insteadOf git://github.com/
# make download-tools
# make -j2 build-tools

git config --global url."https://git.qemu.org/".insteadOf git://git.qemu-project.org/ && git config --global url."https://github.com/".insteadOf git://github.com/
git clone https://github.com/riscv/riscv-gnu-toolchain riscv-gnu-toolchain-rv32imc 
cd riscv-gnu-toolchain-rv32imc && git checkout -f 411d134 && git submodule update --init --recursive && mkdir build && cd build 
../configure --with-arch=rv32imc --prefix=/opt/riscv32imc --with-abi=ilp32 && make -j2
export PATH=/opt/riscv32i/bin:$PATH && riscv32-unknown-elf-gcc --version
