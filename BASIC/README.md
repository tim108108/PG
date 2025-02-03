### docker 環境
```docker
docker build -t basic_image .
docker run -it -v ./project:/app --name basic_container basic_image 

docker run -d --name basic_container basic_image 
docker exec -it basic_container /bin/bash
```
> 1. 建立docker image
> 2. 背景執行docker container
> 3. 執行docker container