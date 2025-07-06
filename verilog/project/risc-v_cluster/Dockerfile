FROM ubuntu:latest
WORKDIR /app

RUN apt update \
    && apt install iverilog gtkwave -y \
    && apt install verilator -y \
    && apt install git make gcc g++ -y 