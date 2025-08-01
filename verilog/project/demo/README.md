# How-to-setup-env
## Linux

FPGA toolchain reference [icestorm](http://www.clifford.at/icestorm/)  
gcc toolchain reference [riscv-gnu-toolchain](https://pingu98.wordpress.com/2019/04/08/how-to-build-your-own-cpu-from-scratch-inside-an-fpga/)  

或者，您可以下載 xPack 或 SiFive 提供的預構建工具鏈
+ https://xpack.github.io/riscv-none-embed-gcc/install/
+ https://www.sifive.com/software
+ `icesprog` is command tool stand for iCESugar Program，it depend libusb and hidapi  
`$sudo apt-get install libhidapi-dev`  
`$sudo apt-get install libusb-1.0-0-dev`
  
安裝 [icestorm](http://www.clifford.at/icestorm/) 步驟如下:  
├──icestorm-build  
&emsp;&emsp;&emsp;└──icestorm  
&emsp;&emsp;&emsp;└──arachne-pnr  
&emsp;&emsp;&emsp;└──yosys  

```sh
sudo apt-get install build-essential clang bison flex libreadline-dev \
                     gawk tcl-dev libffi-dev git mercurial graphviz   \
                     xdot pkg-config python python3 libftdi-dev

mkdir icestorm-build
cd icestorm-build

git clone https://github.com/cliffordwolf/icestorm.git icestorm
cd icestorm
make -j$(nproc)
sudo make install
cd ..


git clone https://github.com/cseed/arachne-pnr.git arachne-pnr
cd arachne-pnr
make -j$(nproc)
sudo make install
cd ..

git clone https://github.com/cliffordwolf/yosys.git yosys
cd yosys
make -j$(nproc)
sudo make install
cd ..

```
# FPGA教程
强烈推荐学习此教程，[open-fpga-verilog-tutorial](https://github.com/Obijuan/open-fpga-verilog-tutorial/wiki/Home_EN) `src/basic/open-fpga-verilog-tutorial`目录中有对应的例程
[Icarus Verilog](https://hackmd.io/@HankTsai/SkP1XmgpT)

## dockerfile
```
docker build --no-cache -t verilog_image .
docker run -it --rm -e DISPLAY=$DISPLAY -v ./:/app -v /tmp/.X11-unix:/tmp/.X11-unix --name verilog_container verilog_image
```
## iverilog && gtkwave
```
iverilog -o adder_tb adder.v adder_tb.v
vvp adder_tb
gtkwave adder_tb.vcd
```
```
iverilog -o mean_tb mean.v mean_tb.v
vvp mean_tb
gtkwave mean_tb.vcd
```
