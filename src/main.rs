use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    // 生成ループ
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let random_number: u32 = rng.gen();
        println!("Generated number: {}", random_number);
    }

    // モード設定
    // tuiかなぁ

    // 位置指定
    let mut input = String::new();
    println!("Enter position:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("You entered: {}", input.trim());

    // 入力値
    input.clear();
    println!("Enter value:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("You entered: {}", input.trim());
}
