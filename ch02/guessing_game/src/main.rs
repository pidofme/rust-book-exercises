use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 另有 String::from()

        io::stdin()
            // read_line 追加数据到字符串，不会覆盖字符串里原有的数据
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // 逗号用来结束 match 分支，最后一个不是必须的
        }; // 注意这里需要分号结尾

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // 注意这里需要分号结尾，其他分支不需要分号的差异
                println!("You win!");
                break; // 这里的分号也是可选的，也许要看一下语法规范
            }
        }
    }
}
