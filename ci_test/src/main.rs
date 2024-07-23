use std::io::stdin;

fn main() {
    // 1. 奇数・偶数の判定
    println!(" 奇数・偶数を判定します");

    println!("入力：");

    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();

    let n = n.trim();
    let n:i32 = n.parse().unwrap();

    if n % 2 == 0 {
        println!("出力：Even");
    } else {
        println!("出力：Odd");
    }
}