use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Загаданное число: {secret_number}");

    loop {
        println!("Введите ваше число.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать строку...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Введите число!");
                continue;
            }
        };

        println!("Вы гадаете: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Выше"),
            Ordering::Greater => println!("Ниже"),
            Ordering::Equal => {
                println!("Вы победили!");
                break;
            }
        }
    }
}
