use rand::Rng;
use std::io;

fn main(){
    println!("A program for learning Rust");
    println!("select a mode: 1.Guess a Number 2.");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    match mode.trim().parse::<i32>(){
        Ok(1) => GuessNumber(),
        _ => println!("Invalid mode")
    }
}

fn GuessNumber(){
    println!("Guess a number between 1 and 100.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
// Or use:
// let mut rng = rand::thread_rng();
// let secret_number = rng.gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number.");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
