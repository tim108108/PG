## RUST

[Rust入門祕籍](https://shihyu.github.io/rust_hacks/ch1/00.html)
[Rust 程式設計語言](https://rust-lang.tw/book-tw/ch00-00-introduction.html)
[Rust语言圣经(Rust Course)](https://course.rs/about-book.html)

## 安裝
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# cargo
- `cargo new "專案名"` 新建專案  
- `cargo build` 編譯專案
- `cargo run ` 編譯後執行

### docker 環境
```docker
docker build -t rust_image .  
docker run -d --name rust_container rust_image 
docker run -it --name rust_container rust_image 
docker exec -it rust_container /bin/bash
```
> 1. 建立docker image
> 2. 背景執行docker container
> 3. 執行docker container