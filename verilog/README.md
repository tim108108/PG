# Verilog


## 安裝環境
iverilog + GTKWave  
安装iverilog：`sudo apt install iverilog`  
安装GTKWave：`sudo apt install gtkwave`  

## DeltaMOOCx
https://youtube.com/playlist?list=PLI6pJZaOCtF3_-vE7VUn9RdhQ6KXpcFQD&si=KG4upjZ-4argRTTQ  
https://ys-hayashi.me/2024/03/systemverilog-useful-syntax-01/

## refer
https://steveicarus.github.io/iverilog/  
https://zhuanlan.zhihu.com/p/95081329

### docker 環境
```docker
# build verilog_container
docker build -t verilog_image .
# 執行退出後自動刪除
docker run -it --rm -e DISPLAY=$DISPLAY -v ./project:/app -v /tmp/.X11-unix:/tmp/.X11-unix --name verilog_container verilog_image 
# 執行 verilog_container
docker run -it -e DISPLAY=$DISPLAY -v ./project:/app -v /tmp/.X11-unix:/tmp/.X11-unix --name verilog_container verilog_image 
# 背景執行
docker run -d --name verilog_container verilog_image 
# 進入 verilog_container
docker exec -it verilog_container /bin/bash
```
> 1. 建立docker image
> 2. 背景執行docker container
> 3. 執行docker container
