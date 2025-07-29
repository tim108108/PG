FROM ubuntu:latest
WORKDIR /app

RUN apt update \
    && apt install iverilog -y \
    && apt install gtkwave -y 

# docker build -t verilog_image .
# docker run -it --rm -e DISPLAY=$DISPLAY -v ./:/app -v /tmp/.X11-unix:/tmp/.X11-unix --name verilog_container verilog_image
