## C

- [C99](https://www.dii.uchile.cl/~daespino/files/Iso_C_1999_definition.pdf)
- [C 語言程式設計講義](https://hackmd.io/@Hf0NNmZcQUyjuplfGUqMuQ/H1KuX6Tz_)
- [C 程式語言](https://dywang.csie.cyut.edu.tw/dywang/download/pdf/clanguage.pdf)
- [語言技術: C 語言](https://openhome.cc/Gossip/CGossip/)
- [C 語言的總覽](https://programming.im.ncnu.edu.tw/C_index.html)

### docker 環境
```sh
docker build -t c_image .
docker run -it -v ./project:/app --rm --name c_container c_image
docker exec -it c_container /bin/bash
```
> 1. 建立docker image
> 2. 背景執行docker container
> 3. 執行docker container