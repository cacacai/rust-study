// 猜谜游戏
//io 标准库 输入输出
extern crate rand;//引入依赖cate

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");// 宏定义，
    let secret_number = rand::thread_rng().gen_range(1,101);//主动生成随机数

    println!("the secret number: {}",secret_number);

    println!("Please input a number");
    let mut guess = String::new();// mut 表示可变变量
    io::stdin().read_line(&mut guess).expect("Failed to readLine");//获取控制台std输入，同时进行异常处理


    println!("You guess: {}",guess);//占位符输出
}



/*
获取输入的demo
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
*/