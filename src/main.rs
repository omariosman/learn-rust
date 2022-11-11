use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guess game");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("The secret number: {} ", secret_number);

    loop {
        println!("Enter an input number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is smaller"),
            Ordering::Greater => println!("Your number is greater"),
            Ordering::Equal => {
                println!("Congrats, You Won!!");
                break;
            }
        }
    }
}
