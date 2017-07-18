extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜号码！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密号码是：{}", secret_number);

    println!("输入你的号码");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("读取失败");

    let guess: u32 = guess.trim().parse()
        .expect("号码！号码！号码！");

    println!("你猜的号码：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("太小！"),
        Ordering::Greater => println!("太大！"),
        Ordering::Equal   => println!("你赢了！"),
    }
}
