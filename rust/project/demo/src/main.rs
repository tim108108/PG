use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let mut guess = String::new();
    let secert_number = rand::thread_rng().gen_range(1..101);

    println!("神秘數字{secert_number}");

    println!("猜數遊戲！");
    println!("輸入：");
    io::stdin().read_line(&mut guess).expect("無法運行");
    println!("你猜的是：{}", guess);
}