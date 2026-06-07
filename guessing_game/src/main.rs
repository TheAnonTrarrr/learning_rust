use std::io;

fn main() {
    println!("Угадай число!");
    println!("Введите число.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Не удалось прочитать строку...");

    println!("Вы загадали: {guess}");
}
